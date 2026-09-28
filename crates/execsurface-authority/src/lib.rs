#![forbid(unsafe_code)]

//! Internal evidence-authority vocabulary for M11.
//!
//! This crate is intentionally outside the public/default raw observation,
//! canonical surface, and baseline v2 serialization path. It defines the
//! authority/completeness contract before runtime integration.

use std::collections::BTreeSet;

pub const EVIDENCE_CONTRACT_MAJOR: u16 = 1;
pub const EVIDENCE_CONTRACT_MINOR: u16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceProposition {
    ProcessSpawnOccurrence,
    ProcessExecAttemptPath,
    ProcessExecSuccess,
    ProcessExecObjectIdentity,
    FilePathAccessIntent,
    FileOpenSuccess,
    FileOpenObjectIdentity,
    FdReadEffect,
    FdWriteEffect,
    FdLifecycle,
    RenameDeleteEffect,
    NetworkConnectAttemptDestination,
    NetworkConnectSuccess,
    TraceRelativePath,
    CausalExecutableChain,
}

impl EvidenceProposition {
    pub const ALL: [Self; 15] = [
        Self::ProcessSpawnOccurrence,
        Self::ProcessExecAttemptPath,
        Self::ProcessExecSuccess,
        Self::ProcessExecObjectIdentity,
        Self::FilePathAccessIntent,
        Self::FileOpenSuccess,
        Self::FileOpenObjectIdentity,
        Self::FdReadEffect,
        Self::FdWriteEffect,
        Self::FdLifecycle,
        Self::RenameDeleteEffect,
        Self::NetworkConnectAttemptDestination,
        Self::NetworkConnectSuccess,
        Self::TraceRelativePath,
        Self::CausalExecutableChain,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProcessSpawnOccurrence => "process_spawn_occurrence",
            Self::ProcessExecAttemptPath => "process_exec_attempt_path",
            Self::ProcessExecSuccess => "process_exec_success",
            Self::ProcessExecObjectIdentity => "process_exec_object_identity",
            Self::FilePathAccessIntent => "file_path_access_intent",
            Self::FileOpenSuccess => "file_open_success",
            Self::FileOpenObjectIdentity => "file_open_object_identity",
            Self::FdReadEffect => "fd_read_effect",
            Self::FdWriteEffect => "fd_write_effect",
            Self::FdLifecycle => "fd_lifecycle",
            Self::RenameDeleteEffect => "rename_delete_effect",
            Self::NetworkConnectAttemptDestination => "network_connect_attempt_destination",
            Self::NetworkConnectSuccess => "network_connect_success",
            Self::TraceRelativePath => "trace_relative_path",
            Self::CausalExecutableChain => "causal_executable_chain",
        }
    }
}

pub const ALL_EVIDENCE_PROPOSITIONS: [EvidenceProposition; 15] = EvidenceProposition::ALL;

