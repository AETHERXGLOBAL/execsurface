use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    build_bundle, digest_bundle, recompute_outer_digests_unchecked, verify_bundle,
    verify_bundle_for_context, AuthorityState, CapabilityState, CompletenessClass,
    CompletenessState, ObserverHealth, ProvenanceReference, ResourceDescriptor, Verdict,
    VerificationInput, SLSA_PROVENANCE_TYPE, SVR_PASS_PROPERTY, SVR_RECORDED_PROPERTY,
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
        host_identity: "https://github.com/AETHERXGLOBAL/execsurface/actions/jobs/fixture"
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
            "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/p5-a2",
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
        created_at: "2026-09-30T16:40:00Z".to_owned(),
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

fn rebind_svr_to_current_scai(
    bundle: &mut execsurface_p5_attestation_provenance::VerificationBundle,
) {
    const PREFIX: &str = "EXECSURFACE_SCAI_SHA256_";
    recompute_outer_digests_unchecked(bundle);
    let scai_hex = bundle
        .scai_digest
        .strip_prefix("sha256:")
        .expect("labeled SCAI digest");
    bundle
        .svr
        .predicate
        .properties
        .retain(|property| !property.starts_with(PREFIX));
    bundle
        .svr
        .predicate
        .properties
        .push(format!("{PREFIX}{scai_hex}"));
    recompute_outer_digests_unchecked(bundle);
}

#[test]
fn a2_01_same_subject_complete_scai_svr_composition_verifies() {
    let input = complete_input();
    let bundle = build_bundle(&input).expect("complete A2 fixture");
    let attribute = &bundle.scai.predicate.attributes[0];
    assert_eq!(bundle.runtime_trace.subject, bundle.scai.subject);
    assert_eq!(bundle.runtime_trace.subject, bundle.svr.subject);
    assert_eq!(attribute.target, bundle.runtime_trace.subject[0]);
    assert_eq!(
        attribute.conditions.verifier_identity,
        bundle.scai.predicate.producer
    );
    assert_eq!(bundle.svr.predicate.verifier.policies, vec![input.policy]);
    assert!(bundle
        .svr
        .predicate
        .properties
        .contains(&SVR_RECORDED_PROPERTY.to_owned()));
    verify_bundle(&bundle).expect("same-subject composition verifies");
}

#[test]
fn a2_02_scai_subject_substitution_fails_after_digest_recompute() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.scai.subject[0] = descriptor("other", '4', "https://example.invalid/subject");
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("SCAI subject substitution")
            .reason_code,
        "cross_statement_subject_mismatch"
    );
}

#[test]
fn a2_03_svr_subject_substitution_fails_after_digest_recompute() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.svr.subject[0] = descriptor("other", '4', "https://example.invalid/subject");
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("SVR subject substitution")
            .reason_code,
        "cross_statement_subject_mismatch"
    );
}

#[test]
fn a2_04_scai_target_substitution_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.scai.predicate.attributes[0].target =
        descriptor("other", '4', "https://example.invalid/target");
    rebind_svr_to_current_scai(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("SCAI target substitution")
            .reason_code,
        "scai_target_subject_mismatch"
    );
}

#[test]
fn a2_05_svr_policy_substitution_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.svr.predicate.verifier.policies[0] =
        descriptor("other-policy", '5', "https://example.invalid/policy");
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("SVR policy substitution")
            .reason_code,
        "policy_binding_mismatch"
    );
}

#[test]
fn a2_06_runtime_trace_evidence_descriptor_substitution_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.scai.predicate.attributes[0].evidence =
        descriptor("other-trace", '6', "https://example.invalid/trace");
    rebind_svr_to_current_scai(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("runtime evidence substitution")
            .reason_code,
        "runtime_trace_evidence_mismatch"
    );
}

