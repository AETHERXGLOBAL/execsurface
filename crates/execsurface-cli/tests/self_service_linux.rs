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
fn version_reports_current_package_without_touching_schema_versions() {
    let output = Command::new(binary())
        .arg("--version")
        .output()
        .expect("version");
    assert!(output.status.success());
    let expected = format!("execsurface {}", env!("CARGO_PKG_VERSION"));
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
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
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[PASS] Linux"));
    assert!(stdout.contains("[PASS] x86_64"));
    assert!(stdout.contains("[PASS] ptrace observer available"));
    assert!(stdout.contains("[PASS] workspace writable"));
    let expected = format!("[PASS] ExecSurface {}", env!("CARGO_PKG_VERSION"));
    assert!(stdout.contains(&expected));
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

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!marker.exists(), "init executed the user target command");
    let policy = fs::read_to_string(dir.join("execsurface-policy.json")).expect("policy");
    assert!(policy.contains("\"schema_version\": 2"));
    let workflow =
        fs::read_to_string(dir.join(".github/workflows/execsurface.yml")).expect("workflow");
    let expected_channel = if env!("CARGO_PKG_VERSION").starts_with("1.") {
        "AETHERXGLOBAL/execsurface@v1"
    } else {
        "AETHERXGLOBAL/execsurface@v0.1"
    };
    assert!(
        workflow.contains(expected_channel),
        "generated workflow must use the qualified stable channel for package version {}",
        env!("CARGO_PKG_VERSION")
    );
    assert!(workflow.contains("3d3c42e5aac5ba805825da76410c181273ba90b1"));
    assert!(workflow.contains("require-custody: \"true\""));
    assert!(workflow.contains("${{ vars.EXECSURFACE_BASELINE_DIGEST }}"));
    assert!(workflow.contains("${{ vars.EXECSURFACE_POLICY_SHA256 }}"));
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
#[test]
fn action_contract_exposes_and_forwards_custody_inputs() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let action = fs::read_to_string(root.join("action.yml")).expect("action.yml");
    let runner = fs::read_to_string(root.join("action/run.sh")).expect("action runner");

    for input in [
        "expected-baseline-digest:",
        "expected-policy-sha256:",
        "require-custody:",
    ] {
        assert!(
            action.contains(input),
            "Action contract must expose R4 custody input {input}"
        );
    }

    assert!(action.contains("EXECSURFACE_EXPECTED_BASELINE_DIGEST"));
    assert!(action.contains("EXECSURFACE_EXPECTED_POLICY_SHA256"));
    assert!(action.contains("EXECSURFACE_REQUIRE_CUSTODY"));

    assert!(runner.contains("--expect-baseline-digest"));
    assert!(runner.contains("--expect-policy-sha256"));
    assert!(
        runner.contains("require-custody=true requires both"),
        "runner must fail closed when custody is required but either external pin is absent"
    );
}
