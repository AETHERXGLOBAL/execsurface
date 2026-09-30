use std::{collections::{BTreeMap, BTreeSet}, fs};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use execsurface_p5_attestation_provenance::{
    build_bundle, AuthorityState, CapabilityState, CompletenessClass, CompletenessState,
    ObserverHealth, ResourceDescriptor, Verdict, VerificationInput,
};
use execsurface_p5_slsa_provenance_binding::{
    attach_verified_reference, bind_optional_slsa_v1_statement, bind_slsa_v1_statement,
    sha256_labeled, SlsaBindingContext, VerificationState, SLSA_PROVENANCE_TYPE, STATEMENT_TYPE,
    UPSTREAM_BLOB_SHA, UPSTREAM_COMMIT, UPSTREAM_PATH, UPSTREAM_REPOSITORY,
};
use serde_json::Value;

const SUBJECT_NAME: &str =
    "https://us-central1-docker.pkg.dev/argo-local-khalk/khalk-docker-ar/prod-prov-image";
const SUBJECT_HEX: &str = "7e9b6e7ba2842c91cf49f3e214d04a7a496f8214356f41d81a6e6dcad11f11e3";
const SOURCE_REPOSITORY: &str = "git+https://github.com/khalkie/gcb-prod-prov";
const SOURCE_REVISION: &str = "2ce3f90facdb51aeb950d5bc641e981be61fdf48";
const WORKFLOW_PATH: &str = "cloudbuild.yaml";
const BUILDER_ID: &str = "https://cloudbuild.googleapis.com/GoogleHostedWorker";

fn fixture_document() -> Value {
    let path = std::env::var("P5_A3_FIXTURE").expect("P5_A3_FIXTURE must point to pinned upstream fixture");
    let bytes = fs::read(path).expect("read pinned upstream fixture");
    serde_json::from_slice(&bytes).expect("parse pinned upstream fixture wrapper")
}

fn upstream_statement_bytes() -> Vec<u8> {
    let document = fixture_document();
    let payload = document["provenance_summary"]["provenance"][0]["envelope"]["payload"]
        .as_str()
        .expect("pinned fixture DSSE payload");
    STANDARD.decode(payload).expect("decode pinned DSSE payload")
}

fn embedded_statement() -> Value {
    fixture_document()["provenance_summary"]["provenance"][0]["build"]
        ["inTotoSlsaProvenanceV1"]
        .clone()
}

fn context(bytes: &[u8]) -> SlsaBindingContext {
    SlsaBindingContext {
        expected_statement_digest: sha256_labeled(bytes),
        artifact_name: SUBJECT_NAME.to_owned(),
        artifact_digest: format!("sha256:{SUBJECT_HEX}"),
        source_repository: SOURCE_REPOSITORY.to_owned(),
        source_revision: SOURCE_REVISION.to_owned(),
        workflow_path: WORKFLOW_PATH.to_owned(),
    }
}

fn mutate_statement<F>(bytes: &[u8], mutate: F) -> Vec<u8>
where
    F: FnOnce(&mut Value),
{
    let mut value: Value = serde_json::from_slice(bytes).expect("parse statement for mutation");
    mutate(&mut value);
    serde_json::to_vec(&value).expect("serialize mutated statement")
}

fn descriptor(name: &str, hex: &str, uri: &str) -> ResourceDescriptor {
    ResourceDescriptor {
        name: Some(name.to_owned()),
        uri: Some(uri.to_owned()),
        digest: BTreeMap::from([("sha256".to_owned(), hex.to_owned())]),
        media_type: None,
    }
}

