#![cfg(feature = "semantics-v3")]

use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::attestation_v3::{
    manifest_for, verify_manifest, AttestedVerification, DigestBinding, ObserverHealth,
    ProvenanceBinding, VerificationBindings, VerificationVerdict, SLSA_PROVENANCE_V1,
};
use execsurface_model::authority_v3::{
    AuthorityRecord, AuthorityState, EvidenceReference, PropositionCompleteness,
};
use execsurface_model::canonical::{CanonicalExecutable, CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::FileOperation;

fn digest(ch: char) -> DigestBinding {
    DigestBinding::sha256(std::iter::repeat_n(ch, 64).collect::<String>())
}

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

fn proposition(target: &str) -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: Some(executable("/usr/bin/cat", "cat")),
        execution_chain: vec![
            executable("/bin/bash", "bash"),
            executable("/usr/bin/cat", "cat"),
        ],
        operation: FileOperation::Open,
        target: path(target),
        open_intent: None,
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

fn requirement(target: &str, guarantees: EvidenceGuarantees) -> ProofRequirement {
    ProofRequirement::for_proposition(
        proposition(target),
        guarantees,
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Capability,
        ]),
    )
}

fn authority(
    target: &str,
    guarantees: EvidenceGuarantees,
    authority: AuthorityState,
    completeness: PropositionCompleteness,
) -> AuthorityRecord {
    let mut proof = ProofCarryingObservation::new(
        proposition(target),
        guarantees,
        BackendSemanticProfile {
            name: "ptrace-candidate".into(),
            semantic_profile_version: 1,
        },
    );
    proof.completeness = BTreeMap::from([
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::Capability,
            CompletenessState::Complete,
        ),
    ]);

    let mut reason_codes = BTreeSet::new();
    if matches!(authority, AuthorityState::Ambiguous | AuthorityState::Lost) {
        reason_codes.insert("bounded-cross-layer-reason".into());
    }
    if authority == AuthorityState::Unsupported {
        reason_codes.insert("unsupported_family".into());
    }

    AuthorityRecord {
        proof,
        authority,
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
        workflow_label: Some("candidate-final".into()),
        signer_label: Some("candidate-signer".into()),
    }
}

fn attested(
    target: &str,
    guarantees: EvidenceGuarantees,
    authority_state: AuthorityState,
    completeness: PropositionCompleteness,
    health: ObserverHealth,
    verdict: VerificationVerdict,
    provenance_verified: bool,
) -> AttestedVerification {
    AttestedVerification {
        authority: authority(target, guarantees, authority_state, completeness),
        observer_health: health,
        verdict,
        bindings: bindings(provenance_verified),
    }
}

#[test]
fn x01_wrong_proposition_cannot_be_laundered_by_pass_and_verified_provenance() {
    let value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    assert!(!value.semantic_pass_admissible(&requirement(
        "$WORKSPACE/b.txt",
        weak_guarantees()
    )));
}

#[test]
fn x02_ambiguous_authority_cannot_be_laundered_by_attestation() {
    let value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Ambiguous,
        PropositionCompleteness::Incomplete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    assert!(value.validate().is_ok());
    assert!(!value.semantic_pass_admissible(&requirement(
        "$WORKSPACE/a.txt",
        weak_guarantees()
    )));
}

#[test]
fn x03_observer_loss_blocks_semantic_pass_even_with_direct_authority_and_provenance() {
    let value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Lost,
        VerificationVerdict::Pass,
        true,
    );
    assert!(!value.semantic_pass_admissible(&requirement(
        "$WORKSPACE/a.txt",
        weak_guarantees()
    )));
}

#[test]
fn x04_backend_signer_and_workflow_labels_cannot_upgrade_weak_guarantees() {
    let mut value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.authority.proof.backend_profile.name = "kernel-super-authoritative".into();
    value.bindings.signer_label = Some("root-of-trust".into());
    value.bindings.workflow_label = Some("production-approved".into());
    assert!(!value.semantic_pass_admissible(&requirement(
        "$WORKSPACE/a.txt",
        strong_guarantees()
    )));
}

#[test]
fn x05_cross_subject_provenance_substitution_fails_before_semantic_admission() {
    let mut value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.bindings.provenance.as_mut().unwrap().subject_digest = digest('9');
    assert!(value.validate().is_err());
    assert!(!value.semantic_pass_admissible(&requirement(
        "$WORKSPACE/a.txt",
        weak_guarantees()
    )));
}

#[test]
fn x06_expected_verifier_context_cannot_be_replayed_across_attestations() {
    let expected = bindings(true);
    let mut value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.bindings.verifier = digest('9');
    assert!(value.semantic_pass_admissible(&requirement(
        "$WORKSPACE/a.txt",
        weak_guarantees()
    )));
    assert!(!value.matches_expected_bindings(&expected));
}

#[test]
fn x07_default_empty_requirement_never_becomes_a_cross_layer_bypass() {
    let value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    assert!(!value.semantic_pass_admissible(&ProofRequirement::default()));
}

#[test]
fn x08_invalidating_semantic_ambiguity_survives_authority_and_attestation_layers() {
    let mut value = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    value.authority.proof.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Complete,
    );
    value
        .authority
        .proof
        .ambiguity_codes
        .insert("object_identity_conflict".into());
    let requirement = ProofRequirement::for_proposition(
        proposition("$WORKSPACE/a.txt"),
        weak_guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity]),
    );
    assert!(!value.semantic_pass_admissible(&requirement));
}

#[test]
fn x09_duplicate_manifest_role_fails_closed_on_integrated_binding_set() {
    let value = bindings(true);
    let mut manifest = manifest_for(&value).unwrap();
    manifest.push(manifest[0].clone());
    assert!(verify_manifest(&value, &manifest).is_err());
}

#[test]
fn x10_provenance_verification_status_never_manufactures_semantic_authority() {
    let verified = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        true,
    );
    let unverified = attested(
        "$WORKSPACE/a.txt",
        weak_guarantees(),
        AuthorityState::Direct,
        PropositionCompleteness::Complete,
        ObserverHealth::Healthy,
        VerificationVerdict::Pass,
        false,
    );
    let req = requirement("$WORKSPACE/a.txt", weak_guarantees());
    assert!(verified.semantic_pass_admissible(&req));
    assert!(unverified.semantic_pass_admissible(&req));
    assert_ne!(verified.bindings, unverified.bindings);
}
