#![cfg(feature = "semantics-v3")]

use std::collections::BTreeSet;

use execsurface_model::authority_v3::{
    compare_under_requirement, AuthorityRecord, AuthorityState, ComparisonResult,
    EvidenceReference, PropositionCompleteness,
};
use execsurface_model::canonical::{
    CanonicalExecutable, CanonicalPath, PathClass, PathResolution,
};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::FileOperation;

fn path(value: &str) -> CanonicalPath {
    CanonicalPath {
        value: value.to_owned(),
        class: PathClass::Workspace,
        resolution: PathResolution::Lexical,
    }
}

fn executable(value: &str, family: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: CanonicalPath {
            value: value.to_owned(),
            class: PathClass::System,
            resolution: PathResolution::Lexical,
        },
        family: family.to_owned(),
    }
}

fn attempt_proposition(target: &str) -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: Some(executable("/usr/bin/cat", "cat")),
        execution_chain: vec![executable("/bin/bash", "bash"), executable("/usr/bin/cat", "cat")],
        operation: FileOperation::Open,
        target: path(target),
        open_intent: None,
    }
}

fn success_proposition() -> Proposition {
    Proposition::ProcessExecSucceeded {
        from: Some(executable("/bin/bash", "bash")),
        executable: executable("/usr/bin/cat", "cat"),
    }
}

fn weak_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn strong_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::SyscallResultPostOperation]),
        identity_bases: BTreeSet::from([IdentityBasis::KernelObjectGrounded]),
        temporal_bindings: BTreeSet::from([TemporalBinding::SuccessfulOperationResult]),
        causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
    }
}

fn proof(proposition: Proposition, guarantees: EvidenceGuarantees, backend: &str) -> ProofCarryingObservation {
    let mut proof = ProofCarryingObservation::new(
        proposition,
        guarantees,
        BackendSemanticProfile {
            name: backend.to_owned(),
            semantic_profile_version: 1,
        },
    );
    proof.completeness.insert(
        CompletenessDimension::SessionScope,
        CompletenessState::Complete,
    );
    proof.completeness.insert(
        CompletenessDimension::Capability,
        CompletenessState::Complete,
    );
    proof
}

fn record(
    proposition: Proposition,
    guarantees: EvidenceGuarantees,
    authority: AuthorityState,
    completeness: PropositionCompleteness,
) -> AuthorityRecord {
    AuthorityRecord {
        proof: proof(proposition, guarantees, "ptrace-candidate"),
        authority,
        completeness,
        evidence: EvidenceReference {
            digest: format!("sha256:{}", "a".repeat(64)),
            derivation: None,
        },
        reason_codes: BTreeSet::new(),
    }
}

fn requirement(proposition: Proposition, guarantees: EvidenceGuarantees) -> ProofRequirement {
    ProofRequirement::for_proposition(
        proposition,
        guarantees,
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Capability,
        ]),
    )
}

#[test]
fn a2_01_unsupported_complete_is_rejected() {
    let mut value = record(
        attempt_proposition("$WORKSPACE/input.txt"),
        weak_guarantees(),
        AuthorityState::Unsupported,
        PropositionCompleteness::Complete,
    );
    value.reason_codes.insert("unsupported_family".to_owned());
    assert!(value.validate().is_err());
}

#[test]
fn a2_02_non_unsupported_not_applicable_is_rejected() {
    let value = record(
        attempt_proposition("$WORKSPACE/input.txt"),
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::NotApplicable,
    );
    assert!(value.validate().is_err());
}

#[test]
fn a2_03_ambiguous_complete_is_rejected() {
    let mut value = record(
        attempt_proposition("$WORKSPACE/input.txt"),
        weak_guarantees(),
        AuthorityState::Ambiguous,
        PropositionCompleteness::Complete,
    );
    value.reason_codes.insert("object_identity_conflict".to_owned());
    assert!(value.validate().is_err());
}

#[test]
fn a2_04_lost_complete_is_rejected() {
    let mut value = record(
        attempt_proposition("$WORKSPACE/input.txt"),
        weak_guarantees(),
        AuthorityState::Lost,
        PropositionCompleteness::Complete,
    );
    value.reason_codes.insert("observer_loss".to_owned());
    assert!(value.validate().is_err());
}

#[test]
fn a2_05_unsupported_without_reason_is_rejected() {
    let value = record(
        attempt_proposition("$WORKSPACE/input.txt"),
        weak_guarantees(),
        AuthorityState::Unsupported,
        PropositionCompleteness::NotApplicable,
    );
    assert!(value.validate().is_err());
}

#[test]
fn a2_06_ambiguous_or_lost_without_reason_is_rejected() {
    for authority in [AuthorityState::Ambiguous, AuthorityState::Lost] {
        let value = record(
            attempt_proposition("$WORKSPACE/input.txt"),
            weak_guarantees(),
            authority,
            PropositionCompleteness::Incomplete,
        );
        assert!(value.validate().is_err());
    }
}

