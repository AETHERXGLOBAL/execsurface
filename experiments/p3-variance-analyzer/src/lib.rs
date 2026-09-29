use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use execsurface_baseline::{
    verify_lock, BaselineLock, CommandIdentity, ObserverIdentity, PlatformIdentity, ToolIdentity,
};
use execsurface_model::canonical::CanonicalEffect;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const INPUT_SCHEMA_VERSION: u32 = 1;
pub const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedLearningManifest {
    pub schema_version: u32,
    pub runs: Vec<TrustedLearningRun>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedLearningRun {
    pub evidence_digest: String,
    pub observation_complete: bool,
    pub baseline: BaselineLock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VarianceReport {
    pub schema_version: u32,
    pub learning_set_digest: String,
    pub run_count: usize,
    pub profile: ComparableProfile,
    pub sources: Vec<SourceRunRef>,
    pub invariant_effects: Vec<CanonicalEffect>,
    pub union_effects: Vec<CanonicalEffect>,
    pub recurrence: Vec<EffectRecurrence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceRunRef {
    pub evidence_digest: String,
    pub baseline_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComparableProfile {
    pub tool: ToolIdentity,
    pub command: CommandIdentity,
    pub platform: PlatformIdentity,
    pub observer: ObserverIdentity,
    pub canonical_schema_version: u32,
    pub normalization_profile_version: u32,
    pub semantic_roots: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EffectRecurrence {
    pub effect: CanonicalEffect,
    pub support_count: usize,
    pub source_evidence_digests: Vec<String>,
    pub class: RecurrenceClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecurrenceClass {
    Invariant,
    VariableCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyzeError {
    UnsupportedManifestSchema(u32),
    NeedAtLeastTwoRuns,
    InvalidEvidenceDigest(String),
    DuplicateEvidenceDigest(String),
    IncompleteRun(String),
    InvalidBaseline {
        evidence_digest: String,
        reason: String,
    },
    IncomparableProfile {
        evidence_digest: String,
    },
    Serialization(String),
}

impl fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedManifestSchema(version) => {
                write!(f, "unsupported learning manifest schema: {version}")
            }
            Self::NeedAtLeastTwoRuns => {
                write!(f, "at least two distinct trusted runs are required")
            }
            Self::InvalidEvidenceDigest(digest) => {
                write!(f, "invalid evidence digest: {digest}")
            }
            Self::DuplicateEvidenceDigest(digest) => {
                write!(f, "duplicate evidence artifact: {digest}")
            }
            Self::IncompleteRun(digest) => {
                write!(
                    f,
                    "incomplete observation cannot enter learning set: {digest}"
                )
            }
            Self::InvalidBaseline {
                evidence_digest,
                reason,
            } => write!(
                f,
                "invalid baseline for evidence {evidence_digest}: {reason}"
            ),
            Self::IncomparableProfile { evidence_digest } => write!(
                f,
                "learning run is not comparable with frozen profile: {evidence_digest}"
            ),
            Self::Serialization(reason) => write!(f, "serialization failure: {reason}"),
        }
    }
}

impl std::error::Error for AnalyzeError {}

pub fn analyze(manifest: &TrustedLearningManifest) -> Result<VarianceReport, AnalyzeError> {
    if manifest.schema_version != INPUT_SCHEMA_VERSION {
        return Err(AnalyzeError::UnsupportedManifestSchema(
            manifest.schema_version,
        ));
    }
    if manifest.runs.len() < 2 {
        return Err(AnalyzeError::NeedAtLeastTwoRuns);
    }

    let mut seen_evidence = BTreeSet::new();
    let mut runs = Vec::with_capacity(manifest.runs.len());

    for run in &manifest.runs {
        validate_sha256(&run.evidence_digest)?;
        if !seen_evidence.insert(run.evidence_digest.clone()) {
            return Err(AnalyzeError::DuplicateEvidenceDigest(
                run.evidence_digest.clone(),
            ));
        }
        if !run.observation_complete {
            return Err(AnalyzeError::IncompleteRun(run.evidence_digest.clone()));
        }
        verify_lock(&run.baseline).map_err(|error| AnalyzeError::InvalidBaseline {
            evidence_digest: run.evidence_digest.clone(),
            reason: error.to_string(),
        })?;
        runs.push(run.clone());
    }

    runs.sort_by(|left, right| left.evidence_digest.cmp(&right.evidence_digest));

    let profile = profile_for(&runs[0].baseline);
    for run in &runs[1..] {
        if profile_for(&run.baseline) != profile {
            return Err(AnalyzeError::IncomparableProfile {
                evidence_digest: run.evidence_digest.clone(),
            });
        }
    }

    let mut support: BTreeMap<CanonicalEffect, BTreeSet<String>> = BTreeMap::new();
    let mut sources = Vec::with_capacity(runs.len());

    for run in &runs {
        sources.push(SourceRunRef {
            evidence_digest: run.evidence_digest.clone(),
            baseline_digest: run.baseline.baseline_digest.clone(),
        });

        let unique_effects: BTreeSet<_> = run
            .baseline
            .payload
            .canonical_surface
            .effects
            .iter()
            .cloned()
            .collect();
        for effect in unique_effects {
            support
                .entry(effect)
                .or_default()
                .insert(run.evidence_digest.clone());
        }
    }

    let run_count = runs.len();
    let union_effects = support.keys().cloned().collect::<Vec<_>>();
    let invariant_effects = support
        .iter()
        .filter_map(|(effect, sources)| (sources.len() == run_count).then_some(effect.clone()))
        .collect::<Vec<_>>();

    let recurrence = support
        .into_iter()
        .map(|(effect, evidence)| {
            let source_evidence_digests = evidence.into_iter().collect::<Vec<_>>();
            let support_count = source_evidence_digests.len();
            EffectRecurrence {
                effect,
                support_count,
                source_evidence_digests,
                class: if support_count == run_count {
                    RecurrenceClass::Invariant
                } else {
                    RecurrenceClass::VariableCandidate
                },
            }
        })
        .collect::<Vec<_>>();

    let learning_set_digest = digest_learning_set(&sources)?;

    Ok(VarianceReport {
        schema_version: REPORT_SCHEMA_VERSION,
        learning_set_digest,
        run_count,
        profile,
        sources,
        invariant_effects,
        union_effects,
        recurrence,
    })
}

pub fn serialize_report(report: &VarianceReport) -> Result<Vec<u8>, AnalyzeError> {
    let mut bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| AnalyzeError::Serialization(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn profile_for(lock: &BaselineLock) -> ComparableProfile {
    let mut observer = lock.payload.observer.clone();
    observer.capabilities.sort();
    observer.capabilities.dedup();
    observer.limitations.sort();
    observer.limitations.dedup();

    let mut semantic_roots = lock
        .payload
        .canonical_surface
        .normalization
        .semantic_roots
        .clone();
    semantic_roots.sort();
    semantic_roots.dedup();

    ComparableProfile {
        tool: lock.payload.tool.clone(),
        command: lock.payload.command.clone(),
        platform: lock.payload.platform.clone(),
        observer,
        canonical_schema_version: lock.payload.canonical_surface.schema_version,
        normalization_profile_version: lock.payload.canonical_surface.normalization.profile_version,
        semantic_roots,
    }
}

fn validate_sha256(value: &str) -> Result<(), AnalyzeError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(AnalyzeError::InvalidEvidenceDigest(value.to_owned()));
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(AnalyzeError::InvalidEvidenceDigest(value.to_owned()));
    }
    Ok(())
}

#[derive(Serialize)]
struct LearningSetDigestEnvelope<'a> {
    schema_version: u32,
    sources: &'a [SourceRunRef],
}

fn digest_learning_set(sources: &[SourceRunRef]) -> Result<String, AnalyzeError> {
    let bytes = serde_json::to_vec(&LearningSetDigestEnvelope {
        schema_version: REPORT_SCHEMA_VERSION,
        sources,
    })
    .map_err(|error| AnalyzeError::Serialization(error.to_string()))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{digest:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_baseline::{
        build_lock, BaselinePayload, CommandIdentity, ObserverIdentity, PlatformIdentity,
        ToolIdentity,
    };
    use execsurface_model::canonical::{
        CanonicalExecutable, CanonicalPath, CanonicalSurface, NormalizationMetadata, PathClass,
        PathResolution,
    };
    use execsurface_model::FileOperation;

    fn path_effect(path: &str) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: None,
            execution_chain: vec![],
            operation: FileOperation::Read,
            target: CanonicalPath {
                value: path.to_owned(),
                class: PathClass::System,
                resolution: PathResolution::Lexical,
            },
            open_intent: None,
        }
    }

    fn baseline(effects: Vec<CanonicalEffect>, architecture: &str) -> BaselineLock {
        build_lock(BaselinePayload::new(
            ToolIdentity {
                name: "execsurface".to_owned(),
                version: "0.1.0-alpha.4".to_owned(),
            },
            CommandIdentity {
                executable: CanonicalExecutable {
                    path: CanonicalPath {
                        value: "/usr/bin/example".to_owned(),
                        class: PathClass::System,
                        resolution: PathResolution::Lexical,
                    },
                    family: "example".to_owned(),
                },
                argument_count: 0,
                label: None,
            },
            PlatformIdentity {
                os: "linux".to_owned(),
                architecture: architecture.to_owned(),
            },
            ObserverIdentity {
                name: "linux-ptrace-metadata-v2".to_owned(),
                capabilities: vec!["descendant_tracking".to_owned()],
                limitations: vec!["test".to_owned()],
            },
            CanonicalSurface {
                schema_version: 2,
                normalization: NormalizationMetadata {
                    profile_version: 3,
                    semantic_roots: vec!["workspace".to_owned()],
                },
                effects,
            },
        ))
        .expect("build baseline")
    }

    fn evidence_digest(value: u8) -> String {
        format!("sha256:{value:064x}")
    }

    fn run(value: u8, effects: Vec<CanonicalEffect>) -> TrustedLearningRun {
        TrustedLearningRun {
            evidence_digest: evidence_digest(value),
            observation_complete: true,
            baseline: baseline(effects, "x86_64"),
        }
    }

    #[test]
    fn computes_invariant_union_and_provenance_without_authorization() {
        let a = path_effect("/a");
        let b = path_effect("/b");
        let c = path_effect("/c");
        let manifest = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![
                run(1, vec![a.clone(), b.clone()]),
                run(2, vec![a.clone(), c.clone()]),
                run(3, vec![a.clone(), b.clone()]),
            ],
        };

        let report = analyze(&manifest).expect("analyze");
        assert_eq!(report.run_count, 3);
        assert_eq!(report.invariant_effects, vec![a.clone()]);
        assert_eq!(report.union_effects, vec![a, b.clone(), c.clone()]);

        let b_record = report
            .recurrence
            .iter()
            .find(|record| record.effect == b)
            .expect("b recurrence");
        assert_eq!(b_record.support_count, 2);
        assert_eq!(b_record.class, RecurrenceClass::VariableCandidate);
        assert_eq!(
            b_record.source_evidence_digests,
            vec![evidence_digest(1), evidence_digest(3)]
        );

        let c_record = report
            .recurrence
            .iter()
            .find(|record| record.effect == c)
            .expect("c recurrence");
        assert_eq!(c_record.support_count, 1);
        assert_eq!(c_record.class, RecurrenceClass::VariableCandidate);
    }

    #[test]
    fn input_order_does_not_change_report_bytes() {
        let a = path_effect("/a");
        let b = path_effect("/b");
        let first = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![run(1, vec![a.clone()]), run(2, vec![a.clone(), b.clone()])],
        };
        let second = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![run(2, vec![a.clone(), b]), run(1, vec![a])],
        };

        let first_bytes = serialize_report(&analyze(&first).expect("first")).expect("serialize");
        let second_bytes = serialize_report(&analyze(&second).expect("second")).expect("serialize");
        assert_eq!(first_bytes, second_bytes);
    }

    #[test]
    fn duplicate_evidence_cannot_inflate_support() {
        let a = path_effect("/a");
        let duplicate = run(1, vec![a.clone()]);
        let manifest = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![duplicate.clone(), duplicate],
        };
        assert!(matches!(
            analyze(&manifest),
            Err(AnalyzeError::DuplicateEvidenceDigest(_))
        ));
    }

    #[test]
    fn incomplete_run_is_rejected_not_interpreted_as_absence() {
        let a = path_effect("/a");
        let mut incomplete = run(2, vec![a.clone()]);
        incomplete.observation_complete = false;
        let manifest = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![run(1, vec![a]), incomplete],
        };
        assert!(matches!(
            analyze(&manifest),
            Err(AnalyzeError::IncompleteRun(_))
        ));
    }

    #[test]
    fn incomparable_profile_is_rejected() {
        let a = path_effect("/a");
        let manifest = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![
                run(1, vec![a.clone()]),
                TrustedLearningRun {
                    evidence_digest: evidence_digest(2),
                    observation_complete: true,
                    baseline: baseline(vec![a], "aarch64"),
                },
            ],
        };
        assert!(matches!(
            analyze(&manifest),
            Err(AnalyzeError::IncomparableProfile { .. })
        ));
    }

    #[test]
    fn corrupted_baseline_is_rejected() {
        let a = path_effect("/a");
        let mut corrupted = run(2, vec![a.clone()]);
        corrupted.baseline.baseline_digest = "sha256:deadbeef".to_owned();
        let manifest = TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![run(1, vec![a]), corrupted],
        };
        assert!(matches!(
            analyze(&manifest),
            Err(AnalyzeError::InvalidBaseline { .. })
        ));
    }
}
