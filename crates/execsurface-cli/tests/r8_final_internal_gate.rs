#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

const V3_ALLOW_TRUNCATE_BLOCK_POLICY: &str = r#"{
  "schema_version": 3,
  "default_action": "allow",
  "rules": [{
    "id": "block-truncating-open",
    "action": "block",
    "match": {
      "effect": "file_open",
      "open_intent": { "truncate": true }
    }
  }]
}
"#;

const V3_ALLOW_POLICY: &str = r#"{
  "schema_version": 3,
  "default_action": "allow",
  "rules": []
}
"#;

const V3_BLOCK_POLICY: &str = r#"{
  "schema_version": 3,
  "default_action": "block",
  "rules": []
}
"#;

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn temp_dir(name: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "execsurface-stage2-r8-{name}-{}-{id}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("create R8 temp dir");
    path
}

fn learn(dir: &Path, baseline: &Path) {
    let output = Command::new(cli())
        .current_dir(dir)
        .args(["learn", "--output"])
        .arg(baseline)
        .args(["--", "/bin/sh", "-c", "true"])
        .output()
        .expect("learn R8 baseline");
    assert!(
        output.status.success(),
        "baseline learn failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn baseline_digest(path: &Path) -> String {
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(path).expect("read baseline")).expect("baseline JSON");
    value["baseline_digest"]
        .as_str()
        .expect("baseline digest")
        .to_owned()
}

fn policy_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn run_check(
    dir: &Path,
    baseline: &Path,
    policy: &Path,
    expected_baseline: &str,
    expected_policy: &str,
    json_output: Option<&Path>,
    target: &str,
) -> std::process::Output {
    let mut command = Command::new(cli());
    command
        .current_dir(dir)
        .args(["check", "--baseline"])
        .arg(baseline)
        .args(["--policy"])
        .arg(policy)
        .args(["--expect-baseline-digest", expected_baseline])
        .args(["--expect-policy-sha256", expected_policy]);

    if let Some(path) = json_output {
        command.args(["--json-output"]).arg(path);
    }

    command
        .args(["--", "/bin/sh", "-c", target])
        .output()
        .expect("run R8 check")
}

fn report_verdict(path: &Path) -> String {
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(path).expect("read verdict report"))
            .expect("verdict report JSON");
    value["verdict"]
        .as_str()
        .expect("verdict string")
        .to_owned()
}

#[test]
fn r8_c1_custody_v3_truncate_policy_and_safe_report_materialization_compose() {
    let dir = temp_dir("c1-v3-block");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy-v3.json");
    let target_file = dir.join("target.bin");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    fs::write(&policy, V3_ALLOW_TRUNCATE_BLOCK_POLICY).expect("write v3 policy");

    let expected_baseline = baseline_digest(&baseline);
    let expected_policy = policy_digest(V3_ALLOW_TRUNCATE_BLOCK_POLICY.as_bytes());
    let target = format!("printf x > '{}'", target_file.display());

    let output = run_check(
        &dir,
        &baseline,
        &policy,
        &expected_baseline,
        &expected_policy,
        Some(&report),
        &target,
    );

    assert_eq!(
        output.status.code(),
        Some(20),
        "v3 truncate rule must survive custody + observation + verdict materialization: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fs::read(&target_file).expect("target bytes"), b"x");
    assert_eq!(report_verdict(&report), "block");

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r8_c2_unauthorized_baseline_pin_dominates_policy_and_target_execution() {
    let dir = temp_dir("c2-custody-dominates");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy-v3.json");
    let marker = dir.join("TARGET_RAN");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    fs::write(&policy, V3_ALLOW_POLICY).expect("write v3 allow policy");

    let wrong_baseline = format!("sha256:{}", "0".repeat(64));
    let expected_policy = policy_digest(V3_ALLOW_POLICY.as_bytes());
    let target = format!("touch '{}'", marker.display());

    let output = run_check(
        &dir,
        &baseline,
        &policy,
        &wrong_baseline,
        &expected_policy,
        Some(&report),
        &target,
    );

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "custody mismatch must reject before target execution"
    );
    assert!(
        !report.exists(),
        "custody failure must not be laundered into a policy verdict report"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r8_c3_policy_output_alias_is_rejected_before_target_under_v3_custody() {
    let dir = temp_dir("c3-policy-output-alias");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy-v3.json");
    let marker = dir.join("TARGET_RAN");

    learn(&dir, &baseline);
    fs::write(&policy, V3_ALLOW_POLICY).expect("write v3 allow policy");

    let expected_baseline = baseline_digest(&baseline);
    let expected_policy = policy_digest(V3_ALLOW_POLICY.as_bytes());
    let before = fs::read(&policy).expect("policy bytes before");
    let target = format!("touch '{}'", marker.display());

    let output = run_check(
        &dir,
        &baseline,
        &policy,
        &expected_baseline,
        &expected_policy,
        Some(&policy),
        &target,
    );

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "protected policy/output alias must fail before target execution"
    );
    assert_eq!(
        fs::read(&policy).expect("policy bytes after"),
        before,
        "trusted policy bytes must remain unchanged"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r8_c4_moved_trusted_baseline_inode_cannot_become_v3_report_output() {
    let dir = temp_dir("c4-moved-baseline");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy-v3.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    fs::write(&policy, V3_ALLOW_POLICY).expect("write v3 allow policy");

    let expected_baseline = baseline_digest(&baseline);
    let expected_policy = policy_digest(V3_ALLOW_POLICY.as_bytes());
    let baseline_before = fs::read(&baseline).expect("baseline bytes before");
    let target = format!("mv '{}' '{}'", baseline.display(), report.display());

    let output = run_check(
        &dir,
        &baseline,
        &policy,
        &expected_baseline,
        &expected_policy,
        Some(&report),
        &target,
    );

    assert_eq!(
        output.status.code(),
        Some(2),
        "trusted-object move must fail before report materialization"
    );
    assert!(!baseline.exists(), "target must have moved the baseline pathname");
    assert_eq!(
        fs::read(&report).expect("moved trusted object"),
        baseline_before,
        "ExecSurface must not overwrite the moved trusted baseline inode"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r8_c5_target_policy_mutation_cannot_change_the_verified_policy_already_consumed() {
    let dir = temp_dir("c5-policy-mutation");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy-v3.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    fs::write(&policy, V3_BLOCK_POLICY).expect("write blocking v3 policy");

    let expected_baseline = baseline_digest(&baseline);
    let expected_policy = policy_digest(V3_BLOCK_POLICY.as_bytes());
    let replacement = V3_ALLOW_POLICY.replace('\n', "");
    let target = format!(
        "printf '%s' '{}' > '{}'",
        replacement,
        policy.display()
    );

    let output = run_check(
        &dir,
        &baseline,
        &policy,
        &expected_baseline,
        &expected_policy,
        Some(&report),
        &target,
    );

    assert_eq!(
        output.status.code(),
        Some(20),
        "post-preflight policy-file mutation must not replace the verified in-memory policy: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(&policy).expect("mutated policy"),
        replacement,
        "target must actually replace the on-disk policy to exercise the temporal boundary"
    );
    assert_eq!(report_verdict(&report), "block");

    let _ = fs::remove_dir_all(dir);
}
