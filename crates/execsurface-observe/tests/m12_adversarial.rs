#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{
    experimental_ebpf_backend_descriptor, observe_command, observe_command_with_options,
    reference_backend_descriptor, CommandSpec, ObservationCapability, ObserveOptions,
};
use serde_json::json;

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface-fixture")
}

fn temp_dir(name: &str) -> PathBuf {
    let path = env::temp_dir().join(format!("execsurface-m12-{name}-{}", process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn ab_from_path(path: &str) -> Option<char> {
    match Path::new(path).file_name().and_then(OsStr::to_str) {
        Some("A") => Some('A'),
        Some("B") => Some('B'),
        _ => None,
    }
}

fn has_warning(observation: &execsurface_model::Observation, code: &str) -> bool {
    observation
        .warnings
        .iter()
        .any(|warning| warning.code == code)
}

fn write_result(name: &str, value: &serde_json::Value) {
    if let Ok(dir) = env::var("M12_RESULT_DIR") {
        let dir = PathBuf::from(dir);
        fs::create_dir_all(&dir).expect("create M12 result dir");
        fs::write(
            dir.join(name),
            serde_json::to_vec_pretty(value).expect("serialize M12 result"),
        )
        .expect("write M12 result");
    }
}

#[test]
fn path_toctou_counterexample_is_replayed_and_non_pass_eligible() {
    let target = PathBuf::from(env::var("M12_PATH_TOCTOU_BIN").expect("M12_PATH_TOCTOU_BIN"));
    let runs: usize = env::var("M12_PATH_RUNS")
        .unwrap_or_else(|_| "300".to_owned())
        .parse()
        .expect("M12_PATH_RUNS integer");
    assert!(target.is_absolute() && target.exists());
    assert!(runs >= 50);

    let original_cwd = env::current_dir().expect("cwd");
    let workspace = temp_dir("path-toctou");
    fs::write(workspace.join("A"), b"A").expect("A");
    fs::write(workspace.join("B"), b"B").expect("B");
    env::set_current_dir(&workspace).expect("enter workspace");

    let mut valid_truth_runs = 0_u64;
    let mut mismatch_runs = 0_u64;
    let mut mismatch_fail_closed_runs = 0_u64;
    let mut observer_errors = 0_u64;

    for _ in 0..runs {
        let observation = match observe_command(&CommandSpec::new(target.as_os_str().to_owned())) {
            Ok(value) => value,
            Err(_) => {
                observer_errors += 1;
                continue;
            }
        };

        let truth = match observation.outcome.exit_code {
            Some(11) => Some('A'),
            Some(12) => Some('B'),
            _ => None,
        };
        let Some(truth) = truth else {
            continue;
        };
        valid_truth_runs += 1;

        let intents: Vec<char> = observation
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                RawEventKind::FilePathAccess {
                    operation: FileOperation::Open,
                    path,
                    ..
                } => ab_from_path(path),
                _ => None,
            })
            .collect();

        if intents.len() == 1 && intents[0] != truth {
            mismatch_runs += 1;
            if !observation.complete && has_warning(&observation, "shared_fd_table_ambiguity") {
                mismatch_fail_closed_runs += 1;
            }
        }
    }

    env::set_current_dir(&original_cwd).expect("restore cwd");
    let _ = fs::remove_dir_all(&workspace);

    let result = json!({
        "schema": "execsurface-m12-path-toctou-adversarial-v1",
        "requested_runs": runs,
        "valid_truth_runs": valid_truth_runs,
        "observer_errors": observer_errors,
        "path_intent_vs_kernel_truth_mismatches": mismatch_runs,
        "mismatch_runs_fail_closed": mismatch_fail_closed_runs,
        "classification": if mismatch_runs > 0 && mismatch_fail_closed_runs == mismatch_runs {
            "PATH_TOCTOU_REPRODUCED_AND_FAIL_CLOSED_FOR_FIXTURE"
        } else if mismatch_runs == 0 {
            "COUNTEREXAMPLE_NOT_REPRODUCED_INCONCLUSIVE"
        } else {
            "COUNTEREXAMPLE_REPRODUCED_WITH_PASS_ELIGIBLE_GAP"
        },
        "claim_boundary": "FilePathAccess is pathname access-attempt metadata, not kernel-object identity"
    });
    write_result("path-toctou.json", &result);

    assert!(
        valid_truth_runs > 0,
        "path fixture produced no valid kernel truth"
    );
    assert!(
        mismatch_runs > 0,
        "known PATH-TOCTOU counterexample did not reproduce"
    );
    assert_eq!(
        mismatch_fail_closed_runs, mismatch_runs,
        "a reproduced PATH-TOCTOU mismatch remained pass-eligible"
    );
}

#[test]
fn shared_fd_counterexample_is_replayed_and_fail_closed() {
    let target = PathBuf::from(env::var("M12_FD_SHARE_BIN").expect("M12_FD_SHARE_BIN"));
    assert!(target.is_absolute() && target.exists());

    let original_cwd = env::current_dir().expect("cwd");
    let mut divergent_attempts = 0_u64;
    let mut fail_closed_attempts = 0_u64;
    let attempts = 3_u64;
    let mut truth_reads = 0_u64;
    let mut emitted_reads = 0_u64;

    for attempt in 0..attempts {
        let workspace = temp_dir(&format!("fd-share-{attempt}"));
        fs::write(workspace.join("A"), b"A").expect("A");
        fs::write(workspace.join("B"), b"B").expect("B");
        env::set_current_dir(&workspace).expect("enter workspace");
        let observation = observe_command(&CommandSpec::new(target.as_os_str().to_owned()))
            .expect("observe fd-share target");
        env::set_current_dir(&original_cwd).expect("restore cwd");

        assert_eq!(observation.outcome.exit_code, Some(0), "target failed");
        let truth_bytes = fs::read(workspace.join("fd_truth.log")).expect("fd truth log");
        let truth: Vec<char> = truth_bytes
            .iter()
            .filter_map(|byte| match *byte {
                b'A' => Some('A'),
                b'B' => Some('B'),
                _ => None,
            })
            .collect();
        assert!(truth.len() >= 50, "insufficient target truth");

        let emitted: Vec<char> = observation
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Read,
                    path,
                    ..
                } => ab_from_path(path),
                _ => None,
            })
            .collect();

        truth_reads += truth.len() as u64;
        emitted_reads += emitted.len() as u64;
        let paired_mismatch = truth
            .iter()
            .zip(emitted.iter())
            .any(|(truth, emitted)| truth != emitted);
        if truth.len() != emitted.len() || paired_mismatch {
            divergent_attempts += 1;
        }

        if !observation.complete && has_warning(&observation, "shared_fd_table_ambiguity") {
            fail_closed_attempts += 1;
        }
        let _ = fs::remove_dir_all(&workspace);
    }

    let result = json!({
        "schema": "execsurface-m12-shared-fd-adversarial-v1",
        "attempts": attempts,
        "divergent_attempts": divergent_attempts,
        "fail_closed_attempts": fail_closed_attempts,
        "target_truth_reads": truth_reads,
        "emitted_attributed_reads": emitted_reads,
        "classification": if fail_closed_attempts == attempts && divergent_attempts > 0 {
            "SHARED_FD_COUNTEREXAMPLE_REPRODUCED_AND_FAIL_CLOSED"
        } else if fail_closed_attempts == attempts {
            "FAIL_CLOSED_COUNTEREXAMPLE_NOT_REPRODUCED_INCONCLUSIVE"
        } else {
            "SHARED_FD_FAIL_CLOSED_REGRESSION"
        }
    });
    write_result("shared-fd.json", &result);

    assert_eq!(
        fail_closed_attempts, attempts,
        "shared-FD ambiguity did not always fail closed"
    );
    assert!(
        divergent_attempts > 0,
        "known shared-FD divergence did not reproduce"
    );
}

