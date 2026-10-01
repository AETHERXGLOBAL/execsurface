#![forbid(unsafe_code)]

//! Explicit, non-default hybrid authority adapter for M11.4.
//!
//! This crate models only the bounded kernel-authority capabilities accepted by
//! M10. It performs no backend loading, privilege escalation, daemon setup, or
//! fallback to ptrace.

use execsurface_authority::{
    BackendEvidenceContract, CapabilityFingerprint, EvidenceAuthority, EvidenceContractVersion,
    EvidenceHealth, EvidenceProposition, PropositionAuthority, PropositionComparability,
    PropositionSupport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HybridSelection {
    #[default]
    Disabled,
    ExplicitResearchV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HybridPrerequisite {
    LinuxPlatform,
    X86_64Architecture,
    BpfLsmActive,
    KernelBtfReadable,
    BpfOperationPermitted,
    PrivilegeExplicitlyAcknowledged,
    ProducerLossAccountingAvailable,
    ExecSuccessConfirmationAvailable,
    ConnectCompletionConfirmationAvailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HybridAdapterError {
    NotExplicitlySelected,
    MissingPrerequisite(HybridPrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridCapabilityContext {
    pub platform: String,
    pub architecture: String,
    pub kernel_release: Option<String>,
    pub bpf_lsm_active: bool,
    pub kernel_btf_readable: bool,
    pub bpf_operation_permitted: bool,
    pub privilege_explicitly_acknowledged: bool,
    pub producer_loss_accounting_available: bool,
    pub exec_success_confirmation_available: bool,
    pub connect_completion_confirmation_available: bool,
}

impl Default for HybridCapabilityContext {
    fn default() -> Self {
        Self {
            platform: std::env::consts::OS.to_owned(),
            architecture: std::env::consts::ARCH.to_owned(),
            kernel_release: None,
            bpf_lsm_active: false,
            kernel_btf_readable: false,
            bpf_operation_permitted: false,
            privilege_explicitly_acknowledged: false,
            producer_loss_accounting_available: false,
            exec_success_confirmation_available: false,
            connect_completion_confirmation_available: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExplicitHybridAdapter {
    selection: HybridSelection,
    context: HybridCapabilityContext,
}

impl ExplicitHybridAdapter {
    pub fn explicit_research_v1(context: HybridCapabilityContext) -> Self {
        Self {
            selection: HybridSelection::ExplicitResearchV1,
            context,
        }
    }

    pub const fn selection(&self) -> HybridSelection {
        self.selection
    }

    pub fn contract(&self) -> Result<BackendEvidenceContract, HybridAdapterError> {
        if self.selection != HybridSelection::ExplicitResearchV1 {
            return Err(HybridAdapterError::NotExplicitlySelected);
        }

        self.validate_prerequisites()?;
        Ok(hybrid_contract_v1(&self.context))
    }

    fn validate_prerequisites(&self) -> Result<(), HybridAdapterError> {
        let checks = [
            (
                self.context.platform == "linux",
                HybridPrerequisite::LinuxPlatform,
            ),
            (
                self.context.architecture == "x86_64",
                HybridPrerequisite::X86_64Architecture,
            ),
            (
                self.context.bpf_lsm_active,
                HybridPrerequisite::BpfLsmActive,
            ),
            (
                self.context.kernel_btf_readable,
                HybridPrerequisite::KernelBtfReadable,
            ),
            (
                self.context.bpf_operation_permitted,
                HybridPrerequisite::BpfOperationPermitted,
            ),
            (
                self.context.privilege_explicitly_acknowledged,
                HybridPrerequisite::PrivilegeExplicitlyAcknowledged,
            ),
            (
                self.context.producer_loss_accounting_available,
                HybridPrerequisite::ProducerLossAccountingAvailable,
            ),
            (
                self.context.exec_success_confirmation_available,
                HybridPrerequisite::ExecSuccessConfirmationAvailable,
            ),
            (
                self.context.connect_completion_confirmation_available,
                HybridPrerequisite::ConnectCompletionConfirmationAvailable,
            ),
        ];

        checks
            .into_iter()
            .find_map(|(available, prerequisite)| {
                (!available).then_some(HybridAdapterError::MissingPrerequisite(prerequisite))
            })
            .map_or(Ok(()), Err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HybridHealthReport {
    pub producer_drops: u64,
    pub malformed_records: u64,
    pub wrong_session_records: u64,
    pub unknown_role_records: u64,
}

impl HybridHealthReport {
    pub const fn evidence_health(self) -> EvidenceHealth {
        if self.producer_drops > 0 {
            EvidenceHealth::IncompleteLoss
        } else if self.malformed_records > 0
            || self.wrong_session_records > 0
            || self.unknown_role_records > 0
        {
            EvidenceHealth::IncompleteAmbiguity
        } else {
            EvidenceHealth::Complete
        }
    }
}

/// M11.4 deliberately grants no cross-backend baseline equivalence.
pub const fn ptrace_hybrid_comparability_v1(
    _proposition: EvidenceProposition,
) -> PropositionComparability {
    PropositionComparability::NotComparable
}

fn hybrid_contract_v1(context: &HybridCapabilityContext) -> BackendEvidenceContract {
    use EvidenceAuthority::{
        KernelObjectCandidate, KernelObjectSuccessBound, KernelSuccessConfirmed,
    };
    use EvidenceProposition::*;
    use PropositionSupport::{Supported, Unsupported};

    BackendEvidenceContract {
        version: EvidenceContractVersion::M11_V1,
        fingerprint: CapabilityFingerprint {
            source_id: "linux-bpf-lsm-hybrid-m10-bounded-v1".to_owned(),
            implementation_contract_version: "m11-explicit-hybrid-authority-v1".to_owned(),
            platform: context.platform.clone(),
            architecture: context.architecture.clone(),
            privacy_profile: "metadata-only-v1".to_owned(),
            health_contract_version: "m11-health-v1".to_owned(),
            kernel_capability_mode: Some("bpf-lsm+tracepoints-explicit-research-v1".to_owned()),
        },
        propositions: vec![
            PropositionAuthority {
                proposition: ProcessSpawnOccurrence,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: ProcessExecAttemptPath,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: ProcessExecSuccess,
                support: Supported(KernelSuccessConfirmed),
            },
            PropositionAuthority {
                proposition: ProcessExecObjectIdentity,
                support: Supported(KernelObjectSuccessBound),
            },
            PropositionAuthority {
                proposition: FilePathAccessIntent,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: FileOpenSuccess,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: FileOpenObjectIdentity,
                support: Supported(KernelObjectCandidate),
            },
            PropositionAuthority {
                proposition: FdReadEffect,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: FdWriteEffect,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: FdLifecycle,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: RenameDeleteEffect,
                support: Unsupported,
            },
            PropositionAuthority {
                proposition: NetworkConnectAttemptDestination,
                support: Supported(KernelObjectCandidate),
            },
            PropositionAuthority {
                proposition: NetworkConnectSuccess,
                support: Supported(KernelSuccessConfirmed),
            },
            PropositionAuthority {
                proposition: TraceRelativePath,
                support: Unsupported,
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
    use execsurface_authority::ptrace_contract_v1;

    fn accepted_context() -> HybridCapabilityContext {
        HybridCapabilityContext {
            platform: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
            kernel_release: Some("controlled-test-kernel".to_owned()),
            bpf_lsm_active: true,
            kernel_btf_readable: true,
            bpf_operation_permitted: true,
            privilege_explicitly_acknowledged: true,
            producer_loss_accounting_available: true,
            exec_success_confirmation_available: true,
            connect_completion_confirmation_available: true,
        }
    }

    #[test]
    fn adapter_is_disabled_by_default() {
        let adapter = ExplicitHybridAdapter::default();
        assert_eq!(adapter.selection(), HybridSelection::Disabled);
        assert_eq!(
            adapter.contract(),
            Err(HybridAdapterError::NotExplicitlySelected)
        );
    }

    #[test]
    fn every_prerequisite_fails_closed_when_absent() {
        let base = accepted_context();
        let cases: Vec<(HybridCapabilityContext, HybridPrerequisite)> = vec![
            (
                HybridCapabilityContext {
                    platform: "windows".to_owned(),
                    ..base.clone()
                },
                HybridPrerequisite::LinuxPlatform,
            ),
            (
                HybridCapabilityContext {
                    architecture: "aarch64".to_owned(),
                    ..base.clone()
                },
                HybridPrerequisite::X86_64Architecture,
            ),
            (
                HybridCapabilityContext {
                    bpf_lsm_active: false,
                    ..base.clone()
                },
                HybridPrerequisite::BpfLsmActive,
            ),
            (
                HybridCapabilityContext {
                    kernel_btf_readable: false,
                    ..base.clone()
                },
                HybridPrerequisite::KernelBtfReadable,
            ),
            (
                HybridCapabilityContext {
                    bpf_operation_permitted: false,
                    ..base.clone()
                },
                HybridPrerequisite::BpfOperationPermitted,
            ),
            (
                HybridCapabilityContext {
                    privilege_explicitly_acknowledged: false,
                    ..base.clone()
                },
                HybridPrerequisite::PrivilegeExplicitlyAcknowledged,
            ),
            (
                HybridCapabilityContext {
                    producer_loss_accounting_available: false,
                    ..base.clone()
                },
                HybridPrerequisite::ProducerLossAccountingAvailable,
            ),
            (
                HybridCapabilityContext {
                    exec_success_confirmation_available: false,
                    ..base.clone()
                },
                HybridPrerequisite::ExecSuccessConfirmationAvailable,
            ),
            (
                HybridCapabilityContext {
                    connect_completion_confirmation_available: false,
                    ..base.clone()
                },
                HybridPrerequisite::ConnectCompletionConfirmationAvailable,
            ),
        ];

        for (context, expected) in cases {
            let adapter = ExplicitHybridAdapter::explicit_research_v1(context);
            assert_eq!(
                adapter.contract(),
                Err(HybridAdapterError::MissingPrerequisite(expected))
            );
        }
    }

    #[test]
    fn accepted_context_yields_exact_bounded_contract() {
        let contract = ExplicitHybridAdapter::explicit_research_v1(accepted_context())
            .contract()
            .expect("accepted explicit hybrid context");
        contract
            .validate_total_partition()
            .expect("hybrid contract must classify the full proposition universe");

        assert_eq!(
            contract.support_for(EvidenceProposition::FileOpenObjectIdentity),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::KernelObjectCandidate
            ))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::ProcessExecSuccess),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::KernelSuccessConfirmed
            ))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::ProcessExecObjectIdentity),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::KernelObjectSuccessBound
            ))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::NetworkConnectAttemptDestination),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::KernelObjectCandidate
            ))
        );
        assert_eq!(
            contract.support_for(EvidenceProposition::NetworkConnectSuccess),
            Some(PropositionSupport::Supported(
                EvidenceAuthority::KernelSuccessConfirmed
            ))
        );
    }

    #[test]
    fn unproved_hybrid_propositions_remain_unsupported() {
        let contract = ExplicitHybridAdapter::explicit_research_v1(accepted_context())
            .contract()
            .expect("accepted explicit hybrid context");

        for proposition in [
            EvidenceProposition::ProcessSpawnOccurrence,
            EvidenceProposition::ProcessExecAttemptPath,
            EvidenceProposition::FilePathAccessIntent,
            EvidenceProposition::FileOpenSuccess,
            EvidenceProposition::FdReadEffect,
            EvidenceProposition::FdWriteEffect,
            EvidenceProposition::FdLifecycle,
            EvidenceProposition::RenameDeleteEffect,
            EvidenceProposition::TraceRelativePath,
            EvidenceProposition::CausalExecutableChain,
        ] {
            assert_eq!(
                contract.support_for(proposition),
                Some(PropositionSupport::Unsupported),
                "unexpected hybrid promotion for {proposition:?}"
            );
        }
    }

    #[test]
    fn hybrid_health_fails_closed_on_loss_or_ambiguity() {
        assert_eq!(
            HybridHealthReport::default().evidence_health(),
            EvidenceHealth::Complete
        );
        assert_eq!(
            HybridHealthReport {
                producer_drops: 1,
                ..HybridHealthReport::default()
            }
            .evidence_health(),
            EvidenceHealth::IncompleteLoss
        );
        assert_eq!(
            HybridHealthReport {
                wrong_session_records: 1,
                ..HybridHealthReport::default()
            }
            .evidence_health(),
            EvidenceHealth::IncompleteAmbiguity
        );
    }

    #[test]
    fn no_cross_backend_equivalence_or_fallback_is_granted() {
        let ptrace = ptrace_contract_v1();
        let hybrid = ExplicitHybridAdapter::explicit_research_v1(accepted_context())
            .contract()
            .expect("accepted explicit hybrid context");

        assert_ne!(ptrace.fingerprint.source_id, hybrid.fingerprint.source_id);
        for proposition in EvidenceProposition::ALL {
            let comparability = ptrace_hybrid_comparability_v1(proposition);
            assert_eq!(comparability, PropositionComparability::NotComparable);
            assert!(!comparability.permits_symmetric_baseline_reuse());
        }
    }
}
