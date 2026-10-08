mod self_service;
mod typed_evidence;

use std::collections::BTreeMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

use execsurface_baseline::{
    build_lock, parse_and_verify, write_lockfile, BaselinePayload, CommandIdentity,
    ObserverIdentity, PlatformIdentity, ToolIdentity, DEFAULT_LOCKFILE_NAME,
};
use execsurface_diff::{diff, CandidateSnapshot, DiffReport};
use execsurface_model::{FileOperation, Observation, RawEventKind};
use execsurface_normalize::{canonicalize, canonicalize_executable, NormalizationConfig};
use execsurface_observe::{observe_command, observe_command_with_backend, CommandSpec};
use execsurface_policy::{
    builtin_review_policy, error_report, evaluate, FindingAction, Policy, Verdict, VerdictReport,
};
use execsurface_report::render_markdown;
use sha2::{Digest, Sha256};

const EXPERIMENTAL_LIBBPF_BACKEND_ID: &str = "linux-libbpf-metadata-experimental-v1";

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            eprintln!("ExecSurface: ERROR");
            eprintln!("execsurface: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let mut args = env::args_os();
    let _binary = args.next();
    let subcommand = args.next().ok_or_else(|| usage("missing subcommand"))?;
    let remaining: Vec<OsString> = args.collect();

    match subcommand.to_string_lossy().as_ref() {
        "--version" | "-V" | "version" => {
            println!("execsurface {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        "--help" | "-h" | "help" => {
            println!("ExecSurface {}", env!("CARGO_PKG_VERSION"));
            println!("{}", usage(""));
            Ok(ExitCode::SUCCESS)
        }
        "doctor" => Ok(self_service::run_doctor()),
        "init" => {
            self_service::run_init(&remaining)?;
            Ok(ExitCode::SUCCESS)
        }
        "observe" => {
            run_observe(&remaining)?;
            Ok(ExitCode::SUCCESS)
        }
        "learn" => {
            run_learn(&remaining)?;
            Ok(ExitCode::SUCCESS)
        }
        "check" => run_check(&remaining),
        "render-error" => {
            run_render_error(&remaining)?;
            Ok(ExitCode::SUCCESS)
        }
        _ => Err(usage("unknown subcommand")),
    }
}

#[derive(Debug)]
enum ObserveBackend {
    Ptrace,
    ExperimentalLibbpf { collector: PathBuf },
}

#[derive(Debug)]
struct ObserveArgs {
    backend: ObserveBackend,
    evidence_output: Option<PathBuf>,
    program: OsString,
    command_args: Vec<OsString>,
}

fn run_observe(args: &[OsString]) -> Result<(), String> {
    let parsed = parse_observe_args(args)?;
    match parsed.backend {
        ObserveBackend::Ptrace => {
            let spec = CommandSpec::new(parsed.program).args(parsed.command_args);
            if let Some(path) = parsed.evidence_output.as_ref() {
                let observed =
                    observe_command_with_backend(&spec).map_err(|error| error.to_string())?;
                typed_evidence::write_report(path, &observed)?;
                let json = serde_json::to_string_pretty(&observed.observation)
                    .map_err(|error| format!("cannot serialize observation: {error}"))?;
                println!("{json}");
            } else {
                // Preserve the legacy no-flag path exactly: the existing public
                // raw Observation return and stdout serialization remain in use.
                let observation = observe_command(&spec).map_err(|error| error.to_string())?;
                let json = serde_json::to_string_pretty(&observation)
                    .map_err(|error| format!("cannot serialize observation: {error}"))?;
                println!("{json}");
            }
            Ok(())
        }
        ObserveBackend::ExperimentalLibbpf { collector } => {
            run_experimental_libbpf_observe(&collector, &parsed.program, &parsed.command_args)
        }
    }
}

fn run_experimental_libbpf_observe(
    collector: &PathBuf,
    program: &OsStr,
    command_args: &[OsString],
) -> Result<(), String> {
    let report_dir = create_experimental_report_dir()?;
    let report_path = report_dir.join("report.json");

    let result = (|| {
        let status = Command::new(collector)
            .arg("--report")
            .arg(&report_path)
            .arg("--")
            .arg(program)
            .args(command_args)
            .status()
            .map_err(|error| {
                format!(
                    "cannot launch experimental libbpf collector {}: {error}; no ptrace fallback was attempted",
                    collector.display()
                )
            })?;

        if !status.success() {
            return Err(format!(
                "experimental libbpf collector {} failed with status {status}; no ptrace fallback was attempted",
                collector.display()
            ));
        }

        let report_bytes = fs::read(&report_path).map_err(|error| {
            format!(
                "experimental libbpf collector completed but report {} is unavailable: {error}",
                report_path.display()
            )
        })?;
        let report: serde_json::Value = serde_json::from_slice(&report_bytes).map_err(|error| {
            format!(
                "experimental libbpf collector report {} is invalid JSON: {error}",
                report_path.display()
            )
        })?;
        validate_experimental_libbpf_report(&report)?;

        let json = serde_json::to_string_pretty(&report)
            .map_err(|error| format!("cannot serialize experimental libbpf report: {error}"))?;
        println!("{json}");
        Ok(())
    })();

    let _ = fs::remove_dir_all(&report_dir);
    result
}

fn create_experimental_report_dir() -> Result<PathBuf, String> {
    let epoch_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("cannot derive temporary report identity: {error}"))?
        .as_nanos();
    let base = env::temp_dir();

    for attempt in 0..16u8 {
        let path = base.join(format!(
            "execsurface-ebpf-{}-{epoch_nanos}-{attempt}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "cannot create experimental libbpf report directory {}: {error}",
                    path.display()
                ))
            }
        }
    }

    Err("cannot allocate a unique experimental libbpf report directory".to_owned())
}

