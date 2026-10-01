use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    build_bundle, AuthorityState, CapabilityState, CompletenessClass, CompletenessState,
    ObserverHealth, ProvenanceReference, ResourceDescriptor, Verdict, VerificationInput,
    SLSA_PROVENANCE_TYPE,
};
use execsurface_p5_cross_attestation_redteam::{
    manifest_for_bundle, verify_graph, ExpectedVerificationContext, GraphItem, ROLE_PROVENANCE,
};

fn hex(ch: char) -> String {
    std::iter::repeat_n(ch, 64).collect()
}

fn labeled(ch: char) -> String {
    format!("sha256:{}", hex(ch))
}

fn descriptor(name: &str, ch: char, uri: &str) -> ResourceDescriptor {
    ResourceDescriptor {
        name: Some(name.to_owned()),
        uri: Some(uri.to_owned()),
        digest: BTreeMap::from([("sha256".to_owned(), hex(ch))]),
        media_type: None,
    }
}

fn direct_complete_input() -> VerificationInput {
    let subject = descriptor(
        "a3-subject",
        'a',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/a3-subject",
    );
    VerificationInput {
        subject: subject.clone(),
        command_identity: "cargo test --locked".to_owned(),
        host_identity: "github-hosted-linux-x86_64".to_owned(),
        source_identity: Some(descriptor(
            "source",
            'b',
            "https://github.com/AETHERXGLOBAL/execsurface/tree/a3-source",
        )),
        artifact_identity: Some(subject),
        workflow_identity: Some(descriptor(
            "workflow",
            'c',
            "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/a3",
        )),
        baseline_digest: labeled('d'),
        current_surface_digest: labeled('e'),
        observer_profile: "ptrace-bounded-authority-v1".to_owned(),
        capability_state: CapabilityState {
            capabilities: BTreeSet::from([
                "process.exec.success".to_owned(),
                "file.open.object".to_owned(),
            ]),
        },
        observer_health: ObserverHealth::Healthy,
        authority: AuthorityState::Direct,
        completeness: CompletenessState {
            class: CompletenessClass::Complete,
            reason_codes: BTreeSet::new(),
        },
        policy: descriptor(
            "policy",
            'f',
            "https://github.com/AETHERXGLOBAL/execsurface/policies/a3",
        ),
        verdict: Verdict::Pass,
        evidence_digest: labeled('1'),
        verifier: descriptor(
            "verifier",
            '2',
            "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1",
        ),
        verifier_id: "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1".to_owned(),
        created_at: "2026-10-01T11:45:00Z".to_owned(),
        slsa_provenance: Some(ProvenanceReference {
            predicate_type: SLSA_PROVENANCE_TYPE.to_owned(),
            statement_digest: labeled('3'),
            subject_digest: labeled('a'),
            verified: true,
        }),
    }
}

fn ambiguous_review_input() -> VerificationInput {
    let mut input = direct_complete_input();
    input.verdict = Verdict::Review;
    input.authority = AuthorityState::Ambiguous;
    input.completeness = CompletenessState {
        class: CompletenessClass::Incomplete,
        reason_codes: BTreeSet::from(["authority_ambiguous".to_owned()]),
    };
    input
}

fn lost_review_input() -> VerificationInput {
    let mut input = ambiguous_review_input();
    input.observer_health = ObserverHealth::Lost;
    input.completeness = CompletenessState {
        class: CompletenessClass::Lost,
        reason_codes: BTreeSet::from(["observer_transport_lost".to_owned()]),
    };
    input
}

fn expected(input: &VerificationInput) -> ExpectedVerificationContext {
    ExpectedVerificationContext {
        subject: input.subject.clone(),
        source_identity: input.source_identity.clone(),
        artifact_identity: input.artifact_identity.clone(),
        workflow_identity: input.workflow_identity.clone(),
        command_identity: input.command_identity.clone(),
        host_identity: input.host_identity.clone(),
        baseline_digest: input.baseline_digest.clone(),
        current_surface_digest: input.current_surface_digest.clone(),
        evidence_digest: input.evidence_digest.clone(),
        observer_profile: input.observer_profile.clone(),
        capability_state: input.capability_state.clone(),
        observer_health: input.observer_health,
        policy: input.policy.clone(),
        authority: input.authority,
        completeness: input.completeness.clone(),
        verdict: input.verdict,
        verifier: input.verifier.clone(),
        verifier_id: input.verifier_id.clone(),
        created_at: input.created_at.clone(),
        provenance: input.slsa_provenance.clone(),
    }
}

fn accepted_graph(input: &VerificationInput) -> (execsurface_p5_attestation_provenance::VerificationBundle, ExpectedVerificationContext, Vec<GraphItem>) {
    let bundle = build_bundle(input).expect("fixture must build");
    let context = expected(input);
    let manifest = manifest_for_bundle(&bundle).expect("manifest must build");
    verify_graph(&bundle, &context, &manifest).expect("fixture must verify");
    (bundle, context, manifest)
}