fn review_input() -> VerificationInput {
    let subject = descriptor(SUBJECT_NAME, SUBJECT_HEX, SUBJECT_NAME);
    VerificationInput {
        subject: subject.clone(),
        command_identity: "fixture-command".to_owned(),
        host_identity: "fixture-host".to_owned(),
        source_identity: Some(descriptor("source", &"b".repeat(64), SOURCE_REPOSITORY)),
        artifact_identity: Some(subject),
        workflow_identity: Some(descriptor("workflow", &"c".repeat(64), WORKFLOW_PATH)),
        baseline_digest: format!("sha256:{}", "d".repeat(64)),
        current_surface_digest: format!("sha256:{}", "e".repeat(64)),
        observer_profile: "research-profile".to_owned(),
        capability_state: CapabilityState {
            capabilities: BTreeSet::from(["process.exec.success".to_owned()]),
        },
        observer_health: ObserverHealth::Healthy,
        authority: AuthorityState::Ambiguous,
        completeness: CompletenessState {
            class: CompletenessClass::Incomplete,
            reason_codes: BTreeSet::from(["research_only".to_owned()]),
        },
        policy: descriptor("policy", &"f".repeat(64), "https://example.invalid/policy"),
        verdict: Verdict::Review,
        evidence_digest: format!("sha256:{}", "1".repeat(64)),
        verifier: descriptor(
            "verifier",
            &"2".repeat(64),
            "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1",
        ),
        verifier_id: "https://github.com/AETHERXGLOBAL/execsurface/verifier/v1".to_owned(),
        created_at: "2026-09-30T18:00:00Z".to_owned(),
        slsa_provenance: None,
    }
}

#[test]
fn a3_01_pinned_upstream_fixture_parses_exact_slsa_v1_identity() {
    assert_eq!(UPSTREAM_REPOSITORY, "slsa-framework/slsa-verifier");
    assert_eq!(UPSTREAM_COMMIT, "30d0be3bbab553fc51557377baba2f7572dfc212");
    assert_eq!(UPSTREAM_PATH, "verifiers/internal/gcb/testdata/v1.0-gcloud-container-github-single.json");
    assert_eq!(UPSTREAM_BLOB_SHA, "7102e40887a758e01184ac79ca10cb34b7281627");

    let bytes = upstream_statement_bytes();
    let decoded: Value = serde_json::from_slice(&bytes).expect("decoded statement JSON");
    assert_eq!(decoded, embedded_statement());

    let bound = bind_slsa_v1_statement(&bytes, &context(&bytes), VerificationState::Verified)
        .expect("pinned SLSA v1 statement binds");
    assert_eq!(bound.predicate_type, SLSA_PROVENANCE_TYPE);
    assert_eq!(decoded["_type"], STATEMENT_TYPE);
    assert_eq!(bound.subject_name, SUBJECT_NAME);
    assert_eq!(bound.subject_digest, format!("sha256:{SUBJECT_HEX}"));
    assert_eq!(bound.source_repository, SOURCE_REPOSITORY);
    assert_eq!(bound.source_revision, SOURCE_REVISION);
    assert_eq!(bound.workflow_path, WORKFLOW_PATH);
    assert_eq!(bound.builder_id, BUILDER_ID);
}

#[test]
fn a3_02_statement_digest_is_deterministic_and_exactly_bound() {
    let bytes = upstream_statement_bytes();
    let first = bind_slsa_v1_statement(&bytes, &context(&bytes), VerificationState::Verified)
        .expect("first bind");
    let second = bind_slsa_v1_statement(&bytes, &context(&bytes), VerificationState::Verified)
        .expect("second bind");
    assert_eq!(first.statement_digest, second.statement_digest);
    assert_eq!(first.statement_digest, sha256_labeled(&bytes));

    let mut wrong = context(&bytes);
    wrong.expected_statement_digest = format!("sha256:{}", "0".repeat(64));
    assert_eq!(
        bind_slsa_v1_statement(&bytes, &wrong, VerificationState::Verified)
            .expect_err("wrong statement digest must fail")
            .reason_code,
        "statement_digest_mismatch"
    );
}

#[test]
fn a3_03_exact_subject_digest_and_name_bind_successfully() {
    let bytes = upstream_statement_bytes();
    let bound = bind_slsa_v1_statement(&bytes, &context(&bytes), VerificationState::Verified)
        .expect("exact subject binds");
    assert_eq!(bound.subject_name, SUBJECT_NAME);
    assert_eq!(bound.subject_digest, format!("sha256:{SUBJECT_HEX}"));
}