fn validate_experimental_libbpf_report(report: &serde_json::Value) -> Result<(), String> {
    if report
        .get("protocol_version")
        .and_then(|value| value.as_u64())
        != Some(1)
    {
        return Err("experimental libbpf report has unsupported protocol_version".to_owned());
    }

    let backend_id = report
        .pointer("/backend/id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "experimental libbpf report is missing backend.id".to_owned())?;
    if backend_id != EXPERIMENTAL_LIBBPF_BACKEND_ID {
        return Err(format!(
            "experimental libbpf backend identity mismatch: {backend_id}"
        ));
    }

    if report
        .get("observation_complete")
        .and_then(|value| value.as_bool())
        != Some(false)
    {
        return Err(
            "experimental libbpf report attempted to claim complete/PASS-eligible observation before the parity gate"
                .to_owned(),
        );
    }

    let completeness = report
        .get("completeness")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "experimental libbpf report is missing completeness".to_owned())?;
    if !matches!(
        completeness,
        "incomplete_capability" | "incomplete_loss" | "incomplete_limit"
    ) {
        return Err(format!(
            "experimental libbpf report has unauthorized completeness state: {completeness}"
        ));
    }

    if !report
        .pointer("/backend/capabilities")
        .is_some_and(|value| value.is_array())
        || !report
            .pointer("/backend/unsupported_capabilities")
            .is_some_and(|value| value.is_array())
    {
        return Err(
            "experimental libbpf report is missing explicit supported/unsupported capability sets"
                .to_owned(),
        );
    }

    Ok(())
}

fn parse_observe_args(args: &[OsString]) -> Result<ObserveArgs, String> {
    let mut backend = "ptrace".to_owned();
    let mut collector = None;
    let mut evidence_output = None;
    let mut index = 0usize;

    while index < args.len() {
        if args[index] == "--" {
            let program = args
                .get(index + 1)
                .cloned()
                .ok_or_else(|| usage("missing target command after `--`"))?;
            let command_args = args[index + 2..].to_vec();
            let backend = match backend.as_str() {
                "ptrace" => {
                    if collector.is_some() {
                        return Err(usage(
                            "--collector is valid only with --backend experimental-libbpf",
                        ));
                    }
                    ObserveBackend::Ptrace
                }
                "experimental-libbpf" => {
                    if evidence_output.is_some() {
                        return Err(usage(
                            "--evidence-output is currently supported only with the ptrace backend",
                        ));
                    }
                    ObserveBackend::ExperimentalLibbpf {
                        collector: collector.ok_or_else(|| {
                            usage("--backend experimental-libbpf requires --collector PATH")
                        })?,
                    }
                }
                other => return Err(usage(&format!("unknown observe backend: {other}"))),
            };
            return Ok(ObserveArgs {
                backend,
                evidence_output,
                program,
                command_args,
            });
        }

        match args[index].to_string_lossy().as_ref() {
            "--backend" => {
                backend = path_string(option_value(args, index, "--backend")?);
                index += 2;
            }
            "--collector" => {
                collector = Some(PathBuf::from(option_value(args, index, "--collector")?));
                index += 2;
            }
            "--evidence-output" => {
                if evidence_output.is_some() {
                    return Err(usage("duplicate --evidence-output"));
                }
                evidence_output = Some(PathBuf::from(option_value(
                    args,
                    index,
                    "--evidence-output",
                )?));
                index += 2;
            }
            other => return Err(usage(&format!("unknown observe option: {other}"))),
        }
    }

    Err(usage("expected `--` before the target command"))
}

