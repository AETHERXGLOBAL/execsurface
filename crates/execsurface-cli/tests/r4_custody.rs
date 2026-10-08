#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

const TRUSTED_POLICY: &str =
    "{\n  \"schema_version\": 2,\n  \"default_action\": \"review\",\n  \"rules\": []\n}\n";
const TRUSTED_POLICY_SHA256: &str =
    "sha256:36a0ca08546dbfef6466f66a8636890b50f90d0d7fc4ee6c4e64aa1efea4c80d";
const SUBSTITUTE_POLICY: &str =
    "{\n  \"schema_version\": 2,\n  \"default_action\": \"allow\",\n  \"rules\": []\n}\n";

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn temp_dir(name: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("execsurface-r4-{name}-{}-{id}", std::process::id()));
    fs::create_dir_all(&path).expect("create r4 temp dir");
    path
}

fn learn(dir: &Path, baseline: &Path, label: &str, command: &[&str]) {
    let mut cmd = Command::new(cli());
    cmd.current_dir(dir)
        .args(["learn", "--output"])
        .arg(baseline)
        .args(["--label", label, "--"]);
    cmd.args(command);
    let output = cmd.output().expect("learn baseline");
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

#[test]
fn r4_self_consistent_baseline_substitution_is_rejected_before_target() {
    let dir = temp_dir("baseline-substitution");
    let trusted = dir.join("trusted.lock.json");
    let substitute = dir.join("substitute.lock.json");
    let marker = dir.join("TARGET_RAN");

    learn(&dir, &trusted, "trusted", &["/bin/true"]);
    learn(&dir, &substitute, "substitute", &["/bin/true"]);
    let expected = baseline_digest(&trusted);
    assert_ne!(
        expected,
        baseline_digest(&substitute),
        "fixture requires two self-consistent but differently identified baselines"
    );

    let target = format!("touch {}", marker.display());
    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&substitute)
        .args([
            "--expect-baseline-digest",
            &expected,
            "--",
            "/bin/sh",
            "-c",
            &target,
        ])
        .output()
        .expect("check substituted baseline");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "baseline custody mismatch must be rejected before target execution"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("baseline custody mismatch"),
        "failure must identify the custody invariant rather than pass for an unrelated parse error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_parseable_policy_substitution_is_rejected_before_target() {
    let dir = temp_dir("policy-substitution");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("execsurface-policy.json");
    let marker = dir.join("TARGET_RAN");

    learn(
        &dir,
        &baseline,
        "policy-custody",
        &["/bin/sh", "-c", "true"],
    );
    fs::write(&policy, SUBSTITUTE_POLICY).expect("write substitute policy");

    let target = format!("touch {}", marker.display());
    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--policy"])
        .arg(&policy)
        .args([
            "--expect-policy-sha256",
            TRUSTED_POLICY_SHA256,
            "--",
            "/bin/sh",
            "-c",
            &target,
        ])
        .output()
        .expect("check substituted policy");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "policy custody mismatch must be rejected before target execution"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("policy custody mismatch"),
        "failure must identify policy custody rather than an unrelated error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_malformed_expected_digest_fails_closed_before_target() {
    let dir = temp_dir("malformed-pin");
    let baseline = dir.join("baseline.lock.json");
    let marker = dir.join("TARGET_RAN");

    learn(&dir, &baseline, "malformed-pin", &["/bin/true"]);

    let target = format!("touch {}", marker.display());
    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args([
            "--expect-baseline-digest",
            "sha256:not-a-real-digest",
            "--",
            "/bin/sh",
            "-c",
            &target,
        ])
        .output()
        .expect("check malformed pin");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "malformed pin must fail before target execution"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("invalid expected baseline digest"),
        "malformed pin must fail for the frozen R4 reason: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_correct_baseline_and_policy_pins_preserve_pass() {
    let dir = temp_dir("correct-pins");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("execsurface-policy.json");

    learn(&dir, &baseline, "correct-pins", &["/bin/true"]);
    fs::write(&policy, TRUSTED_POLICY).expect("write trusted policy");
    let expected_baseline = baseline_digest(&baseline);

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--policy"])
        .arg(&policy)
        .args([
            "--expect-baseline-digest",
            &expected_baseline,
            "--expect-policy-sha256",
            TRUSTED_POLICY_SHA256,
            "--",
            "/bin/true",
        ])
        .output()
        .expect("check correct pins");

    assert!(
        output.status.success(),
        "correct external custody pins must preserve ordinary PASS: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_legacy_unpinned_check_remains_compatible_without_new_custody_claim() {
    let dir = temp_dir("legacy-unpinned");
    let baseline = dir.join("baseline.lock.json");

    learn(&dir, &baseline, "legacy-unpinned", &["/bin/true"]);

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--", "/bin/true"])
        .output()
        .expect("legacy unpinned check");

    assert!(
        output.status.success(),
        "R4 must not silently break the existing unpinned pre-v1 CLI path"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_duplicate_baseline_trust_pin_is_rejected_before_target() {
    let dir = temp_dir("duplicate-baseline-pin");
    let baseline = dir.join("baseline.lock.json");
    let marker = dir.join("TARGET_RAN");

    learn(
        &dir,
        &baseline,
        "duplicate-baseline-pin",
        &["/bin/sh", "-c", "true"],
    );
    let expected = baseline_digest(&baseline);
    let wrong = format!("sha256:{}", "0".repeat(64));
    let target = format!("touch {}", marker.display());

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--expect-baseline-digest", &wrong])
        .args(["--expect-baseline-digest", &expected])
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("check duplicate baseline trust pin");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "ambiguous duplicate baseline trust assertions must fail before target execution"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("duplicate --expect-baseline-digest"),
        "duplicate trust assertion must be rejected explicitly: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_duplicate_policy_trust_pin_is_rejected_before_target() {
    let dir = temp_dir("duplicate-policy-pin");
    let baseline = dir.join("baseline.lock.json");
    let policy = dir.join("execsurface-policy.json");
    let marker = dir.join("TARGET_RAN");

    learn(
        &dir,
        &baseline,
        "duplicate-policy-pin",
        &["/bin/sh", "-c", "true"],
    );
    fs::write(&policy, TRUSTED_POLICY).expect("write trusted policy");
    let expected_baseline = baseline_digest(&baseline);
    let wrong = format!("sha256:{}", "0".repeat(64));
    let target = format!("touch {}", marker.display());

    let output = Command::new(cli())
        .current_dir(&dir)
        .args(["check", "--baseline"])
        .arg(&baseline)
        .args(["--policy"])
        .arg(&policy)
        .args(["--expect-baseline-digest", &expected_baseline])
        .args(["--expect-policy-sha256", &wrong])
        .args(["--expect-policy-sha256", TRUSTED_POLICY_SHA256])
        .args(["--", "/bin/sh", "-c", &target])
        .output()
        .expect("check duplicate policy trust pin");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        !marker.exists(),
        "ambiguous duplicate policy trust assertions must fail before target execution"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("duplicate --expect-policy-sha256"),
        "duplicate trust assertion must be rejected explicitly: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}
