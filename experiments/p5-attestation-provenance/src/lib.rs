use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";
pub const RUNTIME_TRACE_TYPE: &str = "https://in-toto.io/attestation/runtime-trace/v0.1";
pub const SCAI_TYPE: &str = "https://in-toto.io/attestation/scai/v0.3";
pub const SVR_TYPE: &str = "https://in-toto.io/attestation/svr/v0.2";
pub const SLSA_PROVENANCE_TYPE: &str = "https://slsa.dev/provenance/v1";
pub const EXECSURFACE_MONITOR_TYPE: &str =
    "https://github.com/AETHERXGLOBAL/execsurface/monitor/v1";
pub const EXECSURFACE_PROCESS_TYPE: &str =
    "https://github.com/AETHERXGLOBAL/execsurface/operation/command/v1";
pub const EXECSURFACE_ATTRIBUTE: &str =
    "https://github.com/AETHERXGLOBAL/execsurface/attestation/attributes/runtime-behavior-verification/v1";
pub const SVR_RECORDED_PROPERTY: &str = "EXECSURFACE_VERIFICATION_RECORDED_V1";
pub const SVR_PASS_PROPERTY: &str = "EXECSURFACE_RUNTIME_BEHAVIOR_PASS_V1";
const SVR_SCAI_BINDING_PREFIX: &str = "EXECSURFACE_SCAI_SHA256_";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub digest: BTreeMap<String, String>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