fn run_learn(args: &[OsString]) -> Result<(), String> {
    let parsed = parse_learn_args(args)?;
    let spec = CommandSpec::new(parsed.program.clone()).args(parsed.command_args.clone());
    let observation = observe_command(&spec).map_err(|error| error.to_string())?;

    if observation.outcome.signal.is_some() || observation.outcome.exit_code != Some(0) {
        return Err(format!(
            "target command did not complete successfully (exit_code={:?}, signal={:?}); no baseline written",
            observation.outcome.exit_code, observation.outcome.signal
        ));
    }

    let normalization = parsed.normalization_config()?;
    let canonical_surface =
        canonicalize(&observation, &normalization).map_err(|error| error.to_string())?;

    let root_exec_path = observation
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            RawEventKind::ProcessExec { path } => Some((event.sequence, path)),
            _ => None,
        })
        .min_by_key(|(sequence, _)| *sequence)
        .map(|(_, path)| path)
        .ok_or_else(|| "observer produced no confirmed root executable event".to_owned())?;

    let executable = canonicalize_executable(root_exec_path, &normalization)
        .map_err(|error| error.to_string())?;
    let argument_count = u32::try_from(parsed.command_args.len())
        .map_err(|_| "target argument count exceeds lockfile format".to_owned())?;

    let payload = BaselinePayload::new(
        ToolIdentity {
            name: "execsurface".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        },
        CommandIdentity {
            executable,
            argument_count,
            label: parsed.label,
        },
        PlatformIdentity {
            os: observation.backend.platform.clone(),
            architecture: observation.backend.architecture.clone(),
        },
        ObserverIdentity {
            name: observation.backend.name.clone(),
            capabilities: observation.backend.capabilities.clone(),
            limitations: observation.backend.limitations.clone(),
        },
        canonical_surface,
    );

    let lock = build_lock(payload).map_err(|error| error.to_string())?;
    write_lockfile(&parsed.output, &lock, parsed.overwrite).map_err(|error| error.to_string())?;

    println!("ExecSurface baseline learned");
    println!("digest: {}", lock.baseline_digest);
    println!("effects: {}", lock.payload.canonical_surface.effects.len());
    println!("lockfile: {}", parsed.output.display());
    Ok(())
}

fn run_check(args: &[OsString]) -> Result<ExitCode, String> {
    let parsed = parse_check_args(args)?;
    if parsed.diff_only
        && (parsed.policy.is_some()
            || parsed.expected_policy_sha256.is_some()
            || parsed.json_output.is_some()
            || parsed.markdown_output.is_some())
    {
        return Err(
            "cannot combine --diff-only with --policy/--expect-policy-sha256/--json-output/--markdown-output"
                .to_owned(),
        );
    }

    let verdict_output_guard = validate_verdict_output_preflight(&parsed)?;

    let baseline_bytes = std::fs::read(&parsed.baseline).map_err(|error| {
        format!(
            "cannot read baseline {}: {error}",
            parsed.baseline.display()
        )
    })?;
    let baseline = parse_and_verify(&baseline_bytes).map_err(|error| error.to_string())?;
    verify_custody_pin(
        "baseline",
        parsed.expected_baseline_digest.as_deref(),
        &baseline.baseline_digest,
    )?;

    // Policy custody is established before the target executes. A parseable
    // policy from the checkout is not authoritative merely because it is valid JSON.
    let policy = if parsed.diff_only {
        None
    } else {
        Some(load_policy(
            parsed.policy.as_ref(),
            parsed.expected_policy_sha256.as_deref(),
        )?)
    };

    let spec = CommandSpec::new(parsed.program.clone()).args(parsed.command_args.clone());
    let observation = observe_command(&spec).map_err(|error| error.to_string())?;
    validate_verdict_output_postflight(&parsed, &observation, &verdict_output_guard)?;
    let normalization = parsed.normalization_config()?;
    let canonical_surface =
        canonicalize(&observation, &normalization).map_err(|error| error.to_string())?;

    let root_exec_path = observation
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            RawEventKind::ProcessExec { path } => Some((event.sequence, path)),
            _ => None,
        })
        .min_by_key(|(sequence, _)| *sequence)
        .map(|(_, path)| path)
        .ok_or_else(|| "observer produced no confirmed root executable event".to_owned())?;

    let executable = canonicalize_executable(root_exec_path, &normalization)
        .map_err(|error| error.to_string())?;
    let argument_count = u32::try_from(parsed.command_args.len())
        .map_err(|_| "target argument count exceeds diff format".to_owned())?;

    let candidate = CandidateSnapshot {
        command: CommandIdentity {
            executable,
            argument_count,
            label: baseline.payload.command.label.clone(),
        },
        platform: PlatformIdentity {
            os: observation.backend.platform.clone(),
            architecture: observation.backend.architecture.clone(),
        },
        observer: ObserverIdentity {
            name: observation.backend.name.clone(),
            capabilities: observation.backend.capabilities.clone(),
            limitations: observation.backend.limitations.clone(),
        },
        canonical_surface,
        target_exit_code: observation.outcome.exit_code,
        target_signal: observation.outcome.signal,
    };

    let report = diff(&baseline, &candidate).map_err(|error| error.to_string())?;

    if parsed.diff_only {
        if parsed.json {
            let json = serde_json::to_string_pretty(&report)
                .map_err(|error| format!("cannot serialize diff report: {error}"))?;
            println!("{json}");
        } else {
            print_diff_report(&report)?;
        }
        return Ok(ExitCode::SUCCESS);
    }

    let (policy, policy_source) = policy.expect("non-diff check loads policy before execution");
    let verdict_report =
        evaluate(&report, &policy, policy_source).map_err(|error| error.to_string())?;

    write_verdict_outputs(
        &verdict_report,
        parsed.json_output.as_ref(),
        parsed.markdown_output.as_ref(),
    )?;

    if parsed.json {
        let json = serde_json::to_string_pretty(&verdict_report)
            .map_err(|error| format!("cannot serialize verdict report: {error}"))?;
        println!("{json}");
    } else {
        print_verdict_report(&verdict_report)?;
    }

    Ok(exit_code_for_verdict(verdict_report.verdict))
}

