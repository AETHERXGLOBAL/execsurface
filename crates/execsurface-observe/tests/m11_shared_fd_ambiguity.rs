use std::fs;
use std::process;

use execsurface_observe::{observe_command, CommandSpec};

fn warning_count(observation: &execsurface_model::Observation, code: &str) -> usize {
    observation
        .warnings
        .iter()
        .filter(|warning| warning.code == code)
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