impl ResourceDescriptor {
    pub fn sha256(name: impl Into<String>, hex_digest: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            uri: None,
            digest: BTreeMap::from([("sha256".to_owned(), hex_digest.into())]),
            media_type: None,
        }
    }

    pub fn sha256_hex(&self) -> Option<&str> {
        self.digest.get("sha256").map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Verdict {
    Pass,
    Review,
    Block,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObserverHealth {
    Healthy,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorityState {
    Direct,
    Ambiguous,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompletenessClass {
    Complete,
    Incomplete,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityState {
    pub capabilities: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompletenessState {
    pub class: CompletenessClass,
    pub reason_codes: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceReference {
    pub predicate_type: String,
    pub statement_digest: String,
    pub subject_digest: String,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationInput {
    pub subject: ResourceDescriptor,
    pub command_identity: String,
    pub host_identity: String,
    pub source_identity: Option<ResourceDescriptor>,
    pub artifact_identity: Option<ResourceDescriptor>,
    pub workflow_identity: Option<ResourceDescriptor>,
    pub baseline_digest: String,
    pub current_surface_digest: String,
    pub observer_profile: String,
    pub capability_state: CapabilityState,
    pub observer_health: ObserverHealth,
    pub authority: AuthorityState,
    pub completeness: CompletenessState,
    pub policy: ResourceDescriptor,
    pub verdict: Verdict,
    pub evidence_digest: String,
    pub verifier: ResourceDescriptor,
    pub verifier_id: String,
    pub created_at: String,
    pub slsa_provenance: Option<ProvenanceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelError {
    pub reason_code: String,
}

impl ModelError {
    fn new(reason_code: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Statement<T> {
    #[serde(rename = "_type")]
    pub statement_type: String,
    pub subject: Vec<ResourceDescriptor>,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub predicate: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeTracePredicate {
    pub monitor: RuntimeMonitor,
    #[serde(rename = "monitoredProcess")]
    pub monitored_process: MonitoredProcess,
    #[serde(rename = "monitorLog")]
    pub monitor_log: MonitorLog,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeMonitor {
    #[serde(rename = "type")]
    pub monitor_type: String,
    #[serde(rename = "configSource")]
    pub config_source: ResourceDescriptor,
    #[serde(rename = "tracePolicy")]
    pub trace_policy: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MonitoredProcess {
    #[serde(rename = "hostID")]
    pub host_id: String,
    #[serde(rename = "type")]
    pub process_type: String,
    pub event: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MonitorLog {
    pub process: Vec<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScaiPredicate {
    pub attributes: Vec<ScaiAttribute>,
    pub producer: ResourceDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScaiAttribute {
    pub attribute: String,
    pub target: ResourceDescriptor,
    pub conditions: VerificationConditions,
    pub evidence: ResourceDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerificationConditions {
    #[serde(rename = "commandIdentity")]
    pub command_identity: String,
    #[serde(rename = "hostIdentity")]
    pub host_identity: String,
    #[serde(rename = "sourceIdentity")]
    pub source_identity: Option<ResourceDescriptor>,
    #[serde(rename = "artifactIdentity")]
    pub artifact_identity: Option<ResourceDescriptor>,
    #[serde(rename = "workflowIdentity")]
    pub workflow_identity: Option<ResourceDescriptor>,
    #[serde(rename = "baselineDigest")]
    pub baseline_digest: String,
    #[serde(rename = "currentSurfaceDigest")]
    pub current_surface_digest: String,
    #[serde(rename = "observerProfile")]
    pub observer_profile: String,
    #[serde(rename = "capabilityState")]
    pub capability_state: CapabilityState,
    #[serde(rename = "capabilityDigest")]
    pub capability_digest: String,
    #[serde(rename = "observerHealth")]
    pub observer_health: ObserverHealth,
    pub authority: AuthorityState,
    pub completeness: CompletenessState,
    #[serde(rename = "completenessDigest")]
    pub completeness_digest: String,
    #[serde(rename = "policyDigest")]
    pub policy_digest: String,
    pub verdict: Verdict,
    #[serde(rename = "evidenceDigest")]
    pub evidence_digest: String,
    #[serde(rename = "runtimeTraceStatementDigest")]
    pub runtime_trace_statement_digest: String,
    #[serde(rename = "slsaProvenance")]
    pub slsa_provenance: Option<ProvenanceReference>,
    #[serde(rename = "verifierIdentity")]
    pub verifier_identity: ResourceDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SvrPredicate {
    pub verifier: SvrVerifier,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SvrVerifier {
    pub id: String,
    pub policies: Vec<ResourceDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerificationBundle {
    pub runtime_trace: Statement<RuntimeTracePredicate>,
    pub scai: Statement<ScaiPredicate>,
    pub svr: Statement<SvrPredicate>,
    pub runtime_trace_digest: String,
    pub scai_digest: String,
    pub svr_digest: String,
    pub bundle_digest: String,
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn validate_labeled_digest(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(is_sha256_hex)
}

fn descriptor_digest(descriptor: &ResourceDescriptor) -> Result<String, ModelError> {
    let value = descriptor
        .sha256_hex()
        .ok_or_else(|| ModelError::new("resource_sha256_missing"))?;
    if !is_sha256_hex(value) {
        return Err(ModelError::new("resource_sha256_malformed"));
    }
    Ok(format!("sha256:{value}"))
}

fn canonical_digest<T: Serialize>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("typed deterministic JSON serialization");
    let mut hasher = Sha256::new();
    hasher.update(b"execsurface:p5:a0:");
    hasher.update(domain.as_bytes());
    hasher.update(b":v1\0");
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}

fn evidence_descriptor(name: &str, labeled_digest: &str) -> ResourceDescriptor {
    let hex = labeled_digest
        .strip_prefix("sha256:")
        .expect("validated internal digest");
    ResourceDescriptor {
        name: Some(name.to_owned()),
        uri: None,
        digest: BTreeMap::from([("sha256".to_owned(), hex.to_owned())]),
        media_type: Some("application/vnd.in-toto+json".to_owned()),
    }
}

fn validate_input(input: &VerificationInput) -> Result<(), ModelError> {
    if input.command_identity.is_empty() {
        return Err(ModelError::new("command_identity_missing"));
    }
    if input.host_identity.is_empty() {
        return Err(ModelError::new("host_identity_missing"));
    }
    if input.observer_profile.is_empty() {
        return Err(ModelError::new("observer_profile_missing"));
    }
    if input.verifier_id.is_empty() {
        return Err(ModelError::new("verifier_id_missing"));
    }
    if input.created_at.is_empty() {
        return Err(ModelError::new("verification_time_missing"));
    }
    if input.capability_state.capabilities.is_empty() {
        return Err(ModelError::new("observer_capabilities_missing"));
    }

    let subject_digest = descriptor_digest(&input.subject)?;
    let _ = descriptor_digest(&input.policy)?;
    let _ = descriptor_digest(&input.verifier)?;

    for digest in [
        &input.baseline_digest,
        &input.current_surface_digest,
        &input.evidence_digest,
    ] {
        if !validate_labeled_digest(digest) {
            return Err(ModelError::new("semantic_digest_malformed"));
        }
    }

    if let Some(artifact) = &input.artifact_identity {
        let _ = descriptor_digest(artifact)?;
        if artifact != &input.subject {
            return Err(ModelError::new("artifact_subject_mismatch"));
        }
    }
    if let Some(source) = &input.source_identity {
        let _ = descriptor_digest(source)?;
    }
    if let Some(workflow) = &input.workflow_identity {
        let _ = descriptor_digest(workflow)?;
    }

    if let Some(provenance) = &input.slsa_provenance {
        if provenance.predicate_type != SLSA_PROVENANCE_TYPE {
            return Err(ModelError::new("slsa_predicate_type_mismatch"));
        }
        if !validate_labeled_digest(&provenance.statement_digest) {
            return Err(ModelError::new("slsa_statement_digest_malformed"));
        }
        if provenance.subject_digest != subject_digest {
            return Err(ModelError::new("slsa_subject_mismatch"));
        }
    }

    if input.verdict == Verdict::Pass {
        if input.observer_health != ObserverHealth::Healthy {
            return Err(ModelError::new("pass_blocked_by_observer_loss"));
        }
        if input.authority != AuthorityState::Direct {
            return Err(ModelError::new("pass_blocked_by_non_direct_authority"));
        }
        if input.completeness.class != CompletenessClass::Complete {
            return Err(ModelError::new("pass_blocked_by_incomplete_evidence"));
        }
    }

    Ok(())
}

pub fn parse_strict_input(json: &str) -> Result<VerificationInput, ModelError> {
    serde_json::from_str(json).map_err(|_| ModelError::new("verification_input_schema_rejected"))
}

pub fn build_bundle(input: &VerificationInput) -> Result<VerificationBundle, ModelError> {
    validate_input(input)?;

    let capability_digest = canonical_digest("capability-state", &input.capability_state);
    let completeness_digest = canonical_digest("completeness-state", &input.completeness);
    let policy_digest = descriptor_digest(&input.policy)?;

    let trace_policy = BTreeMap::from([
        ("authority".to_owned(), format!("{:?}", input.authority)),
        ("capabilityDigest".to_owned(), capability_digest.clone()),
        ("completenessDigest".to_owned(), completeness_digest.clone()),
        ("evidenceDigest".to_owned(), input.evidence_digest.clone()),
        ("observerProfile".to_owned(), input.observer_profile.clone()),
    ]);
    let process_record = BTreeMap::from([
        ("commandIdentity".to_owned(), input.command_identity.clone()),
        ("evidenceDigest".to_owned(), input.evidence_digest.clone()),
        ("observerProfile".to_owned(), input.observer_profile.clone()),
    ]);
    let runtime_trace = Statement {
        statement_type: STATEMENT_TYPE.to_owned(),
        subject: vec![input.subject.clone()],
        predicate_type: RUNTIME_TRACE_TYPE.to_owned(),
        predicate: RuntimeTracePredicate {
            monitor: RuntimeMonitor {
                monitor_type: EXECSURFACE_MONITOR_TYPE.to_owned(),
                config_source: input.policy.clone(),
                trace_policy,
            },
            monitored_process: MonitoredProcess {
                host_id: input.host_identity.clone(),
                process_type: EXECSURFACE_PROCESS_TYPE.to_owned(),
                event: input.command_identity.clone(),
            },
            monitor_log: MonitorLog {
                process: vec![process_record],
            },
        },
    };
    let runtime_trace_digest = canonical_digest("runtime-trace-statement", &runtime_trace);

    let conditions = VerificationConditions {
        command_identity: input.command_identity.clone(),
        host_identity: input.host_identity.clone(),
        source_identity: input.source_identity.clone(),
        artifact_identity: input.artifact_identity.clone(),
        workflow_identity: input.workflow_identity.clone(),
        baseline_digest: input.baseline_digest.clone(),
        current_surface_digest: input.current_surface_digest.clone(),
        observer_profile: input.observer_profile.clone(),
        capability_state: input.capability_state.clone(),
        capability_digest,
        observer_health: input.observer_health,
        authority: input.authority,
        completeness: input.completeness.clone(),
        completeness_digest,
        policy_digest,
        verdict: input.verdict,
        evidence_digest: input.evidence_digest.clone(),
        runtime_trace_statement_digest: runtime_trace_digest.clone(),
        slsa_provenance: input.slsa_provenance.clone(),
        verifier_identity: input.verifier.clone(),
    };
    let scai = Statement {
        statement_type: STATEMENT_TYPE.to_owned(),
        subject: vec![input.subject.clone()],
        predicate_type: SCAI_TYPE.to_owned(),
        predicate: ScaiPredicate {
            attributes: vec![ScaiAttribute {
                attribute: EXECSURFACE_ATTRIBUTE.to_owned(),
                target: input.subject.clone(),
                conditions,
                evidence: evidence_descriptor(
                    "execsurface-runtime-trace.statement.json",
                    &runtime_trace_digest,
                ),
            }],
            producer: input.verifier.clone(),
        },
    };
    let scai_digest = canonical_digest("scai-statement", &scai);

    let scai_hex = scai_digest
        .strip_prefix("sha256:")
        .expect("internal digest has sha256 label");
    let mut properties = BTreeSet::from([
        SVR_RECORDED_PROPERTY.to_owned(),
        format!("{SVR_SCAI_BINDING_PREFIX}{scai_hex}"),
    ]);
    if input.verdict == Verdict::Pass {
        properties.insert(SVR_PASS_PROPERTY.to_owned());
    }
    let svr = Statement {
        statement_type: STATEMENT_TYPE.to_owned(),
        subject: vec![input.subject.clone()],
        predicate_type: SVR_TYPE.to_owned(),
        predicate: SvrPredicate {
            verifier: SvrVerifier {
                id: input.verifier_id.clone(),
                policies: vec![input.policy.clone()],
            },
            time_created: input.created_at.clone(),
            properties: properties.into_iter().collect(),
        },
    };
    let svr_digest = canonical_digest("svr-statement", &svr);
    let bundle_digest = bundle_identity_digest(&runtime_trace_digest, &scai_digest, &svr_digest);

    let bundle = VerificationBundle {
        runtime_trace,
        scai,
        svr,
        runtime_trace_digest,
        scai_digest,
        svr_digest,
        bundle_digest,
    };
    verify_bundle(&bundle)?;
    Ok(bundle)
}

fn bundle_identity_digest(runtime_trace_digest: &str, scai_digest: &str, svr_digest: &str) -> String {
    #[derive(Serialize)]
    struct BundleIdentity<'a> {
        runtime_trace_digest: &'a str,
        scai_digest: &'a str,
        svr_digest: &'a str,
    }

    canonical_digest(
        "verification-bundle",
        &BundleIdentity {
            runtime_trace_digest,
            scai_digest,
            svr_digest,
        },
    )
}

pub fn recompute_outer_digests_unchecked(bundle: &mut VerificationBundle) {
    bundle.runtime_trace_digest = canonical_digest("runtime-trace-statement", &bundle.runtime_trace);
    bundle.scai_digest = canonical_digest("scai-statement", &bundle.scai);
    bundle.svr_digest = canonical_digest("svr-statement", &bundle.svr);
    bundle.bundle_digest = bundle_identity_digest(
        &bundle.runtime_trace_digest,
        &bundle.scai_digest,
        &bundle.svr_digest,
    );
}

pub fn verify_bundle(bundle: &VerificationBundle) -> Result<(), ModelError> {
    if bundle.runtime_trace.statement_type != STATEMENT_TYPE
        || bundle.scai.statement_type != STATEMENT_TYPE
        || bundle.svr.statement_type != STATEMENT_TYPE
    {
        return Err(ModelError::new("statement_type_mismatch"));
    }
    if bundle.runtime_trace.predicate_type != RUNTIME_TRACE_TYPE
        || bundle.scai.predicate_type != SCAI_TYPE
        || bundle.svr.predicate_type != SVR_TYPE
    {
        return Err(ModelError::new("predicate_type_mismatch"));
    }
    if bundle.runtime_trace.subject != bundle.scai.subject
        || bundle.runtime_trace.subject != bundle.svr.subject
    {
        return Err(ModelError::new("cross_statement_subject_mismatch"));
    }
    if bundle.runtime_trace.subject.len() != 1 {
        return Err(ModelError::new("unexpected_subject_cardinality"));
    }

    let runtime_digest = canonical_digest("runtime-trace-statement", &bundle.runtime_trace);
    let scai_digest = canonical_digest("scai-statement", &bundle.scai);
    let svr_digest = canonical_digest("svr-statement", &bundle.svr);
    if runtime_digest != bundle.runtime_trace_digest
        || scai_digest != bundle.scai_digest
        || svr_digest != bundle.svr_digest
    {
        return Err(ModelError::new("statement_digest_mismatch"));
    }

    let [attribute] = bundle.scai.predicate.attributes.as_slice() else {
        return Err(ModelError::new("scai_attribute_cardinality_mismatch"));
    };
    if attribute.attribute != EXECSURFACE_ATTRIBUTE {
        return Err(ModelError::new("scai_attribute_mismatch"));
    }
    if attribute.target != bundle.runtime_trace.subject[0] {
        return Err(ModelError::new("scai_target_subject_mismatch"));
    }

    if bundle.svr.predicate.verifier.policies.len() != 1 {
        return Err(ModelError::new("svr_policy_cardinality_mismatch"));
    }
    let policy = &bundle.svr.predicate.verifier.policies[0];
    if attribute.conditions.policy_digest != descriptor_digest(policy)? {
        return Err(ModelError::new("policy_binding_mismatch"));
    }

    let monitor = &bundle.runtime_trace.predicate.monitor;
    if monitor.monitor_type != EXECSURFACE_MONITOR_TYPE {
        return Err(ModelError::new("runtime_monitor_type_mismatch"));
    }
    if monitor.config_source != *policy {
        return Err(ModelError::new("runtime_monitor_config_policy_mismatch"));
    }

    let expected_capability_digest =
        canonical_digest("capability-state", &attribute.conditions.capability_state);
    if expected_capability_digest != attribute.conditions.capability_digest {
        return Err(ModelError::new("capability_digest_mismatch"));
    }
    let expected_completeness_digest =
        canonical_digest("completeness-state", &attribute.conditions.completeness);
    if expected_completeness_digest != attribute.conditions.completeness_digest {
        return Err(ModelError::new("completeness_digest_mismatch"));
    }

    let expected_trace_policy = BTreeMap::from([
        (
            "authority".to_owned(),
            format!("{:?}", attribute.conditions.authority),
        ),
        (
            "capabilityDigest".to_owned(),
            attribute.conditions.capability_digest.clone(),
        ),
        (
            "completenessDigest".to_owned(),
            attribute.conditions.completeness_digest.clone(),
        ),
        (
            "evidenceDigest".to_owned(),
            attribute.conditions.evidence_digest.clone(),
        ),
        (
            "observerProfile".to_owned(),
            attribute.conditions.observer_profile.clone(),
        ),
    ]);
    if monitor.trace_policy != expected_trace_policy {
        return Err(ModelError::new("runtime_trace_policy_binding_mismatch"));
    }

    let monitored_process = &bundle.runtime_trace.predicate.monitored_process;
    if monitored_process.process_type != EXECSURFACE_PROCESS_TYPE {
        return Err(ModelError::new("runtime_process_type_mismatch"));
    }
    if monitored_process.host_id != attribute.conditions.host_identity {
        return Err(ModelError::new("runtime_host_identity_mismatch"));
    }
    if monitored_process.event != attribute.conditions.command_identity {
        return Err(ModelError::new("runtime_command_identity_mismatch"));
    }

    let [process_record] = bundle.runtime_trace.predicate.monitor_log.process.as_slice() else {
        return Err(ModelError::new("runtime_process_log_cardinality_mismatch"));
    };
    let expected_process_record = BTreeMap::from([
        (
            "commandIdentity".to_owned(),
            attribute.conditions.command_identity.clone(),
        ),
        (
            "evidenceDigest".to_owned(),
            attribute.conditions.evidence_digest.clone(),
        ),
        (
            "observerProfile".to_owned(),
            attribute.conditions.observer_profile.clone(),
        ),
    ]);
    if process_record != &expected_process_record {
        return Err(ModelError::new("runtime_process_log_binding_mismatch"));
    }

    if attribute.conditions.runtime_trace_statement_digest != runtime_digest {
        return Err(ModelError::new("runtime_trace_binding_mismatch"));
    }
    if descriptor_digest(&attribute.evidence)? != runtime_digest {
        return Err(ModelError::new("runtime_trace_evidence_mismatch"));
    }

    let scai_hex = scai_digest
        .strip_prefix("sha256:")
        .expect("internal digest has sha256 label");
    let expected_binding = format!("{SVR_SCAI_BINDING_PREFIX}{scai_hex}");
    if !bundle.svr.predicate.properties.contains(&expected_binding) {
        return Err(ModelError::new("svr_scai_binding_missing"));
    }
    let has_pass = bundle
        .svr
        .predicate
        .properties
        .iter()
        .any(|property| property == SVR_PASS_PROPERTY);
    if has_pass != (attribute.conditions.verdict == Verdict::Pass) {
        return Err(ModelError::new("svr_verdict_binding_mismatch"));
    }

    if attribute.conditions.verdict == Verdict::Pass {
        if attribute.conditions.observer_health != ObserverHealth::Healthy {
            return Err(ModelError::new("pass_laundered_over_observer_loss"));
        }
        if attribute.conditions.authority != AuthorityState::Direct {
            return Err(ModelError::new("pass_laundered_over_weak_authority"));
        }
        if attribute.conditions.completeness.class != CompletenessClass::Complete {
            return Err(ModelError::new("pass_laundered_over_incompleteness"));
        }
    }

    if let Some(provenance) = &attribute.conditions.slsa_provenance {
        if provenance.predicate_type != SLSA_PROVENANCE_TYPE {
            return Err(ModelError::new("slsa_predicate_type_mismatch"));
        }
        if !validate_labeled_digest(&provenance.statement_digest) {
            return Err(ModelError::new("slsa_statement_digest_malformed"));
        }
        let subject_digest = descriptor_digest(&bundle.runtime_trace.subject[0])?;
        if provenance.subject_digest != subject_digest {
            return Err(ModelError::new("slsa_subject_mismatch"));
        }
    }

    let expected_bundle_digest =
        bundle_identity_digest(&bundle.runtime_trace_digest, &bundle.scai_digest, &bundle.svr_digest);
    if expected_bundle_digest != bundle.bundle_digest {
        return Err(ModelError::new("bundle_digest_mismatch"));
    }

    Ok(())
}

pub fn verify_bundle_for_subject(
    bundle: &VerificationBundle,
    expected_subject: &ResourceDescriptor,
) -> Result<(), ModelError> {
    verify_bundle(bundle)?;
    if bundle.runtime_trace.subject.len() != 1
        || bundle.runtime_trace.subject.first() != Some(expected_subject)
    {
        return Err(ModelError::new("expected_subject_mismatch"));
    }
    Ok(())
}

pub fn verify_bundle_for_context(
    bundle: &VerificationBundle,
    expected_workflow: &ResourceDescriptor,
    expected_command: &str,
    expected_host: &str,
) -> Result<(), ModelError> {
    verify_bundle(bundle)?;
    let [attribute] = bundle.scai.predicate.attributes.as_slice() else {
        return Err(ModelError::new("scai_attribute_cardinality_mismatch"));
    };
    if attribute.conditions.workflow_identity.as_ref() != Some(expected_workflow) {
        return Err(ModelError::new("expected_workflow_mismatch"));
    }
    if attribute.conditions.command_identity != expected_command {
        return Err(ModelError::new("expected_command_mismatch"));
    }
    if attribute.conditions.host_identity != expected_host {
        return Err(ModelError::new("expected_host_mismatch"));
    }
    Ok(())
}

pub fn digest_bundle(bundle: &VerificationBundle) -> &str {
    &bundle.bundle_digest
}
