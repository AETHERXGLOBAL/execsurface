//! Deterministic, policy-free execution-surface diffing.
//!
//! M4 reports evidence. It intentionally assigns no security severity and no
//! PASS / REVIEW / BLOCK verdict.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use execsurface_baseline::{
    verify_lock, BaselineError, BaselineLock, CommandIdentity, ObserverIdentity, PlatformIdentity,
};
use execsurface_model::canonical::{
    CanonicalEffect, CanonicalExecutable, CanonicalNetworkEndpoint, CanonicalPath, CanonicalSurface,
};
use execsurface_model::FileOperation;
use serde::{Deserialize, Serialize};

pub const DIFF_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSnapshot {
    pub command: CommandIdentity,
    pub platform: PlatformIdentity,
    pub observer: ObserverIdentity,
    pub canonical_surface: CanonicalSurface,
    pub target_exit_code: Option<i32>,
    pub target_signal: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffReport {
    pub schema_version: u32,
    pub baseline_digest: String,
    pub target: TargetOutcome,
    pub added: Vec<CanonicalEffect>,
    pub removed: Vec<CanonicalEffect>,
    pub changed: Vec<ChangedEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetOutcome {
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedEffect {
    pub subject: EffectSubject,
    pub before: CanonicalEffect,
    pub after: CanonicalEffect,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "subject_type", rename_all = "snake_case")]
pub enum EffectSubject {
    FileAccess {
        actor: Option<CanonicalExecutable>,
        operation: FileOperation,
        path: String,
    },
    FileRename {
        actor: Option<CanonicalExecutable>,
        from_path: String,
    },
    NetworkInet {
        actor: Option<CanonicalExecutable>,
        address_family: String,
        ip: String,
    },
    NetworkUnix {
        actor: Option<CanonicalExecutable>,
        path: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparabilityMismatch {
    pub field: String,
    pub baseline: String,
    pub candidate: String,
}

#[derive(Debug)]
pub enum DiffError {
    InvalidBaseline(BaselineError),
    Incomparable(Vec<ComparabilityMismatch>),
}

impl fmt::Display for DiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBaseline(error) => write!(f, "invalid baseline: {error}"),
            Self::Incomparable(mismatches) => {
                write!(f, "baseline and candidate are not comparable")?;
                for mismatch in mismatches {
                    write!(
                        f,
                        "; {} baseline={} candidate={}",
                        mismatch.field, mismatch.baseline, mismatch.candidate
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for DiffError {}

pub fn diff(
    baseline: &BaselineLock,
    candidate: &CandidateSnapshot,
) -> Result<DiffReport, DiffError> {
    verify_lock(baseline).map_err(DiffError::InvalidBaseline)?;

    let mismatches = comparability_mismatches(baseline, candidate);
    if !mismatches.is_empty() {
        return Err(DiffError::Incomparable(mismatches));
    }

    let baseline_set: BTreeSet<_> = baseline
        .payload
        .canonical_surface
        .effects
        .iter()
        .cloned()
        .collect();
    let candidate_set: BTreeSet<_> = candidate
        .canonical_surface
        .effects
        .iter()
        .cloned()
        .collect();

    let mut removed: BTreeSet<_> = baseline_set.difference(&candidate_set).cloned().collect();
    let mut added: BTreeSet<_> = candidate_set.difference(&baseline_set).cloned().collect();

    let mut before_by_subject: BTreeMap<EffectSubject, Vec<CanonicalEffect>> = BTreeMap::new();
    let mut after_by_subject: BTreeMap<EffectSubject, Vec<CanonicalEffect>> = BTreeMap::new();

    for effect in &removed {
        if let Some(subject) = effect_subject(effect) {
            before_by_subject
                .entry(subject)
                .or_default()
                .push(effect.clone());
        }
    }
    for effect in &added {
        if let Some(subject) = effect_subject(effect) {
            after_by_subject
                .entry(subject)
                .or_default()
                .push(effect.clone());
        }
    }

    let mut changed = Vec::new();
    for (subject, before) in before_by_subject {
        let Some(after) = after_by_subject.get(&subject) else {
            continue;
        };
        if before.len() == 1 && after.len() == 1 {
            let before_effect = before[0].clone();
            let after_effect = after[0].clone();
            removed.remove(&before_effect);
            added.remove(&after_effect);
            changed.push(ChangedEffect {
                subject,
                before: before_effect,
                after: after_effect,
            });
        }
    }
    changed.sort_by(|left, right| left.subject.cmp(&right.subject));

    Ok(DiffReport {
        schema_version: DIFF_SCHEMA_VERSION,
        baseline_digest: baseline.baseline_digest.clone(),
        target: TargetOutcome {
            exit_code: candidate.target_exit_code,
            signal: candidate.target_signal,
        },
        added: added.into_iter().collect(),
        removed: removed.into_iter().collect(),
        changed,
    })
}

pub fn has_drift(report: &DiffReport) -> bool {
    !(report.added.is_empty() && report.removed.is_empty() && report.changed.is_empty())
}

fn comparability_mismatches(
    baseline: &BaselineLock,
    candidate: &CandidateSnapshot,
) -> Vec<ComparabilityMismatch> {
    let mut mismatches = Vec::new();

    mismatch(
        &mut mismatches,
        "platform.os",
        &baseline.payload.platform.os,
        &candidate.platform.os,
    );
    mismatch(
        &mut mismatches,
        "platform.architecture",
        &baseline.payload.platform.architecture,
        &candidate.platform.architecture,
    );
    mismatch(
        &mut mismatches,
        "observer.name",
        &baseline.payload.observer.name,
        &candidate.observer.name,
    );

    let mut baseline_capabilities = baseline.payload.observer.capabilities.clone();
    baseline_capabilities.sort();
    baseline_capabilities.dedup();
    let mut candidate_capabilities = candidate.observer.capabilities.clone();
    candidate_capabilities.sort();
    candidate_capabilities.dedup();
    if baseline_capabilities != candidate_capabilities {
        mismatches.push(ComparabilityMismatch {
            field: "observer.capabilities".to_owned(),
            baseline: format!("{baseline_capabilities:?}"),
            candidate: format!("{candidate_capabilities:?}"),
        });
    }

    if baseline.payload.canonical_surface.schema_version
        != candidate.canonical_surface.schema_version
    {
        mismatches.push(ComparabilityMismatch {
            field: "canonical_surface.schema_version".to_owned(),
            baseline: baseline
                .payload
                .canonical_surface
                .schema_version
                .to_string(),
            candidate: candidate.canonical_surface.schema_version.to_string(),
        });
    }
    if baseline
        .payload
        .canonical_surface
        .normalization
        .profile_version
        != candidate.canonical_surface.normalization.profile_version
    {
        mismatches.push(ComparabilityMismatch {
            field: "normalization.profile_version".to_owned(),
            baseline: baseline
                .payload
                .canonical_surface
                .normalization
                .profile_version
                .to_string(),
            candidate: candidate
                .canonical_surface
                .normalization
                .profile_version
                .to_string(),
        });
    }

    let mut baseline_roots = baseline
        .payload
        .canonical_surface
        .normalization
        .semantic_roots
        .clone();
    baseline_roots.sort();
    baseline_roots.dedup();
    let mut candidate_roots = candidate
        .canonical_surface
        .normalization
        .semantic_roots
        .clone();
    candidate_roots.sort();
    candidate_roots.dedup();
    if baseline_roots != candidate_roots {
        mismatches.push(ComparabilityMismatch {
            field: "normalization.semantic_roots".to_owned(),
            baseline: format!("{baseline_roots:?}"),
            candidate: format!("{candidate_roots:?}"),
        });
    }

    if baseline.payload.command.executable != candidate.command.executable {
        mismatches.push(ComparabilityMismatch {
            field: "command.executable".to_owned(),
            baseline: format!("{:?}", baseline.payload.command.executable),
            candidate: format!("{:?}", candidate.command.executable),
        });
    }
    if baseline.payload.command.argument_count != candidate.command.argument_count {
        mismatches.push(ComparabilityMismatch {
            field: "command.argument_count".to_owned(),
            baseline: baseline.payload.command.argument_count.to_string(),
            candidate: candidate.command.argument_count.to_string(),
        });
    }

    mismatches
}

fn mismatch(
    mismatches: &mut Vec<ComparabilityMismatch>,
    field: &str,
    baseline: &str,
    candidate: &str,
) {
    if baseline != candidate {
        mismatches.push(ComparabilityMismatch {
            field: field.to_owned(),
            baseline: baseline.to_owned(),
            candidate: candidate.to_owned(),
        });
    }
}

fn effect_subject(effect: &CanonicalEffect) -> Option<EffectSubject> {
    match effect {
        CanonicalEffect::FilePathAccess {
            actor,
            operation,
            target,
            ..
        } => Some(EffectSubject::FileAccess {
            actor: actor.clone(),
            operation: *operation,
            path: target.value.clone(),
        }),
        CanonicalEffect::FileRename { actor, from, .. } => Some(EffectSubject::FileRename {
            actor: actor.clone(),
            from_path: from.value.clone(),
        }),
        CanonicalEffect::NetworkConnectAttempt {
            actor, endpoint, ..
        } => match endpoint {
            CanonicalNetworkEndpoint::Inet { ip, .. } => Some(EffectSubject::NetworkInet {
                actor: actor.clone(),
                address_family: "inet".to_owned(),
                ip: ip.clone(),
            }),
            CanonicalNetworkEndpoint::Inet6 { ip, .. } => Some(EffectSubject::NetworkInet {
                actor: actor.clone(),
                address_family: "inet6".to_owned(),
                ip: ip.clone(),
            }),
            CanonicalNetworkEndpoint::Unix { path } => Some(EffectSubject::NetworkUnix {
                actor: actor.clone(),
                path: path.as_ref().map(path_identity),
            }),
            CanonicalNetworkEndpoint::Other { .. } => None,
        },
        CanonicalEffect::ProcessSpawn { .. } | CanonicalEffect::ProcessExec { .. } => None,
    }
}

fn path_identity(path: &CanonicalPath) -> String {
    path.value.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_baseline::{build_lock, BaselinePayload, ToolIdentity};
    use execsurface_model::canonical::{
        NormalizationMetadata, OpenIntent, PathClass, PathResolution,
    };

    fn executable() -> CanonicalExecutable {
        CanonicalExecutable {
            path: CanonicalPath {
                value: "/bin/demo".to_owned(),
                class: PathClass::System,
                resolution: PathResolution::Lexical,
            },
            family: "demo".to_owned(),
        }
    }

    fn surface(effects: Vec<CanonicalEffect>) -> CanonicalSurface {
        CanonicalSurface {
            schema_version: 2,
            normalization: NormalizationMetadata {
                profile_version: 2,
                semantic_roots: vec!["home".to_owned(), "workspace".to_owned()],
            },
            effects,
        }
    }

    fn command() -> CommandIdentity {
        CommandIdentity {
            executable: executable(),
            argument_count: 0,
            label: None,
        }
    }

    fn platform() -> PlatformIdentity {
        PlatformIdentity {
            os: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
        }
    }

    fn observer() -> ObserverIdentity {
        ObserverIdentity {
            name: "linux-ptrace-metadata-only".to_owned(),
            capabilities: vec!["selected_file_paths".to_owned()],
            limitations: vec!["example".to_owned()],
        }
    }

    fn baseline(effects: Vec<CanonicalEffect>) -> BaselineLock {
        build_lock(BaselinePayload::new(
            ToolIdentity {
                name: "execsurface".to_owned(),
                version: "0.0.1".to_owned(),
            },
            command(),
            platform(),
            observer(),
            surface(effects),
        ))
        .unwrap()
    }

    fn candidate(effects: Vec<CanonicalEffect>) -> CandidateSnapshot {
        CandidateSnapshot {
            command: command(),
            platform: platform(),
            observer: observer(),
            canonical_surface: surface(effects),
            target_exit_code: Some(0),
            target_signal: None,
        }
    }

    fn file_open(write: bool) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: Some(executable()),
            execution_chain: vec![executable()],
            operation: FileOperation::Open,
            target: CanonicalPath {
                value: "$WORKSPACE/data.txt".to_owned(),
                class: PathClass::Workspace,
                resolution: PathResolution::Lexical,
            },
            open_intent: Some(OpenIntent {
                read: !write,
                write,
                create: false,
                truncate: false,
                append: false,
                path_only: false,
                resolve_flags: 0,
                other_flags: 0,
            }),
        }
    }

    #[test]
    fn identical_surfaces_have_no_drift() {
        let effect = file_open(false);
        let report = diff(&baseline(vec![effect.clone()]), &candidate(vec![effect])).unwrap();
        assert!(!has_drift(&report));
    }

    #[test]
    fn exact_set_difference_is_added_and_removed() {
        let old = CanonicalEffect::ProcessExec {
            from: None,
            executable: executable(),
        };
        let mut new_exec = executable();
        new_exec.path.value = "/bin/other".to_owned();
        new_exec.family = "other".to_owned();
        let new = CanonicalEffect::ProcessExec {
            from: None,
            executable: new_exec,
        };

        let report = diff(&baseline(vec![old.clone()]), &candidate(vec![new.clone()])).unwrap();
        assert_eq!(report.removed, vec![old]);
        assert_eq!(report.added, vec![new]);
        assert!(report.changed.is_empty());
    }

    #[test]
    fn unique_file_access_subject_becomes_changed() {
        let before = file_open(false);
        let after = file_open(true);
        let report = diff(
            &baseline(vec![before.clone()]),
            &candidate(vec![after.clone()]),
        )
        .unwrap();

        assert!(report.added.is_empty());
        assert!(report.removed.is_empty());
        assert_eq!(report.changed.len(), 1);
        assert_eq!(report.changed[0].before, before);
        assert_eq!(report.changed[0].after, after);
    }

    #[test]
    fn ambiguous_network_pairing_stays_added_removed() {
        let make = |port| CanonicalEffect::NetworkConnectAttempt {
            actor: Some(executable()),
            execution_chain: vec![executable()],
            endpoint: CanonicalNetworkEndpoint::Inet {
                ip: "192.0.2.10".to_owned(),
                port,
            },
        };
        let report = diff(
            &baseline(vec![make(80), make(443)]),
            &candidate(vec![make(8443), make(9443)]),
        )
        .unwrap();

        assert_eq!(report.removed.len(), 2);
        assert_eq!(report.added.len(), 2);
        assert!(report.changed.is_empty());
    }

    #[test]
    fn unique_remote_port_change_is_changed() {
        let make = |port| CanonicalEffect::NetworkConnectAttempt {
            actor: Some(executable()),
            execution_chain: vec![executable()],
            endpoint: CanonicalNetworkEndpoint::Inet {
                ip: "192.0.2.10".to_owned(),
                port,
            },
        };
        let report = diff(&baseline(vec![make(443)]), &candidate(vec![make(8443)])).unwrap();
        assert_eq!(report.changed.len(), 1);
    }

    #[test]
    fn normalization_profile_mismatch_is_incomparable() {
        let baseline = baseline(vec![]);
        let mut candidate = candidate(vec![]);
        candidate.canonical_surface.normalization.profile_version = 3;
        assert!(matches!(
            diff(&baseline, &candidate),
            Err(DiffError::Incomparable(mismatches))
                if mismatches.iter().any(|m| m.field == "normalization.profile_version")
        ));
    }

    #[test]
    fn observer_capability_mismatch_is_incomparable() {
        let baseline = baseline(vec![]);
        let mut candidate = candidate(vec![]);
        candidate
            .observer
            .capabilities
            .push("new_semantics".to_owned());
        assert!(matches!(
            diff(&baseline, &candidate),
            Err(DiffError::Incomparable(mismatches))
                if mismatches.iter().any(|m| m.field == "observer.capabilities")
        ));
    }

    #[test]
    fn corrupted_baseline_is_rejected() {
        let mut baseline = baseline(vec![]);
        baseline.baseline_digest = "sha256:deadbeef".to_owned();
        assert!(matches!(
            diff(&baseline, &candidate(vec![])),
            Err(DiffError::InvalidBaseline(_))
        ));
    }
}
