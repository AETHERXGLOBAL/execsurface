use std::env;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use execsurface_observe::{observe_command, CommandSpec};

const POLICY_PATH: &str = "execsurface-policy.json";
const WORKFLOW_PATH: &str = ".github/workflows/execsurface.yml";
const CHECKOUT_PIN: &str = "3d3c42e5aac5ba805825da76410c181273ba90b1";

pub fn run_doctor() -> ExitCode {
    println!("ExecSurface Doctor");
    println!();

    let mut ready = true;

    if env::consts::OS == "linux" {
        pass("Linux");
    } else {
        fail(
            &format!("unsupported OS: {}", env::consts::OS),
            "ExecSurface currently supports Linux only.",
        );
        ready = false;
    }

    if env::consts::ARCH == "x86_64" {
        pass("x86_64");
    } else {
        fail(
            &format!("unsupported architecture: {}", env::consts::ARCH),
            "ExecSurface currently supports x86_64 only.",
        );
        ready = false;
    }

    if env::consts::OS == "linux" && env::consts::ARCH == "x86_64" {
        match observe_command(&CommandSpec::new("/bin/true")) {
            Ok(observation)
                if observation.complete
                    && observation.outcome.exit_code == Some(0)
                    && observation.outcome.signal.is_none() =>
            {
                pass("ptrace observer available");
            }
            Ok(observation) => {
                fail(
                    "ptrace observer did not produce complete successful evidence",
                    &format!(
                        "exit_code={:?} signal={:?}; see docs/TROUBLESHOOTING.md#ptrace-restrictions",
                        observation.outcome.exit_code, observation.outcome.signal
                    ),
                );
                ready = false;
            }
            Err(error) => {
                fail(
                    "ptrace observer unavailable",
                    &format!("{error}; see docs/TROUBLESHOOTING.md#ptrace-restrictions"),
                );
                ready = false;
            }
        }
    } else {
        skip("ptrace observer check", "requires Linux x86_64");
    }

    match workspace_writable() {
        Ok(()) => pass("workspace writable"),
        Err(error) => {
            fail(
                "workspace is not writable",
                &format!("{error}; use a writable project directory for learn/init output"),
            );
            ready = false;
        }
    }

    pass(&format!("ExecSurface {}", env!("CARGO_PKG_VERSION")));

    println!();
    if ready {
        println!("Ready.");
        println!("Boundary: observed behavior is not all possible behavior.");
        ExitCode::SUCCESS
    } else {
        println!("Not ready.");
        println!("ExecSurface did not change privileges, sysctls, or security settings.");
        ExitCode::from(2)
    }
}

pub fn run_init(args: &[OsString]) -> Result<(), String> {
    let mut command: Option<String> = None;
    let mut github_actions = false;
    let mut force = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].to_string_lossy().as_ref() {
            "--command" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "init: --command requires a value".to_owned())?;
                let value = value.to_string_lossy().into_owned();
                if value.trim().is_empty() {
                    return Err("init: --command cannot be empty".to_owned());
                }
                if value.contains('\n') || value.contains('\r') {
                    return Err(
                        "init: --command must be a single-line shell command in public alpha"
                            .to_owned(),
                    );
                }
                command = Some(value);
                index += 2;
            }
            "--github-actions" => {
                github_actions = true;
                index += 1;
            }
            "--force" => {
                force = true;
                index += 1;
            }
            other => return Err(format!("init: unknown option: {other}")),
        }
    }

    let command = command.ok_or_else(|| {
        r#"init: missing --command. Example: execsurface init --command "cargo test --locked" --github-actions"#
            .to_owned()
    })?;

    let cwd = env::current_dir()
        .map_err(|error| format!("init: cannot read current directory: {error}"))?;
    let policy = cwd.join(POLICY_PATH);
    let workflow = cwd.join(WORKFLOW_PATH);

    let mut targets = vec![policy.clone()];
    if github_actions {
        targets.push(workflow.clone());
    }
    if !force {
        let existing = targets
            .iter()
            .filter(|path| path.exists())
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        if !existing.is_empty() {
            return Err(format!(
                "init: refusing to overwrite existing file(s): {}. Re-run with --force only after review.",
                existing.join(", ")
            ));
        }
    }

    write_file(
        &policy,
        "{\n  \"schema_version\": 2,\n  \"default_action\": \"review\",\n  \"rules\": []\n}\n",
        force,
    )?;

    if github_actions {
        let workflow_body = render_workflow(&command);
        write_file(&workflow, &workflow_body, force)?;
    }

    println!("ExecSurface init");
    println!("[CREATED] {}", policy.display());
    if github_actions {
        println!("[CREATED] {}", workflow.display());
    }
    println!("[NOT RUN] target command");
    println!();
    println!(
        "Next: learn the baseline explicitly using the same Bash wrapper as the GitHub Action:"
    );
    println!(
        "  execsurface learn -- /bin/bash -lc {}",
        shell_quote(&command)
    );
    println!();
    println!("Then check the same command:");
    println!(
        "  execsurface check --policy {} -- /bin/bash -lc {}",
        POLICY_PATH,
        shell_quote(&command)
    );
    println!();
    println!("Review generated files and the baseline before committing them.");
    Ok(())
}

