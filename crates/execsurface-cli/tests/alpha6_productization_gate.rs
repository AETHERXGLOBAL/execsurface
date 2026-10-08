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
        "execsurface-alpha6-productization-{name}-{}-{nonce}",
        std::process::id()
    ))
}

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn landing_and_current_docs_present_alpha6_as_current_not_final() {
    let root = repo_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    let action = fs::read_to_string(root.join("docs/GITHUB_ACTION.md")).expect("Action guide");
    let status = fs::read_to_string(root.join("docs/STATUS.md")).expect("status");
    let quickstart =
        fs::read_to_string(root.join("docs/QUICKSTART_5_MIN.md")).expect("quickstart");

    assert!(
        readme.contains("status-Current%20Supported%20Alpha"),
        "landing badge must say Current Supported Alpha"
    );
    assert!(
        !readme.contains("status-Final%20Supported%20Alpha"),
        "landing badge must not imply Alpha.6 is a final/stable product"
    );
    assert!(
        action.contains("The current supported Alpha Action channel is:"),
        "Action guide must describe @v0.1 as the current supported Alpha channel"
    );
    assert!(
        status.contains("docs/releases/v0.1.0-alpha.6.md"),
        "authoritative status must point current users to the Alpha.6 release record"
    );
    assert!(
        status.contains("Previous release `v0.1.0-alpha.5` remains immutable"),
        "status must preserve Alpha.5 explicitly as historical/rollback evidence"
    );
    assert!(
        quickstart.contains("EXECSURFACE_BASELINE_DIGEST")
            && quickstart.contains("EXECSURFACE_POLICY_SHA256"),
        "Five-Minute Start must name the custody variables required by the generated Action"
    );
    assert!(
        quickstart.contains("execsurface init") && quickstart.contains("copy-paste"),
        "Five-Minute Start must tell users that init prints the custody setup commands"
    );
}

#[test]
fn generated_github_workflow_gates_target_success_before_execsurface() {
    let dir = temp_dir("target-gate");
    fs::create_dir_all(&dir).expect("dir");

    let output = Command::new(binary())
        .args([
            "init",
            "--command",
            "cargo test --locked",
            "--github-actions",
        ])
        .current_dir(&dir)
        .output()
        .expect("init");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let workflow =
        fs::read_to_string(dir.join(".github/workflows/execsurface.yml")).expect("workflow");
    let target_gate = workflow
        .find("- name: Run target command as its own correctness gate")
        .expect("generated workflow must include an independent target correctness gate");
    let execsurface = workflow
        .find("- name: ExecSurface runtime drift")
        .expect("generated workflow must include ExecSurface step");

    assert!(
        target_gate < execsurface,
        "target correctness gate must run before ExecSurface"
    );
    assert!(
        workflow[target_gate..execsurface].contains("run: >-\n            cargo test --locked"),
        "target correctness gate must execute the exact requested command"
    );
    assert!(
        workflow.contains("command: >-\n            cargo test --locked"),
        "ExecSurface step must observe the same requested command"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn init_prints_actionable_custody_setup_after_baseline_learning() {
    let dir = temp_dir("custody-guidance");
    fs::create_dir_all(&dir).expect("dir");

    let output = Command::new(binary())
        .args([
            "init",
            "--command",
            "cargo test --locked",
            "--github-actions",
        ])
        .current_dir(&dir)
        .output()
        .expect("init");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("gh variable set EXECSURFACE_BASELINE_DIGEST"),
        "init must provide a copyable GitHub variable command for the learned baseline digest"
    );
    assert!(
        stdout.contains("gh variable set EXECSURFACE_POLICY_SHA256"),
        "init must provide a copyable GitHub variable command for the policy byte digest"
    );
    assert!(
        stdout.contains("sha256sum execsurface-policy.json"),
        "init must show how to compute the exact-byte policy SHA-256"
    );

    let _ = fs::remove_dir_all(dir);
}
