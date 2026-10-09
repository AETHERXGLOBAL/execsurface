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

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temp_dir(name: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "execsurface-r4-action-{name}-{}-{id}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn learn_action_baseline(dir: &Path, baseline: &Path) {
    let output = Command::new(cli())
        .current_dir(dir)
        .args(["learn", "--output"])
        .arg(baseline)
        .args(["--", "/bin/bash", "-lc", "true"])
        .output()
        .expect("learn action baseline");
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

fn run_action(
    dir: &Path,
    command: &str,
    baseline: &Path,
    policy: &Path,
    require_custody: &str,
    expected_baseline: Option<&str>,
    expected_policy: Option<&str>,
) -> (std::process::Output, String) {
    let output_file = dir.join("github-output.txt");
    // Keep child Action summaries in the test fixture, not the parent CI job summary.
    let step_summary_file = dir.join("github-step-summary.md");
    let runner_temp = dir.join("runner-temp");
    fs::create_dir_all(&runner_temp).expect("runner temp");

    let mut cmd = Command::new("/bin/bash");
    cmd.arg(repo_root().join("action/run.sh"))
        .current_dir(dir)
        .env("EXECSURFACE_ACTION_BIN", cli())
        .env("EXECSURFACE_COMMAND", command)
        .env("EXECSURFACE_BASELINE", baseline)
        .env("EXECSURFACE_POLICY", policy)
        .env("EXECSURFACE_REQUIRE_CUSTODY", require_custody)
        .env("GITHUB_OUTPUT", &output_file)
        .env("GITHUB_STEP_SUMMARY", &step_summary_file)
        .env("RUNNER_TEMP", &runner_temp);

    if let Some(value) = expected_baseline {
        cmd.env("EXECSURFACE_EXPECTED_BASELINE_DIGEST", value);
    }
    if let Some(value) = expected_policy {
        cmd.env("EXECSURFACE_EXPECTED_POLICY_SHA256", value);
    }

    let output = cmd.output().expect("run action script");
    let github_output = fs::read_to_string(&output_file).expect("github output");
    let step_summary = fs::read_to_string(&step_summary_file).expect("isolated action summary");
    assert!(
        step_summary.starts_with("# ExecSurface — "),
        "Action summary should be emitted only to the test-local summary path"
    );
    (output, github_output)
}

#[test]
fn r4_action_missing_both_pins_fails_closed_before_target() {
    let dir = temp_dir("missing-pins");
    let marker = dir.join("TARGET_RAN");
    let command = format!("touch {}", marker.display());

    let (output, github_output) = run_action(
        &dir,
        &command,
        &dir.join("missing.lock.json"),
        &dir.join("missing-policy.json"),
        "true",
        None,
        None,
    );

    assert!(
        output.status.success(),
        "runner script should emit an ERROR result"
    );
    assert!(!marker.exists(), "missing pins must block target execution");
    assert!(github_output.contains("verdict=error"));
    assert!(github_output.contains("exit-code=2"));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("require-custody=true requires both"),
        "preflight reason must remain explicit"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_action_partial_pin_fails_closed_before_target() {
    let dir = temp_dir("partial-pin");
    let marker = dir.join("TARGET_RAN");
    let command = format!("touch {}", marker.display());
    let expected_baseline = format!("sha256:{}", "a".repeat(64));

    let (output, github_output) = run_action(
        &dir,
        &command,
        &dir.join("missing.lock.json"),
        &dir.join("missing-policy.json"),
        "true",
        Some(&expected_baseline),
        None,
    );

    assert!(output.status.success());
    assert!(
        !marker.exists(),
        "partial custody must block target execution"
    );
    assert!(github_output.contains("verdict=error"));
    assert!(github_output.contains("exit-code=2"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_action_invalid_require_custody_value_fails_closed() {
    let dir = temp_dir("invalid-mode");
    let marker = dir.join("TARGET_RAN");
    let command = format!("touch {}", marker.display());

    let (output, github_output) = run_action(
        &dir,
        &command,
        &dir.join("missing.lock.json"),
        &dir.join("missing-policy.json"),
        "definitely",
        None,
        None,
    );

    assert!(output.status.success());
    assert!(
        !marker.exists(),
        "invalid custody mode must not execute target"
    );
    assert!(github_output.contains("verdict=error"));
    assert!(github_output.contains("exit-code=2"));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("require-custody must be true or false")
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_action_correct_external_pins_preserve_pass() {
    let dir = temp_dir("correct-pins");
    let baseline = dir.join("execsurface.lock.json");
    let policy = dir.join("execsurface-policy.json");

    learn_action_baseline(&dir, &baseline);
    fs::write(&policy, TRUSTED_POLICY).expect("write policy");
    let expected_baseline = baseline_digest(&baseline);

    let (output, github_output) = run_action(
        &dir,
        "true",
        &baseline,
        &policy,
        "true",
        Some(&expected_baseline),
        Some(TRUSTED_POLICY_SHA256),
    );

    assert!(
        output.status.success(),
        "correct pins should complete runner: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(github_output.contains("verdict=pass"));
    assert!(github_output.contains("exit-code=0"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_action_wrong_well_formed_baseline_pin_blocks_before_target() {
    let dir = temp_dir("wrong-baseline-pin");
    let baseline = dir.join("execsurface.lock.json");
    let policy = dir.join("execsurface-policy.json");
    let marker = dir.join("TARGET_RAN");

    learn_action_baseline(&dir, &baseline);
    fs::write(&policy, TRUSTED_POLICY).expect("write policy");
    let wrong_baseline = format!("sha256:{}", "0".repeat(64));
    let command = format!("touch {}", marker.display());

    let (output, github_output) = run_action(
        &dir,
        &command,
        &baseline,
        &policy,
        "true",
        Some(&wrong_baseline),
        Some(TRUSTED_POLICY_SHA256),
    );

    assert!(output.status.success());
    assert!(
        !marker.exists(),
        "well-formed but unauthorized baseline identity must block target execution"
    );
    assert!(github_output.contains("verdict=error"));
    assert!(github_output.contains("exit-code=2"));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("baseline custody mismatch"),
        "Action must expose baseline custody rejection: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn r4_action_wrong_well_formed_policy_pin_blocks_before_target() {
    let dir = temp_dir("wrong-policy-pin");
    let baseline = dir.join("execsurface.lock.json");
    let policy = dir.join("execsurface-policy.json");
    let marker = dir.join("TARGET_RAN");

    learn_action_baseline(&dir, &baseline);
    fs::write(&policy, TRUSTED_POLICY).expect("write policy");
    let expected_baseline = baseline_digest(&baseline);
    let wrong_policy = format!("sha256:{}", "0".repeat(64));
    let command = format!("touch {}", marker.display());

    let (output, github_output) = run_action(
        &dir,
        &command,
        &baseline,
        &policy,
        "true",
        Some(&expected_baseline),
        Some(&wrong_policy),
    );

    assert!(output.status.success());
    assert!(
        !marker.exists(),
        "well-formed but unauthorized policy identity must block target execution"
    );
    assert!(github_output.contains("verdict=error"));
    assert!(github_output.contains("exit-code=2"));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("policy custody mismatch"),
        "Action must expose policy custody rejection: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(dir);
}
