use std::collections::{BTreeMap, BTreeSet};

use execsurface_p5_attestation_provenance::{
    verify_bundle, AuthorityState, CapabilityState, CompletenessState, ObserverHealth,
    ProvenanceReference, ResourceDescriptor, VerificationBundle, Verdict,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub const ROLE_RUNTIME_TRACE: &str = "runtime-trace";
pub const ROLE_SCAI: &str = "scai";
pub const ROLE_SVR: &str = "svr";
pub const ROLE_PROVENANCE: &str = "slsa-provenance";

const GRAPH_DIGEST_DOMAIN: &[u8] = b"execsurface:p5:a5:graph-manifest:v1\0";
const ALLOWED_ROLES: [&str; 4] = [ROLE_RUNTIME_TRACE, ROLE_SCAI, ROLE_SVR, ROLE_PROVENANCE];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphError {
    pub reason_code: String,
}

impl GraphError {
    fn new(reason_code: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedVerificationContext {
    pub subject: ResourceDescriptor,
    pub source_identity: Option<ResourceDescriptor>,
    pub artifact_identity: Option<ResourceDescriptor>,
    pub workflow_identity: Option<ResourceDescriptor>,
    pub command_identity: String,
    pub host_identity: String,
    pub baseline_digest: String,
    pub current_surface_digest: String,
    pub evidence_digest: String,
    pub observer_profile: String,
    pub capability_state: CapabilityState,
    pub observer_health: ObserverHealth,
    pub policy: ResourceDescriptor,
    pub authority: AuthorityState,
    pub completeness: CompletenessState,
    pub verdict: Verdict,
    pub verifier: ResourceDescriptor,
    pub verifier_id: String,
    pub created_at: String,
    pub provenance: Option<ProvenanceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GraphItem {
    pub role: String,
    pub digest: String,
}

pub fn manifest_for_bundle(bundle: &VerificationBundle) -> Result<Vec<GraphItem>, GraphError> {
    let [attribute] = bundle.scai.predicate.attributes.as_slice() else {
        return Err(GraphError::new("scai_attribute_cardinality_mismatch"));
    };

    let mut items = vec![
        GraphItem {
            role: ROLE_RUNTIME_TRACE.to_owned(),
            digest: bundle.runtime_trace_digest.clone(),
        },
        GraphItem {
            role: ROLE_SCAI.to_owned(),
            digest: bundle.scai_digest.clone(),
        },
        GraphItem {
            role: ROLE_SVR.to_owned(),
            digest: bundle.svr_digest.clone(),
        },
    ];
    if let Some(provenance) = &attribute.conditions.slsa_provenance {
        items.push(GraphItem {
            role: ROLE_PROVENANCE.to_owned(),
            digest: provenance.statement_digest.clone(),
        });
    }
    Ok(items)
}

fn canonicalize_manifest(items: &[GraphItem]) -> Result<Vec<GraphItem>, GraphError> {
    let mut seen = BTreeSet::new();
    for item in items {
        if !ALLOWED_ROLES.contains(&item.role.as_str()) {
            return Err(GraphError::new("unknown_graph_role"));
        }
        if !seen.insert(item.role.as_str()) {
            return Err(GraphError::new("duplicate_graph_role"));
        }
        if !item.digest.starts_with("sha256:") || item.digest.len() != 71 {
            return Err(GraphError::new("graph_digest_malformed"));
        }
    }

    let mut canonical = items.to_vec();
    canonical.sort_by(|left, right| left.role.cmp(&right.role));
    Ok(canonical)
}

fn verify_svr_properties_canonical(bundle: &VerificationBundle) -> Result<(), GraphError> {
    if bundle
        .svr
        .predicate
        .properties
        .windows(2)
        .any(|pair| pair[0] >= pair[1])
    {
        return Err(GraphError::new("svr_properties_noncanonical"));
    }
    Ok(())
}

pub fn graph_manifest_digest(items: &[GraphItem]) -> Result<String, GraphError> {
    let canonical = canonicalize_manifest(items)?;
    let bytes = serde_json::to_vec(&canonical)
        .map_err(|_| GraphError::new("graph_manifest_serialization_failed"))?;
    let mut hasher = Sha256::new();
    hasher.update(GRAPH_DIGEST_DOMAIN);
    hasher.update(bytes);
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

pub fn verify_manifest(bundle: &VerificationBundle, items: &[GraphItem]) -> Result<(), GraphError> {
    let canonical = canonicalize_manifest(items)?;
    let expected = manifest_for_bundle(bundle)?;
    let expected_map: BTreeMap<&str, &str> = expected
        .iter()
        .map(|item| (item.role.as_str(), item.digest.as_str()))
        .collect();
    let supplied_map: BTreeMap<&str, &str> = canonical
        .iter()
        .map(|item| (item.role.as_str(), item.digest.as_str()))
        .collect();

    if supplied_map.len() != expected_map.len() {
        return Err(GraphError::new("graph_role_set_mismatch"));
    }
    for (role, expected_digest) in expected_map {
        match supplied_map.get(role) {
            Some(actual_digest) if *actual_digest == expected_digest => {}
            Some(_) => return Err(GraphError::new("graph_role_digest_mismatch")),
            None => return Err(GraphError::new("graph_role_set_mismatch")),
        }
    }
    Ok(())
}

pub fn verify_expected_context(
    bundle: &VerificationBundle,
    expected: &ExpectedVerificationContext,
) -> Result<(), GraphError> {
    verify_bundle(bundle).map_err(|error| GraphError::new(error.reason_code))?;
    verify_svr_properties_canonical(bundle)?;

    let [subject] = bundle.runtime_trace.subject.as_slice() else {
        return Err(GraphError::new("unexpected_subject_cardinality"));
    };
    if subject != &expected.subject {
        return Err(GraphError::new("expected_subject_mismatch"));
    }

    let [attribute] = bundle.scai.predicate.attributes.as_slice() else {
        return Err(GraphError::new("scai_attribute_cardinality_mismatch"));
    };
    let conditions = &attribute.conditions;

    if conditions.source_identity != expected.source_identity {
        return Err(GraphError::new("expected_source_mismatch"));
    }
    if conditions.artifact_identity != expected.artifact_identity {
        return Err(GraphError::new("expected_artifact_mismatch"));
    }
    if conditions.workflow_identity != expected.workflow_identity {
        return Err(GraphError::new("expected_workflow_mismatch"));
    }
    if conditions.command_identity != expected.command_identity {
        return Err(GraphError::new("expected_command_mismatch"));
    }
    if conditions.host_identity != expected.host_identity {
        return Err(GraphError::new("expected_host_mismatch"));
    }
    if conditions.baseline_digest != expected.baseline_digest {
        return Err(GraphError::new("expected_baseline_mismatch"));
    }
    if conditions.current_surface_digest != expected.current_surface_digest {
        return Err(GraphError::new("expected_current_surface_mismatch"));
    }
    if conditions.evidence_digest != expected.evidence_digest {
        return Err(GraphError::new("expected_evidence_mismatch"));
    }
    if conditions.observer_profile != expected.observer_profile {
        return Err(GraphError::new("expected_observer_profile_mismatch"));
    }
    if conditions.capability_state != expected.capability_state {
        return Err(GraphError::new("expected_capability_state_mismatch"));
    }
    if conditions.observer_health != expected.observer_health {
        return Err(GraphError::new("expected_observer_health_mismatch"));
    }
    if conditions.authority != expected.authority {
        return Err(GraphError::new("expected_authority_mismatch"));
    }
    if conditions.completeness != expected.completeness {
        return Err(GraphError::new("expected_completeness_mismatch"));
    }
    if conditions.verdict != expected.verdict {
        return Err(GraphError::new("expected_verdict_mismatch"));
    }
    if conditions.slsa_provenance != expected.provenance {
        return Err(GraphError::new("expected_provenance_mismatch"));
    }
    if conditions.verifier_identity != expected.verifier {
        return Err(GraphError::new("expected_verifier_identity_mismatch"));
    }

    let [policy] = bundle.svr.predicate.verifier.policies.as_slice() else {
        return Err(GraphError::new("svr_policy_cardinality_mismatch"));
    };
    if policy != &expected.policy {
        return Err(GraphError::new("expected_policy_mismatch"));
    }
    if bundle.svr.predicate.verifier.id != expected.verifier_id {
        return Err(GraphError::new("expected_verifier_id_mismatch"));
    }
    if bundle.svr.predicate.time_created != expected.created_at {
        return Err(GraphError::new("expected_verification_time_mismatch"));
    }

    Ok(())
}

pub fn verify_graph(
    bundle: &VerificationBundle,
    expected: &ExpectedVerificationContext,
    manifest: &[GraphItem],
) -> Result<(), GraphError> {
    verify_expected_context(bundle, expected)?;
    verify_manifest(bundle, manifest)?;
    Ok(())
}
