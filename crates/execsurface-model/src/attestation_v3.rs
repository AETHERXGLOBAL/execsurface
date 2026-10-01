//! Bounded attestation/provenance bindings for the opt-in Semantics v3 candidate.
//!
//! Cryptographic/provenance validity is intentionally orthogonal to semantic
//! authority. Signer/workflow metadata never manufactures PASS eligibility.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::authority_v3::AuthorityRecord;
use crate::semantics_v3::ProofRequirement;

pub const SLSA_PROVENANCE_V1: &str = "https://slsa.dev/provenance/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationVerdict {
    Pass,
    Review,
    Block,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObserverHealth {
    Healthy,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DigestBinding {
    pub value: String,
}

impl DigestBinding {
    pub fn sha256(hex: impl Into<String>) -> Self {
        Self {
            value: format!("sha256:{}", hex.into()),
        }
    }

    pub fn is_valid(&self) -> bool {
        let Some(hex) = self.value.strip_prefix("sha256:") else {
            return false;
        };
        hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceBinding {
    pub predicate_type: String,
    pub statement_digest: DigestBinding,
    pub subject_digest: DigestBinding,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationBindings {
    pub subject: DigestBinding,
    pub source: DigestBinding,
    pub baseline: DigestBinding,
    pub current_surface: DigestBinding,
    pub evidence: DigestBinding,
    pub verifier: DigestBinding,
    pub provenance: Option<ProvenanceBinding>,
    /// Informational only; never an input to semantic PASS eligibility.
    pub workflow_label: Option<String>,
    /// Informational only; never an input to semantic PASS eligibility.
    pub signer_label: Option<String>,
}

impl VerificationBindings {
    pub fn validate(&self) -> Result<(), String> {
        for (name, digest) in [
            ("subject", &self.subject),
            ("source", &self.source),
            ("baseline", &self.baseline),
            ("current_surface", &self.current_surface),
            ("evidence", &self.evidence),
            ("verifier", &self.verifier),
        ] {
            if !digest.is_valid() {
                return Err(format!(
                    "{name} digest must be sha256:<64 hexadecimal characters>"
                ));
            }
        }

        if let Some(provenance) = &self.provenance {
            if provenance.predicate_type != SLSA_PROVENANCE_V1 {
                return Err("provenance predicate type must be SLSA provenance v1".to_owned());
            }
            if !provenance.statement_digest.is_valid() {
                return Err("provenance statement digest is malformed".to_owned());
            }
            if !provenance.subject_digest.is_valid() {
                return Err("provenance subject digest is malformed".to_owned());
            }
            if provenance.subject_digest != self.subject {
                return Err("provenance subject does not match bound subject".to_owned());
            }
        }

        Ok(())
    }

    /// Exact security-binding equality. Informational labels are deliberately
    /// excluded because they cannot carry semantic authority.
    pub fn binding_material_eq(&self, expected: &Self) -> bool {
        self.subject == expected.subject
            && self.source == expected.source
            && self.baseline == expected.baseline
            && self.current_surface == expected.current_surface
            && self.evidence == expected.evidence
            && self.verifier == expected.verifier
            && self.provenance == expected.provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttestedVerification {
    pub authority: AuthorityRecord,
    pub observer_health: ObserverHealth,
    pub verdict: VerificationVerdict,
    pub bindings: VerificationBindings,
}

impl AttestedVerification {
    pub fn validate(&self) -> Result<(), String> {
        self.authority.validate()?;
        self.bindings.validate()?;
        Ok(())
    }

    pub fn semantic_pass_admissible(&self, requirement: &ProofRequirement) -> bool {
        self.verdict == VerificationVerdict::Pass
            && self.observer_health == ObserverHealth::Healthy
            && self.bindings.validate().is_ok()
            && self.authority.admissible_for(requirement)
    }

    pub fn provenance_verified(&self) -> Option<bool> {
        self.bindings
            .provenance
            .as_ref()
            .map(|provenance| provenance.verified)
    }

    pub fn matches_expected_bindings(&self, expected: &VerificationBindings) -> bool {
        self.bindings.validate().is_ok()
            && expected.validate().is_ok()
            && self.bindings.binding_material_eq(expected)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingRole {
    Subject,
    Source,
    Baseline,
    CurrentSurface,
    Evidence,
    Verifier,
    Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ManifestItem {
    pub role: BindingRole,
    pub digest: DigestBinding,
}

pub fn manifest_for(bindings: &VerificationBindings) -> Result<Vec<ManifestItem>, String> {
    bindings.validate()?;
    let mut manifest = vec![
        ManifestItem {
            role: BindingRole::Subject,
            digest: bindings.subject.clone(),
        },
        ManifestItem {
            role: BindingRole::Source,
            digest: bindings.source.clone(),
        },
        ManifestItem {
            role: BindingRole::Baseline,
            digest: bindings.baseline.clone(),
        },
        ManifestItem {
            role: BindingRole::CurrentSurface,
            digest: bindings.current_surface.clone(),
        },
        ManifestItem {
            role: BindingRole::Evidence,
            digest: bindings.evidence.clone(),
        },
        ManifestItem {
            role: BindingRole::Verifier,
            digest: bindings.verifier.clone(),
        },
    ];
    if let Some(provenance) = &bindings.provenance {
        manifest.push(ManifestItem {
            role: BindingRole::Provenance,
            digest: provenance.statement_digest.clone(),
        });
    }
    manifest.sort();
    Ok(manifest)
}

pub fn verify_manifest(
    bindings: &VerificationBindings,
    manifest: &[ManifestItem],
) -> Result<(), String> {
    let expected = manifest_for(bindings)?;
    let roles = manifest
        .iter()
        .map(|item| item.role)
        .collect::<BTreeSet<_>>();
    if roles.len() != manifest.len() {
        return Err("duplicate manifest role".to_owned());
    }

    let mut actual = manifest.to_vec();
    actual.sort();
    if actual != expected {
        return Err("manifest binding mismatch".to_owned());
    }
    Ok(())
}
