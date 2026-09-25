#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::net::TcpListener;
use std::thread;
use std::time::{Duration, Instant};

use execsurface_model::{NetworkEndpoint, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface-fixture")
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
fn captures_path_based_file_open() {
    let path = std::env::temp_dir().join(format!(
        "execsurface-m1-file-fixture-{}",
        std::process::id()
    ));
    fs::write(&path, b"fixture").expect("write fixture");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("file")
            .arg(path.as_os_str()),
    )
    .expect("observe");

    let expected = path.to_string_lossy();
    assert!(
        observation.events.iter().any(|event| matches!(
            &event.kind,
            RawEventKind::FilePathAccess { path, .. } if path == expected.as_ref()
        )),
        "expected file path in raw observation: {observation:#?}"
    );

    let _ = fs::remove_file(path);
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
fn backend_declares_scope_and_limitations() {
    let observation = observe_command(&CommandSpec::new(fixture()).arg("noop")).expect("observe");
    assert_eq!(observation.backend.name, "linux-ptrace-metadata-only");
    assert_eq!(observation.backend.architecture, "x86_64");
    assert!(!observation.backend.limitations.is_empty());
}
