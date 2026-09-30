use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    build_bundle, recompute_outer_digests_unchecked, AuthorityState, CapabilityState,
    CompletenessClass, CompletenessState, ObserverHealth, ProvenanceReference, ResourceDescriptor,
    Verdict, VerificationBundle, VerificationInput, SLSA_PROVENANCE_TYPE, SVR_PASS_PROPERTY,
};
use execsurface_p5_cross_attestation_redteam::{
    graph_manifest_digest, manifest_for_bundle, verify_graph, ExpectedVerificationContext,
    GraphItem,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

const SCAI_BINDING_PREFIX: &str = "EXECSURFACE_SCAI_SHA256_";

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
        "execsurface-a5-subject",
        'a',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/a5-subject",
    );
    VerificationInput {
        subject: subject.clone(),
        command_identity: "cargo test --locked".to_owned(),
        host_identity: "https://github.com/AETHERXGLOBAL/execsurface/actions/runner/linux-x86_64"
            .to_owned(),
        source_identity: Some(descriptor(
            "source",
            'b',
            "https://github.com/AETHERXGLOBAL/execsurface/tree/a5-source",
        )),
        artifact_identity: Some(subject),
        workflow_identity: Some(descriptor(
            "workflow",
            'c',
            "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/p5-a5",
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
            "https://github.com/AETHERXGLOBAL/execsurface/policies/a5",
        ),
        verdict: Verdict::Pass,
        evidence_digest: labeled('1'),
        verifier: descriptor(
            "execsurface-verifier",
            '2',
            "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1",
        ),
        verifier_id: "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1".to_owned(),
        created_at: "2026-09-30T18:00:00Z".to_owned(),
        slsa_provenance: Some(ProvenanceReference {
            predicate_type: SLSA_PROVENANCE_TYPE.to_owned(),
            statement_digest: labeled('3'),
            subject_digest: labeled('a'),
            verified: true,
        }),
    }
}

fn alternate_subject_input() -> VerificationInput {
    let mut input = complete_input();
    let subject = descriptor(
        "execsurface-a5-other-subject",
        '8',
        "https://github.com/AETHERXGLOBAL/execsurface/artifacts/a5-other-subject",
    );
    input.subject = subject.clone();
    input.artifact_identity = Some(subject);
    input.command_identity = "cargo test --release --locked".to_owned();
    input.evidence_digest = labeled('7');
    if let Some(provenance) = &mut input.slsa_provenance {
        provenance.subject_digest = labeled('8');
        provenance.statement_digest = labeled('9');
    }
    input
}

fn incomplete_input() -> VerificationInput {
    let mut input = complete_input();
    input.verdict = Verdict::Review;
    input.authority = AuthorityState::Ambiguous;
    input.completeness = CompletenessState {
        class: CompletenessClass::Incomplete,
        reason_codes: BTreeSet::from(["observer_scope_incomplete".to_owned()]),
    };
    input
}

fn lost_input() -> VerificationInput {
    let mut input = incomplete_input();
    input.observer_health = ObserverHealth::Lost;
    input.completeness = CompletenessState {
        class: CompletenessClass::Lost,
        reason_codes: BTreeSet::from(["observer_transport_lost".to_owned()]),
    };
    input
}

fn expected_context(input: &VerificationInput) -> ExpectedVerificationContext {
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
        observer_health: input.observer_health.clone(),
        policy: input.policy.clone(),
        authority: input.authority.clone(),
        completeness: input.completeness.clone(),
        verdict: input.verdict.clone(),
        verifier: input.verifier.clone(),
        verifier_id: input.verifier_id.clone(),
        created_at: input.created_at.clone(),
        provenance: input.slsa_provenance.clone(),
    }
}

fn refresh_scai_binding(bundle: &mut VerificationBundle) {
    recompute_outer_digests_unchecked(bundle);
    let scai_hex = bundle
        .scai_digest
        .strip_prefix("sha256:")
        .expect("internal scai digest is labeled");
    bundle
        .svr
        .predicate
        .properties
        .retain(|property| !property.starts_with(SCAI_BINDING_PREFIX));
    bundle
        .svr
        .predicate
        .properties
        .push(format!("{SCAI_BINDING_PREFIX}{scai_hex}"));
    bundle.svr.predicate.properties.sort();
    recompute_outer_digests_unchecked(bundle);
}

