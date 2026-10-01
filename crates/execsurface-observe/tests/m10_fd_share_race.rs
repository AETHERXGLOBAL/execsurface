use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};
use serde_json::json;

fn ab_from_path(path: &str) -> Option<char> {
    match Path::new(path).file_name().and_then(OsStr::to_str) {
        Some("A") => Some('A'),
        Some("B") => Some('B'),
        _ => None,
    }
}

#[test]
#[ignore = "M10 research gate; run only from the dedicated M10 workflow"]
fn m10_fd_share_race_gate() {
    let target = PathBuf::from(env::var("M10_FD_TARGET_BIN").expect("M10_FD_TARGET_BIN is required"));
    let result_path = PathBuf::from(
        env::var("M10_FD_RESULT_PATH").unwrap_or_else(|_| "m10-fd-share-race-result.json".to_owned()),
    );
    let result_path = if result_path.is_absolute() {
        result_path
    } else {
        env::current_dir().unwrap().join(result_path)
    };

    assert!(target.is_absolute(), "M10_FD_TARGET_BIN must be absolute");
    assert!(target.exists(), "target fixture does not exist: {}", target.display());

    let original_cwd = env::current_dir().expect("read current directory");
    let workspace = env::temp_dir().join(format!("execsurface-m10-fd-share-race-{}", process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).expect("create isolated workspace");
    fs::write(workspace.join("A"), b"A").expect("write A fixture");
    fs::write(workspace.join("B"), b"B").expect("write B fixture");
    env::set_current_dir(&workspace).expect("enter isolated workspace");

    let observation_result = observe_command(&CommandSpec::new(target.as_os_str().to_owned()));

    env::set_current_dir(&original_cwd).expect("restore current directory");

    let observation = match observation_result {
        Ok(value) => value,
        Err(error) => {
            let result = json!({
                "schema": "execsurface-m10-fd-share-race-result-v1",
                "classification": "HARNESS_INCONCLUSIVE",
                "reason": "observer_error",
                "error": error.to_string(),
                "source_sha": env::var("GITHUB_SHA").unwrap_or_else(|_| "local".to_owned()),
            });
            fs::write(&result_path, serde_json::to_vec_pretty(&result).unwrap()).unwrap();
            panic!("HARNESS_INCONCLUSIVE: observer error: {error}");
        }
    };

    let truth_path = workspace.join("fd_truth.log");
    let truth_bytes = fs::read(&truth_path).unwrap_or_default();
    let truth: Vec<char> = truth_bytes
        .iter()
        .filter_map(|byte| match *byte {
            b'A' => Some('A'),
            b'B' => Some('B'),
            _ => None,
        })
        .collect();
    let invalid_truth_bytes = truth_bytes.len().saturating_sub(truth.len());

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

    let paired_len = truth.len().min(emitted.len());
    let mut paired_mismatches = 0_u64;
    let mut mismatch_examples = Vec::new();
    for index in 0..paired_len {
        if truth[index] != emitted[index] {
            paired_mismatches += 1;
            if mismatch_examples.len() < 30 {
                mismatch_examples.push(json!({
                    "index": index,
                    "kernel_read_truth": truth[index].to_string(),
                    "execsurface_fd_read_path": emitted[index].to_string(),
                }));
            }
        }
    }

    let count_delta = emitted.len() as i64 - truth.len() as i64;
    let eligible = observation.complete
        && observation.warnings.is_empty()
        && observation.outcome.exit_code == Some(0)
        && truth.len() >= 50
        && invalid_truth_bytes == 0;

    let classification = if !eligible {
        "HARNESS_INCONCLUSIVE"
    } else if count_delta != 0 || paired_mismatches > 0 {
        "PTRACE_FD_ATTRIBUTION_COUNTEREXAMPLE_OBSERVED"
    } else {
        "NO_FD_ATTRIBUTION_MISMATCH_OBSERVED_NOT_PROOF"
    };

    let prefix_len = truth.len().min(emitted.len()).min(100);
    let result = json!({
        "schema": "execsurface-m10-fd-share-race-result-v1",
        "classification": classification,
        "source_sha": env::var("GITHUB_SHA").unwrap_or_else(|_| "local".to_owned()),
        "kernel_release": fs::read_to_string("/proc/sys/kernel/osrelease").ok().map(|s| s.trim().to_owned()),
        "observation_complete": observation.complete,
        "warning_codes": observation.warnings.iter().map(|w| w.code.clone()).collect::<Vec<_>>(),
        "target_exit_code": observation.outcome.exit_code,
        "truth_successful_read_count": truth.len(),
        "emitted_fd_read_count": emitted.len(),
        "count_delta": count_delta,
        "paired_identity_mismatch_count": paired_mismatches,
        "invalid_truth_bytes": invalid_truth_bytes,
        "truth_prefix": truth.iter().take(prefix_len).map(char::to_string).collect::<String>(),
        "emitted_prefix": emitted.iter().take(prefix_len).map(char::to_string).collect::<String>(),
        "mismatch_examples": mismatch_examples,
        "observation_event_count": observation.events.len(),
        "interpretation_boundary": {
            "kills_universal_fd_attribution_if_divergence_observed": true,
            "does_not_prove_bpf_lsm_correctness": true,
            "does_not_authorize_runtime_change": true,
            "does_not_measure_real_world_frequency": true
        }
    });

    fs::write(&result_path, serde_json::to_vec_pretty(&result).unwrap()).expect("write result artifact");

    println!("M10 FD-SHARE classification: {classification}");
    println!("truth successful reads: {}", truth.len());
    println!("emitted fd reads: {}", emitted.len());
    println!("count delta: {count_delta}");
    println!("paired mismatches: {paired_mismatches}");
    println!("result: {}", result_path.display());

    assert!(eligible, "HARNESS_INCONCLUSIVE: eligibility failed");
}
