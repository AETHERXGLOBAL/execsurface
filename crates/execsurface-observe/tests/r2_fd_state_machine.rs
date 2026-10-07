use std::fs;
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};

fn has_fd_write_to(observation: &execsurface_model::Observation, expected: &str) -> bool {
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

fn has_fd_write_elsewhere(observation: &execsurface_model::Observation, excluded: &str) -> bool {
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
fn r2_dup3_unknown_source_cannot_preserve_replaced_destination_identity() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let victim = std::env::temp_dir().join(format!(
        "execsurface-stage2-r2-dup3-{}-victim.txt",
        process::id()
    ));
    fs::write(&victim, b"").expect("create victim");
    let expected_victim = fs::canonicalize(&victim)
        .expect("canonicalize victim")
        .to_string_lossy()
        .into_owned();

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("stage2-dup3-untracked-write")
            .arg(victim.as_os_str()),
    )
    .expect("observe dup3 remediation fixture");

    let victim_len = fs::metadata(&victim).expect("victim metadata").len();
    let _ = fs::remove_file(&victim);

    assert_eq!(victim_len, 0, "kernel write must target the pipe, not victim");
    assert!(
        !has_fd_write_to(&observation, &expected_victim),
        "dup3 from an unknown source must retire stale destination identity"
    );
    assert!(
        !observation.complete || has_fd_write_elsewhere(&observation, &expected_victim),
        "unknown dup3 source must be recovered truthfully or force incomplete evidence"
    );
}

#[test]
fn r2_fcntl_dupfd_unknown_source_is_represented_or_fail_closed() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let observation = observe_command(
        &CommandSpec::new(fixture).arg("stage2-fcntl-dupfd-untracked-write"),
    )
    .expect("observe fcntl duplication remediation fixture");

    let represented_write = observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Write,
                ..
            }
        )
    });

    assert!(
        represented_write || !observation.complete,
        "successful I/O through F_DUPFD_CLOEXEC from an untracked source must be represented or fail closed"
    );
}

#[test]
fn r2_dup2_same_fd_is_a_noop_for_tracked_identity() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let target = std::env::temp_dir().join(format!(
        "execsurface-stage2-r2-same-fd-{}.txt",
        process::id()
    ));
    fs::write(&target, b"").expect("create same-fd target");
    let expected_target = fs::canonicalize(&target)
        .expect("canonicalize same-fd target")
        .to_string_lossy()
        .into_owned();

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("stage2-dup2-same-fd-write")
            .arg(target.as_os_str()),
    )
    .expect("observe same-fd dup2 fixture");

    let target_len = fs::metadata(&target).expect("same-fd target metadata").len();
    let _ = fs::remove_file(&target);

    assert_eq!(target_len, 1, "same-fd dup2 must preserve the live descriptor");
    assert!(
        has_fd_write_to(&observation, &expected_target),
        "same-fd dup2 must preserve the tracked kernel identity used by the subsequent write"
    );
}