fn lexical_absolute_path(path: &Path) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|error| format!("cannot resolve current directory: {error}"))?
            .join(path)
    };

    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::Normal(value) => normalized.push(value),
        }
    }
    Ok(normalized)
}

fn artifact_path_identity(path: &Path) -> Result<PathBuf, String> {
    if path.exists() {
        return fs::canonicalize(path)
            .map_err(|error| format!("cannot resolve path {}: {error}", path.display()));
    }

    let absolute = lexical_absolute_path(path)?;
    let parent = absolute.parent().unwrap_or_else(|| Path::new("/"));
    let resolved_parent = fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
    let name = absolute
        .file_name()
        .ok_or_else(|| format!("artifact path has no file name: {}", path.display()))?;
    Ok(resolved_parent.join(name))
}

#[cfg(unix)]
fn existing_object_identity(path: &Path) -> Result<Option<(u64, u64)>, String> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(Some((metadata.dev(), metadata.ino()))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!(
            "cannot inspect artifact identity {}: {error}",
            path.display()
        )),
    }
}

#[cfg(not(unix))]
fn existing_object_identity(_path: &Path) -> Result<Option<(u64, u64)>, String> {
    Ok(None)
}

fn artifact_paths_alias(left: &Path, right: &Path) -> Result<bool, String> {
    if let (Some(left_id), Some(right_id)) = (
        existing_object_identity(left)?,
        existing_object_identity(right)?,
    ) {
        if left_id == right_id {
            return Ok(true);
        }
    }

    Ok(artifact_path_identity(left)? == artifact_path_identity(right)?)
}

fn verdict_output_paths(parsed: &CheckArgs) -> Vec<(&'static str, &Path)> {
    let mut outputs = Vec::new();
    if let Some(path) = parsed.json_output.as_deref() {
        outputs.push(("JSON", path));
    }
    if let Some(path) = parsed.markdown_output.as_deref() {
        outputs.push(("Markdown", path));
    }
    outputs
}

#[derive(Debug, Clone)]
struct ProtectedArtifactSnapshot {
    kind: &'static str,
    path: PathBuf,
    object_identity: Option<(u64, u64)>,
}

#[derive(Debug, Clone)]
struct VerdictOutputGuard {
    protected: Vec<ProtectedArtifactSnapshot>,
}

