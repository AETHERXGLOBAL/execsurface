use std::env;
use std::ffi::OsString;
use std::process::ExitCode;

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
    if subcommand != "observe" {
        return Err(usage(
            "M1 exposes only the experimental `observe` subcommand",
        ));
    }

    let separator = args
        .next()
        .ok_or_else(|| usage("expected `--` before the target command"))?;
    if separator != "--" {
        return Err(usage("expected `--` before the target command"));
    }

    let program = args
        .next()
        .ok_or_else(|| usage("missing target command after `--`"))?;
    let remaining: Vec<OsString> = args.collect();

    let spec = CommandSpec::new(program).args(remaining);
    let observation = observe_command(&spec).map_err(|error| error.to_string())?;
    let json = serde_json::to_string_pretty(&observation)
        .map_err(|error| format!("cannot serialize observation: {error}"))?;
    println!("{json}");
    Ok(())
}

fn usage(error: &str) -> String {
    format!("{error}\nusage: execsurface observe -- COMMAND [ARGS...]")
}
