use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    build_bundle, digest_bundle, parse_strict_input, verify_bundle, verify_bundle_for_subject,
    AuthorityState, CapabilityState, CompletenessClass, CompletenessState, ObserverHealth,
    ProvenanceReference, ResourceDescriptor, Verdict, VerificationInput, RUNTIME_TRACE_TYPE,
    SCAI_TYPE, SLSA_PROVENANCE_TYPE, STATEMENT_TYPE, SVR_PASS_PROPERTY, SVR_TYPE,
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

fn complete_input() -> VerificationInput {
    let subject = descriptor(
        "execsurface-fixture",
        'a',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/fixture",
    );
    VerificationInput {
        subject: subject.clone(),
        command_identity: "cargo test --locked".to_owned(),
        host_identity: "https://github.com/AETHERXGLOBAL/execsurface/actions/runner/linux-x86_64"
            .to_owned(),
        source_identity: Some(descriptor(
            "source",
            'b',
            "https://github.com/AETHERXGLOBAL/execsurface/tree/source-fixture",
        )),
        artifact_identity: Some(subject),
        workflow_identity: Some(descriptor(
            "workflow",
            'c',
            "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/p5-a0",
        )),
        baseline_digest: labeled('d'),
        current_surface_digest: labeled('e'),
        observer_profile: "ptrace-research-profile-v1".to_owned(),
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
            "verification-policy",
            'f',
            "https://github.com/AETHERXGLOBAL/execsurface/policies/fixture",
        ),
        verdict: Verdict::Pass,
        evidence_digest: labeled('1'),
        verifier: descriptor(
            "execsurface-verifier",
            '2',
            "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1",
        ),
        verifier_id: "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1".to_owned(),
        created_at: "2026-09-30T15:00:00Z".to_owned(),
        slsa_provenance: Some(ProvenanceReference {
            predicate_type: SLSA_PROVENANCE_TYPE.to_owned(),
            statement_digest: labeled('3'),
            subject_digest: labeled('a'),
            verified: true,
        }),
    }
}

fn review_input() -> VerificationInput {
    let mut input = complete_input();
    input.verdict = Verdict::Review;
    input.authority = AuthorityState::Ambiguous;
    input.completeness = CompletenessState {
        class: CompletenessClass::Incomplete,
        reason_codes: BTreeSet::from(["observer_scope_incomplete".to_owned()]),
    };
    input
}

#[test]
fn a0_01_complete_fixture_represents_every_mandatory_binding() {
    let input = complete_input();
    let bundle = build_bundle(&input).expect("complete fixture must build");
    let conditions = &bundle.scai.predicate.attributes[0].conditions;

    assert_eq!(bundle.runtime_trace.statement_type, STATEMENT_TYPE);
    assert_eq!(bundle.runtime_trace.predicate_type, RUNTIME_TRACE_TYPE);
    assert_eq!(bundle.scai.predicate_type, SCAI_TYPE);
    assert_eq!(bundle.svr.predicate_type, SVR_TYPE);
    assert_eq!(conditions.command_identity, input.command_identity);
    assert_eq!(conditions.source_identity, input.source_identity);
    assert_eq!(conditions.artifact_identity, input.artifact_identity);
    assert_eq!(conditions.workflow_identity, input.workflow_identity);
    assert_eq!(conditions.baseline_digest, input.baseline_digest);
    assert_eq!(
        conditions.current_surface_digest,
        input.current_surface_digest
    );
    assert_eq!(conditions.observer_profile, input.observer_profile);
    assert_eq!(conditions.capability_state, input.capability_state);
    assert!(conditions.capability_digest.starts_with("sha256:"));
    assert!(conditions.completeness_digest.starts_with("sha256:"));
    assert_eq!(conditions.policy_digest, labeled('f'));
    assert_eq!(conditions.verdict, Verdict::Pass);
    assert_eq!(conditions.evidence_digest, input.evidence_digest);
    assert_eq!(
        conditions.runtime_trace_statement_digest,
        bundle.runtime_trace_digest
    );
    assert_eq!(conditions.slsa_provenance, input.slsa_provenance);
    assert_eq!(conditions.verifier_identity, input.verifier);
}

#[test]
fn a0_02_identical_semantic_input_serializes_identically() {
    let first = build_bundle(&complete_input()).expect("first build");
    let second = build_bundle(&complete_input()).expect("second build");
    assert_eq!(first, second);
    assert_eq!(digest_bundle(&first), digest_bundle(&second));
    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}

#[test]
fn a0_03_map_and_set_insertion_order_do_not_change_identity() {
    let first = complete_input();
    let mut second = complete_input();
    second.capability_state.capabilities = [
        "file.open.object".to_owned(),
        "process.exec.success".to_owned(),
    ]
    .into_iter()
    .collect();
    let mut policy_digests = BTreeMap::new();
    policy_digests.insert("sha512".to_owned(), "9".repeat(128));
    policy_digests.insert("sha256".to_owned(), hex('f'));
    second.policy.digest = policy_digests;

    let mut first_with_extra = first;
    first_with_extra
        .policy
        .digest
        .insert("sha512".to_owned(), "9".repeat(128));

    let first_bundle = build_bundle(&first_with_extra).expect("first ordered build");
    let second_bundle = build_bundle(&second).expect("second ordered build");
    assert_eq!(first_bundle, second_bundle);
}

