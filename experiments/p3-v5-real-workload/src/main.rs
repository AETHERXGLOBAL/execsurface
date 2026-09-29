use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use execsurface_baseline::{build_lock, parse_and_verify, BaselineLock};
use execsurface_model::canonical::CanonicalEffect;
use execsurface_p3_accepted_variance::{
    build_contract, AcceptanceSelection, SELECTION_SCHEMA_VERSION,
};
use execsurface_p3_gcc_temp_grammar::{
    apply_candidate, eligible_gcc_temp_paths, GCC_TEMP_CANONICAL,
};
use execsurface_p3_variance_analyzer::{
    analyze, EffectRecurrence, RecurrenceClass, TrustedLearningManifest, TrustedLearningRun,
    VarianceReport, INPUT_SCHEMA_VERSION,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Serialize)]
struct V5Report {
    schema_version: u32,
    decision: String,
    run_count: usize,
    raw_learning_set_digest: String,
    projected_learning_set_digest: String,
    raw_variable_candidate_count: usize,
    targeted_raw_variable_candidate_count: usize,
    non_target_raw_variable_candidate_count: usize,
    projected_variable_candidate_count: usize,
    canonical_gcc_invariant_effect_count: usize,
    accepted_variable_effect_count: usize,
    eligible_gcc_paths_by_evidence: BTreeMap<String, Vec<String>>,
    targeted_raw_variables: Vec<EffectEvidence>,
    non_target_raw_variables: Vec<EffectEvidence>,
    missing_or_changed_non_target_variables: Vec<NonTargetMismatch>,
    lingering_targeted_random_paths: Vec<EffectEvidence>,
}

#[derive(Debug, Clone, Serialize)]
struct EffectEvidence {
    effect: CanonicalEffect,
    support_count: usize,
    source_evidence_digests: Vec<String>,
}

#[derive(Debug, Serialize)]
struct NonTargetMismatch {
    effect: CanonicalEffect,
    raw_support_count: usize,
    raw_source_evidence_digests: Vec<String>,
    projected_support_count: Option<usize>,
    projected_source_evidence_digests: Vec<String>,
    projected_class: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let output = args
        .next()
        .map(PathBuf::from)
        .ok_or("usage: p3-v5-analyze <output.json> <baseline.lock.json>...")?;
    let baseline_paths = args.map(PathBuf::from).collect::<Vec<_>>();
    if baseline_paths.len() < 3 {
        return Err("V5 requires at least three independent baseline lockfiles".into());
    }

    let mut raw_runs = Vec::with_capacity(baseline_paths.len());
    let mut projected_runs = Vec::with_capacity(baseline_paths.len());
    let mut eligible_by_evidence = BTreeMap::new();
    let mut all_eligible_paths = BTreeSet::new();

    for path in &baseline_paths {
        let bytes = fs::read(path)?;
        let evidence_digest = format!("sha256:{:x}", Sha256::digest(&bytes));
        let raw_lock = parse_and_verify(&bytes)?;
        let eligible = eligible_gcc_temp_paths(&raw_lock.payload.canonical_surface);
        for value in &eligible {
            all_eligible_paths.insert(value.clone());
        }
        eligible_by_evidence.insert(
            evidence_digest.clone(),
            eligible.iter().cloned().collect::<Vec<_>>(),
        );

        raw_runs.push(TrustedLearningRun {
            evidence_digest: evidence_digest.clone(),
            observation_complete: true,
            baseline: raw_lock.clone(),
        });

        let projected_lock = projected_lock(&raw_lock)?;
        projected_runs.push(TrustedLearningRun {
            evidence_digest,
            observation_complete: true,
            baseline: projected_lock,
        });
    }

    let raw_report = analyze(&TrustedLearningManifest {
        schema_version: INPUT_SCHEMA_VERSION,
        runs: raw_runs,
    })?;
    let projected_report = analyze(&TrustedLearningManifest {
        schema_version: INPUT_SCHEMA_VERSION,
        runs: projected_runs,
    })?;

    let raw_variables = variable_records(&raw_report);
    let projected_variables = variable_records(&projected_report);

    let targeted_raw = raw_variables
        .iter()
        .filter(|record| {
            effect_target(&record.effect).is_some_and(|target| all_eligible_paths.contains(target))
        })
        .cloned()
        .collect::<Vec<_>>();
    let non_target_raw = raw_variables
        .iter()
        .filter(|record| {
            !effect_target(&record.effect).is_some_and(|target| all_eligible_paths.contains(target))
        })
        .cloned()
        .collect::<Vec<_>>();

    let projected_by_effect = projected_report
        .recurrence
        .iter()
        .map(|record| (record.effect.clone(), record))
        .collect::<BTreeMap<_, _>>();

