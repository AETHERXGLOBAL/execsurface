use std::collections::BTreeSet;
use std::fmt;

use execsurface_model::canonical::{CanonicalEffect, CanonicalExecutable, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, FdTableRelationState, IdentityBasis, ObservationPoint,
    ProofCarryingObservation, Proposition, TemporalBinding,
};
use execsurface_model::{FileOperation, Observation, RawEventKind, SpawnMechanism};
use execsurface_normalize::{canonicalize, NormalizationConfig};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    completeness_map, AdapterRecord, AuthorityState, EvidenceReference, PropositionCompleteness,
};

const BACKEND_PROFILE: &str = "linux-ptrace-raw-v2-adapter";
const BACKEND_PROFILE_VERSION: u32 = 1;
const LINEAGE_DERIVATION: &str = "canonical-exec-lineage-v2";
const FORK_FD_DERIVATION: &str = "linux-fork-vfork-fd-table-copy-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PtraceMapping {
    pub evidence_digest: String,
    pub capability_gaps: BTreeSet<String>,
    pub records: Vec<AdapterRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtraceMappingError {
    Serialize(String),
    Normalize(String),
    Record(String),
}

impl fmt::Display for PtraceMappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PtraceMappingError {}

pub fn map_ptrace_v2(
    observation: &Observation,
    normalization: &NormalizationConfig,
) -> Result<PtraceMapping, PtraceMappingError> {
    let evidence_digest = observation_digest(observation)?;
    let warning_codes = observation
        .warnings
        .iter()
        .map(|warning| warning.code.clone())
        .collect::<BTreeSet<_>>();

    let mut records = vec![health_record(
        observation.complete,
        warning_codes.clone(),
        &evidence_digest,
    )?];
    let mut capability_gaps = BTreeSet::from(["P4.FILE.OPEN_OBJECT".to_owned()]);

    if !observation.complete || !warning_codes.is_empty() {
        capability_gaps.insert("P4.EFFECT_MAPPING_BLOCKED_BY_OBSERVER_HEALTH".to_owned());
        sort_records(&mut records)?;
        return Ok(PtraceMapping {
            evidence_digest,
            capability_gaps,
            records,
        });
    }

    let surface = canonicalize(observation, normalization)
        .map_err(|error| PtraceMappingError::Normalize(error.to_string()))?;
    let raw_clone_seen = observation.events.iter().any(|event| {
        matches!(
            event.kind,
            RawEventKind::ProcessSpawn {
                mechanism: SpawnMechanism::Clone,
                ..
            }
        )
    });

    let mut lineage_seen = BTreeSet::<Vec<CanonicalExecutable>>::new();

    for effect in surface.effects {
        match effect {
            CanonicalEffect::ProcessSpawn { actor, mechanism } => {
                records.push(direct_record(
                    "P4.PROC.CREATE_RELATION",
                    Proposition::ProcessChildCreated {
                        actor: actor.clone(),
                        mechanism,
                    },
                    lifecycle_guarantees(),
                    lifecycle_completeness(),
                    &evidence_digest,
                )?);
                records.push(fd_table_record(actor, mechanism, &evidence_digest)?);
                if mechanism == SpawnMechanism::Clone {
                    capability_gaps.insert("P4.FDTABLE.RELATION.EXACT_CLONE".to_owned());
                }
            }
            CanonicalEffect::ProcessExec { from, executable } => {
                records.push(direct_record(
                    "P4.EXEC.SUCCESS",
                    Proposition::ProcessExecSucceeded { from, executable },
                    exec_guarantees(),
                    lifecycle_completeness(),
                    &evidence_digest,
                )?);
            }
            CanonicalEffect::FilePathAccess {
                actor,
                execution_chain,
                operation,
                target,
                open_intent,
            } => {
                add_lineage(
                    &mut records,
                    &mut lineage_seen,
                    &execution_chain,
                    &evidence_digest,
                )?;
                if matches!(operation, FileOperation::Read | FileOperation::Write)
                    && target.resolution == PathResolution::KernelFdResolved
                {
                    records.push(fd_effect_record(
                        actor,
                        execution_chain,
                        operation,
                        target,
                        raw_clone_seen,
                        &evidence_digest,
                    )?);
                } else {
                    let proposition_id = if operation == FileOperation::Delete {
                        "P4.FILE.RENAME_DELETE"
                    } else {
                        "P4.PATH.ACCESS_ATTEMPT"
                    };
                    records.push(attempt_record(
                        proposition_id,
                        Proposition::FilePathnameAttemptObserved {
                            actor,
                            execution_chain,
                            operation,
                            target,
                            open_intent,
                        },
                        pathname_guarantees(),
                        &evidence_digest,
                    )?);
                }
            }
            CanonicalEffect::FileRename {
                actor,
                execution_chain,
                from,
                to,
            } => {
                add_lineage(
                    &mut records,
                    &mut lineage_seen,
                    &execution_chain,
                    &evidence_digest,
                )?;
                records.push(attempt_record(
                    "P4.FILE.RENAME_DELETE",
                    Proposition::FileRenameAttemptObserved {
                        actor,
                        execution_chain,
                        from,
                        to,
                    },
                    pathname_guarantees(),
                    &evidence_digest,
                )?);
            }
            CanonicalEffect::NetworkConnectAttempt {
                actor,
                execution_chain,
                endpoint,
            } => {
                add_lineage(
                    &mut records,
                    &mut lineage_seen,
                    &execution_chain,
                    &evidence_digest,
                )?;
                records.push(attempt_record(
                    "P4.NET.CONNECT_DESTINATION",
                    Proposition::NetworkConnectDestinationAttemptObserved {
                        actor,
                        execution_chain,
                        endpoint,
                    },
                    network_attempt_guarantees(),
                    &evidence_digest,
                )?);
            }
        }
    }

    sort_records(&mut records)?;
    Ok(PtraceMapping {
        evidence_digest,
        capability_gaps,
        records,
    })
}

