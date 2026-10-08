#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn temp_dir(name: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "execsurface-merge-auth-{name}-{}-{id}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("create merge-authorization temp dir");
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
        serde_json::from_slice(&fs::read(path).expect("read baseline")).expect("baseline JSON");
    value["baseline_digest"]
        .as_str()
        .expect("baseline digest")
        .to_owned()
}

#[test]
fn merge_auth_written_object_renamed_onto_report_must_not_be_overwritten() {
    let dir = temp_dir("written-rename-output");
    let baseline = dir.join("baseline.lock.json");
    let source = dir.join("workload-owned.bin");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);

    let target = format!(
        "printf 'WORKLOAD-STATE' > '{}' && mv '{}' '{}'",
        source.display(),
        source.display(),
        report.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run merge-authorization destruction case");

    assert_eq!(
        output.status.code(),
        Some(2),
        "a report path rebound to an object already written by the workload must fail report materialization; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(&report).expect("workload object at report path"),
        b"WORKLOAD-STATE",
        "ExecSurface must not overwrite a workload-written object merely because the workload renamed it after the write"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_written_object_chained_renames_onto_report_must_not_be_overwritten() {
    let dir = temp_dir("written-chained-rename-output");
    let baseline = dir.join("baseline.lock.json");
    let source = dir.join("workload-owned.bin");
    let middle = dir.join("middle.bin");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);

    let target = format!(
        "printf 'WORKLOAD-STATE' > '{}' && mv '{}' '{}' && mv '{}' '{}'",
        source.display(),
        source.display(),
        middle.display(),
        middle.display(),
        report.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run chained-rename destruction case");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        fs::read(&report).expect("workload object after chained rename"),
        b"WORKLOAD-STATE"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_written_child_survives_parent_directory_rename_lineage() {
    let dir = temp_dir("written-parent-rename-output");
    let baseline = dir.join("baseline.lock.json");
    let old_dir = dir.join("old");
    let new_dir = dir.join("new");
    let source = old_dir.join("owned.bin");
    let report = new_dir.join("owned.bin");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);

    let target = format!(
        "mkdir -p '{}' && printf 'WORKLOAD-STATE' > '{}' && mv '{}' '{}'",
        old_dir.display(),
        source.display(),
        old_dir.display(),
        new_dir.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run directory-rename destruction case");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        fs::read(&report).expect("workload child after parent rename"),
        b"WORKLOAD-STATE"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_distinct_report_path_remains_available_after_workload_rename() {
    let dir = temp_dir("distinct-report-control");
    let baseline = dir.join("baseline.lock.json");
    let source = dir.join("workload-owned.bin");
    let final_workload = dir.join("workload-final.bin");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);

    let target = format!(
        "printf 'WORKLOAD-STATE' > '{}' && mv '{}' '{}'",
        source.display(),
        source.display(),
        final_workload.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run distinct-report control");

    assert_eq!(
        output.status.code(),
        Some(10),
        "ordinary drift should remain REVIEW when the report is disjoint; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(&final_workload).expect("final workload object"),
        b"WORKLOAD-STATE"
    );
    let report_json: serde_json::Value =
        serde_json::from_slice(&fs::read(&report).expect("report bytes")).expect("report JSON");
    assert_eq!(report_json["verdict"], "review");

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_written_object_hardlinked_to_report_then_source_removed_is_protected() {
    let dir = temp_dir("written-hardlink-unlink-output");
    let baseline = dir.join("baseline.lock.json");
    let source = dir.join("workload-owned.bin");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);

    let target = format!(
        "printf 'WORKLOAD-STATE' > '{}' && ln '{}' '{}' && rm '{}'",
        source.display(),
        source.display(),
        report.display(),
        source.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run hardlink-unlink destruction case");

    assert_eq!(
        output.status.code(),
        Some(2),
        "a report path rebound to a hardlink of a workload-written object must fail closed"
    );
    assert_eq!(
        fs::read(&report).expect("hardlinked workload object at report path"),
        b"WORKLOAD-STATE",
        "ExecSurface must not overwrite a workload-written inode after its original pathname is removed"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_truncate_only_report_path_must_not_be_overwritten() {
    let dir = temp_dir("truncate-only-output");
    let baseline = dir.join("baseline.lock.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);
    fs::write(&report, b"PREEXISTING").expect("seed report target");

    let target = format!(": > '{}'", report.display());

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run truncate-only destruction case");

    assert_eq!(
        output.status.code(),
        Some(2),
        "a report path truncated by the workload must be treated as workload-mutated even without a later write syscall; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(&report).expect("truncated workload-owned report path"),
        b"",
        "ExecSurface must not overwrite the empty state produced by workload O_TRUNC"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_ftruncate_only_report_object_must_not_be_overwritten() {
    let dir = temp_dir("ftruncate-only-output");
    let baseline = dir.join("baseline.lock.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);
    fs::write(&report, b"PREEXISTING").expect("seed report target");

    let target = format!(
        "python3 -c \"import os; p=r'{}'; fd=os.open(p, os.O_WRONLY); os.ftruncate(fd, 0); os.close(fd)\"",
        report.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run ftruncate-only destruction case");

    assert_eq!(
        output.status.code(),
        Some(2),
        "a report object truncated through ftruncate must remain workload-owned; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(&report).expect("ftruncated report object"),
        b"",
        "ExecSurface must not overwrite the empty state produced by workload ftruncate"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_deleted_report_path_must_not_be_recreated() {
    let dir = temp_dir("deleted-output");
    let baseline = dir.join("baseline.lock.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);
    fs::write(&report, b"PREEXISTING").expect("seed report target");

    let target = format!("rm '{}'", report.display());

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run deleted-output destruction case");

    assert_eq!(
        output.status.code(),
        Some(2),
        "a report path deleted by the workload must not be silently recreated; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !report.exists(),
        "ExecSurface must preserve the workload's final deleted state"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_created_report_path_without_write_must_not_be_overwritten() {
    let dir = temp_dir("created-output");
    let baseline = dir.join("baseline.lock.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);

    let target = format!(
        "python3 -c \"import os; p=r'{}'; fd=os.open(p, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600); os.close(fd)\"",
        report.display()
    );

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("run created-output destruction case");

    assert_eq!(
        output.status.code(),
        Some(2),
        "a report path created by the workload must remain workload-owned even without a write syscall; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(&report).expect("workload-created report path"),
        b"",
        "ExecSurface must not overwrite the empty object created by the workload"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn merge_auth_untouched_preexisting_report_remains_usable() {
    let dir = temp_dir("untouched-preexisting-output");
    let baseline = dir.join("baseline.lock.json");
    let report = dir.join("report.json");

    learn(&dir, &baseline);
    let expected_baseline = baseline_digest(&baseline);
    fs::write(&report, b"STALE-REPORT").expect("seed old report");

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--json-output"])
        .arg(&report)
        .args(["--", "/bin/sh", "-c", "echo controlled-drift >/dev/null"])
        .output()
        .expect("run untouched-output control");

    assert_eq!(
        output.status.code(),
        Some(10),
        "an untouched preexisting report path must remain available for ordinary report replacement; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report_json: serde_json::Value =
        serde_json::from_slice(&fs::read(&report).expect("report bytes")).expect("report JSON");
    assert_eq!(report_json["verdict"], "review");

    let _ = fs::remove_dir_all(dir);
}
