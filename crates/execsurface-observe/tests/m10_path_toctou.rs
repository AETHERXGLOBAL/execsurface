use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};
use serde_json::{json, Value};

fn ab_label_from_path(path: &str) -> Option<&'static str> {
    match Path::new(path).file_name().and_then(OsStr::to_str) {
        Some("A") => Some("A"),
        Some("B") => Some("B"),
        _ => None,
    }
}

fn bump(counter_a: &mut u64, counter_b: &mut u64, label: &str) {
    match label {
        "A" => *counter_a += 1,
        "B" => *counter_b += 1,
        _ => {}
    }
}

#[test]
#[ignore = "M10 research gate; run only from the dedicated M10 workflow"]
fn m10_path_toctou_gate() {
    let target = PathBuf::from(env::var("M10_TARGET_BIN").expect("M10_TARGET_BIN is required"));
    let runs: usize = env::var("M10_RUNS")
        .unwrap_or_else(|_| "500".to_owned())
        .parse()
        .expect("M10_RUNS must be an integer");
    let result_path = PathBuf::from(
        env::var("M10_RESULT_PATH").unwrap_or_else(|_| "m10-path-toctou-result.json".to_owned()),
    );
    let result_path = if result_path.is_absolute() {
        result_path
    } else {
        env::current_dir().unwrap().join(result_path)
    };

    assert!(target.is_absolute(), "M10_TARGET_BIN must be absolute");
    assert!(target.exists(), "target fixture does not exist: {}", target.display());
    assert!(runs > 0, "M10_RUNS must be positive");

    let original_cwd = env::current_dir().expect("read current directory");
    let workspace = env::temp_dir().join(format!("execsurface-m10-path-toctou-{}", process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).expect("create isolated workspace");
    fs::write(workspace.join("A"), b"A").expect("write A fixture");
    fs::write(workspace.join("B"), b"B").expect("write B fixture");
    env::set_current_dir(&workspace).expect("enter isolated workspace");

    let mut eligible_runs = 0_u64;
    let mut incomplete_runs = 0_u64;
    let mut warning_runs = 0_u64;
    let mut observer_errors = 0_u64;
    let mut fixture_errors = 0_u64;
    let mut missing_or_multiple_intents = 0_u64;
    let mut primary_mismatches = 0_u64;
    let mut secondary_mismatches = 0_u64;
    let mut truth_a = 0_u64;
    let mut truth_b = 0_u64;
    let mut intent_a = 0_u64;
    let mut intent_b = 0_u64;
    let mut fd_read_a = 0_u64;
    let mut fd_read_b = 0_u64;
    let mut mismatch_examples: Vec<Value> = Vec::new();
    let mut ineligible_examples: Vec<Value> = Vec::new();

    for iteration in 0..runs {
        let spec = CommandSpec::new(target.as_os_str().to_owned());
        let observation = match observe_command(&spec) {
            Ok(value) => value,
            Err(error) => {
                observer_errors += 1;
                if ineligible_examples.len() < 20 {
                    ineligible_examples.push(json!({
                        "iteration": iteration,
                        "reason": "observer_error",
                        "error": error.to_string(),
                    }));
                }
                continue;
            }
        };

        let truth = match observation.outcome.exit_code {
            Some(11) => Some("A"),
            Some(12) => Some("B"),
            _ => None,
        };

        if let Some(label) = truth {
            bump(&mut truth_a, &mut truth_b, label);
        } else {
            fixture_errors += 1;
        }

        let intents: Vec<&'static str> = observation
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                RawEventKind::FilePathAccess {
                    operation: FileOperation::Open,
                    path,
                    ..
                } => ab_label_from_path(path),
                _ => None,
            })
            .collect();

        let fd_reads: Vec<&'static str> = observation
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Read,
                    path,
                    ..
                } => ab_label_from_path(path),
                _ => None,
            })
            .collect();

        if !observation.complete {
            incomplete_runs += 1;
        }
        if !observation.warnings.is_empty() {
            warning_runs += 1;
        }
        if intents.len() != 1 {
            missing_or_multiple_intents += 1;
        }

        let eligible = observation.complete
            && observation.warnings.is_empty()
            && truth.is_some()
            && intents.len() == 1;

        if !eligible {
            if ineligible_examples.len() < 20 {
                ineligible_examples.push(json!({
                    "iteration": iteration,
                    "complete": observation.complete,
                    "warning_codes": observation.warnings.iter().map(|w| w.code.clone()).collect::<Vec<_>>(),
                    "exit_code": observation.outcome.exit_code,
                    "relevant_intents": intents,
                    "relevant_fd_reads": fd_reads,
                }));
            }
            continue;
        }

        eligible_runs += 1;
        let truth = truth.unwrap();
        let intent = intents[0];
        bump(&mut intent_a, &mut intent_b, intent);

        if intent != truth {
            primary_mismatches += 1;
            if mismatch_examples.len() < 20 {
                mismatch_examples.push(json!({
                    "iteration": iteration,
                    "ptrace_path_intent": intent,
                    "kernel_truth": truth,
                    "fd_read_labels": fd_reads,
                    "observation": observation,
                }));
            }
        }

        if fd_reads.len() == 1 {
            let fd_label = fd_reads[0];
            bump(&mut fd_read_a, &mut fd_read_b, fd_label);
            if fd_label != truth {
                secondary_mismatches += 1;
            }
        }
    }

    env::set_current_dir(&original_cwd).expect("restore current directory");

    let classification = if eligible_runs == 0 {
        "HARNESS_INCONCLUSIVE"
    } else if primary_mismatches > 0 {
        "PTRACE_PATH_TOCTOU_COUNTEREXAMPLE_OBSERVED"
    } else {
        "NO_MISMATCH_OBSERVED_NOT_PROOF"
    };

    let result = json!({
        "schema": "execsurface-m10-path-toctou-result-v1",
        "classification": classification,
        "source_sha": env::var("GITHUB_SHA").unwrap_or_else(|_| "local".to_owned()),
        "kernel_release": fs::read_to_string("/proc/sys/kernel/osrelease").ok().map(|s| s.trim().to_owned()),
        "requested_runs": runs,
        "eligible_runs": eligible_runs,
        "incomplete_runs": incomplete_runs,
        "warning_runs": warning_runs,
        "observer_errors": observer_errors,
        "fixture_errors": fixture_errors,
        "missing_or_multiple_intents": missing_or_multiple_intents,
        "path_intent_vs_kernel_truth_mismatch_count": primary_mismatches,
        "fd_read_vs_kernel_truth_mismatch_count": secondary_mismatches,
        "distributions": {
            "kernel_truth": {"A": truth_a, "B": truth_b},
            "ptrace_path_intent": {"A": intent_a, "B": intent_b},
            "fd_read": {"A": fd_read_a, "B": fd_read_b}
        },
        "mismatch_examples": mismatch_examples,
        "ineligible_examples": ineligible_examples,
        "interpretation_boundary": {
            "kills_universal_ptrace_path_authority_if_mismatch_observed": true,
            "does_not_prove_bpf_lsm_correctness": true,
            "does_not_authorize_runtime_change": true,
            "does_not_measure_real_world_frequency": true
        }
    });

    fs::write(
        &result_path,
        serde_json::to_vec_pretty(&result).expect("serialize result"),
    )
    .expect("write result artifact");

    println!("M10 PATH-TOCTOU classification: {classification}");
    println!("eligible runs: {eligible_runs}/{runs}");
    println!("primary mismatches: {primary_mismatches}");
    println!("secondary fd-read mismatches: {secondary_mismatches}");
    println!("result: {}", result_path.display());

    assert!(eligible_runs > 0, "HARNESS_INCONCLUSIVE: no eligible complete run");
}