fn profile() -> BackendSemanticProfile {
    BackendSemanticProfile {
        name: BACKEND_PROFILE.to_owned(),
        semantic_profile_version: BACKEND_PROFILE_VERSION,
    }
}

fn observation_digest(observation: &Observation) -> Result<String, PtraceMappingError> {
    let bytes = serde_json::to_vec(observation)
        .map_err(|error| PtraceMappingError::Serialize(error.to_string()))?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

fn direct_record(
    proposition_id: &str,
    proposition: Proposition,
    guarantees: EvidenceGuarantees,
    completeness: Vec<(CompletenessDimension, CompletenessState)>,
    digest: &str,
) -> Result<AdapterRecord, PtraceMappingError> {
    let mut proof = ProofCarryingObservation::new(proposition, guarantees, profile());
    proof.completeness = completeness_map(completeness);
    let record = AdapterRecord {
        proposition_id: proposition_id.to_owned(),
        proof,
        authority: AuthorityState::Direct,
        completeness: PropositionCompleteness::Complete,
        evidence: EvidenceReference {
            digest: digest.to_owned(),
            derivation: None,
        },
        reason_codes: BTreeSet::new(),
    };
    record.validate().map_err(PtraceMappingError::Record)?;
    Ok(record)
}

fn attempt_record(
    proposition_id: &str,
    proposition: Proposition,
    guarantees: EvidenceGuarantees,
    digest: &str,
) -> Result<AdapterRecord, PtraceMappingError> {
    let mut proof = ProofCarryingObservation::new(proposition, guarantees, profile());
    proof.completeness = completeness_map([
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::Capability,
            CompletenessState::Complete,
        ),
    ]);
    let record = AdapterRecord {
        proposition_id: proposition_id.to_owned(),
        proof,
        authority: AuthorityState::AttemptOnly,
        completeness: PropositionCompleteness::Complete,
        evidence: EvidenceReference {
            digest: digest.to_owned(),
            derivation: None,
        },
        reason_codes: BTreeSet::new(),
    };
    record.validate().map_err(PtraceMappingError::Record)?;
    Ok(record)
}