/// Authority is proposition-specific. This enum is not a global strength score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceAuthority {
    ArgumentObserved,
    KernelObjectCandidate,
    KernelSuccessConfirmed,
    KernelObjectSuccessBound,
    DerivedLifecycleModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropositionSupport {
    Supported(EvidenceAuthority),
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropositionAuthority {
    pub proposition: EvidenceProposition,
    pub support: PropositionSupport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceContractVersion {
    pub major: u16,
    pub minor: u16,
}

impl EvidenceContractVersion {
    pub const M11_V1: Self = Self {
        major: EVIDENCE_CONTRACT_MAJOR,
        minor: EVIDENCE_CONTRACT_MINOR,
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityFingerprint {
    pub source_id: String,
    pub implementation_contract_version: String,
    pub platform: String,
    pub architecture: String,
    pub privacy_profile: String,
    pub health_contract_version: String,
    pub kernel_capability_mode: Option<String>,
}

impl CapabilityFingerprint {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.source_id.trim().is_empty() {
            return Err("capability fingerprint source id is empty");
        }
        if self.implementation_contract_version.trim().is_empty() {
            return Err("capability fingerprint implementation contract version is empty");
        }
        if self.platform.trim().is_empty() || self.architecture.trim().is_empty() {
            return Err("capability fingerprint platform/architecture is empty");
        }
        if self.privacy_profile.trim().is_empty() {
            return Err("capability fingerprint privacy profile is empty");
        }
        if self.health_contract_version.trim().is_empty() {
            return Err("capability fingerprint health contract version is empty");
        }
        if self
            .kernel_capability_mode
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err("capability fingerprint kernel capability mode is empty");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendEvidenceContract {
    pub version: EvidenceContractVersion,
    pub fingerprint: CapabilityFingerprint,
    pub propositions: Vec<PropositionAuthority>,
}

impl BackendEvidenceContract {
    pub fn validate_total_partition(&self) -> Result<(), &'static str> {
        self.fingerprint.validate()?;

        let mut seen = BTreeSet::new();
        for entry in &self.propositions {
            if !seen.insert(entry.proposition) {
                return Err("evidence contract repeats a proposition");
            }
        }

        let expected: BTreeSet<_> = EvidenceProposition::ALL.into_iter().collect();
        if seen != expected {
            return Err("evidence contract does not classify the full proposition universe");
        }

        Ok(())
    }

    pub fn support_for(&self, proposition: EvidenceProposition) -> Option<PropositionSupport> {
        self.propositions
            .iter()
            .find(|entry| entry.proposition == proposition)
            .map(|entry| entry.support)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceHealth {
    Complete,
    IncompleteLoss,
    IncompleteLimit,
    IncompleteCapability,
    IncompleteAmbiguity,
    Error,
}

impl EvidenceHealth {
    pub const fn pass_eligible(self) -> bool {
        matches!(self, Self::Complete)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropositionComparability {
    EquivalentForProposition,
    OneWayRefinement,
    NotComparable,
    Unknown,
}

impl PropositionComparability {
    pub const fn permits_symmetric_baseline_reuse(self) -> bool {
        matches!(self, Self::EquivalentForProposition)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityEnvelope<T> {
    pub contract: BackendEvidenceContract,
    pub health: EvidenceHealth,
    pub payload: T,
}

impl<T> AuthorityEnvelope<T> {
    pub const fn pass_eligible(&self) -> bool {
        self.health.pass_eligible()
    }
}

/// Internal M11 authority contract for the current public ptrace observer.
///
/// This deliberately preserves the M10 limits. In particular, syscall-entry
/// pathname/destination metadata is argument evidence, and fd/object
/// attribution remains lifecycle-derived rather than kernel-object proof.
pub fn ptrace_contract_v1() -> BackendEvidenceContract {
    use EvidenceAuthority::{ArgumentObserved, DerivedLifecycleModel, KernelSuccessConfirmed};
    use EvidenceProposition::*;
    use PropositionSupport::{Supported, Unsupported};

    BackendEvidenceContract {
        version: EvidenceContractVersion::M11_V1,
        fingerprint: CapabilityFingerprint {
            source_id: "linux-ptrace-metadata-v2".to_owned(),
            implementation_contract_version: "m11-ptrace-authority-v1".to_owned(),
            platform: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
            privacy_profile: "metadata-only-v1".to_owned(),
            health_contract_version: "m11-health-v1".to_owned(),
            kernel_capability_mode: None,
        },
        propositions: vec![
            PropositionAuthority {
                proposition: ProcessSpawnOccurrence,
                support: Supported(KernelSuccessConfirmed),
            },
            PropositionAuthority {
                proposition: ProcessExecAttemptPath,
                support: Supported(ArgumentObserved),
            },
            PropositionAuthority {
                proposition: ProcessExecSuccess,
                support: Supported(KernelSuccessConfirmed),
            },
            PropositionAuthority {
                proposition: ProcessExecObjectIdentity,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: FilePathAccessIntent,
                support: Supported(ArgumentObserved),
            },
            PropositionAuthority {
                proposition: FileOpenSuccess,
                support: Supported(KernelSuccessConfirmed),
            },
            PropositionAuthority {
                proposition: FileOpenObjectIdentity,
                support: Supported(DerivedLifecycleModel),
            },
            PropositionAuthority {
                proposition: FdReadEffect,
                support: Supported(DerivedLifecycleModel),
            },
            PropositionAuthority {
                proposition: FdWriteEffect,
                support: Supported(DerivedLifecycleModel),
            },
            PropositionAuthority {
                proposition: FdLifecycle,
                support: Supported(DerivedLifecycleModel),
            },
            PropositionAuthority {
                proposition: RenameDeleteEffect,
                support: Supported(DerivedLifecycleModel),
            },
            PropositionAuthority {
                proposition: NetworkConnectAttemptDestination,
                support: Supported(ArgumentObserved),
            },
            PropositionAuthority {
                proposition: NetworkConnectSuccess,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: TraceRelativePath,
                support: Supported(DerivedLifecycleModel),
            },
            PropositionAuthority {
                proposition: CausalExecutableChain,
                support: Unsupported,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposition_names_are_unique_and_universe_is_frozen() {
        let mut names = EvidenceProposition::ALL
            .iter()
            .map(|proposition| proposition.as_str())
            .collect::<Vec<_>>();
        let count = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(count, 15);
        assert_eq!(names.len(), count);
    }

    #[test]
    fn ptrace_contract_classifies_every_proposition_once() {
        let contract = ptrace_contract_v1();
        contract
            .validate_total_partition()
            .expect("ptrace authority contract must be total and non-duplicated");
        assert_eq!(contract.propositions.len(), EvidenceProposition::ALL.len());
    }

    #[test]
    fn duplicate_or_missing_proposition_is_rejected() {
        let mut contract = ptrace_contract_v1();
        contract.propositions.pop();
        assert!(contract.validate_total_partition().is_err());

        let mut contract = ptrace_contract_v1();
        contract.propositions.push(contract.propositions[0]);
        assert!(contract.validate_total_partition().is_err());
    }

    #[test]
    fn malformed_fingerprint_is_rejected() {
        let mut contract = ptrace_contract_v1();
        contract.fingerprint.source_id.clear();
        assert!(contract.validate_total_partition().is_err());
    }

    #[test]
    fn ptrace_entry_paths_are_argument_metadata_not_object_authority() {
        let contract = ptrace_contract_v1();
        assert_eq!(
            contract.support_for(EvidenceProposition::ProcessExecAttemptPath),
            Some(PropositionSupport::Supported(EvidenceAuthority::ArgumentObserved))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::FilePathAccessIntent),
            Some(PropositionSupport::Supported(EvidenceAuthority::ArgumentObserved))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::ProcessExecObjectIdentity),
            Some(PropositionSupport::Unsupported)
        );
        assert_ne!(
            contract.support_for(EvidenceProposition::FileOpenObjectIdentity),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::KernelObjectSuccessBound
            ))
        );
    }

    #[test]
    fn ptrace_fd_effects_are_lifecycle_derived() {
        let contract = ptrace_contract_v1();
        for proposition in [
            EvidenceProposition::FdReadEffect,
            EvidenceProposition::FdWriteEffect,
            EvidenceProposition::FdLifecycle,
            EvidenceProposition::FileOpenObjectIdentity,
        ] {
            assert_eq!(
                contract.support_for(proposition),
                Some(PropositionSupport::Supported(
                    EvidenceAuthority::DerivedLifecycleModel
                ))
            );
        }
    }

    #[test]
    fn ptrace_connect_attempt_is_not_silently_promoted_to_success() {
        let contract = ptrace_contract_v1();
        assert_eq!(
            contract.support_for(EvidenceProposition::NetworkConnectAttemptDestination),
            Some(PropositionSupport::Supported(EvidenceAuthority::ArgumentObserved))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::NetworkConnectSuccess),
            Some(PropositionSupport::Unsupported)
        );
    }

    #[test]
    fn only_complete_health_is_pass_eligible() {
        assert!(EvidenceHealth::Complete.pass_eligible());
        for health in [
            EvidenceHealth::IncompleteLoss,
            EvidenceHealth::IncompleteLimit,
            EvidenceHealth::IncompleteCapability,
            EvidenceHealth::IncompleteAmbiguity,
            EvidenceHealth::Error,
        ] {
            assert!(!health.pass_eligible(), "{health:?} must fail closed");
        }
    }

    #[test]
    fn authority_envelope_preserves_fail_closed_health() {
        let envelope = AuthorityEnvelope {
            contract: ptrace_contract_v1(),
            health: EvidenceHealth::IncompleteAmbiguity,
            payload: (),
        };
        assert!(!envelope.pass_eligible());
    }

    #[test]
    fn only_explicit_equivalence_allows_symmetric_baseline_reuse() {
        assert!(PropositionComparability::EquivalentForProposition
            .permits_symmetric_baseline_reuse());
        assert!(!PropositionComparability::OneWayRefinement.permits_symmetric_baseline_reuse());
        assert!(!PropositionComparability::NotComparable.permits_symmetric_baseline_reuse());
        assert!(!PropositionComparability::Unknown.permits_symmetric_baseline_reuse());
    }
}