fn render_workflow(command: &str) -> String {
    format!(
        "name: ExecSurface\n\non:\n  pull_request:\n  push:\n    branches: [main]\n\npermissions:\n  contents: read\n\njobs:\n  execsurface:\n    runs-on: ubuntu-24.04\n    steps:\n      - uses: actions/checkout@{CHECKOUT_PIN} # v7.0.1\n\n      - name: ExecSurface runtime drift\n        uses: AETHERXGLOBAL/execsurface@v0.1\n        with:\n          command: >-\n            {command}\n          baseline: execsurface.lock.json\n          policy: {POLICY_PATH}\n          fail-on-review: \"false\"\n"
    )
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn write_file(path: &Path, content: &str, force: bool) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("init: cannot create {}: {error}", parent.display()))?;
    }

    let mut options = OpenOptions::new();
    options.write(true);
    if force {
        options.create(true).truncate(true);
    } else {
        options.create_new(true);
    }

    let mut file = options
        .open(path)
        .map_err(|error| format!("init: cannot write {}: {error}", path.display()))?;
    file.write_all(content.as_bytes())
        .map_err(|error| format!("init: cannot write {}: {error}", path.display()))
}

fn workspace_writable() -> Result<(), String> {
    let cwd = env::current_dir().map_err(|error| error.to_string())?;
    for attempt in 0..16_u32 {
        let probe = cwd.join(format!(
            ".execsurface-doctor-write-{}-{attempt}",
            std::process::id()
        ));
        match OpenOptions::new().write(true).create_new(true).open(&probe) {
            Ok(mut file) => {
                file.write_all(b"execsurface-doctor\n")
                    .map_err(|error| error.to_string())?;
                drop(file);
                fs::remove_file(&probe).map_err(|error| error.to_string())?;
                return Ok(());
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("could not allocate a unique write probe".to_owned())
}

fn pass(message: &str) {
    println!("[PASS] {message}");
}

fn fail(message: &str, action: &str) {
    println!("[FAIL] {message}");
    println!("       Action: {action}");
}

fn skip(message: &str, reason: &str) {
    println!("[SKIP] {message}: {reason}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_quote_handles_single_quote_without_execution() {
        assert_eq!(shell_quote("echo 'x'"), "'echo '\"'\"'x'\"'\"''");
    }

    #[test]
    fn workflow_uses_stable_channel_and_reviewed_checkout_pin() {
        let workflow = render_workflow("cargo test --locked");
        assert!(workflow.contains("AETHERXGLOBAL/execsurface@v0.1"));
        assert!(workflow.contains(CHECKOUT_PIN));
        assert!(workflow.contains("command: >-\n            cargo test --locked"));
    }
}
