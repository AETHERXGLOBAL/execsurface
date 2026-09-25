#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::net::TcpListener;
use std::thread;
use std::time::{Duration, Instant};

use execsurface_model::{FileOperation, NetworkEndpoint, RawEventKind};
use execsurface_observe::{
    observe_command, observe_command_with_options, CommandSpec, ObserveOptions,
};

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface-fixture")
}

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("execsurface-m65-{name}-{}", std::process::id()))
}

#[test]
fn captures_descendant_exec_without_argv_values() {
    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("spawn")
            .arg("AX_EXEC_SURFACE_SECRET_SENTINEL_9f62d6f2"),
    )
    .expect("observe");

    assert!(
        observation.events.iter().any(|event| matches!(
            &event.kind,
            RawEventKind::ProcessExec { path } if path == "/bin/true"
        )),
        "expected confirmed /bin/true exec event: {observation:#?}"
    );

    let serialized = serde_json::to_string(&observation).expect("serialize");
    assert!(
        !serialized.contains("AX_EXEC_SURFACE_SECRET_SENTINEL_9f62d6f2"),
        "observer output leaked an argv-only sentinel"
    );
}

#[test]
fn captures_path_based_file_open_and_actual_fd_read_write() {
    let path = temp_path("file-rw");
    fs::write(&path, b"fixture").expect("write fixture");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("file-rw")
            .arg(path.as_os_str()),
    )
    .expect("observe");

    let expected = path.to_string_lossy();
    assert!(observation.complete, "{:#?}", observation.warnings);
    assert!(observation.events.iter().any(|event| matches!(
        &event.kind,
        RawEventKind::FilePathAccess { path, .. } if path == expected.as_ref()
    )));
    assert!(observation.events.iter().any(|event| matches!(
        &event.kind,
        RawEventKind::FileDescriptorAccess {
            operation: FileOperation::Read,
            path,
            ..
        } if path == expected.as_ref()
    )));
    assert!(observation.events.iter().any(|event| matches!(
        &event.kind,
        RawEventKind::FileDescriptorAccess {
            operation: FileOperation::Write,
            path,
            ..
        } if path == expected.as_ref()
    )));

    let _ = fs::remove_file(path);
}

#[test]
fn resolves_relative_open_against_trace_time_cwd() {
    let dir = temp_path("relative");
    fs::create_dir_all(&dir).expect("dir");
    fs::write(dir.join("input.txt"), b"x").expect("file");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("relative-file")
            .arg(dir.as_os_str())
            .arg("input.txt"),
    )
    .expect("observe");

    let expected = dir.join("input.txt").to_string_lossy().into_owned();
    assert!(observation.complete, "{:#?}", observation.warnings);
    assert!(observation.events.iter().any(|event| matches!(
        &event.kind,
        RawEventKind::FilePathAccess { path, .. } if path == &expected
    )));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn tracks_dup_and_fork_inherited_file_descriptors() {
    let path = temp_path("dup-fork");
    fs::write(&path, b"fixture").expect("file");
    let expected = path.to_string_lossy().into_owned();

    for mode in ["dup-read", "fork-read"] {
        let observation =
            observe_command(&CommandSpec::new(fixture()).arg(mode).arg(path.as_os_str()))
                .expect("observe");
        assert!(observation.complete, "{mode}: {:#?}", observation.warnings);
        assert!(
            observation.events.iter().any(|event| matches!(
                &event.kind,
                RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Read,
                    path,
                    ..
                } if path == &expected
            )),
            "{mode}: {observation:#?}"
        );
    }

    let _ = fs::remove_file(path);
}

#[test]
fn tracks_clone_files_shared_fd_reads() {
    let path = temp_path("thread");
    fs::write(&path, b"fixture").expect("file");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("thread-read")
            .arg(path.as_os_str())
            .arg("8"),
    )
    .expect("observe");

    let expected = path.to_string_lossy();
    assert!(observation.complete, "{:#?}", observation.warnings);
    let reads = observation
        .events
        .iter()
        .filter(|event| {
            matches!(
                &event.kind,
                RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Read,
                    path,
                    ..
                } if path == expected.as_ref()
            )
        })
        .count();
    assert!(reads >= 8, "expected thread fd reads: {observation:#?}");

    let _ = fs::remove_file(path);
}

#[test]
fn captures_openat2_flags_and_resolve_metadata() {
    let path = temp_path("openat2");
    fs::write(&path, b"x").expect("file");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("openat2")
            .arg(path.as_os_str()),
    )
    .expect("observe");

    let expected = path.to_string_lossy();
    assert!(observation.complete, "{:#?}", observation.warnings);
    assert!(observation.events.iter().any(|event| matches!(
        &event.kind,
        RawEventKind::FileOpenAt2 {
            path,
            flags,
            resolve
        } if path == expected.as_ref() && *flags == libc::O_RDONLY as u64 && *resolve == 0
    )));

    let _ = fs::remove_file(path);
}

#[test]
fn event_budget_overflow_is_fail_closed() {
    let dir = temp_path("budget");
    let observation = observe_command_with_options(
        &CommandSpec::new(fixture())
            .arg("burst")
            .arg(dir.as_os_str())
            .arg("64"),
        ObserveOptions { event_limit: 32 },
    )
    .expect("observe");

    assert!(!observation.complete);
    assert!(observation
        .warnings
        .iter()
        .any(|warning| warning.code == "event_limit_exceeded"));
    assert_eq!(observation.events.len(), 32);

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn high_event_volume_remains_complete_under_default_budget() {
    let dir = temp_path("stress");
    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("burst")
            .arg(dir.as_os_str())
            .arg("256"),
    )
    .expect("observe");

    assert!(observation.complete, "{:#?}", observation.warnings);
    let prefix = dir.to_string_lossy();
    let writes = observation
        .events
        .iter()
        .filter(|event| {
            matches!(
                &event.kind,
                RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Write,
                    path,
                    ..
                } if path.starts_with(prefix.as_ref())
            )
        })
        .count();
    assert!(
        writes >= 256,
        "expected >=256 attributed writes, got {writes}"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn captures_local_network_connect_destination() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let address = listener.local_addr().expect("local addr");
    listener.set_nonblocking(true).expect("nonblocking");

    let acceptor = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match listener.accept() {
                Ok(_) => return,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    if Instant::now() >= deadline {
                        panic!("listener timed out");
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("listener failed: {error}"),
            }
        }
    });

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("network")
            .arg(address.to_string()),
    )
    .expect("observe");

    acceptor.join().expect("acceptor");

    assert!(
        observation.events.iter().any(|event| matches!(
            &event.kind,
            RawEventKind::NetworkConnectAttempt {
                endpoint: NetworkEndpoint::Inet { ip, port }
            } if ip == "127.0.0.1" && *port == address.port()
        )),
        "expected loopback connect target: {observation:#?}"
    );
}

#[test]
fn backend_declares_hardened_scope_and_limitations() {
    let observation = observe_command(&CommandSpec::new(fixture()).arg("noop")).expect("observe");
    assert_eq!(observation.backend.name, "linux-ptrace-metadata-v2");
    assert_eq!(observation.backend.architecture, "x86_64");
    assert!(observation
        .backend
        .capabilities
        .iter()
        .any(|capability| capability == "fd_read_write_attribution"));
    assert!(observation
        .backend
        .capabilities
        .iter()
        .any(|capability| capability == "fail_closed_event_budget"));
    assert!(!observation.backend.limitations.is_empty());
}
