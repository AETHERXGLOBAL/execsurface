use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    build_bundle, recompute_outer_digests_unchecked, verify_bundle, AuthorityState,
    CapabilityState, CompletenessClass, CompletenessState, ObserverHealth, ProvenanceReference,
    ResourceDescriptor, Verdict, VerificationInput, SLSA_PROVENANCE_TYPE,
};

fn hex(ch: char) -> String {
    std::iter::repeat_n(ch, 64).collect()
}

fn labeled(ch: char) -> String {
    format!("sha256:{}", hex(ch))
}

fn descriptor(name: &str, ch: char, uri: Option<&str>) -> ResourceDescriptor {
    ResourceDescriptor {
        name: Some(name.to_owned()),
        uri: uri.map(str::to_owned),
        digest: BTreeMap::from([("sha256".to_owned(), hex(ch))]),
        media_type: None,
    }
}

fn input() -> VerificationInput {
    let verifier_uri = "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1";
    let subject = descriptor("fixture", 'a', Some("https://example.invalid/fixture"));
    VerificationInput {
        subject: subject.clone(),
        command_identity: "cargo test --locked".to_owned(),
        host_identity: "https://github.com/AETHERXGLOBAL/execsurface/actions/jobs/fixture".to_owned(),
        source_identity: Some(descriptor("source", 'b', Some("https://example.invalid/source"))),
        artifact_identity: Some(subject),
        workflow_identity: Some(descriptor(
            "workflow",
            'c',
            Some("https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/p5-a2"),
        )),
        baseline_digest: labeled('d'),
        current_surface_digest: labeled('e'),
        observer_profile: "ptrace-research-profile-v1".to_owned(),
        capability_state: CapabilityState {
            capabilities: BTreeSet::from(["process.exec.success".to_owned()]),
        },
        observer_health: ObserverHealth::Healthy,
        authority: AuthorityState::Direct,
        completeness: CompletenessState {
            class: CompletenessClass::Complete,
            reason_codes: BTreeSet::new(),
        },
        policy: descriptor("policy", 'f', Some("https://example.invalid/policy")),
        verdict: Verdict::Pass,
        evidence_digest: labeled('1'),
        verifier: descriptor("execsurface-verifier", '2', Some(verifier_uri)),
        verifier_id: verifier_uri.to_owned(),
        created_at: "2026-09-30T16:55:00Z".to_owned(),
        slsa_provenance: Some(ProvenanceReference {
            predicate_type: SLSA_PROVENANCE_TYPE.to_owned(),
            statement_digest: labeled('3'),
            subject_digest: labeled('a'),
            verified: true,
        }),
    }
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
fn a2x_01_same_verifier_uri_profile_is_accepted() {
    let bundle = build_bundle(&input()).expect("valid common verifier identity");
    assert_eq!(
        bundle.scai.predicate.producer.uri.as_deref(),
        Some(bundle.svr.predicate.verifier.id.as_str())
    );
    verify_bundle(&bundle).expect("valid common verifier binding");
}

#[test]
fn a2x_02_svr_verifier_id_substitution_fails_after_digest_recompute() {
    let mut bundle = build_bundle(&input()).expect("base");
    bundle.svr.predicate.verifier.id = "https://example.invalid/other-verifier/v9".to_owned();
    recompute_outer_digests_unchecked(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("SVR verifier substitution must fail")
            .reason_code,
        "cross_attestation_verifier_identity_mismatch"
    );
}

#[test]
fn a2x_03_coordinated_scai_substitution_cannot_leave_svr_identity_stale() {
    let mut bundle = build_bundle(&input()).expect("base");
    let other = descriptor(
        "other-verifier",
        '9',
        Some("https://example.invalid/other-verifier/v9"),
    );
    bundle.scai.predicate.producer = other.clone();
    bundle.scai.predicate.attributes[0]
        .conditions
        .verifier_identity = other;
    rebind_svr_to_current_scai(&mut bundle);
    assert_eq!(
        verify_bundle(&bundle)
            .expect_err("coordinated SCAI substitution must fail against SVR verifier")
            .reason_code,
        "cross_attestation_verifier_identity_mismatch"
    );
}

#[test]
fn a2x_04_construction_rejects_missing_or_mismatched_common_verifier_uri() {
    let mut missing = input();
    missing.verifier.uri = None;
    assert_eq!(
        build_bundle(&missing)
            .expect_err("missing common verifier URI must fail")
            .reason_code,
        "verifier_uri_missing"
    );

    let mut mismatch = input();
    mismatch.verifier.uri = Some("https://example.invalid/other-verifier/v9".to_owned());
    assert_eq!(
        build_bundle(&mismatch)
            .expect_err("mismatched verifier URI must fail")
            .reason_code,
        "verifier_identity_mismatch"
    );
}
