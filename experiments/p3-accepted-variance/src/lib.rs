use std::collections::BTreeSet;
use std::fmt;

use execsurface_model::canonical::CanonicalEffect;
use execsurface_p3_variance_analyzer::{
    ComparableProfile, EffectRecurrence, RecurrenceClass, VarianceReport,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SELECTION_SCHEMA_VERSION: u32 = 1;
pub const CONTRACT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptanceSelection {
    pub schema_version: u32,
    pub learning_set_digest: String,
    pub accepted_effects: Vec<CanonicalEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AcceptedVarianceContract {
    pub schema_version: u32,
    pub learning_set_digest: String,
    pub profile: ComparableProfile,
    pub invariant_core_digest: String,
    pub variable_candidates: Vec<VariableEffectRecord>,
    pub accepted_variable_effects: Vec<VariableEffectRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VariableEffectRecord {
    pub effect: CanonicalEffect,
    pub support_count: usize,
    pub source_evidence_digests: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    UnsupportedSelectionSchema(u32),
    LearningSetDigestMismatch,
    DuplicateSelection,
    InvariantSelected,
    UnseenEffectSelected,
    NonVariableEffectSelected,
    Serialization(String),
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSelectionSchema(version) => {
                write!(f, "unsupported acceptance selection schema: {version}")
            }
            Self::LearningSetDigestMismatch => {
                write!(f, "acceptance selection learning-set digest mismatch")
            }
            Self::DuplicateSelection => {
                write!(f, "duplicate effect in explicit acceptance selection")
            }
            Self::InvariantSelected => {
                write!(f, "invariant effect cannot be accepted as variance")
            }
            Self::UnseenEffectSelected => {
                write!(f, "unseen effect cannot be accepted as observed variance")
            }
            Self::NonVariableEffectSelected => {
                write!(f, "selected effect is not a V1 variable candidate")
            }
            Self::Serialization(reason) => write!(f, "serialization failure: {reason}"),
        }
    }
}

impl std::error::Error for ContractError {}

pub fn build_contract(
    report: &VarianceReport,
    selection: &AcceptanceSelection,
) -> Result<AcceptedVarianceContract, ContractError> {
    if selection.schema_version != SELECTION_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSelectionSchema(
            selection.schema_version,
        ));
    }
    if selection.learning_set_digest != report.learning_set_digest {
        return Err(ContractError::LearningSetDigestMismatch);
    }

    let selected = selection
        .accepted_effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if selected.len() != selection.accepted_effects.len() {
        return Err(ContractError::DuplicateSelection);
    }

    let invariant = report
        .invariant_effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let union = report
        .union_effects
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut variable_candidates = report
        .recurrence
        .iter()
        .filter(|record| record.class == RecurrenceClass::VariableCandidate)
        .map(variable_record)
        .collect::<Vec<_>>();
    variable_candidates.sort_by(|left, right| left.effect.cmp(&right.effect));

    let candidate_effects = variable_candidates
        .iter()
        .map(|record| record.effect.clone())
        .collect::<BTreeSet<_>>();

    for effect in &selected {
        if invariant.contains(effect) {
            return Err(ContractError::InvariantSelected);
        }
        if !union.contains(effect) {
            return Err(ContractError::UnseenEffectSelected);
        }
        if !candidate_effects.contains(effect) {
            return Err(ContractError::NonVariableEffectSelected);
        }
    }

    let accepted_variable_effects = variable_candidates
        .iter()
        .filter(|record| selected.contains(&record.effect))
        .cloned()
        .collect::<Vec<_>>();

    Ok(AcceptedVarianceContract {
        schema_version: CONTRACT_SCHEMA_VERSION,
        learning_set_digest: report.learning_set_digest.clone(),
        profile: report.profile.clone(),
        invariant_core_digest: digest_invariant_core(report)?,
        variable_candidates,
        accepted_variable_effects,
    })
}