#[test]
fn a2_07_runtime_trace_digest_field_substitution_fails_closed() {
    let mut bundle = build_bundle(&complete_input()).expect("base");
    bundle.scai.predicate.attributes[0]
        .conditions
        .runtime_trace_statement_digest = labeled('7');
    rebind_svr_to_current_scai(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("runtime trace digest field substitution")
            .reason_code,
        "runtime_trace_binding_mismatch"
    );
}

#[test]
fn a2_08_missing_or_stale_svr_scai_binding_fails_closed() {
    const PREFIX: &str = "EXECSURFACE_SCAI_SHA256_";
    let mut missing = build_bundle(&complete_input()).expect("base");
    missing
        .svr
        .predicate
        .properties
        .retain(|property| !property.starts_with(PREFIX));
    recompute_outer_digests_unchecked(&mut missing);
    assert_eq!(
        verify_bundle(&missing)
            .expect_err("missing SCAI binding")
            .reason_code,
        "svr_scai_binding_missing"
    );

    let mut stale = build_bundle(&complete_input()).expect("base");
    stale.scai.predicate.attributes[0]
        .conditions
        .baseline_digest = labeled('8');
    recompute_outer_digests_unchecked(&mut stale);
    assert_eq!(
        verify_bundle(&stale)
            .expect_err("stale SCAI binding")
            .reason_code,
        "svr_scai_binding_missing"
    );
}

#[test]
fn a2_09_svr_recorded_and_pass_summary_integrity_is_enforced() {
    let mut missing_recorded = build_bundle(&complete_input()).expect("base");
    missing_recorded
        .svr
        .predicate
        .properties
        .retain(|property| property != SVR_RECORDED_PROPERTY);
    recompute_outer_digests_unchecked(&mut missing_recorded);
    assert_eq!(
        verify_bundle(&missing_recorded)
            .expect_err("recorded-result marker must be mandatory")
            .reason_code,
        "svr_recorded_property_missing"
    );

    let mut review = build_bundle(&review_input()).expect("review");
    review
        .svr
        .predicate
        .properties
        .push(SVR_PASS_PROPERTY.to_owned());
    recompute_outer_digests_unchecked(&mut review);
    assert_eq!(
        verify_bundle(&review)
            .expect_err("PASS property laundering")
            .reason_code,
        "svr_verdict_binding_mismatch"
    );
}

#[test]
fn a2_10_producer_or_declared_verifier_substitution_cannot_be_rescued_by_rebinding() {
    let mut producer = build_bundle(&complete_input()).expect("base");
    producer.scai.predicate.producer =
        descriptor("other-verifier", '9', "https://example.invalid/verifier");
    rebind_svr_to_current_scai(&mut producer);
    assert_eq!(
        verify_bundle(&producer)
            .expect_err("producer substitution must fail after digest rebinding")
            .reason_code,
        "scai_producer_verifier_identity_mismatch"
    );

    let mut declared = build_bundle(&complete_input()).expect("base");
    declared.scai.predicate.attributes[0]
        .conditions
        .verifier_identity = descriptor("other-verifier", '9', "https://example.invalid/verifier");
    rebind_svr_to_current_scai(&mut declared);
    assert_eq!(
        verify_bundle(&declared)
            .expect_err("declared verifier substitution must fail after digest rebinding")
            .reason_code,
        "scai_producer_verifier_identity_mismatch"
    );
}

#[test]
fn a2_11_backend_profile_name_cannot_upgrade_weak_semantics() {
    let mut input = review_input();
    input.observer_profile = "trusted-complete-super-backend".to_owned();
    let bundle = build_bundle(&input).expect("renamed weak evidence");
    let conditions = &bundle.scai.predicate.attributes[0].conditions;
    assert_eq!(conditions.verdict, Verdict::Review);
    assert_eq!(conditions.authority, AuthorityState::Ambiguous);
    assert_eq!(conditions.completeness.class, CompletenessClass::Incomplete);
    verify_bundle(&bundle).expect("non-pass weak evidence remains representable");
}