#[test]
fn a0_04_baseline_digest_mutation_changes_assertion_identity() {
    let first = build_bundle(&complete_input()).expect("base");
    let mut changed = complete_input();
    changed.baseline_digest = labeled('4');
    let second = build_bundle(&changed).expect("changed");
    assert_ne!(first.scai_digest, second.scai_digest);
    assert_ne!(first.bundle_digest, second.bundle_digest);
}

#[test]
fn a0_05_current_surface_digest_mutation_changes_assertion_identity() {
    let first = build_bundle(&complete_input()).expect("base");
    let mut changed = complete_input();
    changed.current_surface_digest = labeled('5');
    let second = build_bundle(&changed).expect("changed");
    assert_ne!(first.scai_digest, second.scai_digest);
}

#[test]
fn a0_06_policy_digest_mutation_changes_assertion_identity() {
    let first = build_bundle(&complete_input()).expect("base");
    let mut changed = complete_input();
    changed.policy.digest.insert("sha256".to_owned(), hex('6'));
    let second = build_bundle(&changed).expect("changed");
    assert_ne!(first.runtime_trace_digest, second.runtime_trace_digest);
    assert_ne!(first.scai_digest, second.scai_digest);
    assert_ne!(first.svr_digest, second.svr_digest);
}

#[test]
fn a0_07_evidence_digest_mutation_changes_assertion_identity() {
    let first = build_bundle(&complete_input()).expect("base");
    let mut changed = complete_input();
    changed.evidence_digest = labeled('7');
    let second = build_bundle(&changed).expect("changed");
    assert_ne!(first.runtime_trace_digest, second.runtime_trace_digest);
    assert_ne!(first.scai_digest, second.scai_digest);
}

#[test]
fn a0_08_command_identity_mutation_changes_assertion_identity() {
    let first = build_bundle(&complete_input()).expect("base");
    let mut changed = complete_input();
    changed.command_identity = "cargo test --release --locked".to_owned();
    let second = build_bundle(&changed).expect("changed");
    assert_ne!(first.runtime_trace_digest, second.runtime_trace_digest);
    assert_ne!(first.scai_digest, second.scai_digest);
}

#[test]
fn a0_09_subject_substitution_changes_identity_and_cross_binding_fails() {
    let original = build_bundle(&complete_input()).expect("base");
    let mut changed_input = complete_input();
    let replacement = descriptor(
        "other-artifact",
        '8',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/other",
    );
    changed_input.subject = replacement.clone();
    changed_input.artifact_identity = Some(replacement);
    if let Some(provenance) = &mut changed_input.slsa_provenance {
        provenance.subject_digest = labeled('8');
    }
    let changed = build_bundle(&changed_input).expect("changed subject");
    assert_ne!(original.bundle_digest, changed.bundle_digest);

    let mut mixed = original;
    mixed.svr.subject = changed.svr.subject;
    assert_eq!(
        verify_bundle(&mixed)
            .expect_err("mixed subjects must fail")
            .reason_code,
        "cross_statement_subject_mismatch"
    );
}

#[test]
fn a0_10_source_or_workflow_substitution_changes_assertion_identity() {
    let base = build_bundle(&complete_input()).expect("base");

    let mut source_changed = complete_input();
    source_changed.source_identity = Some(descriptor(
        "source-other",
        '9',
        "https://github.com/AETHERXGLOBAL/execsurface/tree/other",
    ));
    let source_bundle = build_bundle(&source_changed).expect("source changed");
    assert_ne!(base.scai_digest, source_bundle.scai_digest);

    let mut workflow_changed = complete_input();
    workflow_changed.workflow_identity = Some(descriptor(
        "workflow-other",
        '0',
        "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/other",
    ));
    let workflow_bundle = build_bundle(&workflow_changed).expect("workflow changed");
    assert_ne!(base.scai_digest, workflow_bundle.scai_digest);
}

#[test]
fn a0_11_backend_name_substitution_cannot_upgrade_semantics() {
    let base_input = review_input();
    let base = build_bundle(&base_input).expect("review builds");
    let mut renamed = base_input;
    renamed.observer_profile = "trusted-complete-super-backend".to_owned();
    let renamed_bundle = build_bundle(&renamed).expect("renamed review builds");
    let conditions = &renamed_bundle.scai.predicate.attributes[0].conditions;
    assert_eq!(conditions.verdict, Verdict::Review);
    assert_eq!(conditions.authority, AuthorityState::Ambiguous);
    assert_eq!(conditions.completeness.class, CompletenessClass::Incomplete);
    assert_ne!(base.scai_digest, renamed_bundle.scai_digest);

    renamed.verdict = Verdict::Pass;
    assert_eq!(
        build_bundle(&renamed)
            .expect_err("name cannot create pass authority")
            .reason_code,
        "pass_blocked_by_non_direct_authority"
    );
}