pub fn serialize_contract(contract: &AcceptedVarianceContract) -> Result<Vec<u8>, ContractError> {
    let mut bytes = serde_json::to_vec_pretty(contract)
        .map_err(|error| ContractError::Serialization(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn variable_record(record: &EffectRecurrence) -> VariableEffectRecord {
    let mut source_evidence_digests = record.source_evidence_digests.clone();
    source_evidence_digests.sort();
    source_evidence_digests.dedup();
    VariableEffectRecord {
        effect: record.effect.clone(),
        support_count: record.support_count,
        source_evidence_digests,
    }
}

#[derive(Serialize)]
struct InvariantCoreEnvelope<'a> {
    schema_version: u32,
    learning_set_digest: &'a str,
    profile: &'a ComparableProfile,
    invariant_effects: Vec<&'a CanonicalEffect>,
}

fn digest_invariant_core(report: &VarianceReport) -> Result<String, ContractError> {
    let mut invariant_effects = report.invariant_effects.iter().collect::<Vec<_>>();
    invariant_effects.sort();
    invariant_effects.dedup();
    let bytes = serde_json::to_vec(&InvariantCoreEnvelope {
        schema_version: CONTRACT_SCHEMA_VERSION,
        learning_set_digest: &report.learning_set_digest,
        profile: &report.profile,
        invariant_effects,
    })
    .map_err(|error| ContractError::Serialization(error.to_string()))?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_baseline::{CommandIdentity, ObserverIdentity, PlatformIdentity, ToolIdentity};
    use execsurface_model::canonical::{
        CanonicalExecutable, CanonicalPath, PathClass, PathResolution,
    };
    use execsurface_model::FileOperation;
    use execsurface_p3_variance_analyzer::{EffectRecurrence, SourceRunRef};

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

    fn profile() -> ComparableProfile {
        ComparableProfile {
            tool: ToolIdentity {
                name: "execsurface".to_owned(),
                version: "0.1.0-alpha.4".to_owned(),
            },
            command: CommandIdentity {
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
            platform: PlatformIdentity {
                os: "linux".to_owned(),
                architecture: "x86_64".to_owned(),
            },
            observer: ObserverIdentity {
                name: "linux-ptrace-metadata-v2".to_owned(),
                capabilities: vec!["descendant_tracking".to_owned()],
                limitations: vec!["test".to_owned()],
            },
            canonical_schema_version: 2,
            normalization_profile_version: 3,
            semantic_roots: vec!["workspace".to_owned()],
        }
    }

    fn digest(value: u8) -> String {
        format!("sha256:{value:064x}")
    }

    fn recurrence(
        effect: CanonicalEffect,
        support_count: usize,
        sources: &[u8],
        class: RecurrenceClass,
    ) -> EffectRecurrence {
        EffectRecurrence {
            effect,
            support_count,
            source_evidence_digests: sources.iter().copied().map(digest).collect(),
            class,
        }
    }

    fn report() -> VarianceReport {
        let invariant = path_effect("/invariant");
        let rare = path_effect("/rare");
        let frequent = path_effect("/frequent");
        VarianceReport {
            schema_version: 1,
            learning_set_digest: digest(99),
            run_count: 3,
            profile: profile(),
            sources: vec![
                SourceRunRef {
                    evidence_digest: digest(1),
                    baseline_digest: digest(11),
                },
                SourceRunRef {
                    evidence_digest: digest(2),
                    baseline_digest: digest(12),
                },
                SourceRunRef {
                    evidence_digest: digest(3),
                    baseline_digest: digest(13),
                },
            ],
            invariant_effects: vec![invariant.clone()],
            union_effects: vec![invariant.clone(), rare.clone(), frequent.clone()],
            recurrence: vec![
                recurrence(invariant, 3, &[1, 2, 3], RecurrenceClass::Invariant),
                recurrence(rare, 1, &[1], RecurrenceClass::VariableCandidate),
                recurrence(frequent, 2, &[1, 3], RecurrenceClass::VariableCandidate),
            ],
        }
    }

    fn selection(effects: Vec<CanonicalEffect>) -> AcceptanceSelection {
        AcceptanceSelection {
            schema_version: SELECTION_SCHEMA_VERSION,
            learning_set_digest: digest(99),
            accepted_effects: effects,
        }
    }

    #[test]
    fn recurrence_never_auto_authorizes_rare_or_frequent_candidates() {
        let contract = build_contract(&report(), &selection(vec![])).expect("contract");
        assert_eq!(contract.variable_candidates.len(), 2);
        assert!(contract.accepted_variable_effects.is_empty());
    }

    #[test]
    fn explicit_candidate_acceptance_preserves_exact_provenance() {
        let frequent = path_effect("/frequent");
        let contract =
            build_contract(&report(), &selection(vec![frequent.clone()])).expect("contract");
        assert_eq!(contract.accepted_variable_effects.len(), 1);
        let accepted = &contract.accepted_variable_effects[0];
        assert_eq!(accepted.effect, frequent);
        assert_eq!(accepted.support_count, 2);
        assert_eq!(accepted.source_evidence_digests, vec![digest(1), digest(3)]);
    }

    #[test]
    fn invariant_effect_cannot_be_reclassified_as_variance() {
        assert_eq!(
            build_contract(&report(), &selection(vec![path_effect("/invariant")])),
            Err(ContractError::InvariantSelected)
        );
    }

    #[test]
    fn unseen_effect_cannot_be_accepted() {
        assert_eq!(
            build_contract(&report(), &selection(vec![path_effect("/unseen")])),
            Err(ContractError::UnseenEffectSelected)
        );
    }

    #[test]
    fn duplicate_selection_is_rejected() {
        let rare = path_effect("/rare");
        assert_eq!(
            build_contract(&report(), &selection(vec![rare.clone(), rare])),
            Err(ContractError::DuplicateSelection)
        );
    }

    #[test]
    fn learning_set_mismatch_is_rejected() {
        let mut wrong = selection(vec![path_effect("/rare")]);
        wrong.learning_set_digest = digest(98);
        assert_eq!(
            build_contract(&report(), &wrong),
            Err(ContractError::LearningSetDigestMismatch)
        );
    }

    #[test]
    fn unsupported_selection_schema_is_rejected() {
        let mut wrong = selection(vec![]);
        wrong.schema_version = 2;
        assert_eq!(
            build_contract(&report(), &wrong),
            Err(ContractError::UnsupportedSelectionSchema(2))
        );
    }

    #[test]
    fn selection_order_does_not_change_contract_bytes() {
        let rare = path_effect("/rare");
        let frequent = path_effect("/frequent");
        let first = build_contract(&report(), &selection(vec![rare.clone(), frequent.clone()]))
            .expect("first");
        let second = build_contract(&report(), &selection(vec![frequent, rare])).expect("second");
        assert_eq!(
            serialize_contract(&first).expect("first bytes"),
            serialize_contract(&second).expect("second bytes")
        );
    }

    #[test]
    fn support_frequency_does_not_affect_acceptance_without_explicit_selection() {
        let mut changed = report();
        for record in &mut changed.recurrence {
            if record.effect == path_effect("/rare") {
                record.support_count = 2;
                record.source_evidence_digests = vec![digest(1), digest(2)];
            }
        }
        let contract = build_contract(&changed, &selection(vec![])).expect("contract");
        assert!(contract.accepted_variable_effects.is_empty());
    }
}
