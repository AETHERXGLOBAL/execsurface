//! Research-only Semantics v3 prototype.
//!
//! This module is intentionally disconnected from public alpha.4 learn/check,
//! baseline and verdict paths. It prototypes proposition-scoped proof semantics
//! before any public schema integration is considered.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::canonical::{CanonicalExecutable, CanonicalNetworkEndpoint, CanonicalPath, OpenIntent};
use crate::{FileOperation, SpawnMechanism};

pub const SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FdTableRelationState {
    Shared,
    IndependentCopy,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "proposition_kind", rename_all = "snake_case")]
pub enum Proposition {
    ProcessChildCreated {
        actor: Option<CanonicalExecutable>,
        mechanism: SpawnMechanism,
    },
    ProcessExecSucceeded {
        from: Option<CanonicalExecutable>,
        executable: CanonicalExecutable,
    },
    CausalExecLineageObserved {
        execution_chain: Vec<CanonicalExecutable>,
    },
    FilePathnameAttemptObserved {
        actor: Option<CanonicalExecutable>,
        execution_chain: Vec<CanonicalExecutable>,
        operation: FileOperation,
        target: CanonicalPath,
        open_intent: Option<OpenIntent>,
    },
    FileOpenObjectObserved {
        actor: Option<CanonicalExecutable>,
        execution_chain: Vec<CanonicalExecutable>,
        target: CanonicalPath,
    },
    FileFdEffectObserved {
        actor: Option<CanonicalExecutable>,
        execution_chain: Vec<CanonicalExecutable>,
        operation: FileOperation,
        target: CanonicalPath,
    },
    FileRenameAttemptObserved {
        actor: Option<CanonicalExecutable>,
        execution_chain: Vec<CanonicalExecutable>,
        from: CanonicalPath,
        to: CanonicalPath,
    },
    NetworkConnectDestinationAttemptObserved {
        actor: Option<CanonicalExecutable>,
        execution_chain: Vec<CanonicalExecutable>,
        endpoint: CanonicalNetworkEndpoint,
    },
    ObserverHealthObserved {
        complete: bool,
        warning_codes: BTreeSet<String>,
    },
    FdTableRelationObserved {
        actor: Option<CanonicalExecutable>,
        mechanism: SpawnMechanism,
        relation: FdTableRelationState,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationPoint {
    UserspaceArgumentPreKernel,
    PtraceLifecycleEvent,
    SyscallResultPostOperation,
    DerivedRuntimeFdState,
    KernelSecurityHook,
    KernelTracepoint,
    ImportedAttestedTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityBasis {
    None,
    LexicalArgument,
    TraceTimeDirfdResolvedArgument,
    RuntimeFdPathCorrelated,
    KernelObjectGrounded,
    SocketAddressArgument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporalBinding {
    PreOperationIntent,
    SuccessfulOperationResult,
    PostOperationDerivedState,
    KernelDecisionPoint,
    LifecycleTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CausalBinding {
    DirectEvent,
    StateMachineCorrelated,
    LineageDerived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EvidenceGuarantees {
    pub observation_points: BTreeSet<ObservationPoint>,
    pub identity_bases: BTreeSet<IdentityBasis>,
    pub temporal_bindings: BTreeSet<TemporalBinding>,
    pub causal_bindings: BTreeSet<CausalBinding>,
}

impl EvidenceGuarantees {
    pub fn entails(&self, required: &Self) -> bool {
        required
            .observation_points
            .is_subset(&self.observation_points)
            && required.identity_bases.is_subset(&self.identity_bases)
            && required
                .temporal_bindings
                .is_subset(&self.temporal_bindings)
            && required.causal_bindings.is_subset(&self.causal_bindings)
    }

    fn is_empty(&self) -> bool {
        self.observation_points.is_empty()
            && self.identity_bases.is_empty()
            && self.temporal_bindings.is_empty()
            && self.causal_bindings.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessDimension {
    SessionScope,
    Lifecycle,
    Transport,
    ResourceBudget,
    Capability,
    ObjectIdentity,
    FdTableRelation,
    CausalLineage,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum CompletenessState {
    NotRequired,
    Complete,
    Incomplete { reason_code: String },
    Ambiguous { reason_code: String },
    Unsupported { reason_code: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProofRequirement {
    #[serde(default)]
    pub expected_proposition: Option<Proposition>,
    pub guarantees: EvidenceGuarantees,
    pub required_complete: BTreeSet<CompletenessDimension>,
}

impl ProofRequirement {
    pub fn for_proposition(
        proposition: Proposition,
        guarantees: EvidenceGuarantees,
        required_complete: BTreeSet<CompletenessDimension>,
    ) -> Self {
        Self {
            expected_proposition: Some(proposition),
            guarantees,
            required_complete,
        }
    }

    fn is_non_vacuous(&self) -> bool {
        self.expected_proposition.is_some()
            && (!self.guarantees.is_empty() || !self.required_complete.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendSemanticProfile {
    pub name: String,
    pub semantic_profile_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofCarryingObservation {
    pub schema_version: u32,
    pub proposition: Proposition,
    pub guarantees: EvidenceGuarantees,
    pub completeness: BTreeMap<CompletenessDimension, CompletenessState>,
    pub backend_profile: BackendSemanticProfile,
    pub ambiguity_codes: BTreeSet<String>,
}

impl ProofCarryingObservation {
    pub fn new(
        proposition: Proposition,
        guarantees: EvidenceGuarantees,
        backend_profile: BackendSemanticProfile,
    ) -> Self {
        Self {
            schema_version: SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION,
            proposition,
            guarantees,
            completeness: BTreeMap::new(),
            backend_profile,
            ambiguity_codes: BTreeSet::new(),
        }
    }

    fn ambiguity_invalidates(&self, requirement: &ProofRequirement) -> bool {
        for code in &self.ambiguity_codes {
            match code.as_str() {
                "object_identity_conflict" => {
                    if requirement
                        .required_complete
                        .contains(&CompletenessDimension::ObjectIdentity)
                    {
                        return true;
                    }
                }
                _ => {
                    // Unknown ambiguity semantics must never silently increase authority.
                    return true;
                }
            }
        }
        false
    }

    pub fn satisfies(&self, requirement: &ProofRequirement) -> bool {
        if self.schema_version != SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION {
            return false;
        }
        if !requirement.is_non_vacuous() {
            return false;
        }
        if requirement.expected_proposition.as_ref() != Some(&self.proposition) {
            return false;
        }
        if !self.guarantees.entails(&requirement.guarantees) {
            return false;
        }
        if self.ambiguity_invalidates(requirement) {
            return false;
        }
        requirement
            .required_complete
            .iter()
            .all(|dimension| self.completeness.get(dimension) == Some(&CompletenessState::Complete))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::{PathClass, PathResolution};

    fn target_path() -> CanonicalPath {
        CanonicalPath {
            value: "$WORKSPACE/input.txt".to_owned(),
            class: PathClass::Workspace,
            resolution: PathResolution::Lexical,
        }
    }

    fn pathname_attempt() -> Proposition {
        Proposition::FilePathnameAttemptObserved {
            actor: None,
            execution_chain: vec![],
            operation: FileOperation::Open,
            target: target_path(),
            open_intent: None,
        }
    }

    fn ptrace_argument_guarantees() -> EvidenceGuarantees {
        EvidenceGuarantees {
            observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
            identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
            temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
            causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
        }
    }

    fn requirement_for(
        guarantees: EvidenceGuarantees,
        required_complete: BTreeSet<CompletenessDimension>,
    ) -> ProofRequirement {
        ProofRequirement::for_proposition(pathname_attempt(), guarantees, required_complete)
    }

    fn base_record() -> ProofCarryingObservation {
        let mut record = ProofCarryingObservation::new(
            pathname_attempt(),
            ptrace_argument_guarantees(),
            BackendSemanticProfile {
                name: "linux-ptrace-semantics-v3-prototype".to_owned(),
                semantic_profile_version: 1,
            },
        );
        record.completeness.insert(
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        );
        record.completeness.insert(
            CompletenessDimension::Lifecycle,
            CompletenessState::Complete,
        );
        record
    }

    #[test]
    fn weak_path_argument_does_not_entail_kernel_object_grounding() {
        let record = base_record();
        let requirement = requirement_for(
            EvidenceGuarantees {
                identity_bases: BTreeSet::from([IdentityBasis::KernelObjectGrounded]),
                ..EvidenceGuarantees::default()
            },
            BTreeSet::new(),
        );

        assert!(!record.satisfies(&requirement));
    }

    #[test]
    fn ambiguity_blocks_required_completeness() {
        let mut record = base_record();
        record.completeness.insert(
            CompletenessDimension::ObjectIdentity,
            CompletenessState::Ambiguous {
                reason_code: "shared_fd_table_ambiguity".to_owned(),
            },
        );
        let requirement = requirement_for(
            ptrace_argument_guarantees(),
            BTreeSet::from([
                CompletenessDimension::SessionScope,
                CompletenessDimension::ObjectIdentity,
            ]),
        );

        assert!(!record.satisfies(&requirement));
    }

    #[test]
    fn exact_required_guarantees_and_completeness_are_admissible() {
        let record = base_record();
        let requirement = requirement_for(
            ptrace_argument_guarantees(),
            BTreeSet::from([
                CompletenessDimension::SessionScope,
                CompletenessDimension::Lifecycle,
            ]),
        );

        assert!(record.satisfies(&requirement));
    }

    #[test]
    fn default_requirement_fails_closed() {
        let record = base_record();
        assert!(!record.satisfies(&ProofRequirement::default()));
    }

    #[test]
    fn proposition_mismatch_fails_closed() {
        let record = base_record();
        let mut wrong = pathname_attempt();
        if let Proposition::FilePathnameAttemptObserved { target, .. } = &mut wrong {
            target.value = "$WORKSPACE/other.txt".to_owned();
        }
        let requirement = ProofRequirement::for_proposition(
            wrong,
            ptrace_argument_guarantees(),
            BTreeSet::from([CompletenessDimension::SessionScope]),
        );
        assert!(!record.satisfies(&requirement));
    }

    #[test]
    fn explicit_object_identity_conflict_blocks_complete_claim() {
        let mut record = base_record();
        record.completeness.insert(
            CompletenessDimension::ObjectIdentity,
            CompletenessState::Complete,
        );
        record
            .ambiguity_codes
            .insert("object_identity_conflict".to_owned());
        let requirement = requirement_for(
            ptrace_argument_guarantees(),
            BTreeSet::from([CompletenessDimension::ObjectIdentity]),
        );
        assert!(!record.satisfies(&requirement));
    }

    #[test]
    fn unknown_ambiguity_code_fails_closed_for_admission() {
        let mut record = base_record();
        record
            .ambiguity_codes
            .insert("unrecognized_future_ambiguity".to_owned());
        let requirement = requirement_for(
            ptrace_argument_guarantees(),
            BTreeSet::from([CompletenessDimension::SessionScope]),
        );
        assert!(!record.satisfies(&requirement));
    }

    #[test]
    fn deterministic_serialization_is_independent_of_set_insertion_order() {
        let mut first = base_record();
        first.ambiguity_codes.insert("zeta".to_owned());
        first.ambiguity_codes.insert("alpha".to_owned());

        let mut second = base_record();
        second.ambiguity_codes.insert("alpha".to_owned());
        second.ambiguity_codes.insert("zeta".to_owned());

        assert_eq!(
            serde_json::to_vec(&first).expect("serialize first"),
            serde_json::to_vec(&second).expect("serialize second")
        );
    }

    #[test]
    fn same_canonical_value_with_different_authority_is_not_equal_evidence() {
        let first = base_record();
        let mut second = base_record();
        second
            .guarantees
            .identity_bases
            .insert(IdentityBasis::KernelObjectGrounded);

        assert_ne!(first, second);
    }

    #[test]
    fn fd_table_relation_unknown_is_first_class_proposition_state() {
        let proposition = Proposition::FdTableRelationObserved {
            actor: None,
            mechanism: SpawnMechanism::Clone,
            relation: FdTableRelationState::Unknown,
        };
        let encoded = serde_json::to_vec(&proposition).expect("serialize fd-table relation");
        let decoded: Proposition =
            serde_json::from_slice(&encoded).expect("deserialize fd-table relation");
        assert_eq!(decoded, proposition);
    }

    #[test]
    fn observer_health_warning_codes_serialize_deterministically() {
        let first = Proposition::ObserverHealthObserved {
            complete: false,
            warning_codes: BTreeSet::from(["zeta".to_owned(), "alpha".to_owned()]),
        };
        let second = Proposition::ObserverHealthObserved {
            complete: false,
            warning_codes: BTreeSet::from(["alpha".to_owned(), "zeta".to_owned()]),
        };
        assert_eq!(
            serde_json::to_vec(&first).expect("serialize first health proposition"),
            serde_json::to_vec(&second).expect("serialize second health proposition")
        );
    }
}