fn accepted_graph(
    input: &VerificationInput,
) -> (
    VerificationBundle,
    ExpectedVerificationContext,
    Vec<GraphItem>,
) {
    let bundle = build_bundle(input).expect("valid bounded fixture");
    let expected = expected_context(input);
    let manifest = manifest_for_bundle(&bundle).expect("valid graph manifest");
    verify_graph(&bundle, &expected, &manifest).expect("base graph must verify");
    (bundle, expected, manifest)
}

fn assert_graph_rejected(
    bundle: &VerificationBundle,
    expected: &ExpectedVerificationContext,
    manifest: &[GraphItem],
    label: &str,
) {
    assert!(
        verify_graph(bundle, expected, manifest).is_err(),
        "{label}: adversarial graph unexpectedly verified"
    );
}

fn domain_digest<T: Serialize>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("deterministic JSON");
    let mut hasher = Sha256::new();
    hasher.update(b"execsurface:p5:a0:");
    hasher.update(domain.as_bytes());
    hasher.update(b":v1\0");
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}

#[test]
fn a5_01_mix_and_match_subjects_fail_closed_after_outer_rehash() {
    let (mut first, expected, manifest) = accepted_graph(&complete_input());
    let second = build_bundle(&alternate_subject_input()).expect("second valid bundle");
    first.svr = second.svr;
    recompute_outer_digests_unchecked(&mut first);
    assert_graph_rejected(&first, &expected, &manifest, "mixed subject");
}

#[test]
fn a5_02_stale_runtime_trace_cannot_carry_fresh_verdict() {
    let stale = build_bundle(&complete_input()).expect("stale valid bundle");
    let mut fresh_input = incomplete_input();
    fresh_input.evidence_digest = labeled('7');
    fresh_input.current_surface_digest = labeled('6');
    let (mut fresh, expected, manifest) = accepted_graph(&fresh_input);
    fresh.runtime_trace = stale.runtime_trace;
    recompute_outer_digests_unchecked(&mut fresh);
    assert_graph_rejected(
        &fresh,
        &expected,
        &manifest,
        "stale trace plus fresh verdict",
    );
}

#[test]
fn a5_03_fresh_trace_cannot_be_laundered_through_stale_policy() {
    let stale = build_bundle(&complete_input()).expect("stale policy bundle");
    let mut fresh_input = complete_input();
    fresh_input.policy = descriptor(
        "fresh-policy",
        '6',
        "https://github.com/AETHERXGLOBAL/execsurface/policies/a5-fresh",
    );
    let (fresh, expected, manifest) = accepted_graph(&fresh_input);
    let mut mixed = stale;
    mixed.runtime_trace = fresh.runtime_trace;
    recompute_outer_digests_unchecked(&mut mixed);
    assert_graph_rejected(
        &mixed,
        &expected,
        &manifest,
        "fresh trace plus stale policy",
    );
}

#[test]
fn a5_04_slsa_provenance_replay_from_other_subject_fails_closed() {
    let (mut target, expected, manifest) = accepted_graph(&complete_input());
    target.scai.predicate.attributes[0]
        .conditions
        .slsa_provenance = alternate_subject_input().slsa_provenance;
    refresh_scai_binding(&mut target);
    assert_graph_rejected(
        &target,
        &expected,
        &manifest,
        "cross-subject provenance replay",
    );
}

#[test]
fn a5_05_baseline_substitution_cannot_survive_full_attacker_rehash() {
    let (mut attacked, expected, manifest) = accepted_graph(&complete_input());
    attacked.scai.predicate.attributes[0]
        .conditions
        .baseline_digest = labeled('4');
    refresh_scai_binding(&mut attacked);
    assert_graph_rejected(&attacked, &expected, &manifest, "baseline substitution");
}

#[test]
fn a5_06_current_surface_substitution_cannot_survive_full_attacker_rehash() {
    let (mut attacked, expected, manifest) = accepted_graph(&complete_input());
    attacked.scai.predicate.attributes[0]
        .conditions
        .current_surface_digest = labeled('5');
    refresh_scai_binding(&mut attacked);
    assert_graph_rejected(
        &attacked,
        &expected,
        &manifest,
        "current-surface substitution",
    );
}

#[test]
fn a5_07_partial_evidence_substitution_breaks_cross_graph_binding() {
    let (mut attacked, expected, manifest) = accepted_graph(&complete_input());
    attacked.scai.predicate.attributes[0]
        .conditions
        .evidence_digest = labeled('7');
    refresh_scai_binding(&mut attacked);
    assert_graph_rejected(
        &attacked,
        &expected,
        &manifest,
        "partial evidence substitution",
    );
}

