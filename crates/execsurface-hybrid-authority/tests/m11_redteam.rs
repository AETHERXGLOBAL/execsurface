use execsurface_authority::{
    EvidenceAuthority, EvidenceHealth, EvidenceProposition, PropositionSupport,
};
use execsurface_hybrid_authority::{
    ExplicitHybridAdapter, HybridAdapterError, HybridCapabilityContext, HybridHealthReport,
    HybridPrerequisite,
};

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
fn producer_loss_and_session_ambiguity_are_never_pass_eligible() {
    for report in [
        HybridHealthReport {
            producer_drops: 1,
            ..HybridHealthReport::default()
        },
        HybridHealthReport {
            malformed_records: 1,
            ..HybridHealthReport::default()
        },
        HybridHealthReport {
            wrong_session_records: 1,
            ..HybridHealthReport::default()
        },
        HybridHealthReport {
            unknown_role_records: 1,
            ..HybridHealthReport::default()
        },
    ] {
        let health = report.evidence_health();
        assert_ne!(health, EvidenceHealth::Complete);
        assert!(!health.pass_eligible());
    }
}

#[test]
fn file_object_candidate_does_not_promote_file_open_success() {
    let contract = ExplicitHybridAdapter::explicit_research_v1(accepted_context())
        .contract()
        .expect("accepted explicit hybrid context");

    assert_eq!(
        contract.support_for(EvidenceProposition::FileOpenObjectIdentity),
        Some(PropositionSupport::Supported(
            EvidenceAuthority::KernelObjectCandidate
        ))
    );
    assert_eq!(
        contract.support_for(EvidenceProposition::FileOpenSuccess),
        Some(PropositionSupport::Unsupported)
    );
}

#[test]
fn success_confirmation_sources_are_mandatory_not_inferred_from_candidates() {
    let mut no_exec_success = accepted_context();
    no_exec_success.exec_success_confirmation_available = false;
    assert_eq!(
        ExplicitHybridAdapter::explicit_research_v1(no_exec_success).contract(),
        Err(HybridAdapterError::MissingPrerequisite(
            HybridPrerequisite::ExecSuccessConfirmationAvailable
        ))
    );

    let mut no_connect_completion = accepted_context();
    no_connect_completion.connect_completion_confirmation_available = false;
    assert_eq!(
        ExplicitHybridAdapter::explicit_research_v1(no_connect_completion).contract(),
        Err(HybridAdapterError::MissingPrerequisite(
            HybridPrerequisite::ConnectCompletionConfirmationAvailable
        ))
    );
}

#[test]
fn unsupported_deployment_contexts_fail_explicitly_without_fallback() {
    let cases = [
        (
            HybridCapabilityContext {
                platform: "windows".to_owned(),
                ..accepted_context()
            },
            HybridPrerequisite::LinuxPlatform,
        ),
        (
            HybridCapabilityContext {
                bpf_lsm_active: false,
                ..accepted_context()
            },
            HybridPrerequisite::BpfLsmActive,
        ),
        (
            HybridCapabilityContext {
                kernel_btf_readable: false,
                ..accepted_context()
            },
            HybridPrerequisite::KernelBtfReadable,
        ),
        (
            HybridCapabilityContext {
                bpf_operation_permitted: false,
                ..accepted_context()
            },
            HybridPrerequisite::BpfOperationPermitted,
        ),
        (
            HybridCapabilityContext {
                privilege_explicitly_acknowledged: false,
                ..accepted_context()
            },
            HybridPrerequisite::PrivilegeExplicitlyAcknowledged,
        ),
        (
            HybridCapabilityContext {
                producer_loss_accounting_available: false,
                ..accepted_context()
            },
            HybridPrerequisite::ProducerLossAccountingAvailable,
        ),
    ];

    for (context, prerequisite) in cases {
        assert_eq!(
            ExplicitHybridAdapter::explicit_research_v1(context).contract(),
            Err(HybridAdapterError::MissingPrerequisite(prerequisite))
        );
    }
}
