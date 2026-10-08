#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

const ALLOW_POLICY: &str =
    "{\n  \"schema_version\": 2,\n  \"default_action\": \"allow\",\n  \"rules\": []\n}\n";

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn temp_dir(name: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("execsurface-r7-{name}-{}-{id}", std::process::id()));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn learn(dir: &Path, baseline: &Path) {
    let output = Command::new(cli())
        .current_dir(dir)
        .args(["learn", "--output"])
        .arg(baseline)
        .args(["--", "/bin/sh", "-c", "true"])
        .output()
        .expect("learn baseline");
    assert!(
        output.status.success(),
        "learn failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn baseline_digest(path: &Path) -> String {
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(path).expect("read baseline")).expect("baseline json");
    value["baseline_digest"]
        .as_str()
        .expect("baseline digest")
        .to_owned()
}

fn policy_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn check(
    dir: &Path,
    baseline: &Path,
    policy: Option<&Path>,
    json_output: Option<&Path>,
    markdown_output: Option<&Path>,
    target: &str,
) -> std::process::Output {
    let expected_baseline = baseline_digest(baseline);
    let mut command = Command::new(cli());
    command
        .current_dir(dir)
        .args(["check", "--baseline"])
        .arg(baseline)
        .args(["--expect-baseline-digest", &expected_baseline]);

    if let Some(policy) = policy {
        let bytes = fs::read(policy).expect("read policy for custody digest");
        let digest = policy_digest(&bytes);
        command
            .args(["--policy"])
            .arg(policy)
            .args(["--expect-policy-sha256", &digest]);
    }

    if let Some(path) = json_output {
        command.args(["--json-output"]).arg(path);
    }
    if let Some(path) = markdown_output {
        command.args(["--markdown-output"]).arg(path);
    }

    command
        .args(["--", "/bin/sh", "-c", target])
        .output()
        .expect("run check")
}

#[test]
fn r7_json_output_cannot_replace_custody_verified_baseline() {
    let dir = temp_dir("baseline-direct");
    let baseline = dir.join("baseline.lock.json");
    let marker = dir.join("TARGET_RAN");
    learn(&dir, &baseline);
    let before = fs::read(&baseline).expect("baseline bytes before");
    let target = format!("touch {}", marker.display());

    let output = check(&dir, &baseline, None, Some(&baseline), None, &target);

    assert_eq!(
        output.status.code(),
        Some(2),
        "trusted-input/output collision must be ERROR"
    );
    assert!(
        !marker.exists(),
        "knowable baseline/output collision must fail before target execution"
    );
    assert_eq!(
        fs::read(&baseline).expect("baseline bytes after"),
        before,
        "custody-verified baseline must not be overwritten by verdict output"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r7_markdown_output_symlink_alias_cannot_replace_verified_policy() {
    let dir = temp_dir("policy-symlink");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy.json");
    let alias = dir.join("summary.md");
    let marker = dir.join("TARGET_RAN");
    learn(&dir, &baseline);
    fs::write(&policy, ALLOW_POLICY).expect("write policy");
    symlink(&policy, &alias).expect("symlink output to policy");
    let before = fs::read(&policy).expect("policy bytes before");
    let target = format!("touch {}", marker.display());

    let output = check(&dir, &baseline, Some(&policy), None, Some(&alias), &target);

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "symlink alias to policy must be rejected before target execution"
    );
    assert_eq!(
        fs::read(&policy).expect("policy bytes after"),
        before,
        "verified policy must survive symlink-output alias"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r7_json_output_hardlink_alias_cannot_replace_verified_baseline() {
    let dir = temp_dir("baseline-hardlink");
    let baseline = dir.join("baseline.lock.json");
    let alias = dir.join("report.json");
    let marker = dir.join("TARGET_RAN");
    learn(&dir, &baseline);
    fs::hard_link(&baseline, &alias).expect("hardlink output to baseline");
    let before = fs::read(&baseline).expect("baseline bytes before");
    let target = format!("touch {}", marker.display());

    let output = check(&dir, &baseline, None, Some(&alias), None, &target);

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "hardlink alias to baseline must be rejected before target execution"
    );
    assert_eq!(
        fs::read(&baseline).expect("baseline bytes after"),
        before,
        "verified baseline inode must survive hardlink-output alias"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r7_json_and_markdown_outputs_cannot_alias_each_other() {
    let dir = temp_dir("output-output");
    let baseline = dir.join("baseline.lock.json");
    let output_path = dir.join("evidence.out");
    let marker = dir.join("TARGET_RAN");
    learn(&dir, &baseline);
    let target = format!("touch {}", marker.display());

    let output = check(
        &dir,
        &baseline,
        None,
        Some(&output_path),
        Some(&output_path),
        &target,
    );

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "knowable JSON/Markdown alias must fail before target execution"
    );
    assert!(
        !output_path.exists(),
        "preflight-rejected output alias must not materialize either format"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r7_verdict_output_cannot_overwrite_workload_written_object() {
    let dir = temp_dir("workload-output");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("policy.json");
    let workload = dir.join("workload-owned.bin");
    learn(&dir, &baseline);
    fs::write(&policy, ALLOW_POLICY).expect("write policy");

    let target = format!("printf WORKLOAD-STATE > {}", workload.display());
    let output = check(
        &dir,
        &baseline,
        Some(&policy),
        Some(&workload),
        None,
        &target,
    );

    assert_eq!(
        output.status.code(),
        Some(2),
        "post-observation workload/output collision must fail report materialization"
    );
    assert_eq!(
        fs::read(&workload).expect("read workload-owned bytes"),
        b"WORKLOAD-STATE",
        "ExecSurface must not overwrite workload-owned state with its report"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r7_target_cannot_rebind_output_path_to_verified_baseline_after_preflight() {
    let dir = temp_dir("post-target-rebind");
    let baseline = dir.join("baseline.lock.json");
    let output_path = dir.join("report.json");
    learn(&dir, &baseline);
    let before = fs::read(&baseline).expect("baseline bytes before");

    let target = format!("ln -s {} {}", baseline.display(), output_path.display());
    let output = check(&dir, &baseline, None, Some(&output_path), None, &target);

    assert_eq!(
        output.status.code(),
        Some(2),
        "post-target output identity rebinding must fail before report write"
    );
    assert!(
        output_path.is_symlink(),
        "target should have executed and created the adversarial alias"
    );
    assert_eq!(
        fs::read(&baseline).expect("baseline bytes after"),
        before,
        "post-target symlink rebinding must not redirect report bytes into the baseline"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r7_target_cannot_move_verified_baseline_onto_output_path() {
    let dir = temp_dir("trusted-object-move");
    let baseline = dir.join("baseline.lock.json");
    let output_path = dir.join("report.json");
    learn(&dir, &baseline);
    let before = fs::read(&baseline).expect("baseline bytes before");

    let target = format!("mv {} {}", baseline.display(), output_path.display());
    let output = check(&dir, &baseline, None, Some(&output_path), None, &target);

    assert_eq!(
        output.status.code(),
        Some(2),
        "report output must not overwrite the preflight-verified baseline object after target rename"
    );
    assert!(
        !baseline.exists(),
        "target should have executed and moved the original baseline pathname"
    );
    assert_eq!(
        fs::read(&output_path).expect("moved baseline object"),
        before,
        "the original verified baseline object must remain intact at its new pathname"
    );

    let _ = fs::remove_dir_all(dir);
}