fn health_record(
    complete: bool,
    warning_codes: BTreeSet<String>,
    digest: &str,
) -> Result<AdapterRecord, PtraceMappingError> {
    let proposition = Proposition::ObserverHealthObserved {
        complete,
        warning_codes: warning_codes.clone(),
    };
    let mut proof =
        ProofCarryingObservation::new(proposition, EvidenceGuarantees::default(), profile());

    if complete && warning_codes.is_empty() {
        proof.completeness = completeness_map([
            (
                CompletenessDimension::SessionScope,
                CompletenessState::Complete,
            ),
            (
                CompletenessDimension::Capability,
                CompletenessState::Complete,
            ),
        ]);
        let record = AdapterRecord {
            proposition_id: "P4.OBSERVER.HEALTH_LOSS".to_owned(),
            proof,
            authority: AuthorityState::Direct,
            completeness: PropositionCompleteness::Complete,
            evidence: EvidenceReference {
                digest: digest.to_owned(),
                derivation: None,
            },
            reason_codes: BTreeSet::new(),
        };
        record.validate().map_err(PtraceMappingError::Record)?;
        return Ok(record);
    }

    let mut reasons = warning_codes;
    if !complete {
        reasons.insert("observation_incomplete".to_owned());
    }
    if reasons.is_empty() {
        reasons.insert("warning_bearing_observation".to_owned());
    }
    let reason = reasons
        .iter()
        .next()
        .cloned()
        .unwrap_or_else(|| "observer_health_unknown".to_owned());
    proof.completeness = completeness_map([(
        CompletenessDimension::SessionScope,
        CompletenessState::Incomplete {
            reason_code: reason.clone(),
        },
    )]);
    proof.ambiguity_codes = reasons.clone();
    let record = AdapterRecord {
        proposition_id: "P4.OBSERVER.HEALTH_LOSS".to_owned(),
        proof,
        authority: AuthorityState::Lost,
        completeness: PropositionCompleteness::Incomplete,
        evidence: EvidenceReference {
            digest: digest.to_owned(),
            derivation: None,
        },
        reason_codes: reasons,
    };
    record.validate().map_err(PtraceMappingError::Record)?;
    Ok(record)
}

fn fd_table_record(
    actor: Option<CanonicalExecutable>,
    mechanism: SpawnMechanism,
    digest: &str,
) -> Result<AdapterRecord, PtraceMappingError> {
    match mechanism {
        SpawnMechanism::Fork | SpawnMechanism::Vfork => {
            let proposition = Proposition::FdTableRelationObserved {
                actor,
                mechanism,
                relation: FdTableRelationState::IndependentCopy,
            };
            let mut proof =
                ProofCarryingObservation::new(proposition, lifecycle_guarantees(), profile());
            proof.completeness = completeness_map([
                (
                    CompletenessDimension::SessionScope,
                    CompletenessState::Complete,
                ),
                (
                    CompletenessDimension::FdTableRelation,
                    CompletenessState::Complete,
                ),
            ]);
            let record = AdapterRecord {
                proposition_id: "P4.FDTABLE.RELATION".to_owned(),
                proof,
                authority: AuthorityState::DerivedBounded,
                completeness: PropositionCompleteness::Complete,
                evidence: EvidenceReference {
                    digest: digest.to_owned(),
                    derivation: Some(FORK_FD_DERIVATION.to_owned()),
                },
                reason_codes: BTreeSet::new(),
            };
            record.validate().map_err(PtraceMappingError::Record)?;
            Ok(record)
        }
        SpawnMechanism::Clone => {
            let reason = "raw_v2_clone_flags_not_retained".to_owned();
            let proposition = Proposition::FdTableRelationObserved {
                actor,
                mechanism,
                relation: FdTableRelationState::Unknown,
            };
            let mut proof =
                ProofCarryingObservation::new(proposition, lifecycle_guarantees(), profile());
            proof.completeness = completeness_map([(
                CompletenessDimension::FdTableRelation,
                CompletenessState::Ambiguous {
                    reason_code: reason.clone(),
                },
            )]);
            proof.ambiguity_codes.insert(reason.clone());
            let record = AdapterRecord {
                proposition_id: "P4.FDTABLE.RELATION".to_owned(),
                proof,
                authority: AuthorityState::Ambiguous,
                completeness: PropositionCompleteness::Incomplete,
                evidence: EvidenceReference {
                    digest: digest.to_owned(),
                    derivation: None,
                },
                reason_codes: BTreeSet::from([reason]),
            };
            record.validate().map_err(PtraceMappingError::Record)?;
            Ok(record)
        }
    }
}

