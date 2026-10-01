//! Typed proposition-authority contract for the opt-in Semantics v3 candidate.
//!
//! Authority is carried by the typed proposition proof itself. Backend names,
//! labels, signer identity, or other metadata never manufacture authority.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::semantics_v3::{ProofCarryingObservation, ProofRequirement, Proposition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityState {
    Direct,
    DerivedBounded,
    AttemptOnly,
    Unsupported,
    Ambiguous,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropositionCompleteness {
    Complete,
    Incomplete,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReference {
    pub digest: String,
    pub derivation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRecord {
    pub proof: ProofCarryingObservation,
    pub authority: AuthorityState,
    pub completeness: PropositionCompleteness,
    pub evidence: EvidenceReference,
    pub reason_codes: BTreeSet<String>,
}

impl AuthorityRecord {
    fn has_valid_sha256_identity(value: &str) -> bool {
        let Some(hex) = value.strip_prefix("sha256:") else {
            return false;
        };
        hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
    }

    fn is_explicit_attempt(proposition: &Proposition) -> bool {
        matches!(
            proposition,
            Proposition::FilePathnameAttemptObserved { .. }
                | Proposition::FileRenameAttemptObserved { .. }
                | Proposition::NetworkConnectDestinationAttemptObserved { .. }
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if !Self::has_valid_sha256_identity(&self.evidence.digest) {
            return Err("evidence digest must be sha256:<64 hexadecimal characters>".to_owned());
        }

        match (self.authority, self.completeness) {
            (AuthorityState::Unsupported, PropositionCompleteness::NotApplicable) => {}
            (AuthorityState::Unsupported, _) => {
                return Err("unsupported authority requires not_applicable completeness".to_owned());
            }
            (_, PropositionCompleteness::NotApplicable) => {
                return Err("not_applicable completeness requires unsupported authority".to_owned());
            }
            (
                AuthorityState::Ambiguous | AuthorityState::Lost,
                PropositionCompleteness::Complete,
            ) => {
                return Err("ambiguous/lost authority cannot be complete".to_owned());
            }
            _ => {}
        }

        if matches!(
            self.authority,
            AuthorityState::Unsupported | AuthorityState::Ambiguous | AuthorityState::Lost
        ) && self.reason_codes.is_empty()
        {
            return Err("unsupported/ambiguous/lost authority requires a reason code".to_owned());
        }

        if self.authority == AuthorityState::AttemptOnly
            && !Self::is_explicit_attempt(&self.proof.proposition)
        {
            return Err("attempt_only authority requires an explicit attempt proposition".to_owned());
        }

        if self.authority == AuthorityState::DerivedBounded
            && self
                .evidence
                .derivation
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .is_none()
        {
            return Err("derived_bounded authority requires derivation identity".to_owned());
        }

        Ok(())
    }

    pub fn admissible_for(&self, requirement: &ProofRequirement) -> bool {
        self.validate().is_ok()
            && self.completeness == PropositionCompleteness::Complete
            && matches!(
                self.authority,
                AuthorityState::Direct | AuthorityState::DerivedBounded | AuthorityState::AttemptOnly
            )
            && self.proof.satisfies(requirement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonResult {
    BothSatisfy,
    LeftOnly,
    RightOnly,
    Neither,
    DifferentProposition,
}

pub fn compare_under_requirement(
    left: &AuthorityRecord,
    right: &AuthorityRecord,
    requirement: &ProofRequirement,
) -> ComparisonResult {
    if left.proof.proposition != right.proof.proposition {
        return ComparisonResult::DifferentProposition;
    }

    match (
        left.admissible_for(requirement),
        right.admissible_for(requirement),
    ) {
        (true, true) => ComparisonResult::BothSatisfy,
        (true, false) => ComparisonResult::LeftOnly,
        (false, true) => ComparisonResult::RightOnly,
        (false, false) => ComparisonResult::Neither,
    }
}
