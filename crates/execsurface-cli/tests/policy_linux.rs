#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use execsurface_policy::{Verdict, VerdictReport};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn temp_dir(name: &str) -> std::path::PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("execsurface-m5-{name}-{}-{id}", std::process::id()));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn learn(dir: &std::path::Path, baseline: &std::path::Path, command: &[&str]) {
    let mut cmd = Command::new(cli());
    cmd.current_dir(dir)
        .args(["learn", "--output"])
        .arg(baseline)
        .arg("--");
    cmd.args(command);
    let status = cmd.stdout(Stdio::null()).status().expect("learn");
    assert!(status.success());
}

fn check(
    dir: &std::path::Path,
    baseline: &std::path::Path,
    policy: Option<&std::path::Path>,
    command: &[&str],
) -> std::process::Output {
    let mut cmd = Command::new(cli());
    cmd.current_dir(dir)
        .args(["check", "--json", "--baseline"])
        .arg(baseline);
    if let Some(policy) = policy {
        cmd.arg("--policy").arg(policy);
    }
    cmd.arg("--").args(command);
    cmd.output().expect("check")
}

#[test]
fn no_drift_is_pass_with_exit_zero() {
    let dir = temp_dir("pass");
    let baseline = dir.join("lock.json");
    learn(&dir, &baseline, &["/bin/true"]);

    let output = check(&dir, &baseline, None, &["/bin/true"]);
    assert_eq!(output.status.code(), Some(0));
    let report: VerdictReport = serde_json::from_slice(&output.stdout).expect("verdict json");
    assert_eq!(report.verdict, Verdict::Pass);
    assert!(report.findings.is_empty());

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn unmatched_drift_is_review_with_stable_exit_code() {
    let dir = temp_dir("review");
    let baseline = dir.join("lock.json");
    learn(&dir, &baseline, &["/bin/sh", "-c", "true"]);

    let output = check(
        &dir,
        &baseline,
        None,
        &["/bin/sh", "-c", "true; /bin/echo x >/dev/null"],
    );
    assert_eq!(output.status.code(), Some(10));
    let report: VerdictReport = serde_json::from_slice(&output.stdout).expect("verdict json");
    assert_eq!(report.verdict, Verdict::Review);
    assert!(!report.findings.is_empty());

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn explicit_policy_can_block_added_exec() {
    let dir = temp_dir("block");
    let baseline = dir.join("lock.json");
    let policy = dir.join("policy.json");
    learn(&dir, &baseline, &["/bin/sh", "-c", "true"]);
    fs::write(
        &policy,
        br#"{
  "schema_version": 1,
  "default_action": "review",
  "rules": [
    {
      "id": "block-added-exec",
      "action": "block",
      "match": {
        "change": "added",
        "effect": "process_exec"
      }
    }
  ]
}"#,
    )
    .unwrap();

    let output = check(
        &dir,
        &baseline,
        Some(&policy),
        &["/bin/sh", "-c", "true; /bin/echo x >/dev/null"],
    );
    assert_eq!(output.status.code(), Some(20));
    let report: VerdictReport = serde_json::from_slice(&output.stdout).expect("verdict json");
    assert_eq!(report.verdict, Verdict::Block);
    assert!(report.findings.iter().any(|finding| finding
        .matched_rules
        .iter()
        .any(|id| id == "block-added-exec")));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn block_default_cannot_turn_no_drift_into_block() {
    let dir = temp_dir("no-drift-block-default");
    let baseline = dir.join("lock.json");
    let policy = dir.join("policy.json");
    learn(&dir, &baseline, &["/bin/true"]);
    fs::write(
        &policy,
        br#"{
  "schema_version": 1,
  "default_action": "block",
  "rules": []
}"#,
    )
    .unwrap();

    let output = check(&dir, &baseline, Some(&policy), &["/bin/true"]);
    assert_eq!(output.status.code(), Some(0));
    let report: VerdictReport = serde_json::from_slice(&output.stdout).expect("verdict json");
    assert_eq!(report.verdict, Verdict::Pass);

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn invalid_policy_is_error_exit_two() {
    let dir = temp_dir("invalid");
    let baseline = dir.join("lock.json");
    let policy = dir.join("policy.json");
    learn(&dir, &baseline, &["/bin/true"]);
    fs::write(
        &policy,
        br#"{
  "schema_version": 1,
  "default_action": "review",
  "rules": [
    {"id":"dup","action":"allow","match":{"effect":"process_exec"}},
    {"id":"dup","action":"block","match":{"effect":"process_exec"}}
  ]
}"#,
    )
    .unwrap();

    let output = check(&dir, &baseline, Some(&policy), &["/bin/true"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("duplicate policy rule id"));

    let _ = fs::remove_dir_all(dir);
}