#[test]
fn a2_07_attempt_only_on_non_attempt_proposition_is_rejected() {
    let value = record(
        success_proposition(),
        strong_guarantees(),
        AuthorityState::AttemptOnly,
        PropositionCompleteness::Complete,
    );
    assert!(value.validate().is_err());
}

#[test]
fn a2_08_derived_bounded_without_derivation_is_rejected() {
    let value = record(
        success_proposition(),
        strong_guarantees(),
        AuthorityState::DerivedBounded,
        PropositionCompleteness::Complete,
    );
    assert!(value.validate().is_err());
}

#[test]
fn a2_09_malformed_or_non_hex_digest_is_rejected() {
    for digest in ["sha256:short".to_owned(), format!("sha256:{}", "z".repeat(64))] {
        let mut value = record(
            attempt_proposition("$WORKSPACE/input.txt"),
            weak_guarantees(),
            AuthorityState::Direct,
            PropositionCompleteness::Complete,
        );
        value.evidence.digest = digest;
        assert!(value.validate().is_err());
    }
}

#[test]
fn a2_10_backend_name_cannot_launder_weak_authority() {
    let proposition = attempt_proposition("$WORKSPACE/input.txt");
    let mut value = record(
        proposition.clone(),
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
    );
    value.proof.backend_profile.name = "kernel-super-authoritative-trusted-backend".to_owned();
    assert!(!value.admissible_for(&requirement(proposition, strong_guarantees())));
}

#[test]
fn a2_11_proposition_substitution_cannot_satisfy_bound_requirement() {
    let value = record(
        attempt_proposition("$WORKSPACE/input.txt"),
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
    );
    let wrong = attempt_proposition("$WORKSPACE/other.txt");
    assert!(!value.admissible_for(&requirement(wrong, weak_guarantees())));
}

#[test]
fn a2_12_unsupported_record_is_never_admissible() {
    let proposition = attempt_proposition("$WORKSPACE/input.txt");
    let mut value = record(
        proposition.clone(),
        weak_guarantees(),
        AuthorityState::Unsupported,
        PropositionCompleteness::NotApplicable,
    );
    value.reason_codes.insert("unsupported_family".to_owned());
    assert!(value.validate().is_ok());
    assert!(!value.admissible_for(&requirement(proposition, weak_guarantees())));
}

#[test]
fn a2_13_attempt_only_satisfies_only_exact_attempt_requirement() {
    let proposition = attempt_proposition("$WORKSPACE/input.txt");
    let value = record(
        proposition.clone(),
        weak_guarantees(),
        AuthorityState::AttemptOnly,
        PropositionCompleteness::Complete,
    );
    assert!(value.validate().is_ok());
    assert!(value.admissible_for(&requirement(proposition, weak_guarantees())));
    assert!(!value.admissible_for(&requirement(success_proposition(), strong_guarantees())));
}

#[test]
fn a2_14_different_typed_propositions_never_compare_equivalent() {
    let left_proposition = attempt_proposition("$WORKSPACE/input.txt");
    let right_proposition = attempt_proposition("$WORKSPACE/other.txt");
    let left = record(
        left_proposition.clone(),
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
    );
    let right = record(
        right_proposition,
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
    );
    assert_eq!(
        compare_under_requirement(&left, &right, &requirement(left_proposition, weak_guarantees())),
        ComparisonResult::DifferentProposition
    );
}

#[test]
fn a2_15_reason_code_serialization_is_deterministic() {
    let proposition = attempt_proposition("$WORKSPACE/input.txt");
    let mut first = record(
        proposition.clone(),
        weak_guarantees(),
        AuthorityState::Unsupported,
        PropositionCompleteness::NotApplicable,
    );
    first.reason_codes.insert("zeta".to_owned());
    first.reason_codes.insert("alpha".to_owned());
    let mut second = record(
        proposition,
        weak_guarantees(),
        AuthorityState::Unsupported,
        PropositionCompleteness::NotApplicable,
    );
    second.reason_codes.insert("alpha".to_owned());
    second.reason_codes.insert("zeta".to_owned());
    assert_eq!(serde_json::to_vec(&first).unwrap(), serde_json::to_vec(&second).unwrap());
}

#[test]
fn a2_16_semantics_v3_ambiguity_still_fails_closed_through_authority() {
    let proposition = attempt_proposition("$WORKSPACE/input.txt");
    let mut value = record(
        proposition.clone(),
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
    );
    value.proof.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Complete,
    );
    value
        .proof
        .ambiguity_codes
        .insert("object_identity_conflict".to_owned());
    let requirement = ProofRequirement::for_proposition(
        proposition,
        weak_guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity]),
    );
    assert!(!value.admissible_for(&requirement));
}