#[test]
fn a0_12_completeness_downgrade_cannot_preserve_pass() {
    let mut input = complete_input();
    input.completeness = CompletenessState {
        class: CompletenessClass::Incomplete,
        reason_codes: BTreeSet::from(["loss_window".to_owned()]),
    };
    assert_eq!(
        build_bundle(&input)
            .expect_err("incomplete pass must fail")
            .reason_code,
        "pass_blocked_by_incomplete_evidence"
    );
}

#[test]
fn a0_13_observer_loss_cannot_become_pass() {
    let mut input = complete_input();
    input.observer_health = ObserverHealth::Lost;
    assert_eq!(
        build_bundle(&input)
            .expect_err("loss must block pass")
            .reason_code,
        "pass_blocked_by_observer_loss"
    );
}

#[test]
fn a0_14_ambiguous_or_unsupported_evidence_cannot_become_pass() {
    for authority in [AuthorityState::Ambiguous, AuthorityState::Unsupported] {
        let mut input = complete_input();
        input.authority = authority;
        assert_eq!(
            build_bundle(&input)
                .expect_err("weak authority must block pass")
                .reason_code,
            "pass_blocked_by_non_direct_authority"
        );
    }
}

#[test]
fn a0_15_runtime_trace_scai_svr_subject_mismatch_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.scai.subject[0] = descriptor(
        "substituted",
        'b',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/substituted",
    );
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("subject mismatch must fail")
            .reason_code,
        "cross_statement_subject_mismatch"
    );
}

#[test]
fn a0_16_slsa_provenance_digest_or_predicate_substitution_fails_binding() {
    let base = build_bundle(&complete_input()).expect("base");

    let mut digest_tampered = base.clone();
    digest_tampered.scai.predicate.attributes[0]
        .conditions
        .slsa_provenance
        .as_mut()
        .expect("provenance")
        .statement_digest = labeled('4');
    assert_eq!(
        verify_bundle(&digest_tampered)
            .expect_err("post-build provenance digest substitution must fail")
            .reason_code,
        "statement_digest_mismatch"
    );

    let mut predicate_tampered = base.clone();
    predicate_tampered.scai.predicate.attributes[0]
        .conditions
        .slsa_provenance
        .as_mut()
        .expect("provenance")
        .predicate_type = "https://example.invalid/provenance/v1".to_owned();
    assert_eq!(
        verify_bundle(&predicate_tampered)
            .expect_err("post-build provenance predicate substitution must fail")
            .reason_code,
        "statement_digest_mismatch"
    );

    let mut digest_changed = complete_input();
    digest_changed
        .slsa_provenance
        .as_mut()
        .expect("provenance")
        .statement_digest = labeled('4');
    let changed = build_bundle(&digest_changed).expect("valid alternate reference");
    assert_ne!(base.scai_digest, changed.scai_digest);
    assert_ne!(base.bundle_digest, changed.bundle_digest);

    let mut wrong_predicate = complete_input();
    wrong_predicate
        .slsa_provenance
        .as_mut()
        .expect("provenance")
        .predicate_type = "https://example.invalid/provenance/v1".to_owned();
    assert_eq!(
        build_bundle(&wrong_predicate)
            .expect_err("predicate substitution must fail")
            .reason_code,
        "slsa_predicate_type_mismatch"
    );

    let mut wrong_subject = complete_input();
    wrong_subject
        .slsa_provenance
        .as_mut()
        .expect("provenance")
        .subject_digest = labeled('5');
    assert_eq!(
        build_bundle(&wrong_subject)
            .expect_err("provenance subject substitution must fail")
            .reason_code,
        "slsa_subject_mismatch"
    );
}

#[test]
fn a0_17_replay_against_another_subject_fails_closed() {
    let bundle = build_bundle(&complete_input()).expect("base");
    let other_subject = descriptor(
        "other-subject",
        '6',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/replay-target",
    );
    assert_eq!(
        verify_bundle_for_subject(&bundle, &other_subject)
            .expect_err("replay must fail")
            .reason_code,
        "expected_subject_mismatch"
    );
}

#[test]
fn a0_18_unknown_field_injection_cannot_upgrade_or_mask_semantic_mutation() {
    let input = review_input();
    let json = serde_json::to_value(&input).expect("serialize input");
    let mut object = json.as_object().expect("top-level object").clone();
    object.insert(
        "verdictOverride".to_owned(),
        serde_json::Value::String("PASS".to_owned()),
    );
    object.insert(
        "trustedBackend".to_owned(),
        serde_json::Value::String("complete".to_owned()),
    );
    let injected = serde_json::to_string(&object).expect("serialize injected input");
    assert_eq!(
        parse_strict_input(&injected)
            .expect_err("unknown fields must fail closed")
            .reason_code,
        "verification_input_schema_rejected"
    );

    let base = build_bundle(&input).expect("review base");
    let mut semantic_change = input;
    semantic_change.evidence_digest = labeled('8');
    let changed = build_bundle(&semantic_change).expect("semantic change");
    assert_ne!(base.scai_digest, changed.scai_digest);
    assert!(!changed
        .svr
        .predicate
        .properties
        .iter()
        .any(|property| property == SVR_PASS_PROPERTY));
}
