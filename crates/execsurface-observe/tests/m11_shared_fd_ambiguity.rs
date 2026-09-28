use std::fs;
use std::process;

use execsurface_model::{FileOperation, RawEventKind, SpawnMechanism};
use execsurface_observe::{
    observe_command, observe_command_with_options, CommandSpec, ObserveOptions,
};

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

fn clone_spawn_count(observation: &execsurface_model::Observation) -> usize {
    observation
        .events
        .iter()
        .filter(|event| {
            matches!(
                &event.kind,
                RawEventKind::ProcessSpawn {
                    mechanism: SpawnMechanism::Clone,
                    ..
                }
            )
        })
        .count()
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
fn known_clone_without_clone_files_is_live_but_v2_still_fails_closed() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let path = std::env::temp_dir().join(format!(
        "execsurface-s5-private-clone-{}-fixture.txt",
        process::id()
    ));
    fs::write(&path, b"p").expect("write private-clone fixture file");
    let expected = fs::canonicalize(&path)
        .expect("canonicalize private-clone path")
        .to_string_lossy()
        .into_owned();

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("clone-private-read")
            .arg(path.as_os_str()),
    )
    .expect("observe private clone fixture");

    let _ = fs::remove_file(&path);

    assert!(
        clone_spawn_count(&observation) >= 1,
        "non-SIGCHLD clone must reach the observer as clone-family evidence"
    );
    assert_eq!(
        warning_count(&observation, "clone_flags_unavailable"),
        0,
        "private clone must retain readable clone flags in the live state machine"
    );
    assert!(
        observed_fd_read_path(&observation, &expected),
        "independent copied fd table must preserve the inherited fd identity for the child read"
    );
    assert!(
        !observation.complete,
        "raw v2 still lacks retained clone flags, so the public guard must remain fail-closed"
    );
    assert_eq!(
        warning_count(&observation, "shared_fd_table_ambiguity"),
        1,
        "raw v2 must not silently infer private fd-table semantics from an unretained clone flag"
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
fn resource_truncation_remains_fail_closed_under_clone_concurrency() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let path = std::env::temp_dir().join(format!(
        "execsurface-s5-resource-limit-{}-fixture.txt",
        process::id()
    ));
    fs::write(&path, b"r").expect("write resource-limit fixture file");

    let observation = observe_command_with_options(
        &CommandSpec::new(fixture)
            .arg("thread-read")
            .arg(path.as_os_str())
            .arg("4"),
        ObserveOptions { event_limit: 2 },
    )
    .expect("observe resource-limited threaded fixture");

    let _ = fs::remove_file(&path);

    assert!(
        !observation.complete,
        "resource truncation must remain non-PASS-eligible regardless of fd-table classification"
    );
    assert!(
        warning_count(&observation, "event_limit_exceeded") >= 1,
        "resource-limited fixture must retain explicit event-budget incompleteness"
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
