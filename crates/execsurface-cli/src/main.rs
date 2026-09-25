use std::collections::BTreeMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::ExitCode;

use execsurface_baseline::{
    build_lock, write_lockfile, BaselinePayload, CommandIdentity, ObserverIdentity,
    PlatformIdentity, ToolIdentity, DEFAULT_LOCKFILE_NAME,
};
use execsurface_model::RawEventKind;
use execsurface_normalize::{canonicalize, canonicalize_executable, NormalizationConfig};
use execsurface_observe::{observe_command, CommandSpec};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("execsurface: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args_os();
    let _binary = args.next();
    let subcommand = args.next().ok_or_else(|| usage("missing subcommand"))?;
    let remaining: Vec<OsString> = args.collect();

    match subcommand.to_string_lossy().as_ref() {
        "observe" => run_observe(&remaining),
        "learn" => run_learn(&remaining),
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

    let executable =
        canonicalize_executable(root_exec_path, &normalization).map_err(|error| error.to_string())?;
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
        let cwd = env::current_dir()
            .map_err(|error| format!("cannot determine current directory: {error}"))?;
        let cwd = path_string(cwd.as_os_str());

        let default_home = env::var_os("HOME").map(|value| path_string(&value));
        let home = self.home.clone().or(default_home);

        let workspace = match &self.workspace {
            Some(path) => Some(path.clone()),
            None if home.as_deref() == Some(cwd.as_str()) => None,
            None => Some(cwd),
        };

        let tmp_roots = if self.tmp_roots.is_empty() {
            let mut roots = Vec::new();
            if let Some(tmpdir) = env::var_os("TMPDIR") {
                roots.push(path_string(&tmpdir));
            }
            if !roots.iter().any(|root| root == "/tmp") {
                roots.push("/tmp".to_owned());
            }
            roots
        } else {
            self.tmp_roots.clone()
        };

        Ok(NormalizationConfig {
            workspace,
            home,
            tmp_roots,
            run_tmp: self.run_tmp.clone(),
            caches: self.caches.clone(),
        })
    }
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

fn option_value<'a>(
    args: &'a [OsString],
    index: usize,
    option: &str,
) -> Result<&'a OsStr, String> {
    args.get(index + 1)
        .map(OsString::as_os_str)
        .ok_or_else(|| usage(&format!("{option} requires a value")))
}

fn path_string(value: &OsStr) -> String {
    value.to_string_lossy().into_owned()
}

fn usage(error: &str) -> String {
    format!(
        "{error}\n\nusage:\n  execsurface observe -- COMMAND [ARGS...]\n  execsurface learn [OPTIONS] -- COMMAND [ARGS...]\n\nlearn options:\n  --output PATH       output lockfile (default: execsurface.lock.json)\n  --overwrite         explicitly replace an existing lockfile\n  --label LABEL       privacy-safe logical command label\n  --workspace PATH    declared workspace root\n  --home PATH         declared home root\n  --tmp PATH          declared temp root; repeatable\n  --run-tmp PATH      declared run-specific temp root\n  --cache NAME=PATH   declared named cache root; repeatable"
    )
}