#[test]
fn a5_08_backend_name_cannot_inflate_ambiguous_graph_to_pass() {
    let input = incomplete_input();
    let (mut attacked, expected, manifest) = accepted_graph(&input);
    attacked.scai.predicate.attributes[0]
        .conditions
        .observer_profile = "trusted-super-authority-backend".to_owned();
    attacked
        .svr
        .predicate
        .properties
        .push(SVR_PASS_PROPERTY.to_owned());
    refresh_scai_binding(&mut attacked);
    assert_graph_rejected(
        &attacked,
        &expected,
        &manifest,
        "backend-name authority inflation",
    );
}

#[test]
fn a5_09_loss_or_incompleteness_cannot_be_masked_by_pass_summary() {
    let input = lost_input();
    let (mut attacked, expected, manifest) = accepted_graph(&input);
    attacked.scai.predicate.attributes[0].conditions.verdict = Verdict::Pass;
    attacked
        .svr
        .predicate
        .properties
        .push(SVR_PASS_PROPERTY.to_owned());
    refresh_scai_binding(&mut attacked);
    assert_graph_rejected(&attacked, &expected, &manifest, "loss masking");
}

#[test]
fn a5_10_workflow_and_source_substitution_fail_expected_context_binding() {
    let input = complete_input();
    let (base, expected, manifest) = accepted_graph(&input);

    let mut workflow_attack = base.clone();
    workflow_attack.scai.predicate.attributes[0]
        .conditions
        .workflow_identity = Some(descriptor(
        "other-workflow",
        '6',
        "https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/other",
    ));
    refresh_scai_binding(&mut workflow_attack);
    assert_graph_rejected(
        &workflow_attack,
        &expected,
        &manifest,
        "workflow substitution",
    );

    let mut source_attack = base;
    source_attack.scai.predicate.attributes[0]
        .conditions
        .source_identity = Some(descriptor(
        "other-source",
        '7',
        "https://github.com/AETHERXGLOBAL/execsurface/tree/other-source",
    ));
    refresh_scai_binding(&mut source_attack);
    assert_graph_rejected(&source_attack, &expected, &manifest, "source substitution");
}

#[test]
fn a5_11_duplicate_or_reordered_semantic_items_cannot_create_new_accepted_identity() {
    let (base, expected, manifest) = accepted_graph(&complete_input());

    let mut duplicate = base.clone();
    duplicate.svr.predicate.properties.push(
        duplicate
            .svr
            .predicate
            .properties
            .first()
            .expect("fixture has properties")
            .clone(),
    );
    recompute_outer_digests_unchecked(&mut duplicate);
    assert_graph_rejected(
        &duplicate,
        &expected,
        &manifest,
        "duplicate SVR semantic property",
    );

    let mut reordered_manifest = manifest.clone();
    reordered_manifest.reverse();
    assert_eq!(
        graph_manifest_digest(&reordered_manifest).expect("reordered manifest digest"),
        graph_manifest_digest(&manifest).expect("base manifest digest"),
        "order-insensitive graph manifest must have one canonical identity"
    );
    verify_graph(&base, &expected, &reordered_manifest)
        .expect("canonical reconstruction of order-insensitive manifest must verify");
}

#[test]
fn a5_12_digest_domains_are_separated_and_cross_domain_substitution_fails() {
    let (bundle, expected, manifest) = accepted_graph(&complete_input());
    let payload = &bundle.runtime_trace;
    let trace_domain = domain_digest("runtime-trace-statement", payload);
    let scai_domain = domain_digest("scai-statement", payload);
    let svr_domain = domain_digest("svr-statement", payload);
    let bundle_domain = domain_digest("verification-bundle", payload);

    assert_ne!(trace_domain, scai_domain);
    assert_ne!(trace_domain, svr_domain);
    assert_ne!(trace_domain, bundle_domain);
    assert_ne!(scai_domain, svr_domain);
    assert_ne!(scai_domain, bundle_domain);
    assert_ne!(svr_domain, bundle_domain);
    assert_eq!(
        trace_domain,
        domain_digest("runtime-trace-statement", payload)
    );

    let mut confused = bundle;
    confused.scai_digest = confused.runtime_trace_digest.clone();
    assert_graph_rejected(
        &confused,
        &expected,
        &manifest,
        "cross-domain digest substitution",
    );
}