fn fd_effect_record(
    actor: Option<CanonicalExecutable>,
    execution_chain: Vec<CanonicalExecutable>,
    operation: FileOperation,
    target: execsurface_model::canonical::CanonicalPath,
    raw_clone_seen: bool,
    digest: &str,
) -> Result<AdapterRecord, PtraceMappingError> {
    let proposition = Proposition::FileFdEffectObserved {
        actor,
        execution_chain,
        operation,
        target,
    };
    let mut proof = ProofCarryingObservation::new(proposition, fd_effect_guarantees(), profile());
    if raw_clone_seen {
        let reason = "raw_v2_clone_flags_not_retained".to_owned();
        proof.completeness = completeness_map([
            (
                CompletenessDimension::SessionScope,
                CompletenessState::Complete,
            ),
            (
                CompletenessDimension::FdTableRelation,
                CompletenessState::Ambiguous {
                    reason_code: reason.clone(),
                },
            ),
            (
                CompletenessDimension::ObjectIdentity,
                CompletenessState::Incomplete {
                    reason_code: reason.clone(),
                },
            ),
        ]);
        proof.ambiguity_codes.insert(reason.clone());
        let record = AdapterRecord {
            proposition_id: "P4.FD.IO_ATTRIBUTION".to_owned(),
            proof,
            authority: AuthorityState::Ambiguous,
            completeness: PropositionCompleteness::Incomplete,
            evidence: EvidenceReference {
                digest: digest.to_owned(),
                derivation: None,
            },
            reason_codes: BTreeSet::from([reason]),
        };
        record.validate().map_err(PtraceMappingError::Record)?;
        return Ok(record);
    }

    proof.completeness = completeness_map([
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::Capability,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::ObjectIdentity,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::FdTableRelation,
            CompletenessState::Complete,
        ),
    ]);
    let record = AdapterRecord {
        proposition_id: "P4.FD.IO_ATTRIBUTION".to_owned(),
        proof,
        authority: AuthorityState::Direct,
        completeness: PropositionCompleteness::Complete,
        evidence: EvidenceReference {
            digest: digest.to_owned(),
            derivation: None,
        },
        reason_codes: BTreeSet::new(),
    };
    record.validate().map_err(PtraceMappingError::Record)?;
    Ok(record)
}

fn add_lineage(
    records: &mut Vec<AdapterRecord>,
    seen: &mut BTreeSet<Vec<CanonicalExecutable>>,
    chain: &[CanonicalExecutable],
    digest: &str,
) -> Result<(), PtraceMappingError> {
    if chain.is_empty() || !seen.insert(chain.to_vec()) {
        return Ok(());
    }
    let proposition = Proposition::CausalExecLineageObserved {
        execution_chain: chain.to_vec(),
    };
    let guarantees = EvidenceGuarantees {
        causal_bindings: BTreeSet::from([CausalBinding::LineageDerived]),
        ..EvidenceGuarantees::default()
    };
    let mut proof = ProofCarryingObservation::new(proposition, guarantees, profile());
    proof.completeness = completeness_map([
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::CausalLineage,
            CompletenessState::Complete,
        ),
    ]);
    let record = AdapterRecord {
        proposition_id: "P4.CAUSAL.EXEC_LINEAGE".to_owned(),
        proof,
        authority: AuthorityState::DerivedBounded,
        completeness: PropositionCompleteness::Complete,
        evidence: EvidenceReference {
            digest: digest.to_owned(),
            derivation: Some(LINEAGE_DERIVATION.to_owned()),
        },
        reason_codes: BTreeSet::new(),
    };
    record.validate().map_err(PtraceMappingError::Record)?;
    records.push(record);
    Ok(())
}

fn lifecycle_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::PtraceLifecycleEvent]),
        identity_bases: BTreeSet::from([IdentityBasis::None]),
        temporal_bindings: BTreeSet::from([TemporalBinding::LifecycleTransition]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn exec_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([
            ObservationPoint::UserspaceArgumentPreKernel,
            ObservationPoint::PtraceLifecycleEvent,
        ]),
        identity_bases: BTreeSet::from([IdentityBasis::TraceTimeDirfdResolvedArgument]),
        temporal_bindings: BTreeSet::from([
            TemporalBinding::PreOperationIntent,
            TemporalBinding::LifecycleTransition,
        ]),
        causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
    }
}

fn pathname_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::TraceTimeDirfdResolvedArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn network_attempt_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::SocketAddressArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn fd_effect_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([
            ObservationPoint::SyscallResultPostOperation,
            ObservationPoint::DerivedRuntimeFdState,
        ]),
        identity_bases: BTreeSet::from([IdentityBasis::RuntimeFdPathCorrelated]),
        temporal_bindings: BTreeSet::from([
            TemporalBinding::SuccessfulOperationResult,
            TemporalBinding::PostOperationDerivedState,
        ]),
        causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
    }
}

fn lifecycle_completeness() -> Vec<(CompletenessDimension, CompletenessState)> {
    vec![
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::Lifecycle,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::Capability,
            CompletenessState::Complete,
        ),
    ]
}