fn validate_verdict_output_preflight(parsed: &CheckArgs) -> Result<VerdictOutputGuard, String> {
    let outputs = verdict_output_paths(parsed);
    let mut protected = vec![("baseline", parsed.baseline.as_path())];
    if let Some(policy) = parsed.policy.as_deref() {
        protected.push(("policy", policy));
    }

    for (output_kind, output) in &outputs {
        for (input_kind, input) in &protected {
            if artifact_paths_alias(output, input)? {
                return Err(format!(
                    "{output_kind} verdict output aliases protected {input_kind} artifact: {}",
                    output.display()
                ));
            }
        }
    }

    if outputs.len() == 2 && artifact_paths_alias(outputs[0].1, outputs[1].1)? {
        return Err(format!(
            "JSON and Markdown verdict outputs alias the same artifact: {}",
            outputs[0].1.display()
        ));
    }

    let protected = protected
        .into_iter()
        .map(|(kind, path)| {
            Ok(ProtectedArtifactSnapshot {
                kind,
                path: path.to_path_buf(),
                object_identity: existing_object_identity(path)?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(VerdictOutputGuard { protected })
}

fn workload_mutated_paths_after_renames(observation: &Observation) -> Result<Vec<PathBuf>, String> {
    let mut mutated = Vec::<PathBuf>::new();
    let mut events = observation.events.iter().collect::<Vec<_>>();
    events.sort_by_key(|event| event.sequence);

    for event in events {
        match &event.kind {
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Write,
                path,
                ..
            } if path.starts_with('/') => {
                let path = lexical_absolute_path(Path::new(path))?;
                if !mutated.iter().any(|existing| existing == &path) {
                    mutated.push(path);
                }
            }
            RawEventKind::FilePathAccess {
                operation,
                path,
                flags,
            } if path.starts_with('/') => {
                let mutates = match operation {
                    FileOperation::Create => true,
                    FileOperation::Open => flags
                        .is_some_and(|flags| flags & libc::O_TRUNC as u64 != 0),
                    _ => false,
                };
                if mutates {
                    let path = lexical_absolute_path(Path::new(path))?;
                    if !mutated.iter().any(|existing| existing == &path) {
                        mutated.push(path);
                    }
                }
            }
            RawEventKind::FileOpenAt2 { path, flags, .. }
                if path.starts_with('/') && flags & libc::O_TRUNC as u64 != 0 =>
            {
                let path = lexical_absolute_path(Path::new(path))?;
                if !mutated.iter().any(|existing| existing == &path) {
                    mutated.push(path);
                }
            }
            RawEventKind::FileRename { from, to }
                if from.starts_with('/') && to.starts_with('/') =>
            {
                let from = lexical_absolute_path(Path::new(from))?;
                let to = lexical_absolute_path(Path::new(to))?;
                if from == to {
                    continue;
                }

                let mut next = Vec::with_capacity(mutated.len());
                for path in mutated.drain(..) {
                    if let Ok(suffix) = path.strip_prefix(&from) {
                        if suffix.as_os_str().is_empty() {
                            next.push(to.clone());
                        } else {
                            next.push(to.join(suffix));
                        }
                    } else if path.strip_prefix(&to).is_ok() {
                        // A successful rename replaces the destination object/tree.
                        // Any earlier write lineage that belonged only to the replaced
                        // destination no longer identifies the final object there.
                    } else {
                        next.push(path);
                    }
                }
                next.sort();
                next.dedup();
                mutated = next;
            }
            _ => {}
        }
    }

    Ok(mutated)
}

fn validate_verdict_output_postflight(
    parsed: &CheckArgs,
    observation: &Observation,
    guard: &VerdictOutputGuard,
) -> Result<(), String> {
    // Re-run path-based checks after the target because the target can rebind
    // an output path after initial preflight.
    let _ = validate_verdict_output_preflight(parsed)?;
    let workload_mutated_paths = workload_mutated_paths_after_renames(observation)?;

    for (output_kind, output) in verdict_output_paths(parsed) {
        let output_object_identity = existing_object_identity(output)?;

        for protected in &guard.protected {
            if output_object_identity.is_some()
                && output_object_identity == protected.object_identity
            {
                return Err(format!(
                    "{output_kind} verdict output aliases preflight-verified {} object after target execution: {} (original path {})",
                    protected.kind,
                    output.display(),
                    protected.path.display()
                ));
            }
        }

        for path in &workload_mutated_paths {
            if artifact_paths_alias(output, path)? {
                return Err(format!(
                    "{output_kind} verdict output overlaps workload-mutated artifact after observed path transitions: {}",
                    output.display()
                ));
            }
        }
    }

    Ok(())
}

fn write_verdict_outputs(
    report: &VerdictReport,
    json_output: Option<&PathBuf>,
    markdown_output: Option<&PathBuf>,
) -> Result<(), String> {
    if let Some(path) = json_output {
        let mut bytes = serde_json::to_vec_pretty(report)
            .map_err(|error| format!("cannot serialize verdict report: {error}"))?;
        bytes.push(b'\n');
        std::fs::write(path, bytes)
            .map_err(|error| format!("cannot write verdict JSON {}: {error}", path.display()))?;
    }

    if let Some(path) = markdown_output {
        std::fs::write(path, render_markdown(report)).map_err(|error| {
            format!("cannot write Markdown summary {}: {error}", path.display())
        })?;
    }

    Ok(())
}

fn run_render_error(args: &[OsString]) -> Result<(), String> {
    let mut message = None;
    let mut json_output = None;
    let mut markdown_output = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].to_string_lossy().as_ref() {
            "--message" => {
                message = Some(path_string(option_value(args, index, "--message")?));
                index += 2;
            }
            "--json-output" => {
                json_output = Some(PathBuf::from(option_value(args, index, "--json-output")?));
                index += 2;
            }
            "--markdown-output" => {
                markdown_output = Some(PathBuf::from(option_value(
                    args,
                    index,
                    "--markdown-output",
                )?));
                index += 2;
            }
            other => return Err(usage(&format!("unknown render-error option: {other}"))),
        }
    }

    let message = message.ok_or_else(|| usage("render-error requires --message"))?;
    let json_output = json_output.ok_or_else(|| usage("render-error requires --json-output"))?;
    let markdown_output =
        markdown_output.ok_or_else(|| usage("render-error requires --markdown-output"))?;
    let report = error_report(message);
    write_verdict_outputs(&report, Some(&json_output), Some(&markdown_output))
}

fn validate_custody_digest(value: &str, kind: &str) -> Result<(), String> {
    let valid = value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    });
    if valid {
        Ok(())
    } else {
        Err(format!(
            "invalid expected {kind} digest: expected sha256:<64 lowercase hex>"
        ))
    }
}

