#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use execsurface_model::{canonical::CanonicalEffect, FileOperation};
use execsurface_normalize::{canonicalize, NormalizationConfig, NormalizeError};
use execsurface_observe::{
    observe_command, observe_command_with_options, CommandSpec, ObserveOptions,
};

#[test]
fn repeated_real_observation_produces_identical_canonical_surface() {
    let first = observe_command(&CommandSpec::new("/bin/true")).expect("first observation");
    let second = observe_command(&CommandSpec::new("/bin/true")).expect("second observation");

    assert!(first.complete, "first observation must be complete");
    assert!(second.complete, "second observation must be complete");

    let first_surface =
        canonicalize(&first, &NormalizationConfig::default()).expect("first canonicalization");
    let second_surface =
        canonicalize(&second, &NormalizationConfig::default()).expect("second canonicalization");

    assert_eq!(first_surface, second_surface);

    let first_json = serde_json::to_string(&first_surface).expect("serialize first surface");
    let second_json = serde_json::to_string(&second_surface).expect("serialize second surface");
    assert_eq!(first_json, second_json);
}


#[test]
fn observer_truncation_is_rejected_end_to_end() {
    let raw = observe_command_with_options(
        &CommandSpec::new("/bin/true"),
        ObserveOptions { event_limit: 0 },
    )
    .expect("observation");

    assert!(!raw.complete);
    assert!(raw
        .warnings
        .iter()
        .any(|warning| warning.code == "event_limit_exceeded"));
    assert_eq!(
        canonicalize(&raw, &NormalizationConfig::default()),
        Err(NormalizeError::IncompleteObservation)
    );
}

#[test]
fn real_shell_to_cat_chain_reaches_fd_read_effect() {
    let path = std::env::temp_dir().join(format!(
        "execsurface-m65-chain-{}",
        std::process::id()
    ));
    std::fs::write(&path, b"fixture").expect("fixture");

    let command = format!("cat '{}' >/dev/null", path.display());
    let raw = observe_command(
        &CommandSpec::new("/bin/sh")
            .arg("-c")
            .arg(command),
    )
    .expect("observation");
    assert!(raw.complete, "{:#?}", raw.warnings);

    let surface = canonicalize(
        &raw,
        &NormalizationConfig {
            tmp_roots: vec!["/tmp".to_owned()],
            ..NormalizationConfig::default()
        },
    )
    .expect("canonicalization");

    let expected = format!(
        "$TMP/{}",
        path.file_name().expect("name").to_string_lossy()
    );

    let effect = surface
        .effects
        .iter()
        .find(|effect| {
            matches!(
                effect,
                CanonicalEffect::FilePathAccess {
                    operation: FileOperation::Read,
                    target,
                    ..
                } if target.value == expected
            )
        })
        .expect("real fd read");

    match effect {
        CanonicalEffect::FilePathAccess {
            actor,
            execution_chain,
            ..
        } => {
            assert_eq!(actor.as_ref().map(|exec| exec.family.as_str()), Some("cat"));
            let families = execution_chain
                .iter()
                .map(|exec| exec.family.as_str())
                .collect::<Vec<_>>();
            assert!(families.starts_with(&["sh"]));
            assert_eq!(families.last().copied(), Some("cat"));
        }
        _ => unreachable!(),
    }

    let _ = std::fs::remove_file(path);
}
