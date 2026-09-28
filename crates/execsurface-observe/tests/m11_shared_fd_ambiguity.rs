use std::fs;
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};

fn warning_count(observation: &execsurface_model::Observation, code: &str) -> usize {
    observation
        .warnings
        .iter()
        .filter(|warning| warning.code == code)
        .count()
}

fn observed_fd_read_path(observation: &execsurface_model::Observation, expected: &str) -> bool {
    observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Read,
                path,
                ..
            } if path == expected
        )
    })
}

#[test]
fn clone_based_threading_fails_closed_for_fd_lifecycle_completeness() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let path = std::env::temp_dir().join(format!(
        "execsurface-m11-shared-fd-{}-fixture.txt",
        process::id()
    ));
    fs::write(&path, b"x").expect("write fixture file");

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("thread-read")
            .arg(path.as_os_str())
            .arg("4"),
    )
    .expect("observe threaded fixture");

    let _ = fs::remove_file(&path);

    assert!(
        !observation.complete,
        "clone-based concurrency must not retain complete=true"
    );
    assert_eq!(
        warning_count(&observation, "shared_fd_table_ambiguity"),
        1,
        "ambiguity warning must be present exactly once"
    );
    assert_eq!(
        warning_count(&observation, "clone_flags_unavailable"),
        0,
        "real thread creation should retain clone flags inside the live ptrace state machine"
    );
}

#[test]
fn shared_fd_reuse_tracks_replacement_path_but_stays_fail_closed() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let old_path = std::env::temp_dir().join(format!(
        "execsurface-s5-shared-old-{}-fixture.txt",
        process::id()
    ));
    let new_path = std::env::temp_dir().join(format!(
        "execsurface-s5-shared-new-{}-fixture.txt",
        process::id()
    ));
    fs::write(&old_path, b"o").expect("write old fixture file");
    fs::write(&new_path, b"n").expect("write new fixture file");

    let expected_old = fs::canonicalize(&old_path)
        .expect("canonicalize old path")
        .to_string_lossy()
        .into_owned();
    let expected_new = fs::canonicalize(&new_path)
        .expect("canonicalize new path")
        .to_string_lossy()
        .into_owned();

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("thread-fd-reuse")
            .arg(old_path.as_os_str())
            .arg(new_path.as_os_str()),
    )
    .expect("observe shared-fd reuse fixture");

    let _ = fs::remove_file(&old_path);
    let _ = fs::remove_file(&new_path);

    assert!(
        !observation.complete,
        "alpha.4 guard must remain fail-closed during the S5 experiment"
    );
    assert_eq!(
        warning_count(&observation, "shared_fd_table_ambiguity"),
        1,
        "public v2 evidence must still report shared-fd ambiguity"
    );
    assert_eq!(
        warning_count(&observation, "clone_flags_unavailable"),
        0,
        "live clone flags must be available for the shared-thread fixture"
    );
    assert!(
        observed_fd_read_path(&observation, &expected_new),
        "read after shared-table fd reuse must be attributed to the replacement path"
    );
    assert!(
        !observed_fd_read_path(&observation, &expected_old),
        "read after fd reuse must not be attributed to stale old-path fd state"
    );
}

#[test]
fn no_clone_control_does_not_invent_shared_fd_ambiguity() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let observation =
        observe_command(&CommandSpec::new(fixture).arg("noop")).expect("observe no-clone control");

    assert_eq!(
        warning_count(&observation, "shared_fd_table_ambiguity"),
        0,
        "no-clone control must not receive the M11.3 ambiguity warning"
    );
}
