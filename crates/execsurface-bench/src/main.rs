use std::env;
use std::ffi::OsString;
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use execsurface_normalize::{canonicalize, NormalizationConfig};
use execsurface_observe::{observe_command, CommandSpec};
use serde::Serialize;

#[derive(Serialize)]
struct BenchmarkReport {
    schema_version: u32,
    repetitions: usize,
    command: CommandSummary,
    direct: TimingSummary,
    observed: TimingSummary,
    canonicalization: TimingSummary,
    overhead: OverheadSummary,
    event_count: EventCountSummary,
    limitations: Vec<&'static str>,
}

#[derive(Serialize)]
struct CommandSummary {
    executable: String,
    argument_count: usize,
}

#[derive(Serialize)]
struct TimingSummary {
    median_us: u64,
    p95_us: u64,
    min_us: u64,
    max_us: u64,
}

#[derive(Serialize)]
struct OverheadSummary {
    median_absolute_us: u64,
    median_slowdown_ratio: Option<f64>,
}

#[derive(Serialize)]
struct EventCountSummary {
    median: usize,
    min: usize,
    max: usize,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("execsurface-bench: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1).collect::<Vec<_>>();
    let mut repetitions = 15_usize;

    if args.first().is_some_and(|arg| arg == "--repetitions") {
        if args.len() < 3 {
            return Err("usage: execsurface-bench [--repetitions N] -- COMMAND [ARGS...]".to_owned());
        }
        repetitions = args[1]
            .to_string_lossy()
            .parse::<usize>()
            .map_err(|_| "--repetitions must be a positive integer".to_owned())?;
        if repetitions == 0 {
            return Err("--repetitions must be greater than zero".to_owned());
        }
        args.drain(0..2);
    }

    if args.first().is_none_or(|arg| arg != "--") {
        return Err("expected -- before command".to_owned());
    }
    args.remove(0);
    let program = args
        .first()
        .cloned()
        .ok_or_else(|| "missing command after --".to_owned())?;
    let command_args = args[1..].to_vec();

    warm_up(&program, &command_args)?;

    let mut direct = Vec::with_capacity(repetitions);
    let mut observed = Vec::with_capacity(repetitions);
    let mut canonical = Vec::with_capacity(repetitions);
    let mut event_counts = Vec::with_capacity(repetitions);

    for _ in 0..repetitions {
        let start = Instant::now();
        let status = direct_command(&program, &command_args)
            .status()
            .map_err(|error| format!("direct command failed to launch: {error}"))?;
        direct.push(micros(start.elapsed()));
        if !status.success() {
            return Err(format!("direct command exited unsuccessfully: {status}"));
        }

        let spec = CommandSpec::new(program.clone()).args(command_args.clone());
        let start = Instant::now();
        let observation =
            observe_command(&spec).map_err(|error| format!("observer failed: {error}"))?;
        observed.push(micros(start.elapsed()));
        if !observation.complete {
            return Err(format!(
                "observer returned incomplete evidence: {:?}",
                observation.warnings
            ));
        }

        event_counts.push(observation.events.len());
        let start = Instant::now();
        canonicalize(&observation, &NormalizationConfig::default())
            .map_err(|error| format!("canonicalization failed: {error}"))?;
        canonical.push(micros(start.elapsed()));
    }

    let direct_summary = timing_summary(&direct);
    let observed_summary = timing_summary(&observed);
    let median_absolute_us = observed_summary
        .median_us
        .saturating_sub(direct_summary.median_us);
    let median_slowdown_ratio = (direct_summary.median_us != 0)
        .then_some(observed_summary.median_us as f64 / direct_summary.median_us as f64);

    let report = BenchmarkReport {
        schema_version: 1,
        repetitions,
        command: CommandSummary {
            executable: program.to_string_lossy().into_owned(),
            argument_count: command_args.len(),
        },
        direct: direct_summary,
        observed: observed_summary,
        canonicalization: timing_summary(&canonical),
        overhead: OverheadSummary {
            median_absolute_us,
            median_slowdown_ratio,
        },
        event_count: event_count_summary(&event_counts),
        limitations: vec![
            "shared CI runners are noisy; values are evidence for engineering decisions, not performance guarantees",
            "microbench slowdown ratios are unstable for very short commands; absolute overhead must also be considered",
            "benchmark records executable identity and argument count, not argument values",
        ],
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&report)
            .map_err(|error| format!("cannot serialize benchmark: {error}"))?
    );
    Ok(())
}

fn warm_up(program: &OsString, args: &[OsString]) -> Result<(), String> {
    let status = direct_command(program, args)
        .status()
        .map_err(|error| format!("warm-up command failed: {error}"))?;
    if !status.success() {
        return Err(format!("warm-up command exited unsuccessfully: {status}"));
    }

    let spec = CommandSpec::new(program.clone()).args(args.to_vec());
    let observation =
        observe_command(&spec).map_err(|error| format!("observer warm-up failed: {error}"))?;
    if !observation.complete {
        return Err(format!(
            "observer warm-up returned incomplete evidence: {:?}",
            observation.warnings
        ));
    }
    Ok(())
}

fn direct_command(program: &OsString, args: &[OsString]) -> Command {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn micros(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_micros()).unwrap_or(u64::MAX)
}

fn timing_summary(values: &[u64]) -> TimingSummary {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    TimingSummary {
        median_us: percentile_u64(&sorted, 50),
        p95_us: percentile_u64(&sorted, 95),
        min_us: *sorted.first().expect("non-empty timings"),
        max_us: *sorted.last().expect("non-empty timings"),
    }
}

fn percentile_u64(sorted: &[u64], percentile: usize) -> u64 {
    let rank = (percentile * sorted.len()).div_ceil(100).saturating_sub(1);
    sorted[rank.min(sorted.len() - 1)]
}

fn event_count_summary(values: &[usize]) -> EventCountSummary {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    EventCountSummary {
        median: sorted[sorted.len() / 2],
        min: *sorted.first().expect("non-empty event counts"),
        max: *sorted.last().expect("non-empty event counts"),
    }
}