fn verify_custody_pin(kind: &str, expected: Option<&str>, actual: &str) -> Result<(), String> {
    let Some(expected) = expected else {
        return Ok(());
    };
    validate_custody_digest(expected, kind)?;
    if expected == actual {
        Ok(())
    } else {
        Err(format!(
            "{kind} custody mismatch: expected {expected}, loaded {actual}"
        ))
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    format!("sha256:{hex}")
}

fn load_policy(
    path: Option<&PathBuf>,
    expected_sha256: Option<&str>,
) -> Result<(Policy, String), String> {
    match path {
        Some(path) => {
            let bytes = std::fs::read(path)
                .map_err(|error| format!("cannot read policy {}: {error}", path.display()))?;
            let actual = sha256_bytes(&bytes);
            verify_custody_pin("policy", expected_sha256, &actual)?;
            let policy: Policy = serde_json::from_slice(&bytes)
                .map_err(|error| format!("cannot parse policy {}: {error}", path.display()))?;
            Ok((policy, path.display().to_string()))
        }
        None => {
            if expected_sha256.is_some() {
                return Err(
                    "--expect-policy-sha256 requires --policy; the built-in policy is not a checkout artifact"
                        .to_owned(),
                );
            }
            Ok((
                builtin_review_policy(),
                "builtin:review-unmatched-drift".to_owned(),
            ))
        }
    }
}

fn print_verdict_report(report: &VerdictReport) -> Result<(), String> {
    println!("ExecSurface: {}", verdict_name(report.verdict));
    if let Some(digest) = &report.baseline_digest {
        println!("baseline: {digest}");
    }
    if let Some(target) = &report.target {
        println!(
            "target: exit_code={:?} signal={:?}",
            target.exit_code, target.signal
        );
    }
    let allow = report
        .findings
        .iter()
        .filter(|finding| finding.action == FindingAction::Allow)
        .count();
    let review = report
        .findings
        .iter()
        .filter(|finding| finding.action == FindingAction::Review)
        .count();
    let block = report
        .findings
        .iter()
        .filter(|finding| finding.action == FindingAction::Block)
        .count();
    println!(
        "findings: total={} allow={} review={} block={}",
        report.findings.len(),
        allow,
        review,
        block
    );
    for finding in &report.findings {
        println!(
            "{:?} {:?} {:?} rules={}",
            finding.action,
            finding.change,
            finding.effect_kind,
            if finding.matched_rules.is_empty() {
                "<default>".to_owned()
            } else {
                finding.matched_rules.join(",")
            }
        );
    }
    Ok(())
}

fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pass => "PASS",
        Verdict::Review => "REVIEW",
        Verdict::Block => "BLOCK",
        Verdict::Error => "ERROR",
    }
}

fn exit_code_for_verdict(verdict: Verdict) -> ExitCode {
    match verdict {
        Verdict::Pass => ExitCode::SUCCESS,
        Verdict::Error => ExitCode::from(2),
        Verdict::Review => ExitCode::from(10),
        Verdict::Block => ExitCode::from(20),
    }
}

fn print_diff_report(report: &DiffReport) -> Result<(), String> {
    println!("ExecSurface diff");
    println!("baseline: {}", report.baseline_digest);
    println!(
        "target: exit_code={:?} signal={:?}",
        report.target.exit_code, report.target.signal
    );
    println!(
        "drift: +{} -{} ~{}",
        report.added.len(),
        report.removed.len(),
        report.changed.len()
    );

    for effect in &report.added {
        println!(
            "+ {}",
            serde_json::to_string(effect)
                .map_err(|error| format!("cannot serialize added effect: {error}"))?
        );
    }
    for effect in &report.removed {
        println!(
            "- {}",
            serde_json::to_string(effect)
                .map_err(|error| format!("cannot serialize removed effect: {error}"))?
        );
    }
    for effect in &report.changed {
        println!(
            "~ {}",
            serde_json::to_string(effect)
                .map_err(|error| format!("cannot serialize changed effect: {error}"))?
        );
    }
    Ok(())
}

struct CheckArgs {
    baseline: PathBuf,
    policy: Option<PathBuf>,
    expected_baseline_digest: Option<String>,
    expected_policy_sha256: Option<String>,
    diff_only: bool,
    json: bool,
    json_output: Option<PathBuf>,
    markdown_output: Option<PathBuf>,
    workspace: Option<String>,
    home: Option<String>,
    tmp_roots: Vec<String>,
    run_tmp: Option<String>,
    caches: BTreeMap<String, String>,
    program: OsString,
    command_args: Vec<OsString>,
}

impl CheckArgs {
    fn normalization_config(&self) -> Result<NormalizationConfig, String> {
        normalization_config(
            self.workspace.clone(),
            self.home.clone(),
            self.tmp_roots.clone(),
            self.run_tmp.clone(),
            self.caches.clone(),
        )
    }
}

