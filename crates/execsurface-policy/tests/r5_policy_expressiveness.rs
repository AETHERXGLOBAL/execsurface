use execsurface_diff::{DiffReport, TargetOutcome};
use execsurface_model::canonical::{
    CanonicalEffect, CanonicalPath, OpenIntent, PathClass, PathResolution,
};
use execsurface_model::FileOperation;
use execsurface_policy::{evaluate, validate_policy, FindingAction, Policy, PolicyError, Verdict};

fn diff_with_added(effect: CanonicalEffect) -> DiffReport {
    DiffReport {
        schema_version: 2,
        baseline_digest: "sha256:r5".to_owned(),
        target: TargetOutcome {
            exit_code: Some(0),
            signal: None,
        },
        added: vec![effect],
        removed: vec![],
        changed: vec![],
    }
}

fn path(value: &str, resolution: PathResolution) -> CanonicalPath {
    CanonicalPath {
        value: value.to_owned(),
        class: PathClass::Workspace,
        resolution,
    }
}

fn file_open(truncate: bool) -> CanonicalEffect {
    CanonicalEffect::FilePathAccess {
        actor: None,
        execution_chain: vec![],
        operation: FileOperation::Open,
        target: path("$WORKSPACE/data.bin", PathResolution::Lexical),
        open_intent: Some(OpenIntent {
            read: false,
            write: true,
            create: false,
            truncate,
            append: false,
            path_only: false,
            resolve_flags: 0,
            other_flags: 0,
        }),
    }
}

fn file_read(resolution: PathResolution) -> CanonicalEffect {
    CanonicalEffect::FilePathAccess {
        actor: None,
        execution_chain: vec![],
        operation: FileOperation::Read,
        target: path("$WORKSPACE/data.bin", resolution),
        open_intent: None,
    }
}

fn rename(from: &str) -> CanonicalEffect {
    CanonicalEffect::FileRename {
        actor: None,
        execution_chain: vec![],
        from: path(from, PathResolution::Lexical),
        to: path("$WORKSPACE/out/final.bin", PathResolution::Lexical),
    }
}

fn parse_policy(json: &str) -> Policy {
    serde_json::from_str(json).expect("R5 policy must parse")
}

#[test]
fn r5_v3_open_intent_separates_truncate_from_non_truncate() {
    let policy = parse_policy(
        r#"{
          "schema_version": 3,
          "default_action": "allow",
          "rules": [{
            "id": "block-truncate",
            "action": "block",
            "match": {
              "effect": "file_open",
              "open_intent": { "truncate": true }
            }
          }]
        }"#,
    );

    let truncate = evaluate(&diff_with_added(file_open(true)), &policy, "r5").expect("truncate");
    let non_truncate =
        evaluate(&diff_with_added(file_open(false)), &policy, "r5").expect("non-truncate");

    assert_eq!(truncate.verdict, Verdict::Block);
    assert_eq!(non_truncate.verdict, Verdict::Pass);
}

#[test]
fn r5_v3_path_resolution_separates_kernel_fd_from_lexical() {
    let policy = parse_policy(
        r#"{
          "schema_version": 3,
          "default_action": "allow",
          "rules": [{
            "id": "block-kernel-fd-read",
            "action": "block",
            "match": {
              "effect": "file_read",
              "path_resolution": "kernel_fd_resolved"
            }
          }]
        }"#,
    );

    let kernel = evaluate(
        &diff_with_added(file_read(PathResolution::KernelFdResolved)),
        &policy,
        "r5",
    )
    .expect("kernel-resolved");
    let lexical = evaluate(
        &diff_with_added(file_read(PathResolution::Lexical)),
        &policy,
        "r5",
    )
    .expect("lexical");

    assert_eq!(kernel.verdict, Verdict::Block);
    assert_eq!(lexical.verdict, Verdict::Pass);
}

#[test]
fn r5_v3_rename_source_separates_same_destination_by_from_prefix() {
    let policy = parse_policy(
        r#"{
          "schema_version": 3,
          "default_action": "allow",
          "rules": [{
            "id": "block-sensitive-rename-source",
            "action": "block",
            "match": {
              "effect": "file_rename",
              "rename_from_prefix": "$WORKSPACE/sensitive"
            }
          }]
        }"#,
    );

    let blocked = evaluate(
        &diff_with_added(rename("$WORKSPACE/sensitive/input.bin")),
        &policy,
        "r5",
    )
    .expect("sensitive source");
    let allowed = evaluate(
        &diff_with_added(rename("$WORKSPACE/public/input.bin")),
        &policy,
        "r5",
    )
    .expect("public source");

    assert_eq!(blocked.verdict, Verdict::Block);
    assert_eq!(allowed.verdict, Verdict::Pass);
}

#[test]
fn r5_v1_v2_reject_v3_only_fields_instead_of_ignoring_them() {
    for schema_version in [1, 2] {
        let policy = parse_policy(&format!(
            r#"{{
              "schema_version": {schema_version},
              "default_action": "review",
              "rules": [{{
                "id": "no-semantic-drift",
                "action": "block",
                "match": {{
                  "effect": "file_open",
                  "path_resolution": "lexical"
                }}
              }}]
            }}"#
        ));

        assert!(matches!(
            validate_policy(&policy),
            Err(PolicyError::InvalidMatcher { rule_id, .. }) if rule_id == "no-semantic-drift"
        ));
    }
}

#[test]
fn r5_v3_empty_open_intent_is_rejected() {
    let policy = parse_policy(
        r#"{
          "schema_version": 3,
          "default_action": "review",
          "rules": [{
            "id": "empty-open-intent",
            "action": "block",
            "match": {
              "effect": "file_open",
              "open_intent": {}
            }
          }]
        }"#,
    );

    assert!(matches!(
        validate_policy(&policy),
        Err(PolicyError::InvalidMatcher { rule_id, .. }) if rule_id == "empty-open-intent"
    ));
}

#[test]
fn r5_v3_inapplicable_matcher_is_rejected() {
    let policy = parse_policy(
        r#"{
          "schema_version": 3,
          "default_action": "review",
          "rules": [{
            "id": "wrong-effect",
            "action": "block",
            "match": {
              "effect": "network_connect",
              "open_intent": { "truncate": true }
            }
          }]
        }"#,
    );

    assert!(matches!(
        validate_policy(&policy),
        Err(PolicyError::InvalidMatcher { rule_id, .. }) if rule_id == "wrong-effect"
    ));
}

#[test]
fn r5_v3_most_restrictive_wins_is_unchanged() {
    let policy = parse_policy(
        r#"{
          "schema_version": 3,
          "default_action": "review",
          "rules": [
            {
              "id": "allow-open",
              "action": "allow",
              "match": { "effect": "file_open" }
            },
            {
              "id": "block-truncate",
              "action": "block",
              "match": {
                "effect": "file_open",
                "open_intent": { "truncate": true }
              }
            }
          ]
        }"#,
    );

    let report = evaluate(&diff_with_added(file_open(true)), &policy, "r5").expect("evaluate");
    assert_eq!(report.verdict, Verdict::Block);
    assert_eq!(report.findings[0].action, FindingAction::Block);
    assert_eq!(
        report.findings[0].matched_rules,
        vec!["allow-open".to_owned(), "block-truncate".to_owned()]
    );
}
