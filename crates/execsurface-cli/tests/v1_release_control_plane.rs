#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("cannot read {path}: {e}"))
}

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "execsurface-v1-r1-{name}-{}-{nonce}",
        std::process::id()
    ))
}

fn run_validator(request: &str, package_version: &str, release_tag: &str) -> std::process::Output {
    let dir = temp_dir("validator");
    fs::create_dir_all(&dir).expect("temp dir");
    let request_path = dir.join("request.json");
    fs::write(&request_path, request).expect("request");

    let output = Command::new("python3")
        .arg(root().join(".github/scripts/release_contract.py"))
        .args([
            "validate-request",
            "--request",
            request_path.to_str().expect("request path"),
            "--package-version",
            package_version,
            "--release-tag-file",
            release_tag,
        ])
        .output()
        .expect("run release contract validator");

    let _ = fs::remove_dir_all(dir);
    output
}

fn run_ruleset_verifier(ruleset_jsonl: &str, tag: &str) -> std::process::Output {
    let dir = temp_dir("ruleset");
    fs::create_dir_all(&dir).expect("temp dir");
    let rulesets_path = dir.join("rulesets.jsonl");
    fs::write(&rulesets_path, ruleset_jsonl).expect("rulesets");

    let output = Command::new("python3")
        .arg(root().join(".github/scripts/verify_tag_ruleset.py"))
        .args([
            "--rulesets-jsonl",
            rulesets_path.to_str().expect("rulesets path"),
            "--tag",
            tag,
        ])
        .output()
        .expect("run tag-ruleset verifier");

    let _ = fs::remove_dir_all(dir);
    output
}

