use execsurface_authority::{
    ptrace_contract_v1, AuthorityRequirementError, EvidenceAuthority, EvidenceProposition,
    PropositionSupport,
};
use execsurface_observe::{reference_backend_descriptor, ObservationCapability};

#[test]
fn legacy_ptrace_path_capabilities_do_not_imply_kernel_object_authority() {
    let descriptor = reference_backend_descriptor();
    descriptor
        .validate_capability_partition()
        .expect("legacy ptrace capability partition must remain valid");

    assert!(descriptor
        .capabilities
        .contains(&ObservationCapability::ProcessExecPathIdentity));
    assert!(descriptor
        .capabilities
        .contains(&ObservationCapability::OpenPathIdentity));

    let authority = ptrace_contract_v1();
    authority
        .validate_total_partition()
        .expect("M11 ptrace authority contract must remain valid");

    assert_eq!(
        authority.support_for(EvidenceProposition::ProcessExecAttemptPath),
        Some(PropositionSupport::Supported(
            EvidenceAuthority::ArgumentObserved
        ))
    );
    assert_eq!(
        authority.support_for(EvidenceProposition::ProcessExecObjectIdentity),
        Some(PropositionSupport::Unsupported)
    );
    assert_eq!(
        authority.support_for(EvidenceProposition::FilePathAccessIntent),
        Some(PropositionSupport::Supported(
            EvidenceAuthority::ArgumentObserved
        ))
    );
    assert_ne!(
        authority.support_for(EvidenceProposition::FileOpenObjectIdentity),
        Some(PropositionSupport::Supported(
            EvidenceAuthority::KernelObjectSuccessBound
        ))
    );
}

#[test]
fn path_toctou_promotion_to_kernel_object_bound_is_explicitly_rejected() {
    let authority = ptrace_contract_v1();

    for proposition in [
        EvidenceProposition::ProcessExecAttemptPath,
        EvidenceProposition::FilePathAccessIntent,
        EvidenceProposition::NetworkConnectAttemptDestination,
    ] {
        assert!(matches!(
            authority.require_exact_authority(
                proposition,
                EvidenceAuthority::KernelObjectSuccessBound,
            ),
            Err(AuthorityRequirementError::AuthorityMismatch { .. })
        ));
    }

    assert!(matches!(
        authority.require_exact_authority(
            EvidenceProposition::ProcessExecObjectIdentity,
            EvidenceAuthority::KernelObjectSuccessBound,
        ),
        Err(AuthorityRequirementError::UnsupportedProposition { .. })
    ));
}