fn parse_check_args(args: &[OsString]) -> Result<CheckArgs, String> {
    let mut baseline = PathBuf::from(DEFAULT_LOCKFILE_NAME);
    let mut policy = None;
    let mut expected_baseline_digest = None;
    let mut expected_policy_sha256 = None;
    let mut diff_only = false;
    let mut json = false;
    let mut json_output = None;
    let mut markdown_output = None;
    let mut workspace = None;
    let mut home = None;
    let mut tmp_roots = Vec::new();
    let mut run_tmp = None;
    let mut caches = BTreeMap::new();

    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if arg == "--" {
            let program = args
                .get(index + 1)
                .cloned()
                .ok_or_else(|| usage("missing target command after `--`"))?;
            return Ok(CheckArgs {
                baseline,
                policy,
                expected_baseline_digest,
                expected_policy_sha256,
                diff_only,
                json,
                json_output,
                markdown_output,
                workspace,
                home,
                tmp_roots,
                run_tmp,
                caches,
                program,
                command_args: args[index + 2..].to_vec(),
            });
        }

        match arg.to_string_lossy().as_ref() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--diff-only" => {
                diff_only = true;
                index += 1;
            }
            "--json-output" => {
                json_output = Some(PathBuf::from(option_value(args, index, "--json-output")?));
                index += 2;
            }
            "--markdown-output" => {
                markdown_output = Some(PathBuf::from(option_value(
                    args,
                    index,
                    "--markdown-output",
                )?));
                index += 2;
            }
            "--policy" => {
                policy = Some(PathBuf::from(option_value(args, index, "--policy")?));
                index += 2;
            }
            "--baseline" => {
                baseline = PathBuf::from(option_value(args, index, "--baseline")?);
                index += 2;
            }
            "--expect-baseline-digest" => {
                if expected_baseline_digest.is_some() {
                    return Err(usage("duplicate --expect-baseline-digest"));
                }
                expected_baseline_digest = Some(path_string(option_value(
                    args,
                    index,
                    "--expect-baseline-digest",
                )?));
                index += 2;
            }
            "--expect-policy-sha256" => {
                if expected_policy_sha256.is_some() {
                    return Err(usage("duplicate --expect-policy-sha256"));
                }
                expected_policy_sha256 = Some(path_string(option_value(
                    args,
                    index,
                    "--expect-policy-sha256",
                )?));
                index += 2;
            }
            "--workspace" => {
                workspace = Some(path_string(option_value(args, index, "--workspace")?));
                index += 2;
            }
            "--home" => {
                home = Some(path_string(option_value(args, index, "--home")?));
                index += 2;
            }
            "--tmp" => {
                tmp_roots.push(path_string(option_value(args, index, "--tmp")?));
                index += 2;
            }
            "--run-tmp" => {
                run_tmp = Some(path_string(option_value(args, index, "--run-tmp")?));
                index += 2;
            }
            "--cache" => {
                let value = path_string(option_value(args, index, "--cache")?);
                let (name, path) = value
                    .split_once('=')
                    .ok_or_else(|| "--cache expects NAME=PATH".to_owned())?;
                if caches.insert(name.to_owned(), path.to_owned()).is_some() {
                    return Err(format!("duplicate cache name: {name}"));
                }
                index += 2;
            }
            other => return Err(usage(&format!("unknown check option: {other}"))),
        }
    }

    Err(usage("expected `--` before the target command"))
}

struct LearnArgs {
    output: PathBuf,
    overwrite: bool,
    label: Option<String>,
    workspace: Option<String>,
    home: Option<String>,
    tmp_roots: Vec<String>,
    run_tmp: Option<String>,
    caches: BTreeMap<String, String>,
    program: OsString,
    command_args: Vec<OsString>,
}

impl LearnArgs {
    fn normalization_config(&self) -> Result<NormalizationConfig, String> {
        normalization_config(
            self.workspace.clone(),
            self.home.clone(),
            self.tmp_roots.clone(),
            self.run_tmp.clone(),
            self.caches.clone(),
        )
    }
}

fn normalization_config(
    workspace: Option<String>,
    home: Option<String>,
    tmp_roots: Vec<String>,
    run_tmp: Option<String>,
    caches: BTreeMap<String, String>,
) -> Result<NormalizationConfig, String> {
    let cwd = env::current_dir()
        .map_err(|error| format!("cannot determine current directory: {error}"))?;
    let cwd = path_string(cwd.as_os_str());

    let default_home = env::var_os("HOME").map(|value| path_string(&value));
    let home = home.or(default_home);

    let workspace = match workspace {
        Some(path) => Some(path),
        None if home.as_deref() == Some(cwd.as_str()) => None,
        None => Some(cwd),
    };

    let tmp_roots = if tmp_roots.is_empty() {
        let mut roots = Vec::new();
        if let Some(tmpdir) = env::var_os("TMPDIR") {
            roots.push(path_string(&tmpdir));
        }
        if !roots.iter().any(|root| root == "/tmp") {
            roots.push("/tmp".to_owned());
        }
        roots
    } else {
        tmp_roots
    };

    Ok(NormalizationConfig {
        workspace,
        home,
        tmp_roots,
        run_tmp,
        caches,
    })
}

fn parse_learn_args(args: &[OsString]) -> Result<LearnArgs, String> {
    let mut output = PathBuf::from(DEFAULT_LOCKFILE_NAME);
    let mut overwrite = false;
    let mut label = None;
    let mut workspace = None;
    let mut home = None;
    let mut tmp_roots = Vec::new();
    let mut run_tmp = None;
    let mut caches = BTreeMap::new();

    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if arg == "--" {
            let program = args
                .get(index + 1)
                .cloned()
                .ok_or_else(|| usage("missing target command after `--`"))?;
            return Ok(LearnArgs {
                output,
                overwrite,
                label,
                workspace,
                home,
                tmp_roots,
                run_tmp,
                caches,
                program,
                command_args: args[index + 2..].to_vec(),
            });
        }

        match arg.to_string_lossy().as_ref() {
            "--overwrite" => {
                overwrite = true;
                index += 1;
            }
            "--output" => {
                let value = option_value(args, index, "--output")?;
                output = PathBuf::from(value);
                index += 2;
            }
            "--label" => {
                let value = option_value(args, index, "--label")?;
                label = Some(path_string(value));
                index += 2;
            }
            "--workspace" => {
                let value = option_value(args, index, "--workspace")?;
                workspace = Some(path_string(value));
                index += 2;
            }
            "--home" => {
                let value = option_value(args, index, "--home")?;
                home = Some(path_string(value));
                index += 2;
            }
            "--tmp" => {
                let value = option_value(args, index, "--tmp")?;
                tmp_roots.push(path_string(value));
                index += 2;
            }
            "--run-tmp" => {
                let value = option_value(args, index, "--run-tmp")?;
                run_tmp = Some(path_string(value));
                index += 2;
            }
            "--cache" => {
                let value = option_value(args, index, "--cache")?;
                let value = path_string(value);
                let (name, path) = value
                    .split_once('=')
                    .ok_or_else(|| "--cache expects NAME=PATH".to_owned())?;
                if caches.insert(name.to_owned(), path.to_owned()).is_some() {
                    return Err(format!("duplicate cache name: {name}"));
                }
                index += 2;
            }
            other => return Err(usage(&format!("unknown learn option: {other}"))),
        }
    }

    Err(usage("expected `--` before the target command"))
}

