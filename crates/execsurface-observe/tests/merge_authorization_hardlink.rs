#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};

use execsurface_observe::{observe_command, CommandSpec};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn temp_dir(name: &str) -> std::path::PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "execsurface-hardlink-boundary-{name}-{}-{id}",
        process::id()
    ));
    fs::create_dir_all(&path).expect("create hardlink-boundary temp dir");
    path
}

#[test]
fn successful_hardlink_creation_marks_raw_v2_incomplete() {
    let dir = temp_dir("success");
    let source = dir.join("source.bin");
    let alias = dir.join("alias.bin");
    let target = format!(
        "printf 'x' > '{}' && ln '{}' '{}'",
        source.display(),
        source.display(),
        alias.display()
    );

    let observation = observe_command(
        &CommandSpec::new("/bin/sh")
            .arg("-c")
            .arg(target),
    )
    .expect("observe successful hardlink creation");

    assert_eq!(fs::read(&alias).expect("alias bytes"), b"x");
    assert!(
        !observation.complete,
        "raw-v2 must not remain authoritative after a successful hardlink alias transition"
    );
    assert!(
        observation
            .warnings
            .iter()
            .any(|warning| warning.code == "hardlink_namespace_transition_unmodeled"),
        "successful hardlink creation must carry an explicit completeness warning"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn failed_hardlink_attempt_does_not_claim_a_successful_alias_transition() {
    let dir = temp_dir("failed");
    let source = dir.join("source.bin");
    let alias = dir.join("alias.bin");
    fs::write(&source, b"source").expect("seed source");
    fs::write(&alias, b"existing").expect("seed destination");

    let target = format!("ln '{}' '{}'", source.display(), alias.display());
    let observation = observe_command(
        &CommandSpec::new("/bin/sh")
            .arg("-c")
            .arg(target),
    )
    .expect("observe failed hardlink attempt");

    assert!(
        !observation
            .warnings
            .iter()
            .any(|warning| warning.code == "hardlink_namespace_transition_unmodeled"),
        "failed link/linkat must not be mislabeled as a successful namespace transition"
    );

    let _ = fs::remove_dir_all(dir);
}