#[test]
fn a3_04_correct_statement_digest_with_wrong_subject_fails_closed() {
    let bytes = upstream_statement_bytes();
    let mut wrong = context(&bytes);
    wrong.artifact_name = "https://example.invalid/other-artifact".to_owned();
    assert_eq!(
        bind_slsa_v1_statement(&bytes, &wrong, VerificationState::Verified)
            .expect_err("wrong subject must fail")
            .reason_code,
        "subject_binding_mismatch"
    );
}

#[test]
fn a3_05_source_repository_or_revision_substitution_fails_closed() {
    let bytes = upstream_statement_bytes();

    let mut wrong_repository = context(&bytes);
    wrong_repository.source_repository = "git+https://github.com/other/repo".to_owned();
    assert_eq!(
        bind_slsa_v1_statement(&bytes, &wrong_repository, VerificationState::Verified)
            .expect_err("source repository substitution must fail")
            .reason_code,
        "source_repository_mismatch"
    );

    let mut wrong_revision = context(&bytes);
    wrong_revision.source_revision = "0000000000000000000000000000000000000000".to_owned();
    assert_eq!(
        bind_slsa_v1_statement(&bytes, &wrong_revision, VerificationState::Verified)
            .expect_err("source revision substitution must fail")
            .reason_code,
        "source_revision_mismatch"
    );
}

#[test]
fn a3_06_workflow_or_build_config_substitution_fails_closed() {
    let bytes = upstream_statement_bytes();
    let mut wrong = context(&bytes);
    wrong.workflow_path = ".github/workflows/other.yml".to_owned();
    assert_eq!(
        bind_slsa_v1_statement(&bytes, &wrong, VerificationState::Verified)
            .expect_err("workflow/build config substitution must fail")
            .reason_code,
        "workflow_path_mismatch"
    );
}

#[test]
fn a3_07_unverified_provenance_cannot_become_verified_reference() {
    let fixture = fixture_document();
    assert!(fixture["provenance_summary"]["provenance"][0]["envelope"]["signatures"]
        .as_array()
        .is_some_and(|signatures| !signatures.is_empty()));

    let bytes = upstream_statement_bytes();
    let bound = bind_slsa_v1_statement(&bytes, &context(&bytes), VerificationState::Unverified)
        .expect("unverified provenance may be parsed and bound without being trusted");
    assert!(!bound.verified);
    assert_eq!(
        bound
            .to_provenance_reference()
            .expect_err("unverified provenance cannot become verified reference")
            .reason_code,
        "provenance_unverified"
    );
}

#[test]
fn a3_08_provenance_absence_cannot_be_laundered_into_success() {
    let bytes = upstream_statement_bytes();
    let absent = bind_optional_slsa_v1_statement(
        None,
        &context(&bytes),
        VerificationState::Verified,
    )
    .expect("absence is explicit");
    assert!(absent.is_none());
}

#[test]
fn a3_09_predicate_type_or_statement_type_substitution_is_rejected() {
    let bytes = upstream_statement_bytes();

    let wrong_statement_type = mutate_statement(&bytes, |value| {
        value["_type"] = Value::String("https://in-toto.io/Statement/v0.1".to_owned());
    });
    assert_eq!(
        bind_slsa_v1_statement(
            &wrong_statement_type,
            &context(&wrong_statement_type),
            VerificationState::Verified,
        )
        .expect_err("statement type substitution must fail")
        .reason_code,
        "statement_type_mismatch"
    );

    let wrong_predicate = mutate_statement(&bytes, |value| {
        value["predicateType"] = Value::String("https://slsa.dev/provenance/v0.2".to_owned());
    });
    assert_eq!(
        bind_slsa_v1_statement(
            &wrong_predicate,
            &context(&wrong_predicate),
            VerificationState::Verified,
        )
        .expect_err("predicate substitution must fail")
        .reason_code,
        "slsa_predicate_type_mismatch"
    );
}