fn option_value<'a>(args: &'a [OsString], index: usize, option: &str) -> Result<&'a OsStr, String> {
    args.get(index + 1)
        .map(OsString::as_os_str)
        .ok_or_else(|| usage(&format!("{option} requires a value")))
}

fn path_string(value: &OsStr) -> String {
    value.to_string_lossy().into_owned()
}

fn usage(error: &str) -> String {
    format!(
        "{error}\n\nusage:\n  execsurface observe -- COMMAND [ARGS...]\n  execsurface observe --backend experimental-libbpf --collector PATH -- COMMAND [ARGS...]\n  execsurface learn [OPTIONS] -- COMMAND [ARGS...]\n\nobserve options:\n  --backend NAME      ptrace (default) or experimental-libbpf\n  --collector PATH    explicit companion path; required for experimental-libbpf\n  --evidence-output PATH\n                      experimental typed evidence file for ptrace; raw stdout is unchanged\n\nlearn options:\n  --output PATH       output lockfile (default: execsurface.lock.json)\n  --overwrite         explicitly replace an existing lockfile\n  --label LABEL       privacy-safe logical command label\n  --workspace PATH    declared workspace root\n  --home PATH         declared home root\n  --tmp PATH          declared temp root; repeatable\n  --run-tmp PATH      declared run-specific temp root\n  --cache NAME=PATH   declared named cache root; repeatable

check options:
  --baseline PATH     baseline lockfile (default: execsurface.lock.json)
  --policy PATH       explicit policy JSON (default: built-in REVIEW for unmatched drift)
  --expect-baseline-digest DIGEST
                      require the verified baseline semantic identity
  --expect-policy-sha256 DIGEST
                      require exact-byte SHA-256 identity for --policy
  --diff-only         emit raw M4 diff and do not evaluate policy
  --json              emit machine-readable JSON to stdout
  --json-output PATH  write verdict JSON directly to a file
  --markdown-output PATH
                      write Markdown verdict summary directly to a file
  --workspace PATH    declared workspace root
  --home PATH         declared home root
  --tmp PATH          declared temp root; repeatable
  --run-tmp PATH      declared run-specific temp root
  --cache NAME=PATH   declared named cache root; repeatable"
    )
}

#[cfg(test)]
mod p8_a3_observe_cli_tests {
    use super::*;

    #[test]
    fn legacy_observe_parse_has_no_evidence_output() {
        let args = vec![OsString::from("--"), OsString::from("/bin/true")];
        let parsed = parse_observe_args(&args).expect("legacy parse");
        assert!(parsed.evidence_output.is_none());
        assert!(matches!(parsed.backend, ObserveBackend::Ptrace));
    }

    #[test]
    fn evidence_output_is_explicit_and_ptrace_scoped() {
        let args = vec![
            OsString::from("--evidence-output"),
            OsString::from("evidence.json"),
            OsString::from("--"),
            OsString::from("/bin/true"),
        ];
        let parsed = parse_observe_args(&args).expect("evidence parse");
        assert_eq!(
            parsed.evidence_output.as_deref(),
            Some(std::path::Path::new("evidence.json"))
        );
        assert!(matches!(parsed.backend, ObserveBackend::Ptrace));
    }

    #[test]
    fn evidence_output_rejects_experimental_libbpf_backend() {
        let args = vec![
            OsString::from("--backend"),
            OsString::from("experimental-libbpf"),
            OsString::from("--collector"),
            OsString::from("/tmp/collector"),
            OsString::from("--evidence-output"),
            OsString::from("evidence.json"),
            OsString::from("--"),
            OsString::from("/bin/true"),
        ];
        let error = parse_observe_args(&args).expect_err("must reject");
        assert!(
            error.contains("--evidence-output is currently supported only with the ptrace backend")
        );
    }

    #[test]
    fn duplicate_evidence_output_is_rejected() {
        let args = vec![
            OsString::from("--evidence-output"),
            OsString::from("one.json"),
            OsString::from("--evidence-output"),
            OsString::from("two.json"),
            OsString::from("--"),
            OsString::from("/bin/true"),
        ];
        let error = parse_observe_args(&args).expect_err("must reject duplicate");
        assert!(error.contains("duplicate --evidence-output"));
    }
}