    let mut mismatches = Vec::new();
    for raw in &non_target_raw {
        let projected = projected_by_effect.get(&raw.effect).copied();
        let matches = projected.is_some_and(|record| {
            record.class == RecurrenceClass::VariableCandidate
                && record.support_count == raw.support_count
                && record.source_evidence_digests == raw.source_evidence_digests
        });
        if !matches {
            mismatches.push(NonTargetMismatch {
                effect: raw.effect.clone(),
                raw_support_count: raw.support_count,
                raw_source_evidence_digests: raw.source_evidence_digests.clone(),
                projected_support_count: projected.map(|record| record.support_count),
                projected_source_evidence_digests: projected
                    .map(|record| record.source_evidence_digests.clone())
                    .unwrap_or_default(),
                projected_class: projected.map(|record| recurrence_class_name(record.class)),
            });
        }
    }

    let lingering_targeted = projected_variables
        .iter()
        .filter(|record| {
            effect_target(&record.effect).is_some_and(|target| all_eligible_paths.contains(target))
        })
        .cloned()
        .collect::<Vec<_>>();

    let canonical_gcc_invariant_effect_count = projected_report
        .invariant_effects
        .iter()
        .filter(|effect| effect_target(effect) == Some(GCC_TEMP_CANONICAL))
        .count();

    let contract = build_contract(
        &projected_report,
        &AcceptanceSelection {
            schema_version: SELECTION_SCHEMA_VERSION,
            learning_set_digest: projected_report.learning_set_digest.clone(),
            accepted_effects: vec![],
        },
    )?;

    let decision = if targeted_raw.is_empty() {
        "P3_V5_TARGETED_VARIANCE_NOT_REPRODUCED"
    } else if non_target_raw.is_empty() {
        "P3_V5_REAL_WORKLOAD_EVIDENCE_INCOMPLETE"
    } else if !mismatches.is_empty() || !lingering_targeted.is_empty() {
        "P3_V5_NON_TARGET_VARIANCE_COLLAPSED"
    } else if !contract.accepted_variable_effects.is_empty() {
        "P3_V5_REAL_WORKLOAD_EVIDENCE_INCOMPLETE"
    } else {
        "P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED"
    };

    let report = V5Report {
        schema_version: REPORT_SCHEMA_VERSION,
        decision: decision.to_owned(),
        run_count: baseline_paths.len(),
        raw_learning_set_digest: raw_report.learning_set_digest,
        projected_learning_set_digest: projected_report.learning_set_digest,
        raw_variable_candidate_count: raw_variables.len(),
        targeted_raw_variable_candidate_count: targeted_raw.len(),
        non_target_raw_variable_candidate_count: non_target_raw.len(),
        projected_variable_candidate_count: projected_variables.len(),
        canonical_gcc_invariant_effect_count,
        accepted_variable_effect_count: contract.accepted_variable_effects.len(),
        eligible_gcc_paths_by_evidence: eligible_by_evidence,
        targeted_raw_variables: targeted_raw,
        non_target_raw_variables: non_target_raw,
        missing_or_changed_non_target_variables: mismatches,
        lingering_targeted_random_paths: lingering_targeted,
    };

    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    fs::write(&output, bytes)?;

    println!("decision={}", report.decision);
    println!("run_count={}", report.run_count);
    println!(
        "raw_variable_candidates={}",
        report.raw_variable_candidate_count
    );
    println!(
        "targeted_raw_variable_candidates={}",
        report.targeted_raw_variable_candidate_count
    );
    println!(
        "non_target_raw_variable_candidates={}",
        report.non_target_raw_variable_candidate_count
    );
    println!(
        "projected_variable_candidates={}",
        report.projected_variable_candidate_count
    );
    println!(
        "canonical_gcc_invariant_effects={}",
        report.canonical_gcc_invariant_effect_count
    );
    println!(
        "accepted_variable_effects={}",
        report.accepted_variable_effect_count
    );
    println!(
        "non_target_mismatches={}",
        report.missing_or_changed_non_target_variables.len()
    );
    println!(
        "lingering_targeted_random_paths={}",
        report.lingering_targeted_random_paths.len()
    );

    if report.decision != "P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED" {
        return Err(format!("V5 gate did not pass: {}", report.decision).into());
    }

    Ok(())
}

fn projected_lock(raw: &BaselineLock) -> Result<BaselineLock, Box<dyn Error>> {
    let mut payload = raw.payload.clone();
    payload.canonical_surface = apply_candidate(&payload.canonical_surface);
    Ok(build_lock(payload)?)
}

fn variable_records(report: &VarianceReport) -> Vec<EffectEvidence> {
    report
        .recurrence
        .iter()
        .filter(|record| record.class == RecurrenceClass::VariableCandidate)
        .map(effect_evidence)
        .collect()
}

fn effect_evidence(record: &EffectRecurrence) -> EffectEvidence {
    EffectEvidence {
        effect: record.effect.clone(),
        support_count: record.support_count,
        source_evidence_digests: record.source_evidence_digests.clone(),
    }
}

fn effect_target(effect: &CanonicalEffect) -> Option<&str> {
    match effect {
        CanonicalEffect::FilePathAccess { target, .. } => Some(target.value.as_str()),
        _ => None,
    }
}

fn recurrence_class_name(class: RecurrenceClass) -> String {
    match class {
        RecurrenceClass::Invariant => "invariant".to_owned(),
        RecurrenceClass::VariableCandidate => "variable_candidate".to_owned(),
    }
}
