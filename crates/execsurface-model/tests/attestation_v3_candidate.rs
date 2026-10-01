#![cfg(feature = "semantics-v3")]

use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::attestation_v3::{
    manifest_for, verify_manifest, AttestedVerification, DigestBinding, ObserverHealth,
    ProvenanceBinding, VerificationBindings, VerificationVerdict, SLSA_PROVENANCE_V1,
};
use execsurface_model::authority_v3::{
    AuthorityRecord, AuthorityState, EvidenceReference, PropositionCompleteness,
};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CompletenessDimension, CompletenessState, EvidenceGuarantees,
    ObservationPoint, ProofCarryingObservation, ProofRequirement, Proposition,
};

fn digest(ch: char) -> DigestBinding {
    DigestBinding::sha256(std::iter::repeat_n(ch, 64).collect::<String>())
}

fn proposition() -> Proposition {
    Proposition::ObserverHealthObserved {
        complete: true,
        warning_codes: BTreeSet::new(),
    }
}

fn requirement() -> ProofRequirement {
    let mut points = BTreeSet::new();
    points.insert(ObservationPoint::ImportedAttestedTrace);
    let guarantees = EvidenceGuarantees {
        observation_points: points,
        ..EvidenceGuarantees::default()
    };
    let mut required = BTreeSet::new();
    required.insert(CompletenessDimension::SessionScope);
    ProofRequirement::for_proposition(proposition(), guarantees, required)
}

fn authority(state: AuthorityState, completeness: PropositionCompleteness) -> AuthorityRecord {
    let req = requirement();
    let mut proof = ProofCarryingObservation::new(
        proposition(),
        req.guarantees.clone(),
        BackendSemanticProfile {
            name: "bounded-test".into(),
            semantic_profile_version: 1,
        },
    );
    proof.completeness = BTreeMap::from([(
        CompletenessDimension::SessionScope,
        CompletenessState::Complete,
    )]);
    let mut reason_codes = BTreeSet::new();
    if matches!(
        state,
        AuthorityState::Ambiguous | AuthorityState::Lost | AuthorityState::Unsupported
    ) {
        reason_codes.insert("bounded-test-reason".into());
    }
    AuthorityRecord {
        proof,
        authority: state,
        completeness,
        evidence: EvidenceReference {
            digest: digest('e').value,
            derivation: None,
        },
        reason_codes,
    }
}

fn bindings(verified: bool) -> VerificationBindings {
    VerificationBindings {
        subject: digest('a'),
        source: digest('b'),
        baseline: digest('c'),
        current_surface: digest('d'),
        evidence: digest('e'),
        verifier: digest('f'),
        provenance: Some(ProvenanceBinding {
            predicate_type: SLSA_PROVENANCE_V1.into(),
            statement_digest: digest('1'),
            subject_digest: digest('a'),
            verified,
        }),
        workflow_label: Some("workflow-a".into()),
        signer_label: Some("signer-a".into()),
    }
}

fn attested(
    state: AuthorityState,
    completeness: PropositionCompleteness,
    health: ObserverHealth,
    verdict: VerificationVerdict,
    verified: bool,
) -> AttestedVerification {
    AttestedVerification {
        authority: authority(state, completeness),
        observer_health: health,
        verdict,
        bindings: bindings(verified),
    }
}

#[test]
fn verified_provenance_does_not_inflate_ambiguous_authority() {
    let value = attested(
        AuthorityState::Ambiguous,
        PropositionCompleteness::Incomplete,
        ObserverHealth::Healthy,
        VerificationVerdict::Review,
        true,
    );
    assert!(!value.semantic_pass_admissible(&requirement()));
}

#[test]
fn pass_label_cannot_launder_ambiguous_authority() {
    let value = attested(
        AuthorityState::Ambiguous,
        PropositionCompleteness::Incomplete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    assert!(!value.semantic_pass_admissible(&requirement()));
}

#[test]
fn observer_loss_cannot_be_laundered_by_verified_provenance() {
    let value = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Lost,
        VerificationVerdict::Pass,
        true,
    );
    assert!(!value.semantic_pass_admissible(&requirement()));
}

#[test]
fn cross_subject_provenance_replay_is_rejected() {
    let mut value = bindings(true);
    value.provenance.as_mut().unwrap().subject_digest = digest('9');
    assert!(value.validate().is_err());
}

#[test]
fn provenance_predicate_substitution_is_rejected() {
    let mut value = bindings(true);
    value.provenance.as_mut().unwrap().predicate_type =
        "https://example.invalid/predicate".into();
    assert!(value.validate().is_err());
}

#[test]
fn malformed_provenance_statement_digest_is_rejected() {
    let mut value = bindings(true);
    value.provenance.as_mut().unwrap().statement_digest = DigestBinding {
        value: "sha256:not-a-digest".into(),
    };
    assert!(value.validate().is_err());
}

#[test]
fn source_substitution_fails_expected_context() {
    let expected = bindings(true);
    let mut value = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.bindings.source = digest('9');
    assert!(!value.matches_expected_bindings(&expected));
}

#[test]
fn baseline_substitution_fails_expected_context() {
    let expected = bindings(true);
    let mut value = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.bindings.baseline = digest('9');
    assert!(!value.matches_expected_bindings(&expected));
}

#[test]
fn current_surface_substitution_fails_expected_context() {
    let expected = bindings(true);
    let mut value = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.bindings.current_surface = digest('9');
    assert!(!value.matches_expected_bindings(&expected));
}

#[test]
fn verifier_substitution_fails_expected_context() {
    let expected = bindings(true);
    let mut value = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.bindings.verifier = digest('9');
    assert!(!value.matches_expected_bindings(&expected));
}

#[test]
fn provenance_verification_state_is_bound_but_not_semantic_authority() {
    let verified = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    let unverified = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        false,
    );
    assert!(verified.semantic_pass_admissible(&requirement()));
    assert!(unverified.semantic_pass_admissible(&requirement()));
    assert!(!verified.matches_expected_bindings(&unverified.bindings));
}

#[test]
fn duplicate_manifest_role_fails_closed() {
    let value = bindings(true);
    let mut manifest = manifest_for(&value).unwrap();
    manifest.push(manifest[0].clone());
    assert!(verify_manifest(&value, &manifest).is_err());
}

#[test]
fn informational_labels_never_change_semantic_pass() {
    let mut value = attested(
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    assert!(value.semantic_pass_admissible(&requirement()));
    value.bindings.workflow_label = Some("forged-workflow".into());
    value.bindings.signer_label = Some("forged-signer".into());
    assert!(value.semantic_pass_admissible(&requirement()));
}

#[test]
fn manifest_order_is_deterministic() {
    let value = bindings(true);
    let first = manifest_for(&value).unwrap();
    let second = manifest_for(&value).unwrap();
    assert_eq!(first, second);
}
