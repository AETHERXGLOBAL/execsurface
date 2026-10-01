use std::collections::BTreeSet;

use execsurface_model::canonical::{CanonicalExecutable, CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::FileOperation;
use execsurface_p4_backend_authority::{
    compare_under_requirement, AdapterRecord, AuthorityState, ComparisonResult, EvidenceReference,
    PropositionCompleteness,
};

fn path(value: &str) -> CanonicalPath {
    CanonicalPath {
        value: value.to_owned(),
        class: PathClass::Workspace,
        resolution: PathResolution::Lexical,
    }
}

fn exe(value: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: CanonicalPath {
            value: value.to_owned(),
            class: PathClass::System,
            resolution: PathResolution::Lexical,
        },
        family: value.rsplit('/').next().unwrap_or(value).to_owned(),
    }
}

fn proposition(actor: &str, target: &str) -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: Some(exe(actor)),
        execution_chain: vec![exe("/bin/bash"), exe(actor)],
        operation: FileOperation::Open,
        target: path(target),
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

fn stronger_guarantees() -> EvidenceGuarantees {
    let mut guarantees = lexical_guarantees();
    guarantees
        .observation_points
        .insert(ObservationPoint::SyscallResultPostOperation);
    guarantees
        .temporal_bindings
        .insert(TemporalBinding::SuccessfulOperationResult);
    guarantees
}

fn requirement_for(proposition: Proposition) -> ProofRequirement {
    ProofRequirement::for_proposition(
        proposition,
        lexical_guarantees(),
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Capability,
        ]),
    )
}

