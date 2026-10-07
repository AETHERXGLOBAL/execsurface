use std::fs;
use std::os::unix::fs::symlink;
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};

fn fd_write_to_path(observation: &execsurface_model::Observation, expected: &str) -> bool {
    observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Write,
                path,
                ..
            } if path == expected
        )
    })
}

fn any_fd_write_to_other_path(
    observation: &execsurface_model::Observation,
    excluded: &str,
) -> bool {
    observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Write,
                path,
                ..
            } if path != excluded
        )
    })
}

#[test]
fn r1_1_dup2_from_untracked_source_never_keeps_stale_destination_identity() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let victim = std::env::temp_dir().join(format!(
        "execsurface-stage2-r1-stale-fd-{}-victim.txt",
        process::id()
    ));
    fs::write(&victim, b"").expect("create victim");
    let expected_victim = fs::canonicalize(&victim)
        .expect("canonicalize victim")
        .to_string_lossy()
        .into_owned();

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("stage2-dup2-untracked-write")
            .arg(victim.as_os_str()),
    )
    .expect("observe stale-fd falsification fixture");

    let victim_len = fs::metadata(&victim).expect("victim metadata").len();
    let _ = fs::remove_file(&victim);

    assert_eq!(
        victim_len, 0,
        "kernel write must go to the pipe after successful dup2, not to victim.txt"
    );

    let stale_attribution = fd_write_to_path(&observation, &expected_victim);
    let replacement_attribution =
        any_fd_write_to_other_path(&observation, &expected_victim);

    assert!(
        !stale_attribution,
        "successful dup2 from an untracked source must never leave the old destination path authoritative"
    );
    assert!(
        !observation.complete || replacement_attribution,
        "if the replacement source identity is unavailable, the observation must fail closed rather than remain complete with no truthful replacement attribution"
    );
}

#[test]
fn r1_2_untracked_fd_io_must_not_disappear_under_complete_true() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let observation = observe_command(
        &CommandSpec::new(fixture).arg("stage2-stderr-write"),
    )
    .expect("observe untracked stderr write");

    let attributed = observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Write,
                fd: libc::STDERR_FILENO,
                ..
            }
        )
    });

    assert!(
        attributed || !observation.complete,
        "successful I/O on an untracked fd must be represented or must make the observation non-authoritative"
    );
}

#[test]
fn r1_3_side_effectful_symlink_truncate_must_not_be_complete_with_link_only_identity() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let root = std::env::temp_dir().join(format!(
        "execsurface-stage2-r1-symlink-{}",
        process::id()
    ));
    let workspace = root.join("workspace");
    let credential = root.join("credential");
    let link = workspace.join("link");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&workspace).expect("create workspace");
    fs::write(&credential, b"secret").expect("write credential target");
    symlink("../credential", &link).expect("create relative symlink");

    let expected_credential = fs::canonicalize(&credential)
        .expect("canonicalize credential target")
        .to_string_lossy()
        .into_owned();
    let expected_link = link.to_string_lossy().into_owned();

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("stage2-symlink-truncate")
            .arg(link.as_os_str()),
    )
    .expect("observe symlink truncate fixture");

    let target_len = fs::metadata(&credential)
        .expect("credential metadata after truncate")
        .len();

    let link_only_open = observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FilePathAccess {
                operation: FileOperation::Open,
                path,
                ..
            } if path == &expected_link
        )
    });
    let object_identity_evidence = observation.events.iter().any(|event| {
        match &event.kind {
            RawEventKind::FilePathAccess { path, .. }
            | RawEventKind::FileOpenAt2 { path, .. }
            | RawEventKind::FileDescriptorAccess { path, .. } => {
                path == &expected_credential
            }
            _ => false,
        }
    });

    let _ = fs::remove_dir_all(&root);

    assert_eq!(
        target_len, 0,
        "the kernel-resolved credential object must actually be truncated"
    );
    assert!(
        link_only_open,
        "the current observer must expose the lexical attempt path for this counterexample"
    );
    assert!(
        object_identity_evidence || !observation.complete,
        "a side effect that occurs during open must not remain complete when evidence contains only lexical link identity and no kernel-resolved target identity"
    );
}
