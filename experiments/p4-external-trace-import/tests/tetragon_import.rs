use std::collections::BTreeSet;

use execsurface_model::semantics_v3::{CompletenessDimension, ProofRequirement};
use execsurface_p4_backend_authority::{AuthorityState, PropositionCompleteness};
use execsurface_p4_external_trace_import::{
    import_tetragon_event, ExternalSchemaIdentity, ImportDecision, IMPORT_PROFILE,
    INCOMPLETE_REASON,
};

fn valid_event() -> String {
    r#"{
      "node_name":"node-a",
      "process_exec":{
        "process":{
          "exec_id":"node-a:100:1",
          "parent_exec_id":"node-a:90:1",
          "binary":"/usr/bin/python3",
          "flags":"execve clone"
        },
        "parent":{
          "exec_id":"node-a:90:1",
          "binary":"/bin/bash"
        }
      }
    }"#
    .to_owned()
}

fn evidence(json: &str) -> execsurface_p4_external_trace_import::ImportedExecEvidence {
    match import_tetragon_event(&ExternalSchemaIdentity::pinned_tetragon(), json).expect("import") {
        ImportDecision::Evidence(record) => *record,
        other => panic!("expected evidence, got {other:?}"),
    }
}

fn non_authoritative(json: &str) -> execsurface_p4_external_trace_import::NonAuthoritativeImport {
    match import_tetragon_event(&ExternalSchemaIdentity::pinned_tetragon(), json).expect("import") {
        ImportDecision::NonAuthoritative(record) => record,
        other => panic!("expected non-authoritative record, got {other:?}"),
    }
}

#[test]
fn qualifying_execve_maps_to_direct_incomplete_only() {
    let record = evidence(&valid_event());
    assert_eq!(record.producer_profile, IMPORT_PROFILE);
    assert_eq!(record.adapter.authority, AuthorityState::Direct);
    assert_eq!(
        record.adapter.completeness,
        PropositionCompleteness::Incomplete
    );
    assert!(record.adapter.reason_codes.contains(INCOMPLETE_REASON));
    assert_eq!(record.adapter.proposition_id, "P4.EXEC.SUCCESS");
}

#[test]
fn imported_event_cannot_satisfy_complete_session_requirement() {
    let record = evidence(&valid_event());
    let requirement = ProofRequirement {
        guarantees: record.adapter.proof.guarantees.clone(),
        required_complete: BTreeSet::from([CompletenessDimension::SessionScope]),
    };
    assert!(!record.adapter.admissible_for(&requirement));
}

#[test]
fn procfs_bootstrap_cannot_become_exec_success_authority() {
    let json = valid_event().replace("execve clone", "procFS");
    let record = non_authoritative(&json);
    assert_ne!(record.authority, AuthorityState::Direct);
    assert!(record
        .reason_codes
        .contains("procfs_bootstrap_not_exec_transition"));
}

#[test]
fn contradictory_execve_procfs_flags_fail_closed() {
    let json = valid_event().replace("execve clone", "execve procFS");
    let record = non_authoritative(&json);
    assert!(record
        .reason_codes
        .contains("contradictory_execve_procfs_flags"));
}

#[test]
fn trunc_filename_fails_closed() {
    let json = valid_event().replace("execve clone", "execve truncFilename");
    let record = non_authoritative(&json);
    assert!(record.reason_codes.contains("external_flag_truncFilename"));
}

#[test]
fn error_unknown_and_miss_flags_each_fail_closed() {
    for flag in ["errorFilename", "unknown", "miss"] {
        let json = valid_event().replace("execve clone", &format!("execve {flag}"));
        let record = non_authoritative(&json);
        assert!(record
            .reason_codes
            .contains(&format!("external_flag_{flag}")));
    }
}

#[test]
fn parent_exec_id_mismatch_fails_closed() {
    let json = valid_event().replace(
        "node-a:90:1\",\n          \"binary",
        "node-a:91:1\",\n          \"binary",
    );
    let record = non_authoritative(&json);
    assert!(record
        .reason_codes
        .contains("parent_exec_identity_mismatch"));
}

#[test]
fn missing_causal_parent_identity_fails_closed() {
    let json = valid_event().replace(
        "\"parent_exec_id\":\"node-a:90:1\",",
        "\"parent_exec_id\":\"\",",
    );
    let record = non_authoritative(&json);
    assert!(record.reason_codes.contains("missing_parent_exec_identity"));
}

#[test]
fn empty_or_relative_executable_identity_fails_closed() {
    for binary in ["", "usr/bin/python3"] {
        let json = valid_event().replace("/usr/bin/python3", binary);
        let record = non_authoritative(&json);
        assert!(record
            .reason_codes
            .contains("invalid_subject_binary_identity"));
    }
}

#[test]
fn non_process_exec_event_is_unsupported() {
    let json = r#"{"process_exit":{"process":{"exec_id":"x"}}}"#;
    match import_tetragon_event(&ExternalSchemaIdentity::pinned_tetragon(), json).expect("import") {
        ImportDecision::Unsupported(record) => {
            assert_eq!(record.reason_code, "unsupported_external_event_type")
        }
        other => panic!("expected unsupported, got {other:?}"),
    }
}

#[test]
fn unpinned_schema_identity_is_rejected() {
    let mut schema = ExternalSchemaIdentity::pinned_tetragon();
    schema.schema_blob = "substituted".to_owned();
    let error = import_tetragon_event(&schema, &valid_event()).expect_err("must reject schema");
    assert_eq!(error.reason_code, "external_schema_identity_mismatch");
}

#[test]
fn producer_backend_name_substitution_cannot_upgrade_authority_or_completeness() {
    let base = evidence(&valid_event());
    let injected = valid_event().replace(
        "\"node_name\":\"node-a\"",
        "\"node_name\":\"node-a\",\"producer\":\"trusted-super-backend\",\"profile\":\"complete\"",
    );
    let changed = evidence(&injected);
    assert_eq!(base.adapter.authority, changed.adapter.authority);
    assert_eq!(base.adapter.completeness, changed.adapter.completeness);
    assert_eq!(changed.producer_profile, IMPORT_PROFILE);
}

#[test]
fn json_object_key_reordering_preserves_raw_and_imported_identity() {
    let first = evidence(&valid_event());
    let reordered = r#"{
      "process_exec":{
        "parent":{"binary":"/bin/bash","exec_id":"node-a:90:1"},
        "process":{"flags":"execve clone","binary":"/usr/bin/python3","parent_exec_id":"node-a:90:1","exec_id":"node-a:100:1"}
      },
      "node_name":"node-a"
    }"#;
    let second = evidence(reordered);
    assert_eq!(first.raw_event_digest, second.raw_event_digest);
    assert_eq!(first.imported_record_digest, second.imported_record_digest);
}

#[test]
fn raw_evidence_mutation_changes_raw_and_imported_identity() {
    let first = evidence(&valid_event());
    let mutated = valid_event().replace("/usr/bin/python3", "/usr/bin/curl");
    let second = evidence(&mutated);
    assert_ne!(first.raw_event_digest, second.raw_event_digest);
    assert_ne!(first.imported_record_digest, second.imported_record_digest);
}
