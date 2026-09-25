use std::collections::BTreeMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::ExitCode;

use execsurface_baseline::{
    build_lock, parse_and_verify, write_lockfile, BaselinePayload, CommandIdentity,
    ObserverIdentity, PlatformIdentity, ToolIdentity, DEFAULT_LOCKFILE_NAME,
};
use execsurface_diff::{diff, CandidateSnapshot, DiffReport};
use execsurface_model::RawEventKind;
use execsurface_normalize::{canonicalize, canonicalize_executable, NormalizationConfig};
use execsurface_observe::{observe_command, CommandSpec};
use execsurface_policy::{
    builtin_review_policy, error_report, evaluate, FindingAction, Policy, Verdict, VerdictReport,
};
use execsurface_report::render_markdown;

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

fn run_observe(args: &[OsString]) -> Result<(), String> {
    let (program, command_args) = parse_observe_target(args)?;
    let spec = CommandSpec::new(program).args(command_args);
    let observation = observe_command(&spec).map_err(|error| error.to_string())?;
    let json = serde_json::to_string_pretty(&observation)
        .map_err(|error| format!("cannot serialize observation: {error}"))?;
    println!("{json}");
    Ok(())
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
            || parsed.json_output.is_some()
            || parsed.markdown_output.is_some())
    {
        return Err(
            "cannot combine --diff-only with --policy/--json-output/--markdown-output".to_owned(),
        );
    }

    let baseline_bytes = std::fs::read(&parsed.baseline).map_err(|error| {
        format!(
            "cannot read baseline {}: {error}",
            parsed.baseline.display()
        )
    })?;
    let baseline = parse_and_verify(&baseline_bytes).map_err(|error| error.to_string())?;

    let spec = CommandSpec::new(parsed.program.clone()).args(parsed.command_args.clone());
    let observation = observe_command(&spec).map_err(|error| error.to_string())?;
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

    let (policy, policy_source) = load_policy(parsed.policy.as_ref())?;
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

fn load_policy(path: Option<&PathBuf>) -> Result<(Policy, String), String> {
    match path {
        Some(path) => {
            let bytes = std::fs::read(path)
                .map_err(|error| format!("cannot read policy {}: {error}", path.display()))?;
            let policy: Policy = serde_json::from_slice(&bytes)
                .map_err(|error| format!("cannot parse policy {}: {error}", path.display()))?;
            Ok((policy, path.display().to_string()))
        }
        None => Ok((
            builtin_review_policy(),
            "builtin:review-unmatched-drift".to_owned(),
        )),
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

fn parse_observe_target(args: &[OsString]) -> Result<(OsString, Vec<OsString>), String> {
    if args.first().is_none_or(|arg| arg != "--") {
        return Err(usage("expected `--` before the target command"));
    }
    let program = args
        .get(1)
        .cloned()
        .ok_or_else(|| usage("missing target command after `--`"))?;
    Ok((program, args[2..].to_vec()))
}

struct CheckArgs {
    baseline: PathBuf,
    policy: Option<PathBuf>,
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
        "{error}\n\nusage:\n  execsurface observe -- COMMAND [ARGS...]\n  execsurface learn [OPTIONS] -- COMMAND [ARGS...]\n\nlearn options:\n  --output PATH       output lockfile (default: execsurface.lock.json)\n  --overwrite         explicitly replace an existing lockfile\n  --label LABEL       privacy-safe logical command label\n  --workspace PATH    declared workspace root\n  --home PATH         declared home root\n  --tmp PATH          declared temp root; repeatable\n  --run-tmp PATH      declared run-specific temp root\n  --cache NAME=PATH   declared named cache root; repeatable

check options:
  --baseline PATH     baseline lockfile (default: execsurface.lock.json)
  --policy PATH       explicit policy JSON (default: built-in REVIEW for unmatched drift)
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
