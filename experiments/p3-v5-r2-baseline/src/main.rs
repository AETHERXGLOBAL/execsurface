use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use execsurface_baseline::{
    build_lock, parse_and_verify, write_lockfile, BaselinePayload, CommandIdentity,
    ObserverIdentity, PlatformIdentity, ToolIdentity,
};
use execsurface_model::{Observation, RawEventKind};
use execsurface_normalize::{canonicalize, canonicalize_executable, NormalizationConfig};

const EXECSURFACE_VERSION: &str = "0.1.0-alpha.4";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let observation_path = PathBuf::from(args.next().ok_or(
        "usage: p3-v5-r2-baseline <observation.json> <output.lock.json> <argument-count>",
    )?);
    let output_path = PathBuf::from(args.next().ok_or(
        "usage: p3-v5-r2-baseline <observation.json> <output.lock.json> <argument-count>",
    )?);
    let argument_count = args
        .next()
        .ok_or("missing argument-count")?
        .to_string_lossy()
        .parse::<u32>()?;
    if args.next().is_some() {
        return Err("unexpected extra arguments".into());
    }

    let observation_bytes = fs::read(&observation_path)?;
    let observation: Observation = serde_json::from_slice(&observation_bytes)?;

    if observation.outcome.exit_code != Some(0) || observation.outcome.signal.is_some() {
        return Err(format!(
            "inadmissible target outcome: exit_code={:?} signal={:?}",
            observation.outcome.exit_code, observation.outcome.signal
        )
        .into());
    }
    if !observation.complete {
        return Err("inadmissible research observation: complete=false".into());
    }
    if !observation.warnings.is_empty() {
        return Err(format!(
            "inadmissible research observation: warning_count={}",
            observation.warnings.len()
        )
        .into());
    }

    let normalization = cli_equivalent_default_normalization()?;
    let canonical_surface = canonicalize(&observation, &normalization)?;

    let root_exec_path = observation
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            RawEventKind::ProcessExec { path } => Some((event.sequence, path)),
            _ => None,
        })
        .min_by_key(|(sequence, _)| *sequence)
        .map(|(_, path)| path)
        .ok_or("observer produced no confirmed root executable event")?;

    let executable = canonicalize_executable(root_exec_path, &normalization)?;
    let payload = BaselinePayload::new(
        ToolIdentity {
            name: "execsurface".to_owned(),
            version: EXECSURFACE_VERSION.to_owned(),
        },
        CommandIdentity {
            executable,
            argument_count,
            label: None,
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

    let lock = build_lock(payload)?;
    write_lockfile(&output_path, &lock, false)?;

    let written = fs::read(&output_path)?;
    let verified = parse_and_verify(&written)?;
    if verified != lock {
        return Err("written lockfile did not round-trip identically".into());
    }

    println!("P3_V5R2_BASELINE_PASS");
    println!("baseline_digest={}", lock.baseline_digest);
    println!("raw_events={}", observation.events.len());
    println!(
        "canonical_effects={}",
        lock.payload.canonical_surface.effects.len()
    );
    println!(
        "semantic_roots={}",
        lock.payload
            .canonical_surface
            .normalization
            .semantic_roots
            .join(",")
    );
    println!("warnings={}", observation.warnings.len());
    println!("complete={}", observation.complete);
    println!("lockfile={}", output_path.display());
    Ok(())
}

fn cli_equivalent_default_normalization() -> Result<NormalizationConfig, Box<dyn Error>> {
    let cwd = env::current_dir()?.to_string_lossy().into_owned();
    let home = env::var_os("HOME").map(|value| value.to_string_lossy().into_owned());
    let workspace = if home.as_deref() == Some(cwd.as_str()) {
        None
    } else {
        Some(cwd)
    };

    let mut tmp_roots = Vec::new();
    if let Some(tmpdir) = env::var_os("TMPDIR") {
        tmp_roots.push(tmpdir.to_string_lossy().into_owned());
    }
    if !tmp_roots.iter().any(|root| root == "/tmp") {
        tmp_roots.push("/tmp".to_owned());
    }

    Ok(NormalizationConfig {
        workspace,
        home,
        tmp_roots,
        run_tmp: None,
        caches: BTreeMap::new(),
    })
}
