pub mod b0_success_evidence;
pub mod b1_open_object;
pub mod b2_rename_delete;

use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CompletenessDimension, CompletenessState, EvidenceGuarantees,
    ProofCarryingObservation, ProofRequirement, Proposition,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityState {
    Direct,
    DerivedBounded,
    AttemptOnly,
    Unsupported,
    Ambiguous,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropositionCompleteness {
    Complete,
    Incomplete,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReference {
    pub digest: String,
    pub derivation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterRecord {
    pub proposition_id: String,
    pub proof: ProofCarryingObservation,
    pub authority: AuthorityState,
    pub completeness: PropositionCompleteness,
    pub evidence: EvidenceReference,
    pub reason_codes: BTreeSet<String>,
}

impl AdapterRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.proposition_id.trim().is_empty() {
            return Err("proposition_id must not be empty".to_owned());
        }
        if !self.evidence.digest.starts_with("sha256:") || self.evidence.digest.len() != 71 {
            return Err("evidence digest must be a sha256 identity".to_owned());
        }
        match (self.authority, self.completeness) {
            (AuthorityState::Unsupported, PropositionCompleteness::NotApplicable) => {}
            (AuthorityState::Unsupported, _) => {
                return Err("unsupported authority requires not_applicable completeness".to_owned())
            }
            (_, PropositionCompleteness::NotApplicable) => {
                return Err("not_applicable completeness requires unsupported authority".to_owned())
            }
            (
                AuthorityState::Ambiguous | AuthorityState::Lost,
                PropositionCompleteness::Complete,
            ) => return Err("ambiguous/lost authority cannot be complete".to_owned()),
            _ => {}
        }
        if self.authority == AuthorityState::AttemptOnly
            && !matches!(
                self.proof.proposition,
                Proposition::FilePathnameAttemptObserved { .. }
                    | Proposition::FileRenameAttemptObserved { .. }
                    | Proposition::NetworkConnectDestinationAttemptObserved { .. }
            )
        {
            return Err(
                "attempt_only authority requires an explicit attempt proposition".to_owned(),
            );
        }
        if matches!(self.authority, AuthorityState::DerivedBounded)
            && self
                .evidence
                .derivation
                .as_deref()
                .is_none_or(str::is_empty)
        {
            return Err("derived_bounded authority requires derivation identity".to_owned());
        }
        if matches!(
            self.authority,
            AuthorityState::Ambiguous | AuthorityState::Lost
        ) && self.reason_codes.is_empty()
        {
            return Err("ambiguous/lost records require explicit reason code".to_owned());
        }
        Ok(())
    }

    pub fn admissible_for(&self, requirement: &ProofRequirement) -> bool {
        self.validate().is_ok()
            && self.completeness == PropositionCompleteness::Complete
            && (matches!(
                self.authority,
                AuthorityState::Direct | AuthorityState::DerivedBounded
            ) || (self.authority == AuthorityState::AttemptOnly
                && self.proposition_id == "P4.PATH.ACCESS_ATTEMPT"
                && matches!(
                    self.proof.proposition,
                    Proposition::FilePathnameAttemptObserved { .. }
                )))
            && self.proof.satisfies(requirement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonResult {
    BothSatisfy,
    LeftOnly,
    RightOnly,
    Neither,
    DifferentProposition,
}

pub fn compare_under_requirement(
    left: &AdapterRecord,
    right: &AdapterRecord,
    requirement: &ProofRequirement,
) -> ComparisonResult {
    if left.proposition_id != right.proposition_id
        || left.proof.proposition != right.proof.proposition
    {
        return ComparisonResult::DifferentProposition;
    }
    match (
        left.admissible_for(requirement),
        right.admissible_for(requirement),
    ) {
        (true, true) => ComparisonResult::BothSatisfy,
        (true, false) => ComparisonResult::LeftOnly,
        (false, true) => ComparisonResult::RightOnly,
        (false, false) => ComparisonResult::Neither,
    }
}

pub fn unsupported_record(
    proposition_id: impl Into<String>,
    proposition: Proposition,
    backend_profile: BackendSemanticProfile,
    evidence_digest: impl Into<String>,
    reason: impl Into<String>,
) -> AdapterRecord {
    let reason = reason.into();
    let mut proof =
        ProofCarryingObservation::new(proposition, EvidenceGuarantees::default(), backend_profile);
    proof.completeness.insert(
        CompletenessDimension::Capability,
        CompletenessState::Unsupported {
            reason_code: reason.clone(),
        },
    );
    AdapterRecord {
        proposition_id: proposition_id.into(),
        proof,
        authority: AuthorityState::Unsupported,
        completeness: PropositionCompleteness::NotApplicable,
        evidence: EvidenceReference {
            digest: evidence_digest.into(),
            derivation: None,
        },
        reason_codes: BTreeSet::from([reason]),
    }
}

pub fn completeness_map(
    entries: impl IntoIterator<Item = (CompletenessDimension, CompletenessState)>,
) -> BTreeMap<CompletenessDimension, CompletenessState> {
    entries.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_model::canonical::{
        CanonicalExecutable, CanonicalPath, PathClass, PathResolution,
    };
    use execsurface_model::semantics_v3::{
        CausalBinding, IdentityBasis, ObservationPoint, TemporalBinding,
    };
    use execsurface_model::FileOperation;

    fn digest(byte: char) -> String {
        format!("sha256:{}", byte.to_string().repeat(64))
    }

    fn path(value: &str) -> CanonicalPath {
        CanonicalPath {
            value: value.to_owned(),
            class: PathClass::Workspace,
            resolution: PathResolution::Lexical,
        }
    }

    fn exe(value: &str, family: &str) -> CanonicalExecutable {
        CanonicalExecutable {
            path: CanonicalPath {
                value: value.to_owned(),
                class: PathClass::System,
                resolution: PathResolution::Lexical,
            },
            family: family.to_owned(),
        }
    }

    fn pathname_prop(actor: &str) -> Proposition {
        Proposition::FilePathnameAttemptObserved {
            actor: Some(exe(actor, actor.rsplit('/').next().unwrap_or(actor))),
            execution_chain: vec![
                exe("/bin/bash", "bash"),
                exe(actor, actor.rsplit('/').next().unwrap_or(actor)),
            ],
            operation: FileOperation::Open,
            target: path("$WORKSPACE/input.txt"),
            open_intent: None,
        }
    }

    fn lexical_guarantees() -> EvidenceGuarantees {
        EvidenceGuarantees {
            observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
            identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
            temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
            causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
        }
    }

    fn object_guarantees() -> EvidenceGuarantees {
        EvidenceGuarantees {
            observation_points: BTreeSet::from([ObservationPoint::SyscallResultPostOperation]),
            identity_bases: BTreeSet::from([IdentityBasis::RuntimeFdPathCorrelated]),
            temporal_bindings: BTreeSet::from([TemporalBinding::SuccessfulOperationResult]),
            causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
        }
    }

    fn profile(name: &str) -> BackendSemanticProfile {
        BackendSemanticProfile {
            name: name.to_owned(),
            semantic_profile_version: 1,
        }
    }

    fn direct_record(
        id: &str,
        proposition: Proposition,
        guarantees: EvidenceGuarantees,
        backend: &str,
        digest_char: char,
    ) -> AdapterRecord {
        let mut proof = ProofCarryingObservation::new(proposition, guarantees, profile(backend));
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
        AdapterRecord {
            proposition_id: id.to_owned(),
            proof,
            authority: AuthorityState::Direct,
            completeness: PropositionCompleteness::Complete,
            evidence: EvidenceReference {
                digest: digest(digest_char),
                derivation: None,
            },
            reason_codes: BTreeSet::new(),
        }
    }

    #[test]
    fn a0_unsupported_cannot_be_complete() {
        let mut record = unsupported_record(
            "P4.NET.CONNECT_DESTINATION",
            pathname_prop("/usr/bin/curl"),
            profile("research-ebpf"),
            digest('a'),
            "unsupported_network_family",
        );
        record.completeness = PropositionCompleteness::Complete;
        assert!(record.validate().is_err());
    }

    #[test]
    fn a0_lost_or_ambiguous_cannot_be_complete() {
        for authority in [AuthorityState::Ambiguous, AuthorityState::Lost] {
            let mut record = direct_record(
                "P4.FILE.OPEN_OBJECT",
                pathname_prop("/usr/bin/cat"),
                lexical_guarantees(),
                "ptrace",
                'b',
            );
            record.authority = authority;
            record.reason_codes.insert("controlled_loss".to_owned());
            assert!(record.validate().is_err());
        }
    }

    #[test]
    fn a0_path_attempt_cannot_satisfy_object_identity_requirement() {
        let proposition = pathname_prop("/usr/bin/cat");
        let record = direct_record(
            "P4.PATH.ACCESS_ATTEMPT",
            proposition.clone(),
            lexical_guarantees(),
            "ptrace",
            'c',
        );
        let requirement = ProofRequirement::for_proposition(
            proposition,
            object_guarantees(),
            BTreeSet::from([
                CompletenessDimension::SessionScope,
                CompletenessDimension::Capability,
            ]),
        );
        assert!(!record.admissible_for(&requirement));
    }

    #[test]
    fn a0_backend_name_never_upgrades_weak_evidence() {
        let proposition = pathname_prop("/usr/bin/cat");
        let requirement = ProofRequirement::for_proposition(
            proposition.clone(),
            object_guarantees(),
            BTreeSet::from([CompletenessDimension::SessionScope]),
        );
        for backend in ["ptrace", "tetragon", "bpf-lsm", "trusted-super-backend"] {
            let record = direct_record(
                "P4.FILE.OPEN_OBJECT",
                proposition.clone(),
                lexical_guarantees(),
                backend,
                'd',
            );
            assert!(!record.admissible_for(&requirement));
        }
    }

    #[test]
    fn a1_same_proposition_can_have_asymmetric_authority() {
        let proposition = pathname_prop("/usr/bin/cat");
        let weak = direct_record(
            "P4.FILE.OPEN_OBJECT",
            proposition.clone(),
            lexical_guarantees(),
            "ptrace-weak-fixture",
            'e',
        );
        let strong = direct_record(
            "P4.FILE.OPEN_OBJECT",
            proposition.clone(),
            object_guarantees(),
            "kernel-object-fixture",
            'f',
        );
        let requirement = ProofRequirement::for_proposition(
            proposition,
            object_guarantees(),
            BTreeSet::from([
                CompletenessDimension::SessionScope,
                CompletenessDimension::Capability,
            ]),
        );
        assert_eq!(
            compare_under_requirement(&weak, &strong, &requirement),
            ComparisonResult::RightOnly
        );
    }

    #[test]
    fn a1_different_propositions_are_never_declared_equivalent() {
        let left = direct_record(
            "P4.PATH.ACCESS_ATTEMPT",
            pathname_prop("/usr/bin/cat"),
            lexical_guarantees(),
            "ptrace",
            '1',
        );
        let right = direct_record(
            "P4.PATH.ACCESS_ATTEMPT",
            pathname_prop("/usr/bin/gcc"),
            lexical_guarantees(),
            "ptrace",
            '2',
        );
        assert_eq!(
            compare_under_requirement(&left, &right, &ProofRequirement::default()),
            ComparisonResult::DifferentProposition
        );
    }

    #[test]
    fn a1_derived_authority_requires_named_derivation() {
        let mut record = direct_record(
            "P4.CAUSAL.EXEC_LINEAGE",
            pathname_prop("/usr/bin/cat"),
            lexical_guarantees(),
            "ptrace",
            '3',
        );
        record.authority = AuthorityState::DerivedBounded;
        assert!(record.validate().is_err());
        record.evidence.derivation = Some("lineage-state-machine-v1".to_owned());
        assert!(record.validate().is_ok());
    }

    #[test]
    fn a1_deterministic_serialization_is_stable() {
        let mut first = unsupported_record(
            "P4.FD.IO_ATTRIBUTION",
            pathname_prop("/usr/bin/cat"),
            profile("research-ebpf"),
            digest('4'),
            "unsupported_fd_io",
        );
        first.reason_codes.insert("zeta".to_owned());
        first.reason_codes.insert("alpha".to_owned());
        let mut second = first.clone();
        second.reason_codes = BTreeSet::from([
            "alpha".to_owned(),
            "unsupported_fd_io".to_owned(),
            "zeta".to_owned(),
        ]);
        assert_eq!(
            serde_json::to_vec(&first).expect("serialize first"),
            serde_json::to_vec(&second).expect("serialize second")
        );
    }
}
