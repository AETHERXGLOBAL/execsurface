use execsurface_authority::{
    ptrace_contract_v1, BackendEvidenceContract, EvidenceProposition, PropositionComparability,
};
use execsurface_hybrid_authority::{
    ptrace_hybrid_comparability_v1, ExplicitHybridAdapter, HybridCapabilityContext,
};

fn accepted_hybrid_context() -> HybridCapabilityContext {
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

fn same_contract_relation(
    left: &BackendEvidenceContract,
    right: &BackendEvidenceContract,
    proposition: EvidenceProposition,
) -> PropositionComparability {
    if left == right && left.support_for(proposition) == right.support_for(proposition) {
        PropositionComparability::EquivalentForProposition
    } else {
        PropositionComparability::Unknown
    }
}

fn symmetric_reuse_allowed(
    required: &[EvidenceProposition],
    relation: impl Fn(EvidenceProposition) -> Option<PropositionComparability>,
) -> bool {
    !required.is_empty()
        && required.iter().copied().all(|proposition| {
            relation(proposition)
                .is_some_and(PropositionComparability::permits_symmetric_baseline_reuse)
        })
}

#[test]
fn identical_complete_contract_is_equivalent_per_proposition() {
    let ptrace = ptrace_contract_v1();
    for proposition in EvidenceProposition::ALL {
        assert_eq!(
            same_contract_relation(&ptrace, &ptrace, proposition),
            PropositionComparability::EquivalentForProposition
        );
    }
}

#[test]
fn fingerprint_change_is_unknown_without_explicit_rule() {
    let left = ptrace_contract_v1();
    let mut right = left.clone();
    right.fingerprint.implementation_contract_version = "different-contract".to_owned();

    for proposition in EvidenceProposition::ALL {
        assert_eq!(
            same_contract_relation(&left, &right, proposition),
            PropositionComparability::Unknown
        );
    }
}

#[test]
fn proposition_mapping_change_is_unknown_without_explicit_rule() {
    let left = ptrace_contract_v1();
    let mut right = left.clone();
    right.propositions[0].support = execsurface_authority::PropositionSupport::Unsupported;

    assert_eq!(
        same_contract_relation(&left, &right, EvidenceProposition::ProcessSpawnOccurrence),
        PropositionComparability::Unknown
    );
}

#[test]
fn ptrace_and_hybrid_are_explicitly_not_comparable_for_all_frozen_propositions() {
    let hybrid = ExplicitHybridAdapter::explicit_research_v1(accepted_hybrid_context())
        .contract()
        .expect("accepted bounded hybrid contract");
    let ptrace = ptrace_contract_v1();

    assert_ne!(ptrace.fingerprint.source_id, hybrid.fingerprint.source_id);

    for proposition in EvidenceProposition::ALL {
        assert_eq!(
            ptrace_hybrid_comparability_v1(proposition),
            PropositionComparability::NotComparable,
            "cross-backend equivalence must not be inferred for {proposition:?}"
        );
    }
}

#[test]
fn symmetric_reuse_requires_every_required_proposition_to_be_equivalent() {
    let required = [
        EvidenceProposition::ProcessExecSuccess,
        EvidenceProposition::FileOpenObjectIdentity,
    ];

    assert!(symmetric_reuse_allowed(&required, |_| Some(
        PropositionComparability::EquivalentForProposition
    )));

    for blocker in [
        PropositionComparability::OneWayRefinement,
        PropositionComparability::NotComparable,
        PropositionComparability::Unknown,
    ] {
        assert!(!symmetric_reuse_allowed(&required, |proposition| {
            if proposition == EvidenceProposition::FileOpenObjectIdentity {
                Some(blocker)
            } else {
                Some(PropositionComparability::EquivalentForProposition)
            }
        }));
    }

    assert!(!symmetric_reuse_allowed(&required, |proposition| {
        (proposition == EvidenceProposition::ProcessExecSuccess)
            .then_some(PropositionComparability::EquivalentForProposition)
    }));
}

#[test]
fn empty_required_set_does_not_grant_reuse() {
    assert!(!symmetric_reuse_allowed(&[], |_| Some(
        PropositionComparability::EquivalentForProposition
    )));
}