fn sort_records(records: &mut [AdapterRecord]) -> Result<(), PtraceMappingError> {
    let mut keyed = records
        .iter()
        .cloned()
        .map(|record| {
            serde_json::to_string(&record)
                .map(|key| (key, record))
                .map_err(|error| PtraceMappingError::Serialize(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    keyed.sort_by(|left, right| left.0.cmp(&right.0));
    for (target, (_, record)) in records.iter_mut().zip(keyed) {
        *target = record;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_model::{
        BackendMetadata, CommandOutcome, NetworkEndpoint, ObserverWarning, RawEvent,
        RAW_OBSERVATION_SCHEMA_VERSION,
    };

    fn backend() -> BackendMetadata {
        BackendMetadata {
            name: "linux-ptrace-metadata-v2".to_owned(),
            platform: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
            capabilities: vec!["test".to_owned()],
            limitations: vec![],
        }
    }

    fn observation(events: Vec<RawEvent>) -> Observation {
        Observation {
            schema_version: RAW_OBSERVATION_SCHEMA_VERSION,
            backend: backend(),
            complete: true,
            outcome: CommandOutcome {
                exit_code: Some(0),
                signal: None,
            },
            events,
            warnings: vec![],
        }
    }

    fn event(sequence: u64, tid: i32, kind: RawEventKind) -> RawEvent {
        RawEvent {
            sequence,
            tid,
            kind,
        }
    }

    fn mapped(events: Vec<RawEvent>) -> PtraceMapping {
        map_ptrace_v2(&observation(events), &NormalizationConfig::default()).expect("map ptrace")
    }

    #[test]
    fn a1_2_open_path_attempt_never_becomes_successful_open_object() {
        let mapping = mapped(vec![event(
            1,
            10,
            RawEventKind::FilePathAccess {
                operation: FileOperation::Open,
                path: "/tmp/input".to_owned(),
                flags: Some(0),
            },
        )]);
        assert!(mapping.capability_gaps.contains("P4.FILE.OPEN_OBJECT"));
        assert!(mapping.records.iter().any(|record| {
            record.proposition_id == "P4.PATH.ACCESS_ATTEMPT"
                && record.authority == AuthorityState::AttemptOnly
        }));
        assert!(!mapping
            .records
            .iter()
            .any(|record| record.proposition_id == "P4.FILE.OPEN_OBJECT"));
    }

    #[test]
    fn a1_2_fd_effect_uses_runtime_fd_identity_after_positive_io() {
        let mapping = mapped(vec![event(
            1,
            10,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Read,
                fd: 3,
                path: "/tmp/input".to_owned(),
            },
        )]);
        let record = mapping
            .records
            .iter()
            .find(|record| record.proposition_id == "P4.FD.IO_ATTRIBUTION")
            .expect("fd record");
        assert_eq!(record.authority, AuthorityState::Direct);
        assert_eq!(record.completeness, PropositionCompleteness::Complete);
        assert!(record
            .proof
            .guarantees
            .identity_bases
            .contains(&IdentityBasis::RuntimeFdPathCorrelated));
    }

    #[test]
    fn a1_2_clone_without_flags_makes_fd_relation_and_fd_io_ambiguous() {
        let mapping = mapped(vec![
            event(
                1,
                10,
                RawEventKind::ProcessSpawn {
                    child_tid: 11,
                    mechanism: SpawnMechanism::Clone,
                },
            ),
            event(
                2,
                10,
                RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Write,
                    fd: 4,
                    path: "/tmp/output".to_owned(),
                },
            ),
        ]);
        let relation = mapping
            .records
            .iter()
            .find(|record| record.proposition_id == "P4.FDTABLE.RELATION")
            .expect("fd-table relation");
        assert_eq!(relation.authority, AuthorityState::Ambiguous);
        assert_eq!(relation.completeness, PropositionCompleteness::Incomplete);
        let io = mapping
            .records
            .iter()
            .find(|record| record.proposition_id == "P4.FD.IO_ATTRIBUTION")
            .expect("fd io");
        assert_eq!(io.authority, AuthorityState::Ambiguous);
        assert_eq!(io.completeness, PropositionCompleteness::Incomplete);
    }

    #[test]
    fn a1_2_fork_fd_relation_is_bounded_derivation_not_direct() {
        let mapping = mapped(vec![event(
            1,
            10,
            RawEventKind::ProcessSpawn {
                child_tid: 11,
                mechanism: SpawnMechanism::Fork,
            },
        )]);
        let relation = mapping
            .records
            .iter()
            .find(|record| record.proposition_id == "P4.FDTABLE.RELATION")
            .expect("fd-table relation");
        assert_eq!(relation.authority, AuthorityState::DerivedBounded);
        assert_eq!(
            relation.evidence.derivation.as_deref(),
            Some(FORK_FD_DERIVATION)
        );
    }

    #[test]
    fn a1_2_rename_and_connect_remain_attempt_only() {
        let mapping = mapped(vec![
            event(
                1,
                10,
                RawEventKind::FileRename {
                    from: "/tmp/a".to_owned(),
                    to: "/tmp/b".to_owned(),
                },
            ),
            event(
                2,
                10,
                RawEventKind::NetworkConnectAttempt {
                    endpoint: NetworkEndpoint::Inet {
                        ip: "127.0.0.1".to_owned(),
                        port: 443,
                    },
                },
            ),
        ]);
        for id in ["P4.FILE.RENAME_DELETE", "P4.NET.CONNECT_DESTINATION"] {
            let record = mapping
                .records
                .iter()
                .find(|record| record.proposition_id == id)
                .expect("attempt record");
            assert_eq!(record.authority, AuthorityState::AttemptOnly);
        }
    }

    #[test]
    fn a1_2_confirmed_exec_maps_to_success() {
        let mapping = mapped(vec![event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/bin/true".to_owned(),
            },
        )]);
        let record = mapping
            .records
            .iter()
            .find(|record| record.proposition_id == "P4.EXEC.SUCCESS")
            .expect("exec success");
        assert_eq!(record.authority, AuthorityState::Direct);
        assert!(matches!(
            record.proof.proposition,
            Proposition::ProcessExecSucceeded { .. }
        ));
    }

    #[test]
    fn a1_2_incomplete_observation_fails_closed_before_effect_mapping() {
        let mut input = observation(vec![event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/bin/true".to_owned(),
            },
        )]);
        input.complete = false;
        let mapping =
            map_ptrace_v2(&input, &NormalizationConfig::default()).expect("map incomplete");
        assert_eq!(mapping.records.len(), 1);
        assert_eq!(mapping.records[0].authority, AuthorityState::Lost);
        assert_eq!(mapping.records[0].proposition_id, "P4.OBSERVER.HEALTH_LOSS");
    }

    #[test]
    fn a1_2_warning_bearing_observation_fails_closed_before_effect_mapping() {
        let mut input = observation(vec![event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/bin/true".to_owned(),
            },
        )]);
        input.warnings.push(ObserverWarning {
            code: "controlled_warning".to_owned(),
            tid: Some(10),
            message: "test".to_owned(),
        });
        let mapping = map_ptrace_v2(&input, &NormalizationConfig::default()).expect("map warning");
        assert_eq!(mapping.records.len(), 1);
        assert_eq!(mapping.records[0].authority, AuthorityState::Lost);
    }

    #[test]
    fn a1_2_execution_chain_is_explicit_bounded_derivation() {
        let mapping = mapped(vec![
            event(
                1,
                10,
                RawEventKind::ProcessExec {
                    path: "/bin/bash".to_owned(),
                },
            ),
            event(
                2,
                10,
                RawEventKind::ProcessExec {
                    path: "/usr/bin/cat".to_owned(),
                },
            ),
            event(
                3,
                10,
                RawEventKind::FilePathAccess {
                    operation: FileOperation::Read,
                    path: "/tmp/input".to_owned(),
                    flags: None,
                },
            ),
        ]);
        let lineage = mapping
            .records
            .iter()
            .find(|record| record.proposition_id == "P4.CAUSAL.EXEC_LINEAGE")
            .expect("lineage record");
        assert_eq!(lineage.authority, AuthorityState::DerivedBounded);
        assert_eq!(
            lineage.evidence.derivation.as_deref(),
            Some(LINEAGE_DERIVATION)
        );
    }

    #[test]
    fn a1_2_mapping_and_digest_are_deterministic() {
        let input = observation(vec![
            event(
                1,
                10,
                RawEventKind::ProcessExec {
                    path: "/bin/true".to_owned(),
                },
            ),
            event(
                2,
                10,
                RawEventKind::FilePathAccess {
                    operation: FileOperation::Open,
                    path: "/tmp/input".to_owned(),
                    flags: Some(0),
                },
            ),
        ]);
        let first = map_ptrace_v2(&input, &NormalizationConfig::default()).expect("first");
        let second = map_ptrace_v2(&input, &NormalizationConfig::default()).expect("second");
        assert_eq!(first, second);
        assert_eq!(
            serde_json::to_vec(&first).expect("serialize first"),
            serde_json::to_vec(&second).expect("serialize second")
        );
    }
}
