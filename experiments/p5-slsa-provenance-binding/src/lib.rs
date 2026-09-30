use std::collections::BTreeMap;

use execsurface_p5_attestation_provenance::{ProvenanceReference, VerificationInput};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";
pub const SLSA_PROVENANCE_TYPE: &str = "https://slsa.dev/provenance/v1";

pub const UPSTREAM_REPOSITORY: &str = "slsa-framework/slsa-verifier";
pub const UPSTREAM_COMMIT: &str = "30d0be3bbab553fc51557377baba2f7572dfc212";
pub const UPSTREAM_PATH: &str =
    "verifiers/internal/gcb/testdata/v1.0-gcloud-container-github-single.json";
pub const UPSTREAM_BLOB_SHA: &str = "7102e40887a758e01184ac79ca10cb34b7281627";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingError {
    pub reason_code: String,
}

impl BindingError {
    fn new(reason_code: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationState {
    Verified,
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlsaBindingContext {
    pub expected_statement_digest: String,
    pub artifact_name: String,
    pub artifact_digest: String,
    pub source_repository: String,
    pub source_revision: String,
    pub workflow_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BoundSlsaProvenance {
    pub statement_digest: String,
    pub predicate_type: String,
    pub subject_name: String,
    pub subject_digest: String,
    pub source_repository: String,
    pub source_revision: String,
    pub workflow_path: String,
    pub builder_id: String,
    pub verified: bool,
}

impl BoundSlsaProvenance {
    pub fn to_provenance_reference(&self) -> Result<ProvenanceReference, BindingError> {
        if !self.verified {
            return Err(BindingError::new("provenance_unverified"));
        }
        Ok(ProvenanceReference {
            predicate_type: self.predicate_type.clone(),
            statement_digest: self.statement_digest.clone(),
            subject_digest: self.subject_digest.clone(),
            verified: true,
        })
    }
}

#[derive(Debug, Deserialize)]
struct SlsaStatement {
    #[serde(rename = "_type")]
    statement_type: String,
    subject: Vec<SlsaSubject>,
    #[serde(rename = "predicateType")]
    predicate_type: String,
    predicate: SlsaPredicate,
}

#[derive(Debug, Deserialize)]
struct SlsaSubject {
    name: String,
    digest: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct SlsaPredicate {
    #[serde(rename = "buildDefinition")]
    build_definition: BuildDefinition,
    #[serde(rename = "runDetails")]
    run_details: RunDetails,
}

#[derive(Debug, Deserialize)]
struct BuildDefinition {
    #[serde(rename = "externalParameters")]
    external_parameters: ExternalParameters,
    #[serde(rename = "resolvedDependencies")]
    resolved_dependencies: Vec<ResolvedDependency>,
}

#[derive(Debug, Deserialize)]
struct ExternalParameters {
    #[serde(rename = "buildConfigSource")]
    build_config_source: BuildConfigSource,
}

#[derive(Debug, Deserialize)]
struct BuildConfigSource {
    path: String,
    #[serde(rename = "ref")]
    reference: String,
    repository: String,
}

#[derive(Debug, Deserialize)]
struct ResolvedDependency {
    uri: String,
    digest: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct RunDetails {
    builder: Builder,
}

#[derive(Debug, Deserialize)]
struct Builder {
    id: String,
}

fn is_hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn parse_labeled_sha256<'a>(value: &'a str, reason: &str) -> Result<&'a str, BindingError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(BindingError::new(reason));
    };
    if !is_hex(hex, 64) {
        return Err(BindingError::new(reason));
    }
    Ok(hex)
}

pub fn sha256_labeled(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("sha256:{digest:x}")
}

pub fn bind_slsa_v1_statement(
    statement_bytes: &[u8],
    context: &SlsaBindingContext,
    verification_state: VerificationState,
) -> Result<BoundSlsaProvenance, BindingError> {
    let expected_statement_hex = parse_labeled_sha256(
        &context.expected_statement_digest,
        "expected_statement_digest_malformed",
    )?;
    let actual_statement_digest = sha256_labeled(statement_bytes);
    if actual_statement_digest.strip_prefix("sha256:") != Some(expected_statement_hex) {
        return Err(BindingError::new("statement_digest_mismatch"));
    }

    let statement: SlsaStatement = serde_json::from_slice(statement_bytes)
        .map_err(|_| BindingError::new("slsa_statement_schema_rejected"))?;
    if statement.statement_type != STATEMENT_TYPE {
        return Err(BindingError::new("statement_type_mismatch"));
    }
    if statement.predicate_type != SLSA_PROVENANCE_TYPE {
        return Err(BindingError::new("slsa_predicate_type_mismatch"));
    }

    let expected_artifact_hex = parse_labeled_sha256(
        &context.artifact_digest,
        "expected_artifact_digest_malformed",
    )?;
    let subject = statement
        .subject
        .iter()
        .find(|subject| subject.name == context.artifact_name)
        .ok_or_else(|| BindingError::new("subject_binding_mismatch"))?;
    let subject_hex = subject
        .digest
        .get("sha256")
        .ok_or_else(|| BindingError::new("subject_sha256_missing"))?;
    if !is_hex(subject_hex, 64) {
        return Err(BindingError::new("subject_sha256_malformed"));
    }
    if subject_hex != expected_artifact_hex {
        return Err(BindingError::new("subject_binding_mismatch"));
    }

    let source = &statement
        .predicate
        .build_definition
        .external_parameters
        .build_config_source;
    if source.repository.is_empty() {
        return Err(BindingError::new("source_repository_missing"));
    }
    if source.repository != context.source_repository {
        return Err(BindingError::new("source_repository_mismatch"));
    }
    if source.path.is_empty() {
        return Err(BindingError::new("workflow_path_missing"));
    }
    if source.path != context.workflow_path {
        return Err(BindingError::new("workflow_path_mismatch"));
    }
    if source.reference.is_empty() {
        return Err(BindingError::new("source_ref_missing"));
    }

    let dependency_uri = format!("{}@{}", source.repository, source.reference);
    let source_dependency = statement
        .predicate
        .build_definition
        .resolved_dependencies
        .iter()
        .find(|dependency| dependency.uri == dependency_uri)
        .ok_or_else(|| BindingError::new("source_revision_missing"))?;
    let source_revision = source_dependency
        .digest
        .get("gitCommit")
        .ok_or_else(|| BindingError::new("source_revision_missing"))?;
    if source_revision.is_empty() {
        return Err(BindingError::new("source_revision_missing"));
    }
    if source_revision != &context.source_revision {
        return Err(BindingError::new("source_revision_mismatch"));
    }

    let builder_id = &statement.predicate.run_details.builder.id;
    if builder_id.is_empty() {
        return Err(BindingError::new("builder_identity_missing"));
    }

    Ok(BoundSlsaProvenance {
        statement_digest: actual_statement_digest,
        predicate_type: statement.predicate_type,
        subject_name: subject.name.clone(),
        subject_digest: format!("sha256:{subject_hex}"),
        source_repository: source.repository.clone(),
        source_revision: source_revision.clone(),
        workflow_path: source.path.clone(),
        builder_id: builder_id.clone(),
        verified: verification_state == VerificationState::Verified,
    })
}

pub fn bind_optional_slsa_v1_statement(
    statement_bytes: Option<&[u8]>,
    context: &SlsaBindingContext,
    verification_state: VerificationState,
) -> Result<Option<BoundSlsaProvenance>, BindingError> {
    statement_bytes
        .map(|bytes| bind_slsa_v1_statement(bytes, context, verification_state))
        .transpose()
}

pub fn attach_verified_reference(
    input: &mut VerificationInput,
    bound: &BoundSlsaProvenance,
) -> Result<(), BindingError> {
    input.slsa_provenance = Some(bound.to_provenance_reference()?);
    Ok(())
}
