//! Research-only Semantics v3 prototype.
//!
//! This module is intentionally disconnected from public alpha.4 learn/check,
//! baseline and verdict paths. It prototypes proposition-scoped proof semantics
//! before any public schema integration is considered.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::canonical::{
    CanonicalExecutable, CanonicalNetworkEndpoint, CanonicalPath, OpenIntent,
};
use crate::{FileOperation, SpawnMechanism};

pub const SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION: u32 = 3;

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
    FilePathnameAttemptObserved {
        actor: Option<CanonicalExecutable>,
        execution_chain: Vec<CanonicalExecutable>,
        operation: FileOperation,
        target: CanonicalPath,
        open_intent: Option<OpenIntent>,
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
            && required.temporal_bindings.is_subset(&self.temporal_bindings)
            && required.causal_bindings.is_subset(&self.causal_bindings)
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
    pub guarantees: EvidenceGuarantees,
    pub required_complete: BTreeSet<CompletenessDimension>,
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

    pub fn satisfies(&self, requirement: &ProofRequirement) -> bool {
        if self.schema_version != SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION {
            return false;
        }
        if !self.guarantees.entails(&requirement.guarantees) {
            return false;
        }
        requirement.required_complete.iter().all(|dimension| {
            self.completeness.get(dimension) == Some(&CompletenessState::Complete)
        })
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

    fn base_record() -> ProofCarryingObservation {
        let mut record = ProofCarryingObservation::new(
            pathname_attempt(),
            ptrace_argument_guarantees(),
            BackendSemanticProfile {
                name: "linux-ptrace-semantics-v3-prototype".to_owned(),
                semantic_profile_version: 1,
            },
        );
        record
            .completeness
            .insert(CompletenessDimension::SessionScope, CompletenessState::Complete);
        record
            .completeness
            .insert(CompletenessDimension::Lifecycle, CompletenessState::Complete);
        record
    }

    #[test]
    fn weak_path_argument_does_not_entail_kernel_object_grounding() {
        let record = base_record();
        let requirement = ProofRequirement {
            guarantees: EvidenceGuarantees {
                identity_bases: BTreeSet::from([IdentityBasis::KernelObjectGrounded]),
                ..EvidenceGuarantees::default()
            },
            required_complete: BTreeSet::new(),
        };

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
        let requirement = ProofRequirement {
            guarantees: ptrace_argument_guarantees(),
            required_complete: BTreeSet::from([
                CompletenessDimension::SessionScope,
                CompletenessDimension::ObjectIdentity,
            ]),
        };

        assert!(!record.satisfies(&requirement));
    }

    #[test]
    fn exact_required_guarantees_and_completeness_are_admissible() {
        let record = base_record();
        let requirement = ProofRequirement {
            guarantees: ptrace_argument_guarantees(),
            required_complete: BTreeSet::from([
                CompletenessDimension::SessionScope,
                CompletenessDimension::Lifecycle,
            ]),
        };

        assert!(record.satisfies(&requirement));
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
}