#[test]
fn a2_12_loss_incompleteness_or_ambiguity_cannot_produce_or_preserve_pass() {
    let mut lost = complete_input();
    lost.observer_health = ObserverHealth::Lost;
    assert_eq!(
        build_bundle(&lost)
            .expect_err("loss blocks PASS")
            .reason_code,
        "pass_blocked_by_observer_loss"
    );

    let mut incomplete = complete_input();
    incomplete.completeness = CompletenessState {
        class: CompletenessClass::Incomplete,
        reason_codes: BTreeSet::from(["incomplete".to_owned()]),
    };
    assert_eq!(
        build_bundle(&incomplete)
            .expect_err("incomplete blocks PASS")
            .reason_code,
        "pass_blocked_by_incomplete_evidence"
    );

    let mut ambiguous = complete_input();
    ambiguous.authority = AuthorityState::Ambiguous;
    assert_eq!(
        build_bundle(&ambiguous)
            .expect_err("ambiguous blocks PASS")
            .reason_code,
        "pass_blocked_by_non_direct_authority"
    );

    let mut forged = build_bundle(&review_input()).expect("review");
    forged
        .svr
        .predicate
        .properties
        .push(SVR_PASS_PROPERTY.to_owned());
    recompute_outer_digests_unchecked(&mut forged);
    assert_eq!(
        verify_bundle(&forged)
            .expect_err("forged PASS summary")
            .reason_code,
        "svr_verdict_binding_mismatch"
    );
}

#[test]
fn a2_13_semantic_field_mutations_change_assertion_identity_and_workflow_replay_fails() {
    let original_input = complete_input();
    let original = build_bundle(&original_input).expect("original");

    let mut variants = Vec::new();
    let mut baseline = original_input.clone();
    baseline.baseline_digest = labeled('4');
    variants.push(build_bundle(&baseline).expect("baseline variant"));
    let mut current = original_input.clone();
    current.current_surface_digest = labeled('5');
    variants.push(build_bundle(&current).expect("current variant"));
    let mut evidence = original_input.clone();
    evidence.evidence_digest = labeled('6');
    variants.push(build_bundle(&evidence).expect("evidence variant"));
    let mut source = original_input.clone();
    source.source_identity = Some(descriptor(
        "other-source",
        '7',
        "https://example.invalid/source",
    ));
    variants.push(build_bundle(&source).expect("source variant"));
    let mut workflow = original_input.clone();
    workflow.workflow_identity = Some(descriptor(
        "other-workflow",
        '8',
        "https://example.invalid/workflow",
    ));
    let workflow_bundle = build_bundle(&workflow).expect("workflow variant");
    variants.push(workflow_bundle.clone());

    for variant in &variants {
        assert_ne!(original.scai_digest, variant.scai_digest);
        assert_ne!(digest_bundle(&original), digest_bundle(variant));
    }

    assert_eq!(
        verify_bundle_for_context(
            &workflow_bundle,
            original_input.workflow_identity.as_ref().expect("workflow"),
            &original_input.command_identity,
            &original_input.host_identity,
        )
        .expect_err("workflow replay against original context")
        .reason_code,
        "expected_workflow_mismatch"
    );
}

#[test]
fn a2_14_identical_composition_is_deterministic_and_statement_domains_are_distinct() {
    let first = build_bundle(&complete_input()).expect("first");
    let second = build_bundle(&complete_input()).expect("second");
    assert_eq!(first.runtime_trace_digest, second.runtime_trace_digest);
    assert_eq!(first.scai_digest, second.scai_digest);
    assert_eq!(first.svr_digest, second.svr_digest);
    assert_eq!(first.bundle_digest, second.bundle_digest);
    assert_ne!(first.runtime_trace_digest, first.scai_digest);
    assert_ne!(first.runtime_trace_digest, first.svr_digest);
    assert_ne!(first.scai_digest, first.svr_digest);
    assert_ne!(first.bundle_digest, first.runtime_trace_digest);
    assert_ne!(first.bundle_digest, first.scai_digest);
    assert_ne!(first.bundle_digest, first.svr_digest);
}
