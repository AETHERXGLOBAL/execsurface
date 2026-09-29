use std::collections::{BTreeSet};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use execsurface_baseline::{build_lock, parse_and_verify, BaselineLock};
use execsurface_diff::{diff, CandidateSnapshot, DiffReport};
use execsurface_model::canonical::CanonicalEffect;
use execsurface_p3_accepted_variance::{build_contract, AcceptanceSelection, SELECTION_SCHEMA_VERSION};
use execsurface_p3_gcc_temp_grammar::{apply_candidate, eligible_gcc_temp_paths, GCC_TEMP_CANONICAL};
use execsurface_p3_variance_analyzer::{
    analyze, RecurrenceClass, TrustedLearningManifest, TrustedLearningRun, INPUT_SCHEMA_VERSION,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
struct CheckEvaluation {
    schema_version: u32,
    raw_learning_set_digest: String,
    projected_learning_set_digest: String,
    raw_single_baseline_findings: usize,
    gcc_ephemeral_eligible_findings: usize,
    projected_gcc_random_findings: usize,
    invariant_core_matches: usize,
    missing_invariant_effects: Vec<CanonicalEffect>,
    observed_variable_matches: Vec<CanonicalEffect>,
    explicitly_accepted_variable_matches: Vec<CanonicalEffect>,
    unseen_effects: Vec<CanonicalEffect>,
    residual_after_ephemeral_classification: usize,
    residual_after_explicit_variance: usize,
    non_target_projection_mismatches: Vec<CanonicalEffect>,
    eligible_current_gcc_paths: Vec<String>,
    raw_diff: DiffReport,
    projected_diff: DiffReport,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let output = PathBuf::from(args.next().ok_or(
        "usage: p3-v5-r4-evaluate <output.json> <check.lock.json> <learning1.lock.json> ... <learning6.lock.json>",
    )?);
    let check_path = PathBuf::from(args.next().ok_or("missing check lockfile")?);
    let learning_paths = args.map(PathBuf::from).collect::<Vec<_>>();
    if learning_paths.len() != 6 {
        return Err(format!("R4 requires exactly six learning lockfiles, got {}", learning_paths.len()).into());
    }

    let learning = learning_paths
        .iter()
        .map(|path| read_lock_with_evidence(path))
        .collect::<Result<Vec<_>, _>>()?;
    let check_bytes = fs::read(&check_path)?;
    let check = parse_and_verify(&check_bytes)?;

    let raw_runs = learning
        .iter()
        .map(|(digest, lock)| TrustedLearningRun {
            evidence_digest: digest.clone(),
            observation_complete: true,
            baseline: lock.clone(),
        })
        .collect::<Vec<_>>();
    let projected_runs = learning
        .iter()
        .map(|(digest, lock)| {
            Ok(TrustedLearningRun {
                evidence_digest: digest.clone(),
                observation_complete: true,
                baseline: projected_lock(lock)?,
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let raw_report = analyze(&TrustedLearningManifest {
        schema_version: INPUT_SCHEMA_VERSION,
        runs: raw_runs,
    })?;
    let projected_report = analyze(&TrustedLearningManifest {
        schema_version: INPUT_SCHEMA_VERSION,
        runs: projected_runs,
    })?;

    let contract = build_contract(
        &projected_report,
        &AcceptanceSelection {
            schema_version: SELECTION_SCHEMA_VERSION,
            learning_set_digest: projected_report.learning_set_digest.clone(),
            accepted_effects: vec![],
        },
    )?;
    if !contract.accepted_variable_effects.is_empty() {
        return Err("R4 freeze requires NO_ACCEPTED_VARIANCE".into());
    }

    let raw_baseline = &learning[0].1;
    let raw_candidate = candidate_from_lock(&check);
    let raw_diff = diff(raw_baseline, &raw_candidate)?;

    let projected_baseline = projected_lock(raw_baseline)?;
    let projected_check = projected_lock(&check)?;
    let projected_candidate = candidate_from_lock(&projected_check);
    let projected_diff = diff(&projected_baseline, &projected_candidate)?;

    let baseline_eligible = eligible_gcc_temp_paths(&raw_baseline.payload.canonical_surface);
    let current_eligible = eligible_gcc_temp_paths(&check.payload.canonical_surface);
    let gcc_ephemeral_eligible_findings = targeted_diff_count(
        &raw_diff,
        &baseline_eligible,
        &current_eligible,
    );

    let projected_gcc_random_findings = projected_diff
        .added
        .iter()
        .chain(projected_diff.removed.iter())
        .filter(|effect| effect_target(effect) == Some(GCC_TEMP_CANONICAL))
        .count()
        + projected_diff
            .changed
            .iter()
            .filter(|change| {
                effect_target(&change.before) == Some(GCC_TEMP_CANONICAL)
                    || effect_target(&change.after) == Some(GCC_TEMP_CANONICAL)
            })
            .count();

    let projected_current = projected_check
        .payload
        .canonical_surface
        .effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let projected_invariant = projected_report
        .invariant_effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let projected_union = projected_report
        .union_effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let projected_variables = projected_report
        .recurrence
        .iter()
        .filter(|record| record.class == RecurrenceClass::VariableCandidate)
        .map(|record| record.effect.clone())
        .collect::<BTreeSet<_>>();

    let invariant_core_matches = projected_invariant
        .intersection(&projected_current)
        .count();
    let missing_invariant_effects = projected_invariant
        .difference(&projected_current)
        .cloned()
        .collect::<Vec<_>>();
    let observed_variable_matches = projected_current
        .intersection(&projected_variables)
        .cloned()
        .collect::<Vec<_>>();
    let unseen_effects = projected_current
        .difference(&projected_union)
        .cloned()
        .collect::<Vec<_>>();

    let raw_current = check
        .payload
        .canonical_surface
        .effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let non_target_projection_mismatches = raw_current
        .iter()
        .filter(|effect| {
            let targeted = effect_target(effect)
                .is_some_and(|target| current_eligible.contains(target));
            !targeted && !projected_current.contains(*effect)
        })
        .cloned()
        .collect::<Vec<_>>();

    let raw_single_baseline_findings = diff_count(&raw_diff);
    let residual_after_ephemeral_classification = diff_count(&projected_diff);
    let residual_after_explicit_variance = missing_invariant_effects.len()
        + observed_variable_matches.len()
        + unseen_effects.len();

    let report = CheckEvaluation {
        schema_version: 1,
        raw_learning_set_digest: raw_report.learning_set_digest,
        projected_learning_set_digest: projected_report.learning_set_digest,
        raw_single_baseline_findings,
        gcc_ephemeral_eligible_findings,
        projected_gcc_random_findings,
        invariant_core_matches,
        missing_invariant_effects,
        observed_variable_matches,
        explicitly_accepted_variable_matches: vec![],
        unseen_effects,
        residual_after_ephemeral_classification,
        residual_after_explicit_variance,
        non_target_projection_mismatches,
        eligible_current_gcc_paths: current_eligible.into_iter().collect(),
        raw_diff,
        projected_diff,
    };

    if report.raw_learning_set_digest
        != "sha256:437a9f8fca9f0ff4c8573c1cfee06b46cb325d1d1a890649ba536acd8e2e648d"
    {
        return Err("raw learning-set digest does not match R3-C freeze".into());
    }
    if report.projected_learning_set_digest
        != "sha256:16e8519c83ea374359d1e44872d570ccc15be88b7ccf8f9e3b54b5e3d6b7ab30"
    {
        return Err("projected learning-set digest does not match R3-C freeze".into());
    }
    if report.gcc_ephemeral_eligible_findings == 0 {
        return Err("R4 check did not reproduce any bounded GCC ephemeral finding".into());
    }
    if report.projected_gcc_random_findings != 0 {
        return Err("targeted GCC random identity remained after bounded projection".into());
    }
    if !report.non_target_projection_mismatches.is_empty() {
        return Err("bounded projection changed non-target current effects".into());
    }
    if !report.explicitly_accepted_variable_matches.is_empty() {
        return Err("empty acceptance freeze was violated".into());
    }

    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    fs::write(&output, bytes)?;

    println!("R4_CHECK_EVALUATION_PASS");
    println!("raw_single_baseline_findings={}", report.raw_single_baseline_findings);
    println!("gcc_ephemeral_eligible_findings={}", report.gcc_ephemeral_eligible_findings);
    println!("invariant_core_matches={}", report.invariant_core_matches);
    println!("observed_variable_matches={}", report.observed_variable_matches.len());
    println!("explicitly_accepted_variable_matches=0");
    println!("unseen_effects={}", report.unseen_effects.len());
    println!(
        "residual_after_ephemeral_classification={}",
        report.residual_after_ephemeral_classification
    );
    println!(
        "residual_after_explicit_variance={}",
        report.residual_after_explicit_variance
    );
    Ok(())
}

fn read_lock_with_evidence(path: &PathBuf) -> Result<(String, BaselineLock), Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let evidence_digest = format!("sha256:{:x}", Sha256::digest(&bytes));
    Ok((evidence_digest, parse_and_verify(&bytes)?))
}

fn projected_lock(raw: &BaselineLock) -> Result<BaselineLock, Box<dyn Error>> {
    let mut payload = raw.payload.clone();
    payload.canonical_surface = apply_candidate(&payload.canonical_surface);
    Ok(build_lock(payload)?)
}

fn candidate_from_lock(lock: &BaselineLock) -> CandidateSnapshot {
    CandidateSnapshot {
        command: lock.payload.command.clone(),
        platform: lock.payload.platform.clone(),
        observer: lock.payload.observer.clone(),
        canonical_surface: lock.payload.canonical_surface.clone(),
        target_exit_code: Some(0),
        target_signal: None,
    }
}

fn diff_count(report: &DiffReport) -> usize {
    report.added.len() + report.removed.len() + report.changed.len()
}

fn targeted_diff_count(
    report: &DiffReport,
    before_paths: &BTreeSet<String>,
    after_paths: &BTreeSet<String>,
) -> usize {
    report
        .added
        .iter()
        .filter(|effect| effect_target(effect).is_some_and(|target| after_paths.contains(target)))
        .count()
        + report
            .removed
            .iter()
            .filter(|effect| effect_target(effect).is_some_and(|target| before_paths.contains(target)))
            .count()
        + report
            .changed
            .iter()
            .filter(|change| {
                effect_target(&change.before).is_some_and(|target| before_paths.contains(target))
                    || effect_target(&change.after)
                        .is_some_and(|target| after_paths.contains(target))
            })
            .count()
}

fn effect_target(effect: &CanonicalEffect) -> Option<&str> {
    match effect {
        CanonicalEffect::FilePathAccess { target, .. } => Some(target.value.as_str()),
        _ => None,
    }
}
