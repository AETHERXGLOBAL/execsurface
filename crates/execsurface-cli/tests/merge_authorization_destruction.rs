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
