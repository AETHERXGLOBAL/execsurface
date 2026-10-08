#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|error| {
        panic!("cannot read {path}: {error}");
    })
}

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "execsurface-v1-rc1-round-b-{name}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn release_contract(args: &[&str]) -> Output {
    Command::new("python3")
        .arg(root().join(".github/scripts/release_contract.py"))
        .args(args)
        .current_dir(root())
        .output()
        .expect("run release contract")
}

#[test]
fn release_classifier_rejects_mismatched_and_prerelease_v1_identities() {
    let mismatched = release_contract(&[
        "classify",
        "--version",
        "1.0.0",
        "--tag",
        "v1.0.1",
    ]);
    assert_eq!(mismatched.status.code(), Some(2));

    let prerelease = release_contract(&[
        "classify",
        "--version",
        "1.0.0-rc.1",
        "--tag",
        "v1.0.0-rc.1",
    ]);
    assert_eq!(prerelease.status.code(), Some(2));

    let stable = release_contract(&[
        "classify",
        "--version",
        "1.0.0",
        "--tag",
        "v1.0.0",
    ]);
    assert!(stable.status.success());
    let stdout = String::from_utf8_lossy(&stable.stdout);
    assert!(stdout.contains(r#""stable_channel": "v1""#));
    assert!(stdout.contains(r#""is_prerelease": false"#));
}

#[test]
fn release_request_validation_rejects_wrong_channel_and_stale_action_tag() {
    let dir = temp_dir("release-request");
    let request = dir.join("request.json");

    fs::write(
        &request,
        r#"{
  "version": "1.0.0",
  "tag": "v1.0.0",
  "stable_channel": "v0.1",
  "request_revision": 1
}
"#,
    )
    .expect("write wrong-channel request");

    let request_arg = request.to_string_lossy().into_owned();
    let wrong_channel = release_contract(&[
        "validate-request",
        "--request",
        &request_arg,
        "--package-version",
        "1.0.0",
        "--release-tag-file",
        "v1.0.0",
    ]);
    assert_eq!(wrong_channel.status.code(), Some(2));

    fs::write(
        &request,
        r#"{
  "version": "1.0.0",
  "tag": "v1.0.0",
  "stable_channel": "v1",
  "request_revision": 1
}
"#,
    )
    .expect("write stable request");

    let stale_tag = release_contract(&[
        "validate-request",
        "--request",
        &request_arg,
        "--package-version",
        "1.0.0",
        "--release-tag-file",
        "v0.1.0-alpha.6",
    ]);
    assert_eq!(stale_tag.status.code(), Some(2));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn rc_branch_cannot_accidentally_claim_public_v1_or_external_validation() {
    let request = read(".release/release-request.json");
    let readme = read("README.md");
    let status = read("docs/STATUS.md");

    assert!(
        request.contains(r#""version": "0.1.0-alpha.6""#)
            && request.contains(r#""tag": "v0.1.0-alpha.6""#)
            && request.contains(r#""stable_channel": "v0.1""#),
        "internal RC branch must not arm the real v1 release request"
    );

    for doc in [&readme, &status] {
        assert!(
            doc.contains("v0.1.0-alpha.6"),
            "current public documents must still identify Alpha.6 as published"
        );
        for forbidden in [
            "independently validated",
            "externally certified",
            "proven in production",
            "industry validated",
            "universally production-ready",
        ] {
            assert!(
                !doc.to_ascii_lowercase().contains(forbidden),
                "current public document makes forbidden maturity claim: {forbidden}"
            );
        }
    }
}

#[test]
fn release_workflows_enforce_governance_and_consumer_ordering() {
    let promote = read(".github/workflows/promote-release.yml");
    let release = read(".github/workflows/release.yml");

    let governance = promote
        .find("- name: Verify immutable stable-v1 tag governance")
        .expect("stable-v1 governance guard");
    let tag_creation = promote
        .find("- name: Create immutable version tag")
        .expect("immutable tag creation");
    assert!(
        governance < tag_creation,
        "stable-v1 tag governance must execute before immutable tag creation"
    );

    let immutable_consumer = release
        .find("immutable-action-consumer:")
        .expect("immutable Action consumer");
    let promote_stable = release
        .find("promote-stable:")
        .expect("stable channel promotion");
    let stable_consumer = release
        .find("stable-action-consumer:")
        .expect("stable Action consumer");
    let publish = release
        .find("publish-crates:")
        .expect("registry publication");

    assert!(
        immutable_consumer < promote_stable,
        "immutable Action proof must be defined before moving-channel promotion"
    );
    assert!(
        promote_stable < stable_consumer && stable_consumer < publish,
        "registry publication must remain downstream of moving-channel consumer proof"
    );

    assert!(
        release.contains("needs: [release, binary-consumer, cargo-tag-consumer, immutable-action-consumer]"),
        "stable-channel promotion must depend on immutable artifact/consumer proof"
    );
    assert!(
        release.contains("needs: [release, stable-action-consumer]"),
        "registry publication must depend on stable-channel Action proof"
    );
}

#[test]
fn environment_floor_negative_evidence_remains_in_stable_contract() {
    let compatibility = read("docs/COMPATIBILITY.md");
    assert!(compatibility.contains("PASS on Ubuntu 24.04 x86_64"));
    assert!(compatibility.contains("FAIL on Ubuntu 22.04"));
    assert!(compatibility.contains("GLIBC_2.39"));
    assert!(compatibility.contains("exact-version local build/install"));
}
