#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use execsurface_diff::DiffReport;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn temp_dir(name: &str) -> std::path::PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("execsurface-m4-{name}-{}-{id}", std::process::id()));
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

fn check_json(
    dir: &std::path::Path,
    baseline: &std::path::Path,
    command: &[&str],
) -> std::process::Output {
    let mut cmd = Command::new(cli());
    cmd.current_dir(dir)
        .args(["check", "--diff-only", "--json", "--baseline"])
        .arg(baseline)
        .arg("--");
    cmd.args(command);
    cmd.output().expect("check")
}

#[test]
fn real_learn_then_check_same_command_has_no_drift() {
    let dir = temp_dir("no-drift");
    let baseline = dir.join("lock.json");
    learn(&dir, &baseline, &["/bin/true"]);

    let output = check_json(&dir, &baseline, &["/bin/true"]);
    assert!(output.status.success());
    let report: DiffReport = serde_json::from_slice(&output.stdout).expect("diff json");
    assert!(report.added.is_empty());
    assert!(report.removed.is_empty());
    assert!(report.changed.is_empty());

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn real_check_reports_controlled_process_drift_without_policy_failure() {
    let dir = temp_dir("drift");
    let baseline = dir.join("lock.json");
    learn(&dir, &baseline, &["/bin/sh", "-c", "true"]);

    let output = check_json(
        &dir,
        &baseline,
        &["/bin/sh", "-c", "true; /bin/echo x >/dev/null"],
    );
    assert!(
        output.status.success(),
        "M4 drift is evidence, not a policy exit failure"
    );
    let report: DiffReport = serde_json::from_slice(&output.stdout).expect("diff json");
    assert!(!(report.added.is_empty() && report.removed.is_empty() && report.changed.is_empty()));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn corrupted_baseline_is_rejected() {
    let dir = temp_dir("corrupt");
    let baseline = dir.join("lock.json");
    learn(&dir, &baseline, &["/bin/true"]);

    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&baseline).unwrap()).unwrap();
    value["baseline_digest"] = serde_json::Value::String("sha256:deadbeef".to_owned());
    fs::write(&baseline, serde_json::to_vec_pretty(&value).unwrap()).unwrap();

    let output = check_json(&dir, &baseline, &["/bin/true"]);
    assert!(!output.status.success());

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn argv_secret_is_absent_from_json_diff() {
    let dir = temp_dir("privacy");
    let baseline = dir.join("lock.json");
    let secret = "AX_M4_SECRET_26d1f4";
    learn(&dir, &baseline, &["/bin/true", secret]);

    let output = check_json(&dir, &baseline, &["/bin/true", secret]);
    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains(secret));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(secret));

    let _ = fs::remove_dir_all(dir);
}