#[test]
fn alpha6_and_stable_v1_release_contracts_are_classified_explicitly() {
    let alpha = run_validator(
        r#"{"version":"0.1.0-alpha.6","tag":"v0.1.0-alpha.6","stable_channel":"v0.1","request_revision":1}"#,
        "0.1.0-alpha.6",
        "v0.1.0-alpha.6",
    );
    assert!(
        alpha.status.success(),
        "current Alpha.6 contract must remain valid: {}",
        String::from_utf8_lossy(&alpha.stderr)
    );
    let alpha_stdout = String::from_utf8_lossy(&alpha.stdout);
    assert!(alpha_stdout.contains(r#""release_kind": "prerelease""#));
    assert!(alpha_stdout.contains(r#""stable_channel": "v0.1""#));

    let stable = run_validator(
        r#"{"version":"1.0.0","tag":"v1.0.0","stable_channel":"v1","request_revision":1}"#,
        "1.0.0",
        "v1.0.0",
    );
    assert!(
        stable.status.success(),
        "stable v1 contract must be accepted: {}",
        String::from_utf8_lossy(&stable.stderr)
    );
    let stable_stdout = String::from_utf8_lossy(&stable.stdout);
    assert!(stable_stdout.contains(r#""release_kind": "stable""#));
    assert!(stable_stdout.contains(r#""stable_channel": "v1""#));
}

#[test]
fn unsupported_or_mismatched_release_contracts_fail_closed() {
    let wrong_channel = run_validator(
        r#"{"version":"1.0.0","tag":"v1.0.0","stable_channel":"v0.1","request_revision":1}"#,
        "1.0.0",
        "v1.0.0",
    );
    assert!(!wrong_channel.status.success());

    let rc = run_validator(
        r#"{"version":"1.0.0-rc.1","tag":"v1.0.0-rc.1","stable_channel":"v1","request_revision":1}"#,
        "1.0.0-rc.1",
        "v1.0.0-rc.1",
    );
    assert!(!rc.status.success());

    let mismatch = run_validator(
        r#"{"version":"1.0.0","tag":"v1.0.1","stable_channel":"v1","request_revision":1}"#,
        "1.0.0",
        "v1.0.1",
    );
    assert!(!mismatch.status.success());

    let v2 = run_validator(
        r#"{"version":"2.0.0","tag":"v2.0.0","stable_channel":"v2","request_revision":1}"#,
        "2.0.0",
        "v2.0.0",
    );
    assert!(!v2.status.success());
}

#[test]
fn workflows_use_release_classifier_and_do_not_hardcode_alpha_stable_channel() {
    let promote = read(".github/workflows/promote-release.yml");
    let release = read(".github/workflows/release.yml");

    assert!(
        promote.contains(".github/scripts/release_contract.py"),
        "promotion must use the testable release classifier"
    );
    assert!(
        promote.contains(".github/scripts/verify_tag_ruleset.py"),
        "stable-v1 promotion must verify immutable tag governance before tag creation"
    );
    assert!(
        !promote.contains(r#"test "$stable_channel" = "v0.1""#),
        "promotion must not hard-code v0.1 as the only accepted stable channel"
    );

    assert!(
        release.contains("stable-channel:")
            && release.contains("release-kind:")
            && release.contains("is-prerelease:"),
        "release job must export classified channel and release type"
    );
    assert!(
        release.contains("STABLE_CHANNEL:") && release.contains("IS_PRERELEASE:"),
        "release publication/promotion must consume classified release metadata"
    );
}

#[test]
fn stable_publication_and_channel_promotion_are_conditional_and_generic() {
    let release = read(".github/workflows/release.yml");

    assert!(
        release.contains(r#"if [[ "$IS_PRERELEASE" == "true" ]]"#),
        "GitHub release publication must branch between prerelease and stable metadata"
    );
    assert!(
        release.contains(r#"git tag -fa "$STABLE_CHANNEL""#),
        "stable channel promotion must move the classified channel"
    );
    assert!(
        !release.contains("Promote stable v0.1 channel"),
        "promotion job name must no longer encode only the Alpha channel"
    );
    assert!(
        !release.contains("uses: AETHERXGLOBAL/execsurface@v0.1"),
        "stable consumer proof must not be hard-coded to @v0.1"
    );
    assert!(
        release.contains(r#"ref: ${{ needs.release.outputs.stable-channel }}"#),
        "stable consumer proof must checkout the classified moving channel"
    );
}

#[test]
fn v1_r1_does_not_arm_a_real_v1_release_request() {
    let request = read(".release/release-request.json");
    assert!(
        request.contains(r#""version": "0.1.0-alpha.6""#),
        "V1-R1 must not change the active release request to v1"
    );
    assert!(
        !request.contains(r#""tag": "v1.0.0""#),
        "V1-R1 must not arm an actual v1 release"
    );
}

#[test]
fn immutable_v1_tag_governance_is_fail_closed_and_no_bypass() {
    let valid = r#"{"id":1,"name":"Protect v1 immutable releases","target":"tag","enforcement":"active","conditions":{"ref_name":{"exclude":[],"include":["refs/tags/v1.*"]}},"rules":[{"type":"deletion"},{"type":"update"}],"bypass_actors":[],"current_user_can_bypass":"never"}"#;
    let output = run_ruleset_verifier(valid, "v1.0.0");
    assert!(
        output.status.success(),
        "valid immutable v1 tag ruleset must qualify: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let alpha_only = r#"{"id":2,"name":"Alpha only","target":"tag","enforcement":"active","conditions":{"ref_name":{"exclude":[],"include":["refs/tags/v0.1.0-alpha.5"]}},"rules":[{"type":"deletion"},{"type":"update"}],"bypass_actors":[],"current_user_can_bypass":"never"}"#;
    assert!(!run_ruleset_verifier(alpha_only, "v1.0.0").status.success());

    let bypass = r#"{"id":3,"name":"v1 with bypass","target":"tag","enforcement":"active","conditions":{"ref_name":{"exclude":[],"include":["refs/tags/v1.*"]}},"rules":[{"type":"deletion"},{"type":"update"}],"bypass_actors":[{"actor_id":1}],"current_user_can_bypass":"always"}"#;
    assert!(!run_ruleset_verifier(bypass, "v1.0.0").status.success());

    let missing_update = r#"{"id":4,"name":"v1 deletion only","target":"tag","enforcement":"active","conditions":{"ref_name":{"exclude":[],"include":["refs/tags/v1.*"]}},"rules":[{"type":"deletion"}],"bypass_actors":[],"current_user_can_bypass":"never"}"#;
    assert!(!run_ruleset_verifier(missing_update, "v1.0.0")
        .status
        .success());
}

#[test]
fn published_action_installer_accepts_bounded_alpha_and_final_v1_tags() {
    let install = read("action/install.sh");

    assert!(
        install.contains(r#"^v0\.1\.0-alpha\."#),
        "published Action installer must retain the qualified Alpha tag line"
    );
    assert!(
        install.contains(r#"^v1\.[0-9]+\.[0-9]+$"#),
        "published Action installer must accept final stable v1 tags"
    );
    assert!(
        install.contains("invalid pinned release tag"),
        "unsupported release-tag forms must remain fail-closed"
    );
}