fn record(
    proposition_id: &str,
    proposition: Proposition,
    authority: AuthorityState,
    guarantees: EvidenceGuarantees,
) -> AdapterRecord {
    let mut proof = ProofCarryingObservation::new(
        proposition,
        guarantees,
        BackendSemanticProfile {
            name: "promotion-a2-ptrace".to_owned(),
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

    AdapterRecord {
        proposition_id: proposition_id.to_owned(),
        proof,
        authority,
        completeness: PropositionCompleteness::Complete,
        evidence: EvidenceReference {
            digest: format!("sha256:{}", "a".repeat(64)),
            derivation: None,
        },
        reason_codes: BTreeSet::new(),
    }
}

#[test]
fn s01_backend_profile_name_cannot_rescue_proposition_mismatch() {
    let expected = proposition("/usr/bin/cat", "$WORKSPACE/input.txt");
    let requirement = requirement_for(expected);
    let mut substituted = record(
        "P4.PATH.ACCESS_ATTEMPT",
        proposition("/usr/bin/python3", "$WORKSPACE/other.txt"),
        AuthorityState::Direct,
        lexical_guarantees(),
    );
    substituted.proof.backend_profile.name = "trusted-perfect-kernel-authority".to_owned();
    substituted.proof.backend_profile.semantic_profile_version = u32::MAX;
    assert!(!substituted.admissible_for(&requirement));
}

#[test]
fn s02_matching_direct_record_satisfies_only_bound_contract() {
    let expected = proposition("/usr/bin/cat", "$WORKSPACE/input.txt");
    let candidate = record(
        "P4.FILE.OPEN_OBJECT",
        expected.clone(),
        AuthorityState::Direct,
        lexical_guarantees(),
    );
    assert!(candidate.admissible_for(&requirement_for(expected)));
    assert!(!candidate.admissible_for(&requirement_for(proposition(
        "/usr/bin/cat",
        "$WORKSPACE/other.txt"
    ))));
}

#[test]
fn s03_attempt_only_is_exact_path_attempt_not_open_object_success() {
    let p = proposition("/usr/bin/cat", "$WORKSPACE/input.txt");
    let requirement = requirement_for(p.clone());
    let attempt = record(
        "P4.PATH.ACCESS_ATTEMPT",
        p.clone(),
        AuthorityState::AttemptOnly,
        lexical_guarantees(),
    );
    assert!(attempt.admissible_for(&requirement));

    let relabeled = record(
        "P4.FILE.OPEN_OBJECT",
        p,
        AuthorityState::AttemptOnly,
        lexical_guarantees(),
    );
    assert!(!relabeled.admissible_for(&requirement));
}

#[test]
fn s04_unsupported_cannot_validate_as_complete() {
    let mut candidate = record(
        "P4.FILE.OPEN_OBJECT",
        proposition("/usr/bin/cat", "$WORKSPACE/input.txt"),
        AuthorityState::Unsupported,
        lexical_guarantees(),
    );
    candidate.completeness = PropositionCompleteness::Complete;
    assert!(candidate.validate().is_err());
}

#[test]
fn s05_ambiguous_and_lost_cannot_validate_as_complete() {
    for authority in [AuthorityState::Ambiguous, AuthorityState::Lost] {
        let mut candidate = record(
            "P4.FILE.OPEN_OBJECT",
            proposition("/usr/bin/cat", "$WORKSPACE/input.txt"),
            authority,
            lexical_guarantees(),
        );
        candidate.reason_codes.insert("controlled_gap".to_owned());
        assert!(candidate.validate().is_err());
    }
}

#[test]
fn s06_derived_bounded_requires_named_derivation() {
    let mut candidate = record(
        "P4.FILE.OPEN_OBJECT",
        proposition("/usr/bin/cat", "$WORKSPACE/input.txt"),
        AuthorityState::DerivedBounded,
        lexical_guarantees(),
    );
    assert!(candidate.validate().is_err());
    candidate.evidence.derivation = Some("bounded-state-machine-v1".to_owned());
    assert!(candidate.validate().is_ok());
}

#[test]
fn s07_different_propositions_compare_as_different_even_with_empty_requirement() {
    let left = record(
        "P4.PATH.ACCESS_ATTEMPT",
        proposition("/usr/bin/cat", "$WORKSPACE/input.txt"),
        AuthorityState::Direct,
        lexical_guarantees(),
    );
    let right = record(
        "P4.PATH.ACCESS_ATTEMPT",
        proposition("/usr/bin/python3", "$WORKSPACE/input.txt"),
        AuthorityState::Direct,
        lexical_guarantees(),
    );
    assert_eq!(
        compare_under_requirement(&left, &right, &ProofRequirement::default()),
        ComparisonResult::DifferentProposition
    );
}

#[test]
fn s08_default_requirement_cannot_admit_valid_record() {
    let candidate = record(
        "P4.FILE.OPEN_OBJECT",
        proposition("/usr/bin/cat", "$WORKSPACE/input.txt"),
        AuthorityState::Direct,
        lexical_guarantees(),
    );
    assert!(!candidate.admissible_for(&ProofRequirement::default()));
}

#[test]
fn s09_unknown_ambiguity_cannot_silently_grant_authority() {
    let p = proposition("/usr/bin/cat", "$WORKSPACE/input.txt");
    let requirement = requirement_for(p.clone());
    let mut candidate = record(
        "P4.FILE.OPEN_OBJECT",
        p,
        AuthorityState::Direct,
        lexical_guarantees(),
    );
    candidate
        .proof
        .ambiguity_codes
        .insert("future_unclassified_authority_gap".to_owned());
    assert!(!candidate.admissible_for(&requirement));
}

#[test]
fn s10_stronger_guarantees_cannot_rescue_mismatched_proposition() {
    let requirement = requirement_for(proposition("/usr/bin/cat", "$WORKSPACE/input.txt"));
    let candidate = record(
        "P4.FILE.OPEN_OBJECT",
        proposition("/usr/bin/python3", "$WORKSPACE/other.txt"),
        AuthorityState::Direct,
        stronger_guarantees(),
    );
    assert!(!candidate.admissible_for(&requirement));
}
