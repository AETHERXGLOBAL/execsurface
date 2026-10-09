#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::Path;
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

fn temp_dir(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "execsurface-v1-rc1-{name}-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn v1_source_identity_requires_the_published_stable_release_state() {
    let cargo = read("Cargo.toml");
    let action_tag = read("action/release-tag.txt");
    let release_request = read(".release/release-request.json");
    let decision = read("docs/release/V1_0_0_RELEASE_DECISION.md");
    let notes = read("docs/releases/v1.0.0.md");
    let readme = read("README.md");
    let status = read("docs/STATUS.md");

    assert!(
        cargo.contains(r#"version = "1.0.0""#),
        "published v1 source version must remain 1.0.0"
    );
    assert_eq!(
        action_tag.trim(),
        "v1.0.0",
        "published Action source must retain the immutable v1.0.0 identity"
    );
    assert!(
        release_request.contains(r#""version": "1.0.0""#)
            && release_request.contains(r#""tag": "v1.0.0""#)
            && release_request.contains(r#""stable_channel": "v1""#)
            && release_request.contains(r#""request_revision": 2"#),
        "published state must retain the exact authorized v1.0.0 release request"
    );
    assert!(
        decision.contains("RELEASE_V1_0_AUTHORIZED_BOUNDED"),
        "published v1 state must retain explicit bounded release authorization"
    );
    assert!(
        notes.contains("PUBLISHED STABLE RELEASE — INTERNALLY QUALIFIED BOUNDED"),
        "release record must advance from publication-pending to the published stable state"
    );
    assert!(
        readme.contains("Current stable release") && readme.contains("v1.0.0"),
        "README must identify v1.0.0 as the current stable release after publication"
    );
    assert!(
        status.contains("Public release: `v1.0.0`"),
        "authoritative status must identify v1.0.0 as public after publication"
    );
}
#[test]
fn rc1_init_generates_future_stable_v1_action_channel() {
    let dir = temp_dir("init-v1-channel");
    fs::create_dir_all(&dir).expect("create temp dir");

    let output = Command::new(env!("CARGO_BIN_EXE_execsurface"))
        .args([
            "init",
            "--command",
            "cargo test --locked",
            "--github-actions",
        ])
        .current_dir(&dir)
        .output()
        .expect("run init");

    assert!(
        output.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let workflow =
        fs::read_to_string(dir.join(".github/workflows/execsurface.yml")).expect("workflow");
    assert!(
        workflow.contains("uses: AETHERXGLOBAL/execsurface@v1"),
        "v1 candidate must generate the stable @v1 Action channel"
    );
    assert!(
        !workflow.contains("uses: AETHERXGLOBAL/execsurface@v0.1"),
        "v1 candidate must not generate the Alpha moving channel"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn published_v1_documents_preserve_alpha6_as_historical_rollback_evidence() {
    let readme = read("README.md");
    let status = read("docs/STATUS.md");

    assert!(
        readme.contains("Current stable release") && readme.contains("v1.0.0"),
        "published v1 README must identify the stable release"
    );
    assert!(
        status.contains("Public release: `v1.0.0`"),
        "published v1 status must identify the stable release"
    );
    assert!(
        status.contains("Previous public Alpha `v0.1.0-alpha.6` remains immutable"),
        "Alpha.6 must remain explicit historical/rollback evidence"
    );
}
#[test]
fn rc1_registry_publish_chain_is_complete_and_dependency_ordered() {
    let workflow = read(".github/workflows/publish-crates.yml");

    let expected = [
        "execsurface-model",
        "execsurface-observe",
        "execsurface-normalize",
        "execsurface-baseline",
        "execsurface-diff",
        "execsurface-policy",
        "execsurface-report",
        "execsurface",
    ];

    let publish_lines: Vec<&str> = workflow
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("publish_one "))
        .collect();

    let expected_lines: Vec<String> = expected
        .iter()
        .map(|crate_name| format!("publish_one {crate_name}"))
        .collect();

    assert_eq!(
        publish_lines,
        expected_lines
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "publish chain must cover every publishable crate exactly once in dependency order"
    );

    assert!(
        workflow.contains("cargo publish -p execsurface-model --locked --dry-run"),
        "first publishable crate must retain a real registry dry-run before publication"
    );
    assert!(
        workflow.contains("cargo install execsurface --version \"=$version\" --locked"),
        "publish chain must end with a zero-contact exact-version registry install"
    );
}
