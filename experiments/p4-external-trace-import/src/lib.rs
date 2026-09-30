use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::canonical::{CanonicalExecutable, CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ProofCarryingObservation, Proposition, TemporalBinding,
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
pub const IMPORT_PROFILE: &str = "external-tetragon-json-import-v1";
pub const PROPOSITION_ID: &str = "P4.EXEC.SUCCESS";
pub const INCOMPLETE_REASON: &str = "external_session_completeness_unproven";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalSchemaIdentity {
    pub repository: String,
    pub source_commit: String,
    pub schema_path: String,
    pub schema_blob: String,
}

impl ExternalSchemaIdentity {
    pub fn pinned_tetragon() -> Self {
        Self {
            repository: TETRAGON_REPOSITORY.to_owned(),
            source_commit: TETRAGON_SOURCE_COMMIT.to_owned(),
            schema_path: TETRAGON_EVENTS_PROTO_PATH.to_owned(),
            schema_blob: TETRAGON_EVENTS_PROTO_BLOB.to_owned(),
        }
    }

    fn is_pinned(&self) -> bool {
        self == &Self::pinned_tetragon()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedExecEvidence {
    pub external: ExternalSchemaIdentity,
    pub producer_profile: String,
    pub raw_event_digest: String,
    pub imported_record_digest: String,
    pub subject_exec_id: String,
    pub parent_exec_id: String,
    pub adapter: AdapterRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonAuthoritativeImport {
    pub proposition_id: String,
    pub raw_event_digest: String,
    pub imported_record_digest: String,
    pub authority: AuthorityState,
    pub completeness: PropositionCompleteness,
    pub reason_codes: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedImport {
    pub proposition_id: String,
    pub raw_event_digest: String,
    pub reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum ImportDecision {
    Evidence(ImportedExecEvidence),
    NonAuthoritative(NonAuthoritativeImport),
    Unsupported(UnsupportedImport),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportError {
    pub reason_code: String,
}

fn sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let sorted: BTreeMap<String, Value> = map
                .iter()
                .map(|(key, value)| (key.clone(), canonicalize(value)))
                .collect();
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(values) => Value::Array(values.iter().map(canonicalize).collect()),
        _ => value.clone(),
    }
}

fn canonical_raw_digest(value: &Value) -> String {
    let bytes = serde_json::to_vec(&canonicalize(value)).expect("canonical JSON serialization");
    sha256(&bytes)
}

fn classify_path(path: &str) -> PathClass {
    if path.starts_with("/bin/")
        || path.starts_with("/sbin/")
        || path.starts_with("/usr/")
        || path.starts_with("/lib/")
    {
        PathClass::System
    } else {
        PathClass::OutsideDeclaredRoots
    }
}

fn executable(path: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: CanonicalPath {
            value: path.to_owned(),
            class: classify_path(path),
            resolution: PathResolution::Lexical,
        },
        family: path.rsplit('/').next().unwrap_or(path).to_owned(),
    }
}

fn stable_record_digest<T: Serialize>(record: &T) -> String {
    let bytes = serde_json::to_vec(record).expect("deterministic imported record serialization");
    sha256(&bytes)
}

fn non_authoritative(raw_event_digest: String, reasons: BTreeSet<String>) -> ImportDecision {
    #[derive(Serialize)]
    struct Identity<'a> {
        proposition_id: &'a str,
        producer_profile: &'a str,
        raw_event_digest: &'a str,
        authority: AuthorityState,
        completeness: PropositionCompleteness,
        reason_codes: &'a BTreeSet<String>,
    }

    let authority = AuthorityState::Ambiguous;
    let completeness = PropositionCompleteness::Incomplete;
    let digest = stable_record_digest(&Identity {
        proposition_id: PROPOSITION_ID,
        producer_profile: IMPORT_PROFILE,
        raw_event_digest: &raw_event_digest,
        authority,
        completeness,
        reason_codes: &reasons,
    });
    ImportDecision::NonAuthoritative(NonAuthoritativeImport {
        proposition_id: PROPOSITION_ID.to_owned(),
        raw_event_digest,
        imported_record_digest: digest,
        authority,
        completeness,
        reason_codes: reasons,
    })
}

fn text<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_str()
}

pub fn import_tetragon_event(
    schema: &ExternalSchemaIdentity,
    json: &str,
) -> Result<ImportDecision, ImportError> {
    if !schema.is_pinned() {
        return Err(ImportError {
            reason_code: "external_schema_identity_mismatch".to_owned(),
        });
    }

    let value: Value = serde_json::from_str(json).map_err(|_| ImportError {
        reason_code: "malformed_external_json".to_owned(),
    })?;
    let raw_event_digest = canonical_raw_digest(&value);

    let Some(process_exec) = value.get("process_exec") else {
        return Ok(ImportDecision::Unsupported(UnsupportedImport {
            proposition_id: PROPOSITION_ID.to_owned(),
            raw_event_digest,
            reason_code: "unsupported_external_event_type".to_owned(),
        }));
    };

    let mut reasons = BTreeSet::new();
    if value.get("aggregation_info").is_some_and(|v| !v.is_null()) {
        reasons.insert("aggregated_response_not_admissible".to_owned());
    }

    let subject_exec_id = text(process_exec, &["process", "exec_id"]).unwrap_or_default();
    let subject_binary = text(process_exec, &["process", "binary"]).unwrap_or_default();
    let flags = text(process_exec, &["process", "flags"]).unwrap_or_default();
    let parent_exec_id = text(process_exec, &["process", "parent_exec_id"]).unwrap_or_default();
    let parent_event_exec_id = text(process_exec, &["parent", "exec_id"]).unwrap_or_default();
    let parent_binary = text(process_exec, &["parent", "binary"]).unwrap_or_default();

    let tokens: BTreeSet<&str> = flags.split_whitespace().collect();
    let has_execve = tokens.contains("execve");
    let has_procfs = tokens.contains("procFS");

    if !has_execve {
        reasons.insert(if has_procfs {
            "procfs_bootstrap_not_exec_transition".to_owned()
        } else {
            "execve_flag_missing".to_owned()
        });
    }
    if has_execve && has_procfs {
        reasons.insert("contradictory_execve_procfs_flags".to_owned());
    }
    for banned in ["truncFilename", "errorFilename", "unknown", "miss"] {
        if tokens.contains(banned) {
            reasons.insert(format!("external_flag_{banned}"));
        }
    }
    if subject_exec_id.is_empty() {
        reasons.insert("missing_subject_exec_id".to_owned());
    }
    if !subject_binary.starts_with('/') || subject_binary.is_empty() {
        reasons.insert("invalid_subject_binary_identity".to_owned());
    }
    if parent_exec_id.is_empty() || parent_event_exec_id.is_empty() {
        reasons.insert("missing_parent_exec_identity".to_owned());
    } else if parent_exec_id != parent_event_exec_id {
        reasons.insert("parent_exec_identity_mismatch".to_owned());
    }
    if !parent_binary.starts_with('/') || parent_binary.is_empty() {
        reasons.insert("invalid_parent_binary_identity".to_owned());
    }

    if !reasons.is_empty() {
        return Ok(non_authoritative(raw_event_digest, reasons));
    }

    let mut proof = ProofCarryingObservation::new(
        Proposition::ProcessExecSucceeded {
            from: Some(executable(parent_binary)),
            executable: executable(subject_binary),
        },
        EvidenceGuarantees {
            observation_points: BTreeSet::new(),
            identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
            temporal_bindings: BTreeSet::from([TemporalBinding::LifecycleTransition]),
            causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
        },
        BackendSemanticProfile {
            name: IMPORT_PROFILE.to_owned(),
            semantic_profile_version: 1,
        },
    );
    proof.completeness.insert(
        CompletenessDimension::SessionScope,
        CompletenessState::Incomplete {
            reason_code: INCOMPLETE_REASON.to_owned(),
        },
    );
    proof.completeness.insert(
        CompletenessDimension::Transport,
        CompletenessState::Incomplete {
            reason_code: INCOMPLETE_REASON.to_owned(),
        },
    );
    proof.completeness.insert(
        CompletenessDimension::Capability,
        CompletenessState::Complete,
    );
    proof.completeness.insert(
        CompletenessDimension::Lifecycle,
        CompletenessState::Complete,
    );
    proof.completeness.insert(
        CompletenessDimension::CausalLineage,
        CompletenessState::Complete,
    );

    let adapter = AdapterRecord {
        proposition_id: PROPOSITION_ID.to_owned(),
        proof,
        authority: AuthorityState::Direct,
        completeness: PropositionCompleteness::Incomplete,
        evidence: EvidenceReference {
            digest: raw_event_digest.clone(),
            derivation: None,
        },
        reason_codes: BTreeSet::from([INCOMPLETE_REASON.to_owned()]),
    };
    adapter
        .validate()
        .map_err(|reason_code| ImportError { reason_code })?;

    #[derive(Serialize)]
    struct Identity<'a> {
        external: &'a ExternalSchemaIdentity,
        producer_profile: &'a str,
        raw_event_digest: &'a str,
        subject_exec_id: &'a str,
        parent_exec_id: &'a str,
        adapter: &'a AdapterRecord,
    }
    let imported_record_digest = stable_record_digest(&Identity {
        external: schema,
        producer_profile: IMPORT_PROFILE,
        raw_event_digest: &raw_event_digest,
        subject_exec_id,
        parent_exec_id,
        adapter: &adapter,
    });

    Ok(ImportDecision::Evidence(ImportedExecEvidence {
        external: schema.clone(),
        producer_profile: IMPORT_PROFILE.to_owned(),
        raw_event_digest,
        imported_record_digest,
        subject_exec_id: subject_exec_id.to_owned(),
        parent_exec_id: parent_exec_id.to_owned(),
        adapter,
    }))
}