#[test]
fn a3_01_verified_provenance_does_not_inflate_ambiguous_review() {
    let input = ambiguous_review_input();
    let (bundle, context, manifest) = accepted_graph(&input);
    assert_eq!(context.verdict, Verdict::Review);
    assert_eq!(context.authority, AuthorityState::Ambiguous);
    verify_graph(&bundle, &context, &manifest).expect("bounded REVIEW must remain representable");
}

#[test]
fn a3_02_verified_provenance_cannot_launder_ambiguous_authority_to_pass() {
    let mut input = ambiguous_review_input();
    input.verdict = Verdict::Pass;
    let error = build_bundle(&input).expect_err("ambiguous PASS must fail closed");
    assert_eq!(error.reason_code, "pass_blocked_by_non_direct_authority");
}

#[test]
fn a3_03_verified_provenance_cannot_launder_observer_loss_to_pass() {
    let mut input = lost_review_input();
    input.verdict = Verdict::Pass;
    let error = build_bundle(&input).expect_err("lost-observer PASS must fail closed");
    assert_eq!(error.reason_code, "pass_blocked_by_observer_loss");
}

#[test]
fn a3_04_cross_subject_provenance_replay_is_rejected() {
    let mut input = direct_complete_input();
    input.slsa_provenance.as_mut().expect("provenance").subject_digest = labeled('9');
    let error = build_bundle(&input).expect_err("cross-subject provenance must fail");
    assert_eq!(error.reason_code, "slsa_subject_mismatch");
}

#[test]
fn a3_05_provenance_predicate_type_substitution_is_rejected() {
    let mut input = direct_complete_input();
    input.slsa_provenance.as_mut().expect("provenance").predicate_type =
        "https://example.invalid/fake-provenance/v1".to_owned();
    let error = build_bundle(&input).expect_err("predicate substitution must fail");
    assert_eq!(error.reason_code, "slsa_predicate_type_mismatch");
}

#[test]
fn a3_06_malformed_provenance_statement_digest_is_rejected() {
    let mut input = direct_complete_input();
    input.slsa_provenance.as_mut().expect("provenance").statement_digest =
        "sha256:not-a-digest".to_owned();
    let error = build_bundle(&input).expect_err("malformed provenance digest must fail");
    assert_eq!(error.reason_code, "slsa_statement_digest_malformed");
}

#[test]
fn a3_07_source_substitution_fails_expected_context_binding() {
    let input = direct_complete_input();
    let (bundle, mut context, manifest) = accepted_graph(&input);
    context.source_identity = Some(descriptor(
        "other-source",
        '8',
        "https://github.com/AETHERXGLOBAL/execsurface/tree/other-source",
    ));
    assert!(verify_graph(&bundle, &context, &manifest).is_err());
}

#[test]
fn a3_08_baseline_substitution_fails_expected_context_binding() {
    let input = direct_complete_input();
    let (bundle, mut context, manifest) = accepted_graph(&input);
    context.baseline_digest = labeled('8');
    assert!(verify_graph(&bundle, &context, &manifest).is_err());
}

#[test]
fn a3_09_current_surface_substitution_fails_expected_context_binding() {
    let input = direct_complete_input();
    let (bundle, mut context, manifest) = accepted_graph(&input);
    context.current_surface_digest = labeled('9');
    assert!(verify_graph(&bundle, &context, &manifest).is_err());
}

#[test]
fn a3_10_verifier_identity_substitution_fails_expected_context_binding() {
    let input = direct_complete_input();
    let (bundle, mut context, manifest) = accepted_graph(&input);
    context.verifier = descriptor(
        "other-verifier",
        '9',
        "https://github.com/AETHERXGLOBAL/execsurface/verifier/other",
    );
    context.verifier_id = "https://github.com/AETHERXGLOBAL/execsurface/verifier/other".to_owned();
    assert!(verify_graph(&bundle, &context, &manifest).is_err());
}

#[test]
fn a3_11_provenance_verification_state_is_bound_but_not_semantic_authority() {
    let mut input = direct_complete_input();
    input.slsa_provenance.as_mut().expect("provenance").verified = false;
    let (bundle, mut context, manifest) = accepted_graph(&input);

    assert_eq!(context.verdict, Verdict::Pass);
    assert_eq!(context.authority, AuthorityState::Direct);

    context.provenance.as_mut().expect("provenance").verified = true;
    assert!(
        verify_graph(&bundle, &context, &manifest).is_err(),
        "verified=false provenance context must not satisfy verified=true expectation"
    );
}

#[test]
fn a3_12_duplicate_or_substituted_provenance_graph_role_fails_closed() {
    let input = direct_complete_input();
    let (bundle, context, manifest) = accepted_graph(&input);

    let provenance = manifest
        .iter()
        .find(|item| item.role == ROLE_PROVENANCE)
        .expect("provenance role present")
        .clone();

    let mut duplicate = manifest.clone();
    duplicate.push(provenance);
    assert!(verify_graph(&bundle, &context, &duplicate).is_err());

    let mut substituted = manifest;
    let provenance_item = substituted
        .iter_mut()
        .find(|item| item.role == ROLE_PROVENANCE)
        .expect("provenance role present");
    provenance_item.digest = labeled('9');
    assert!(verify_graph(&bundle, &context, &substituted).is_err());
}
