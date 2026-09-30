use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::canonical::{CanonicalExecutable, CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CompletenessDimension, CompletenessState, EvidenceGuarantees,
    ProofCarryingObservation, Proposition,
};
use execsurface_p4_backend_authority::{
    AdapterRecord, AuthorityState, EvidenceReference, PropositionCompleteness,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const TETRAGON_REPOSITORY: &str = "cilium/tetragon";
pub const TETRAGON_SOURCE_COMMIT: &str = "666efe6f91e3605ad58683ad226d759d9cf970ca";
pub const TETRAGON_EVENTS_PROTO_PATH: &str = "api/v1/tetragon/events.proto";
pub const TETRAGON_EVENTS_PROTO_BLOB: &str = "d6bd56769241da983f7e0042a54d5b3a81f40817";
pub const EXEC_PROPOSITION_ID: &str = "P4.EXEC.SUCCESS";
const IMPORT_PROFILE_VERSION: u32 = 1;
const EXTERNAL_INCOMPLETE: &str = "external_session_completeness_unproven";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalSchemaPin {
    pub repository: String,
    pub source_commit: String,
    pub schema_path: String,
    pub schema_blob: String,
}

impl ExternalSchemaPin {
    pub fn frozen_tetragon() -> Self {
        Self {
            repository: TETRAGON_REPOSITORY.to_owned(),
            source_commit: TETRAGON_SOURCE_COMMIT.to_owned(),
            schema_path: TETRAGON_EVENTS_PROTO_PATH.to_owned(),
            schema_blob: TETRAGON_EVENTS_PROTO_BLOB.to_owned(),
        }
    }

    fn validate_frozen(&self) -> Result<(), String> {
        if self == &Self::frozen_tetragon() {
            Ok(())
        } else {
            Err("external_schema_pin_mismatch".to_owned())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalTraceProvenance {
    pub importer_profile: String,
    pub external_repository: String,
    pub external_source_commit: String,
    pub external_schema_path: String,
    pub external_schema_blob: String,
    pub raw_event_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedTraceRecord {
    pub adapter: AdapterRecord,
    pub provenance: ExternalTraceProvenance,
}

impl ImportedTraceRecord {
    pub fn identity_digest(&self) -> String {
        sha256(&serde_json::to_vec(self).expect("imported trace record serializes"))
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn write_canonical_json(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
        Value::Number(value) => out.push_str(&value.to_string()),
        Value::String(value) => out.push_str(&serde_json::to_string(value).expect("string serializes")),
        Value::Array(values) => {
            out.push('[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    out.push(',');
                }
                write_canonical_json(value, out);
            }
            out.push(']');
        }
        Value::Object(values) => {
            out.push('{');
            let mut keys: Vec<_> = values.keys().collect();
            keys.sort_unstable();
            for (index, key) in keys.into_iter().enumerate() {
                if index != 0 {
                    out.push(',');
                }
                out.push_str(&serde_json::to_string(key).expect("object key serializes"));
                out.push(':');
                write_canonical_json(&values[key], out);
            }
            out.push('}');
        }
    }
}

pub fn canonical_raw_digest(value: &Value) -> String {
    let mut canonical = String::new();
    write_canonical_json(value, &mut canonical);
    sha256(canonical.as_bytes())
}

fn profile() -> BackendSemanticProfile {
    BackendSemanticProfile {
        name: format!("external-import:tetragon-json@{TETRAGON_SOURCE_COMMIT}"),
        semantic_profile_version: IMPORT_PROFILE_VERSION,
    }
}

fn canonical_executable(path: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: CanonicalPath {
            value: path.to_owned(),
            class: PathClass::Unknown,
            resolution: PathResolution::Lexical,
        },
        family: path.rsplit('/').next().unwrap_or(path).to_owned(),
    }
}

fn unknown_exec_proposition() -> Proposition {
    Proposition::ProcessExecSucceeded {
        from: None,
        executable: canonical_executable("<unmapped-external-executable>"),
    }
}

fn provenance(raw_digest: String, pin: &ExternalSchemaPin) -> ExternalTraceProvenance {
    ExternalTraceProvenance {
        importer_profile: profile().name,
        external_repository: pin.repository.clone(),
        external_source_commit: pin.source_commit.clone(),
        external_schema_path: pin.schema_path.clone(),
        external_schema_blob: pin.schema_blob.clone(),
        raw_event_digest: raw_digest,
    }
}

fn incomplete_map() -> BTreeMap<CompletenessDimension, CompletenessState> {
    BTreeMap::from([
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Incomplete {
                reason_code: EXTERNAL_INCOMPLETE.to_owned(),
            },
        ),
        (
            CompletenessDimension::Transport,
            CompletenessState::Incomplete {
                reason_code: "external_transport_completeness_unproven".to_owned(),
            },
        ),
        (
            CompletenessDimension::Capability,
            CompletenessState::Complete,
        ),
    ])
}

fn unsupported_record(raw_digest: String, pin: &ExternalSchemaPin) -> ImportedTraceRecord {
    let reason = "external_event_type_not_process_exec".to_owned();
    let mut proof = ProofCarryingObservation::new(
        unknown_exec_proposition(),
        EvidenceGuarantees::default(),
        profile(),
    );
    proof.completeness.insert(
        CompletenessDimension::Capability,
        CompletenessState::Unsupported {
            reason_code: reason.clone(),
        },
    );
    ImportedTraceRecord {
        adapter: AdapterRecord {
            proposition_id: EXEC_PROPOSITION_ID.to_owned(),
            proof,
            authority: AuthorityState::Unsupported,
            completeness: PropositionCompleteness::NotApplicable,
            evidence: EvidenceReference {
                digest: raw_digest.clone(),
                derivation: None,
            },
            reason_codes: BTreeSet::from([reason]),
        },
        provenance: provenance(raw_digest, pin),
    }
}

fn ambiguous_record(
    proposition: Proposition,
    raw_digest: String,
    pin: &ExternalSchemaPin,
    reasons: BTreeSet<String>,
) -> ImportedTraceRecord {
    let mut proof = ProofCarryingObservation::new(
        proposition,
        EvidenceGuarantees::default(),
        profile(),
    );
    proof.completeness = incomplete_map();
    proof.ambiguity_codes = reasons.clone();
    ImportedTraceRecord {
        adapter: AdapterRecord {
            proposition_id: EXEC_PROPOSITION_ID.to_owned(),
            proof,
            authority: AuthorityState::Ambiguous,
            completeness: PropositionCompleteness::Incomplete,
            evidence: EvidenceReference {
                digest: raw_digest.clone(),
                derivation: None,
            },
            reason_codes: reasons,
        },
        provenance: provenance(raw_digest, pin),
    }
}

fn direct_incomplete_record(
    proposition: Proposition,
    raw_digest: String,
    pin: &ExternalSchemaPin,
) -> ImportedTraceRecord {
    let mut proof = ProofCarryingObservation::new(
        proposition,
        EvidenceGuarantees::default(),
        profile(),
    );
    proof.completeness = incomplete_map();
    ImportedTraceRecord {
        adapter: AdapterRecord {
            proposition_id: EXEC_PROPOSITION_ID.to_owned(),
            proof,
            authority: AuthorityState::Direct,
            completeness: PropositionCompleteness::Incomplete,
            evidence: EvidenceReference {
                digest: raw_digest.clone(),
                derivation: None,
            },
            reason_codes: BTreeSet::from([EXTERNAL_INCOMPLETE.to_owned()]),
        },
        provenance: provenance(raw_digest, pin),
    }
}

fn string_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key)?.as_str().filter(|value| !value.is_empty())
}

fn is_absolute_nonempty(value: Option<&str>) -> bool {
    value.is_some_and(|value| value.starts_with('/') && value.len() > 1)
}

pub fn import_tetragon_process_exec(
    raw: &str,
    pin: &ExternalSchemaPin,
) -> Result<ImportedTraceRecord, String> {
    pin.validate_frozen()?;
    let value: Value = serde_json::from_str(raw).map_err(|_| "external_json_decode_failed".to_owned())?;
    let raw_digest = canonical_raw_digest(&value);

    let Some(process_exec) = value.get("process_exec") else {
        let record = unsupported_record(raw_digest, pin);
        record.adapter.validate()?;
        return Ok(record);
    };

    let process = process_exec.get("process").unwrap_or(&Value::Null);
    let parent = process_exec.get("parent").unwrap_or(&Value::Null);
    let binary = string_field(process, "binary").unwrap_or("");
    let parent_binary = string_field(parent, "binary").unwrap_or("");
    let proposition = Proposition::ProcessExecSucceeded {
        from: is_absolute_nonempty(Some(parent_binary)).then(|| canonical_executable(parent_binary)),
        executable: canonical_executable(if binary.is_empty() {
            "<unmapped-external-executable>"
        } else {
            binary
        }),
    };

    let mut reasons = BTreeSet::new();
    let flags: BTreeSet<&str> = string_field(process, "flags")
        .unwrap_or("")
        .split_whitespace()
        .collect();

    let has_execve = flags.contains("execve");
    let has_procfs = flags.contains("procFS");
    if !has_execve {
        reasons.insert(if has_procfs {
            "tetragon_procfs_not_exec_transition".to_owned()
        } else {
            "tetragon_execve_origin_unproven".to_owned()
        });
    }
    if has_execve && has_procfs {
        reasons.insert("tetragon_conflicting_exec_origin_flags".to_owned());
    }
    for flag in ["truncFilename", "errorFilename", "unknown", "miss"] {
        if flags.contains(flag) {
            reasons.insert(format!("tetragon_flag_{flag}"));
        }
    }

    let process_exec_id = string_field(process, "exec_id");
    let parent_exec_id = string_field(process, "parent_exec_id");
    let parent_object_exec_id = string_field(parent, "exec_id");
    if process_exec_id.is_none() {
        reasons.insert("tetragon_process_exec_id_missing".to_owned());
    }
    if !is_absolute_nonempty(string_field(process, "binary")) {
        reasons.insert("tetragon_process_binary_identity_invalid".to_owned());
    }
    if parent_exec_id.is_none() || parent_object_exec_id.is_none() {
        reasons.insert("tetragon_parent_exec_identity_missing".to_owned());
    } else if parent_exec_id != parent_object_exec_id {
        reasons.insert("tetragon_parent_exec_identity_mismatch".to_owned());
    }
    if !is_absolute_nonempty(string_field(parent, "binary")) {
        reasons.insert("tetragon_parent_binary_identity_invalid".to_owned());
    }
    if value.get("aggregation_info").is_some() {
        reasons.insert("tetragon_process_exec_aggregation_not_accepted".to_owned());
    }

    let record = if reasons.is_empty() {
        direct_incomplete_record(proposition, raw_digest, pin)
    } else {
        ambiguous_record(proposition, raw_digest, pin, reasons)
    };
    record.adapter.validate()?;
    Ok(record)
}
