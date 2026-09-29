use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

use execsurface_p3_variance_analyzer::{analyze, serialize_report, TrustedLearningManifest};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let manifest_path = args.next().ok_or_else(|| {
        "usage: execsurface-p3-variance-analyzer <trusted-learning-manifest.json>".to_owned()
    })?;
    if args.next().is_some() {
        return Err(
            "usage: execsurface-p3-variance-analyzer <trusted-learning-manifest.json>".to_owned(),
        );
    }

    let bytes = fs::read(&manifest_path).map_err(|error| {
        format!(
            "failed to read {}: {error}",
            manifest_path.to_string_lossy()
        )
    })?;
    let manifest: TrustedLearningManifest = serde_json::from_slice(&bytes)
        .map_err(|error| format!("invalid trusted learning manifest: {error}"))?;
    let report = analyze(&manifest).map_err(|error| error.to_string())?;
    let output = serialize_report(&report).map_err(|error| error.to_string())?;

    io::stdout()
        .write_all(&output)
        .map_err(|error| format!("failed to write report: {error}"))?;
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