#[test]
fn a3_10_replay_under_different_artifact_source_or_workflow_context_is_rejected() {
    let bytes = upstream_statement_bytes();

    let mut other_artifact = context(&bytes);
    other_artifact.artifact_name = "https://example.invalid/replayed-artifact".to_owned();
    assert!(bind_slsa_v1_statement(&bytes, &other_artifact, VerificationState::Verified).is_err());

    let mut other_source = context(&bytes);
    other_source.source_repository = "git+https://github.com/replay/source".to_owned();
    assert!(bind_slsa_v1_statement(&bytes, &other_source, VerificationState::Verified).is_err());

    let mut other_workflow = context(&bytes);
    other_workflow.workflow_path = "other-build.yaml".to_owned();
    assert!(bind_slsa_v1_statement(&bytes, &other_workflow, VerificationState::Verified).is_err());
}

#[test]
fn a3_11_malformed_truncated_or_missing_required_binding_fields_fail_closed() {
    let malformed = br#"{"_type":"https://in-toto.io/Statement/v1""#;
    let malformed_context = SlsaBindingContext {
        expected_statement_digest: sha256_labeled(malformed),
        artifact_name: SUBJECT_NAME.to_owned(),
        artifact_digest: format!("sha256:{SUBJECT_HEX}"),
        source_repository: SOURCE_REPOSITORY.to_owned(),
        source_revision: SOURCE_REVISION.to_owned(),
        workflow_path: WORKFLOW_PATH.to_owned(),
    };
    assert!(bind_slsa_v1_statement(malformed, &malformed_context, VerificationState::Verified).is_err());

    let bytes = upstream_statement_bytes();
    let variants = [
        mutate_statement(&bytes, |value| {
            value["subject"][0]["digest"].as_object_mut().expect("digest object").remove("sha256");
        }),
        mutate_statement(&bytes, |value| {
            value["predicate"]["buildDefinition"]["externalParameters"]["buildConfigSource"]
                .as_object_mut().expect("source object").remove("repository");
        }),
        mutate_statement(&bytes, |value| {
            value["predicate"]["buildDefinition"]["resolvedDependencies"] = Value::Array(vec![]);
        }),
        mutate_statement(&bytes, |value| {
            value["predicate"]["buildDefinition"]["externalParameters"]["buildConfigSource"]
                .as_object_mut().expect("source object").remove("path");
        }),
    ];

    for variant in variants {
        assert!(bind_slsa_v1_statement(&variant, &context(&variant), VerificationState::Verified).is_err());
    }
}

#[test]
fn a3_12_provenance_presence_does_not_infer_slsa_level_or_semantic_authority() {
    let bytes = upstream_statement_bytes();
    let bound = bind_slsa_v1_statement(&bytes, &context(&bytes), VerificationState::Verified)
        .expect("valid bound provenance");
    let serialized = serde_json::to_value(&bound).expect("serialize bounded record");
    let object = serialized.as_object().expect("bound record object");
    assert!(!object.contains_key("slsaLevel"));
    assert!(!object.contains_key("slsa_level"));
    assert!(!object.contains_key("authority"));
    assert!(!object.contains_key("completeness"));
    assert!(!object.contains_key("verdict"));

    let mut input = review_input();
    let original_authority = input.authority;
    let original_completeness = input.completeness.clone();
    let original_verdict = input.verdict;
    attach_verified_reference(&mut input, &bound).expect("attach verified reference");
    assert_eq!(input.authority, original_authority);
    assert_eq!(input.completeness, original_completeness);
    assert_eq!(input.verdict, original_verdict);

    let bundle = build_bundle(&input).expect("review bundle remains valid with provenance reference");
    assert_eq!(bundle.scai.predicate.attributes[0].conditions.authority, AuthorityState::Ambiguous);
    assert_eq!(bundle.scai.predicate.attributes[0].conditions.verdict, Verdict::Review);
}
