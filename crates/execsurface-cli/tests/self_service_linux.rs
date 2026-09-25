#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "execsurface-m66-{name}-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn version_reports_public_alpha_without_touching_schema_versions() {
    let output = Command::new(binary()).arg("--version").output().expect("version");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "execsurface 0.1.0-alpha.1"
    );
}

#[test]
fn doctor_proves_supported_ci_readiness() {
    let dir = temp_dir("doctor");
    fs::create_dir_all(&dir).expect("dir");
    let output = Command::new(binary())
        .arg("doctor")
        .current_dir(&dir)
        .output()
        .expect("doctor");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[PASS] Linux"));
    assert!(stdout.contains("[PASS] x86_64"));
    assert!(stdout.contains("[PASS] ptrace observer available"));
    assert!(stdout.contains("[PASS] workspace writable"));
    assert!(stdout.contains("[PASS] ExecSurface 0.1.0-alpha.1"));
    assert!(stdout.contains("Ready."));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn init_generates_files_but_never_runs_the_target_command() {
    let dir = temp_dir("init");
    fs::create_dir_all(&dir).expect("dir");
    let marker = dir.join("MUST_NOT_EXIST");
    let command = format!("touch {}", marker.display());

    let output = Command::new(binary())
        .args(["init", "--command", &command, "--github-actions"])
        .current_dir(&dir)
        .output()
        .expect("init");

    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(!marker.exists(), "init executed the user target command");
    let policy = fs::read_to_string(dir.join("execsurface-policy.json")).expect("policy");
    assert!(policy.contains("\"schema_version\": 2"));
    let workflow =
        fs::read_to_string(dir.join(".github/workflows/execsurface.yml")).expect("workflow");
    assert!(workflow.contains("AETHERXGLOBAL/execsurface@v0.1"));
    assert!(workflow.contains("3d3c42e5aac5ba805825da76410c181273ba90b1"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[NOT RUN] target command"));
    assert!(stdout.contains("/bin/bash -lc"));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn init_refuses_existing_files_without_explicit_force() {
    let dir = temp_dir("overwrite");
    fs::create_dir_all(&dir).expect("dir");
    fs::write(dir.join("execsurface-policy.json"), b"keep-me\n").expect("seed");

    let output = Command::new(binary())
        .args(["init", "--command", "true"])
        .current_dir(&dir)
        .output()
        .expect("init");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        fs::read_to_string(dir.join("execsurface-policy.json")).expect("policy"),
        "keep-me\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("refusing to overwrite"));
    let _ = fs::remove_dir_all(dir);
}
