use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    build_bundle, recompute_outer_digests_unchecked, verify_bundle, verify_bundle_for_context,
    AuthorityState, CapabilityState, CompletenessClass, CompletenessState, ObserverHealth,
    ProvenanceReference, ResourceDescriptor, Verdict, VerificationInput, EXECSURFACE_MONITOR_TYPE,
    EXECSURFACE_PROCESS_TYPE, RUNTIME_TRACE_TYPE, SLSA_PROVENANCE_TYPE, STATEMENT_TYPE,
    SVR_PASS_PROPERTY,
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
        host_identity: "https://github.com/AETHERXGLOBAL/execsurface/actions/jobs/fixture".to_owned(),
        source_identity: Some(descriptor(
            "source",
            'b',
            "https://github.com/AETHERXGLOBAL/execsurface/tree/source-fixture",
        )),
        artifact_identity: Some(subject),
        workflow_identity: Some(descriptor(
            "workflow",
            'c',
            "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/p5-a1",
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
            "https://github.com/AETHERXGLOBAL/execsurface",
        ),
        verifier_id: "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1".to_owned(),
        created_at: "2026-09-30T16:20:00Z".to_owned(),
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
fn a1_01_complete_fixture_maps_exact_bounded_runtime_trace_contract() {
    let input = complete_input();
    let bundle = build_bundle(&input).expect("complete A1 fixture");
    let trace = &bundle.runtime_trace;

    assert_eq!(trace.statement_type, STATEMENT_TYPE);
    assert_eq!(trace.predicate_type, RUNTIME_TRACE_TYPE);
    assert_eq!(trace.predicate.monitor.monitor_type, EXECSURFACE_MONITOR_TYPE);
    assert_eq!(trace.predicate.monitor.config_source, input.policy);
    assert_eq!(
        trace.predicate.monitored_process.process_type,
        EXECSURFACE_PROCESS_TYPE
    );
    assert_eq!(trace.predicate.monitored_process.host_id, input.host_identity);
    assert_eq!(trace.predicate.monitored_process.event, input.command_identity);
    assert_eq!(trace.predicate.monitor_log.process.len(), 1);
    verify_bundle(&bundle).expect("complete A1 mapping verifies");
}

#[test]
fn a1_02_missing_or_substituted_monitor_identity_fails_closed() {
    for value in ["", "https://example.invalid/trusted-monitor/v99"] {
        let mut bundle = build_bundle(&complete_input()).expect("base");
        bundle.runtime_trace.predicate.monitor.monitor_type = value.to_owned();
        recompute_outer_digests_unchecked(&mut bundle);
        assert_eq!(
            verify_bundle(&bundle)
                .expect_err("monitor substitution must fail")
                .reason_code,
            "runtime_monitor_type_mismatch"
        );
    }
}

#[test]
fn a1_03_runtime_trace_subject_substitution_fails_cross_binding() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.runtime_trace.subject[0] = descriptor(
        "other-subject",
        '4',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/other",
    );
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("subject substitution must fail")
            .reason_code,
        "cross_statement_subject_mismatch"
    );
}

#[test]
fn a1_04_empty_or_truncated_bounded_process_log_fails_closed() {
    let mut empty = build_bundle(&complete_input()).expect("base");
    empty.runtime_trace.predicate.monitor_log.process.clear();
    recompute_outer_digests_unchecked(&mut empty);
    assert_eq!(
        verify_bundle(&empty)
            .expect_err("empty bounded log must fail")
            .reason_code,
        "runtime_process_log_cardinality_mismatch"
    );

    let mut duplicated = build_bundle(&complete_input()).expect("base");
    let record = duplicated.runtime_trace.predicate.monitor_log.process[0].clone();
    duplicated
        .runtime_trace
        .predicate
        .monitor_log
        .process
        .push(record);
    recompute_outer_digests_unchecked(&mut duplicated);
    assert_eq!(
        verify_bundle(&duplicated)
            .expect_err("unexpected extra bounded log record must fail")
            .reason_code,
        "runtime_process_log_cardinality_mismatch"
    );
}

#[test]
fn a1_05_backend_name_cannot_upgrade_and_trace_profile_mismatch_fails() {
    let mut renamed = review_input();
    renamed.observer_profile = "trusted-complete-super-backend".to_owned();
    let bundle = build_bundle(&renamed).expect("renamed review bundle");
    let conditions = &bundle.scai.predicate.attributes[0].conditions;
    assert_eq!(conditions.verdict, Verdict::Review);
    assert_eq!(conditions.authority, AuthorityState::Ambiguous);
    assert_eq!(conditions.completeness.class, CompletenessClass::Incomplete);

    let mut mismatch = bundle;
    mismatch
        .runtime_trace
        .predicate
        .monitor
        .trace_policy
        .insert("observerProfile".to_owned(), "other-profile".to_owned());
    recompute_outer_digests_unchecked(&mut mismatch);
    assert_eq!(
        verify_bundle(&mismatch)
            .expect_err("runtime/SCAI profile mismatch must fail")
            .reason_code,
        "runtime_trace_policy_binding_mismatch"
    );
}

#[test]
fn a1_06_process_log_command_evidence_or_profile_mutation_fails_closed() {
    for (key, value) in [
        ("commandIdentity", "other-command"),
        ("evidenceDigest", "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
        ("observerProfile", "other-profile"),
    ] {
        let mut bundle = build_bundle(&complete_input()).expect("base");
        bundle.runtime_trace.predicate.monitor_log.process[0]
            .insert(key.to_owned(), value.to_owned());
        recompute_outer_digests_unchecked(&mut bundle);
        assert_eq!(
            verify_bundle(&bundle)
                .expect_err("process-log semantic mutation must fail")
                .reason_code,
            "runtime_process_log_binding_mismatch"
        );
    }
}

#[test]
fn a1_07_monitored_process_command_substitution_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.runtime_trace.predicate.monitored_process.event = "cargo test --release".to_owned();
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("command substitution must fail")
            .reason_code,
        "runtime_command_identity_mismatch"
    );
}

