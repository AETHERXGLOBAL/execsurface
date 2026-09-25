#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use execsurface_baseline::{parse_and_verify, BaselineLock};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn temp_dir(name: &str) -> std::path::PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("execsurface-m3-{name}-{}-{id}", std::process::id()));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface")
}

#[test]
fn learn_is_byte_reproducible_and_does_not_capture_argv_secret() {
    let dir = temp_dir("repro");
    let first = dir.join("first.lock.json");
    let second = dir.join("second.lock.json");
    let secret = "AX_M3_ARGV_SECRET_7c4300f1";

    for output in [&first, &second] {
        let status = Command::new(cli())
            .current_dir(&dir)
            .args(["learn", "--output"])
            .arg(output)
            .args(["--", "/bin/true", secret])
            .stdout(Stdio::null())
            .status()
            .expect("run learn");
        assert!(status.success());
    }

    let first_bytes = fs::read(&first).expect("first bytes");
    let second_bytes = fs::read(&second).expect("second bytes");
    assert_eq!(first_bytes, second_bytes);
    assert!(!String::from_utf8_lossy(&first_bytes).contains(secret));

    let lock: BaselineLock = parse_and_verify(&first_bytes).expect("verified lock");
    assert_eq!(lock.payload.command.argument_count, 1);
    assert_eq!(lock.payload.command.executable.family, "true");

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn learn_refuses_silent_overwrite_and_allows_explicit_overwrite() {
    let dir = temp_dir("overwrite");
    let output = dir.join("baseline.json");

    let first = Command::new(cli())
        .current_dir(&dir)
        .args(["learn", "--output"])
        .arg(&output)
        .args(["--", "/bin/true"])
        .stdout(Stdio::null())
        .status()
        .expect("first learn");
    assert!(first.success());

    let second = Command::new(cli())
        .current_dir(&dir)
        .args(["learn", "--output"])
        .arg(&output)
        .args(["--", "/bin/true"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("second learn");
    assert!(!second.success());

    let overwrite = Command::new(cli())
        .current_dir(&dir)
        .args(["learn", "--overwrite", "--output"])
        .arg(&output)
        .args(["--", "/bin/true"])
        .stdout(Stdio::null())
        .status()
        .expect("overwrite learn");
    assert!(overwrite.success());
    parse_and_verify(&fs::read(&output).expect("read")).expect("verified lock");

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn failed_target_does_not_create_baseline() {
    let dir = temp_dir("failure");
    let output = dir.join("failed.json");

    let status = Command::new(cli())
        .current_dir(&dir)
        .args(["learn", "--output"])
        .arg(&output)
        .args(["--", "/bin/false"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("learn false");

    assert!(!status.success());
    assert!(!output.exists());
    let _ = fs::remove_dir_all(dir);
}