#[test]
fn event_budget_loss_remains_fail_closed() {
    let dir = temp_dir("loss");
    let observation = observe_command_with_options(
        &CommandSpec::new(fixture())
            .arg("burst")
            .arg(dir.as_os_str())
            .arg("64"),
        ObserveOptions { event_limit: 16 },
    )
    .expect("observe loss fixture");

    assert!(!observation.complete);
    assert!(has_warning(&observation, "event_limit_exceeded"));
    assert_eq!(observation.events.len(), 16);
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn failed_open_attempt_is_not_promoted_to_fd_success_effect() {
    let dir = temp_dir("failed-open");
    let missing = dir.join("definitely-missing-input");
    let script = r#"cat -- "$1" >/dev/null 2>&1 || true"#;
    let observation = observe_command(
        &CommandSpec::new("/bin/sh")
            .arg("-c")
            .arg(script)
            .arg("m12-sh")
            .arg(missing.as_os_str()),
    )
    .expect("observe failed open");

    let expected = missing.to_string_lossy();
    let saw_attempt = observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FilePathAccess {
                operation: FileOperation::Open,
                path,
                ..
            } if path == expected.as_ref()
        )
    });
    let promoted_success = observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FileDescriptorAccess { path, .. } if path == expected.as_ref()
        )
    });

    assert!(saw_attempt, "failed open attempt was not observed");
    assert!(
        !promoted_success,
        "failed open was promoted to a successful fd effect"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn observation_sessions_do_not_leak_prior_file_identity() {
    let dir = temp_dir("session-isolation");
    let first_path = dir.join("first-session-only.txt");
    fs::write(&first_path, b"secret-independent-marker").expect("write fixture");

    let first = observe_command(
        &CommandSpec::new(fixture())
            .arg("file")
            .arg(first_path.as_os_str()),
    )
    .expect("first observation");
    let expected = first_path.to_string_lossy();
    assert!(first.events.iter().any(|event| match &event.kind {
        RawEventKind::FilePathAccess { path, .. }
        | RawEventKind::FileDescriptorAccess { path, .. } => path == expected.as_ref(),
        _ => false,
    }));

    let second =
        observe_command(&CommandSpec::new(fixture()).arg("noop")).expect("second observation");
    assert!(
        !second.events.iter().any(|event| match &event.kind {
            RawEventKind::FilePathAccess { path, .. }
            | RawEventKind::FileDescriptorAccess { path, .. } => path == expected.as_ref(),
            _ => false,
        }),
        "second observation leaked first-session file identity"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn unsupported_experimental_capabilities_cannot_be_confused_with_default() {
    let reference = reference_backend_descriptor();
    let experimental = experimental_ebpf_backend_descriptor();

    reference
        .validate_capability_partition()
        .expect("reference partition");
    experimental
        .validate_capability_partition()
        .expect("experimental partition");

    assert_eq!(reference.id, "linux-ptrace-metadata-v2");
    assert_ne!(reference.id, experimental.id);
    assert!(experimental
        .unsupported_capabilities
        .contains(&ObservationCapability::OpenPathIdentity));
    assert!(experimental
        .unsupported_capabilities
        .contains(&ObservationCapability::NetworkConnectDestination));
}