#[test]
fn a1_08_monitored_process_host_substitution_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.runtime_trace.predicate.monitored_process.host_id =
        "https://github.com/AETHERXGLOBAL/execsurface/actions/jobs/other".to_owned();
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("host substitution must fail")
            .reason_code,
        "runtime_host_identity_mismatch"
    );
}

#[test]
fn a1_09_replay_under_different_expected_workflow_or_command_is_rejected() {
    let input = complete_input();
    let bundle = build_bundle(&input).expect("base");
    let expected_workflow = input.workflow_identity.as_ref().expect("workflow");

    verify_bundle_for_context(
        &bundle,
        expected_workflow,
        &input.command_identity,
        &input.host_identity,
    )
    .expect("original context verifies");

    let other_workflow = descriptor(
        "other-workflow",
        '5',
        "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/other",
    );
    assert_eq!(
        verify_bundle_for_context(
            &bundle,
            &other_workflow,
            &input.command_identity,
            &input.host_identity,
        )
        .expect_err("workflow replay must fail")
        .reason_code,
        "expected_workflow_mismatch"
    );
    assert_eq!(
        verify_bundle_for_context(&bundle, expected_workflow, "other-command", &input.host_identity)
            .expect_err("command replay must fail")
            .reason_code,
        "expected_command_mismatch"
    );
    assert_eq!(
        verify_bundle_for_context(
            &bundle,
            expected_workflow,
            &input.command_identity,
            "https://github.com/AETHERXGLOBAL/execsurface/actions/jobs/other",
        )
        .expect_err("host replay must fail")
        .reason_code,
        "expected_host_mismatch"
    );
}

#[test]
fn a1_10_policy_config_substitution_or_cardinality_corruption_fails_without_panic() {
    let mut config_substitution = build_bundle(&complete_input()).expect("base");
    config_substitution
        .runtime_trace
        .predicate
        .monitor
        .config_source = descriptor(
        "other-policy",
        '6',
        "https://github.com/AETHERXGLOBAL/execsurface/policies/other",
    );
    recompute_outer_digests_unchecked(&mut config_substitution);
    assert_eq!(
        verify_bundle(&config_substitution)
            .expect_err("config-source substitution must fail")
            .reason_code,
        "runtime_monitor_config_policy_mismatch"
    );

    let mut no_policy = build_bundle(&complete_input()).expect("base");
    no_policy.svr.predicate.verifier.policies.clear();
    recompute_outer_digests_unchecked(&mut no_policy);
    assert_eq!(
        verify_bundle(&no_policy)
            .expect_err("zero-policy corruption must fail closed")
            .reason_code,
        "svr_policy_cardinality_mismatch"
    );
}

#[test]
fn a1_11_observer_loss_or_incompleteness_cannot_be_laundered_by_runtime_trace() {
    let mut lost_review = review_input();
    lost_review.observer_health = ObserverHealth::Lost;
    lost_review.completeness = CompletenessState {
        class: CompletenessClass::Lost,
        reason_codes: BTreeSet::from(["observer_loss".to_owned()]),
    };
    let mut bundle = build_bundle(&lost_review).expect("loss may be recorded as non-pass");
    assert!(!bundle
        .svr
        .predicate
        .properties
        .iter()
        .any(|property| property == SVR_PASS_PROPERTY));

    bundle
        .svr
        .predicate
        .properties
        .push(SVR_PASS_PROPERTY.to_owned());
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("PASS laundering over loss must fail")
            .reason_code,
        "svr_verdict_binding_mismatch"
    );

    let mut invalid_pass = complete_input();
    invalid_pass.observer_health = ObserverHealth::Lost;
    assert_eq!(
        build_bundle(&invalid_pass)
            .expect_err("loss must block PASS construction")
            .reason_code,
        "pass_blocked_by_observer_loss"
    );
}

#[test]
fn a1_12_mapping_is_deterministic_and_runtime_change_rebinds_scai_evidence() {
    let first = build_bundle(&complete_input()).expect("first");
    let second = build_bundle(&complete_input()).expect("second");
    assert_eq!(first.runtime_trace_digest, second.runtime_trace_digest);
    assert_eq!(first.scai_digest, second.scai_digest);

    let mut changed_input = complete_input();
    changed_input.command_identity = "cargo test --locked --release".to_owned();
    let changed = build_bundle(&changed_input).expect("changed mapping");
    assert_ne!(first.runtime_trace_digest, changed.runtime_trace_digest);
    assert_ne!(first.scai_digest, changed.scai_digest);

    let first_evidence = first.scai.predicate.attributes[0]
        .evidence
        .sha256_hex()
        .expect("first evidence digest");
    let changed_evidence = changed.scai.predicate.attributes[0]
        .evidence
        .sha256_hex()
        .expect("changed evidence digest");
    assert_eq!(
        first_evidence,
        first
            .runtime_trace_digest
            .strip_prefix("sha256:")
            .expect("labeled digest")
    );
    assert_eq!(
        changed_evidence,
        changed
            .runtime_trace_digest
            .strip_prefix("sha256:")
            .expect("labeled digest")
    );
    assert_ne!(first_evidence, changed_evidence);
}
