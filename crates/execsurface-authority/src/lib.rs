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

pub const ALL_EVIDENCE_PROPOSITIONS: [EvidenceProposition; 15] = [
    EvidenceProposition::ProcessSpawnOccurrence,
    EvidenceProposition::ProcessExecAttemptPath,
    EvidenceProposition::ProcessExecSuccess,
    EvidenceProposition::ProcessExecObjectIdentity,
    EvidenceProposition::FilePathAccessIntent,
    EvidenceProposition::FileOpenSuccess,
    EvidenceProposition::FileOpenObjectIdentity,
    EvidenceProposition::FdReadEffect,
    EvidenceProposition::FdWriteEffect,
    EvidenceProposition::FdLifecycle,
    EvidenceProposition::RenameDeleteEffect,
    EvidenceProposition::NetworkConnectAttemptDestination,
    EvidenceProposition::NetworkConnectSuccess,
    EvidenceProposition::TraceRelativePath,
    EvidenceProposition::CausalExecutableChain,
];

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendEvidenceContract {
    pub version: EvidenceContractVersion,
    pub fingerprint: CapabilityFingerprint,
    pub propositions: Vec<PropositionAuthority>,
}

impl BackendEvidenceContract {
    pub fn validate_total_partition(&self) -> Result<(), &'static str> {
        let mut seen = BTreeSet::new();
        for entry in &self.propositions {
            if !seen.insert(entry.proposition) {
                return Err("evidence contract repeats a proposition");
            }
        }

        let expected: BTreeSet<_> = ALL_EVIDENCE_PROPOSITIONS.into_iter().collect();
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
    pub fn pass_eligible(self) -> bool {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityEnvelope<T> {
    pub contract: BackendEvidenceContract,
    pub health: EvidenceHealth,
    pub payload: T,
}

impl<T> AuthorityEnvelope<T> {
    pub fn pass_eligible(&self) -> bool {
        self.health.pass_eligible()
    }
}

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
    fn ptrace_contract_classifies_every_proposition_once() {
        let contract = ptrace_contract_v1();
        contract
            .validate_total_partition()
            .expect("ptrace authority contract must be total and non-duplicated");
        assert_eq!(contract.propositions.len(), ALL_EVIDENCE_PROPOSITIONS.len());
    }

    #[test]
    fn ptrace_entry_paths_are_argument_metadata_not_object_authority() {
        let contract = ptrace_contract_v1();
        assert_eq!(
            contract.support_for(EvidenceProposition::ProcessExecAttemptPath),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::ArgumentObserved
            ))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::FilePathAccessIntent),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::ArgumentObserved
            ))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::ProcessExecObjectIdentity),
            Some(PropositionSupport::Unsupported)
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
    fn ptrace_does_not_claim_connect_success_authority() {
        let contract = ptrace_contract_v1();
        assert_eq!(
            contract.support_for(EvidenceProposition::NetworkConnectAttemptDestination),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::ArgumentObserved
            ))
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
            assert!(!health.pass_eligible());
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
}
