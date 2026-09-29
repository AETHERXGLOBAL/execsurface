use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

pub const SCHEMA_VERSION: u32 = 1;
pub const BACKEND_ID: &str = "ptrace-reference-research-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropositionId {
    ProcessCreateRelation,
    ExecSuccess,
    PathAccessAttempt,
    FileOpenObject,
    FdIoAttribution,
    FileRenameDelete,
    NetConnectDestination,
    CausalExecLineage,
    ObserverHealthLoss,
    FdTableRelation,
}

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
pub enum CompletenessState {
    Complete,
    Incomplete,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PropositionRecord {
    pub schema_version: u32,
    pub proposition: PropositionId,
    pub backend_id: String,
    pub subject_identity: String,
    pub object_identity: Option<String>,
    pub authority: AuthorityState,
    pub completeness: CompletenessState,
    pub evidence_reference: Option<String>,
    pub derivation_reference: Option<String>,
    pub causal_binding: Option<String>,
    pub ambiguity_or_loss_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropositionBundle {
    pub schema_version: u32,
    pub records: Vec<PropositionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    UnsupportedMustBeNotApplicable,
    NotApplicableRequiresUnsupported,
    AmbiguousOrLostCannotBeComplete,
    DirectOrAttemptRequiresEvidenceReference,
    DerivedRequiresDerivationReference,
    AmbiguousOrLostRequiresReason,
    EmptyBackendId,
    EmptySubjectIdentity,
    WrongSchemaVersion,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ValidationError {}

impl PropositionRecord {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(ValidationError::WrongSchemaVersion);
        }
        if self.backend_id.trim().is_empty() {
            return Err(ValidationError::EmptyBackendId);
        }
        if self.subject_identity.trim().is_empty() {
            return Err(ValidationError::EmptySubjectIdentity);
        }

        if self.authority == AuthorityState::Unsupported
            && self.completeness != CompletenessState::NotApplicable
        {
            return Err(ValidationError::UnsupportedMustBeNotApplicable);
        }
        if self.completeness == CompletenessState::NotApplicable
            && self.authority != AuthorityState::Unsupported
        {
            return Err(ValidationError::NotApplicableRequiresUnsupported);
        }
        if matches!(
            self.authority,
            AuthorityState::Ambiguous | AuthorityState::Lost
        ) && self.completeness == CompletenessState::Complete
        {
            return Err(ValidationError::AmbiguousOrLostCannotBeComplete);
        }
        if matches!(
            self.authority,
            AuthorityState::Direct | AuthorityState::AttemptOnly
        ) && self
            .evidence_reference
            .as_deref()
            .map(str::trim)
            .is_none_or(str::is_empty)
        {
            return Err(ValidationError::DirectOrAttemptRequiresEvidenceReference);
        }
        if self.authority == AuthorityState::DerivedBounded
            && self
                .derivation_reference
                .as_deref()
                .map(str::trim)
                .is_none_or(str::is_empty)
        {
            return Err(ValidationError::DerivedRequiresDerivationReference);
        }
        if matches!(
            self.authority,
            AuthorityState::Ambiguous | AuthorityState::Lost
        ) && self
            .ambiguity_or_loss_reason
            .as_deref()
            .map(str::trim)
            .is_none_or(str::is_empty)
        {
            return Err(ValidationError::AmbiguousOrLostRequiresReason);
        }

        Ok(())
    }
}

impl PropositionBundle {
    pub fn new(mut records: Vec<PropositionRecord>) -> Result<Self, ValidationError> {
        for record in &records {
            record.validate()?;
        }
        records.sort();
        Ok(Self {
            schema_version: SCHEMA_VERSION,
            records,
        })
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn sha256(&self) -> Result<String, serde_json::Error> {
        let bytes = self.canonical_json_bytes()?;
        let digest = Sha256::digest(bytes);
        Ok(format!("sha256:{digest:x}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_record(proposition: PropositionId) -> PropositionRecord {
        PropositionRecord {
            schema_version: SCHEMA_VERSION,
            proposition,
            backend_id: BACKEND_ID.to_owned(),
            subject_identity: "process:42".to_owned(),
            object_identity: None,
            authority: AuthorityState::Direct,
            completeness: CompletenessState::Complete,
            evidence_reference: Some("sha256:evidence".to_owned()),
            derivation_reference: None,
            causal_binding: None,
            ambiguity_or_loss_reason: None,
        }
    }

    #[test]
    fn states_roundtrip_without_semantic_loss() {
        let states = [
            AuthorityState::Direct,
            AuthorityState::DerivedBounded,
            AuthorityState::AttemptOnly,
            AuthorityState::Unsupported,
            AuthorityState::Ambiguous,
            AuthorityState::Lost,
        ];
        for state in states {
            let bytes = serde_json::to_vec(&state).expect("serialize authority state");
            let decoded: AuthorityState =
                serde_json::from_slice(&bytes).expect("deserialize authority state");
            assert_eq!(decoded, state);
        }
    }

    #[test]
    fn bundle_serialization_is_deterministic_across_input_order() {
        let a = base_record(PropositionId::ExecSuccess);
        let b = base_record(PropositionId::ProcessCreateRelation);

        let first = PropositionBundle::new(vec![a.clone(), b.clone()]).expect("first bundle");
        let second = PropositionBundle::new(vec![b, a]).expect("second bundle");

        assert_eq!(
            first.canonical_json_bytes().expect("first bytes"),
            second.canonical_json_bytes().expect("second bytes")
        );
        assert_eq!(
            first.sha256().expect("first hash"),
            second.sha256().expect("second hash")
        );
    }

    #[test]
    fn unsupported_cannot_claim_complete() {
        let mut record = base_record(PropositionId::FileOpenObject);
        record.authority = AuthorityState::Unsupported;
        record.completeness = CompletenessState::Complete;
        record.evidence_reference = None;
        assert_eq!(
            record.validate(),
            Err(ValidationError::UnsupportedMustBeNotApplicable)
        );
    }

    #[test]
    fn not_applicable_is_reserved_for_unsupported_propositions() {
        let mut record = base_record(PropositionId::FileOpenObject);
        record.completeness = CompletenessState::NotApplicable;
        assert_eq!(
            record.validate(),
            Err(ValidationError::NotApplicableRequiresUnsupported)
        );
    }

    #[test]
    fn ambiguous_and_lost_states_cannot_be_complete() {
        for authority in [AuthorityState::Ambiguous, AuthorityState::Lost] {
            let mut record = base_record(PropositionId::FdIoAttribution);
            record.authority = authority;
            record.completeness = CompletenessState::Complete;
            record.ambiguity_or_loss_reason = Some("counterexample".to_owned());
            assert_eq!(
                record.validate(),
                Err(ValidationError::AmbiguousOrLostCannotBeComplete)
            );
        }
    }

    #[test]
    fn direct_and_attempt_authority_require_evidence_reference() {
        for authority in [AuthorityState::Direct, AuthorityState::AttemptOnly] {
            let mut record = base_record(PropositionId::PathAccessAttempt);
            record.authority = authority;
            record.evidence_reference = None;
            assert_eq!(
                record.validate(),
                Err(ValidationError::DirectOrAttemptRequiresEvidenceReference)
            );
        }
    }

    #[test]
    fn bounded_derivation_requires_explicit_derivation_reference() {
        let mut record = base_record(PropositionId::CausalExecLineage);
        record.authority = AuthorityState::DerivedBounded;
        record.evidence_reference = None;
        record.derivation_reference = None;
        assert_eq!(
            record.validate(),
            Err(ValidationError::DerivedRequiresDerivationReference)
        );
    }

    #[test]
    fn ambiguity_and_loss_require_reason() {
        for authority in [AuthorityState::Ambiguous, AuthorityState::Lost] {
            let mut record = base_record(PropositionId::ObserverHealthLoss);
            record.authority = authority;
            record.completeness = CompletenessState::Incomplete;
            record.evidence_reference = None;
            record.ambiguity_or_loss_reason = None;
            assert_eq!(
                record.validate(),
                Err(ValidationError::AmbiguousOrLostRequiresReason)
            );
        }
    }

    #[test]
    fn unsupported_no_event_is_not_negative_proof() {
        let mut record = base_record(PropositionId::NetConnectDestination);
        record.authority = AuthorityState::Unsupported;
        record.completeness = CompletenessState::NotApplicable;
        record.evidence_reference = None;
        record.object_identity = None;
        record
            .validate()
            .expect("unsupported record remains explicit");
        assert_eq!(record.authority, AuthorityState::Unsupported);
        assert_ne!(record.completeness, CompletenessState::Complete);
    }

    #[test]
    fn incomplete_direct_evidence_remains_distinct_from_unsupported() {
        let mut record = base_record(PropositionId::FdTableRelation);
        record.completeness = CompletenessState::Incomplete;
        record.ambiguity_or_loss_reason = Some("dependent evidence incomplete".to_owned());
        record
            .validate()
            .expect("direct-but-incomplete remains representable");
        assert_eq!(record.authority, AuthorityState::Direct);
        assert_eq!(record.completeness, CompletenessState::Incomplete);
    }
}
