use std::collections::BTreeSet;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::b0_success_evidence::{EvidenceState, SuccessEvidenceRecord, TargetProposition};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PostOpenBinding {
    pub fd: i32,
    pub fd_generation: u64,
    pub originating_entry_sequence: u64,
    pub binding_sequence: u64,
    pub object_identity: String,
    pub actor_tid: i32,
    pub causal_chain_digest: String,
}

impl PostOpenBinding {
    fn validate(&self) -> Result<(), String> {
        if self.fd < 0 {
            return Err("post-open binding fd must be non-negative".to_owned());
        }
        if self.fd_generation == 0 {
            return Err("post-open binding requires non-zero fd generation".to_owned());
        }
        if self.originating_entry_sequence == 0 || self.binding_sequence == 0 {
            return Err("post-open binding sequence must be non-zero".to_owned());
        }
        if self.binding_sequence <= self.originating_entry_sequence {
            return Err("post-open binding must follow originating entry".to_owned());
        }
        if self.object_identity.trim().is_empty() {
            return Err("post-open object identity must not be empty".to_owned());
        }
        validate_sha256(&self.causal_chain_digest, "causal_chain_digest")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum OpenObjectAuthority {
    SuccessBounded { proof_digest: String },
    NotSuccessful,
    Ambiguous { reason_codes: BTreeSet<String> },
    Lost { reason_codes: BTreeSet<String> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OpenObjectRecord {
    pub success_evidence: SuccessEvidenceRecord,
    pub binding: Option<PostOpenBinding>,
    pub authority: OpenObjectAuthority,
}

impl OpenObjectRecord {
    pub fn build(
        success_evidence: SuccessEvidenceRecord,
        binding: Option<PostOpenBinding>,
    ) -> Result<Self, String> {
        success_evidence.validate()?;

        if success_evidence.attempt.proposition != TargetProposition::FileOpenObject {
            return Err("B1 accepts only P4.FILE.OPEN_OBJECT evidence".to_owned());
        }

        let authority = match &success_evidence.state {
            EvidenceState::Lost { reason_codes } => OpenObjectAuthority::Lost {
                reason_codes: reason_codes.clone(),
            },
            EvidenceState::SuccessObserved {
                returned_fd: Some(returned_fd),
                ..
            } => match binding.as_ref() {
                None => OpenObjectAuthority::Ambiguous {
                    reason_codes: BTreeSet::from(["post_open_binding_missing".to_owned()]),
                },
                Some(binding) => {
                    binding.validate()?;
                    if binding.fd != *returned_fd {
                        OpenObjectAuthority::Ambiguous {
                            reason_codes: BTreeSet::from(["returned_fd_binding_mismatch".to_owned()]),
                        }
                    } else if binding.originating_entry_sequence
                        != success_evidence.attempt.entry_sequence
                    {
                        OpenObjectAuthority::Ambiguous {
                            reason_codes: BTreeSet::from(["post_open_entry_mismatch".to_owned()]),
                        }
                    } else if binding.actor_tid != success_evidence.attempt.actor.tid {
                        OpenObjectAuthority::Ambiguous {
                            reason_codes: BTreeSet::from(["post_open_actor_mismatch".to_owned()]),
                        }
                    } else if binding.causal_chain_digest
                        != success_evidence.attempt.actor.causal_chain_digest
                    {
                        OpenObjectAuthority::Ambiguous {
                            reason_codes: BTreeSet::from(["post_open_causal_chain_mismatch".to_owned()]),
                        }
                    } else if binding.binding_sequence
                        <= success_evidence
                            .exit
                            .as_ref()
                            .expect("B0 success requires exit")
                            .exit_sequence
                    {
                        OpenObjectAuthority::Ambiguous {
                            reason_codes: BTreeSet::from([
                                "post_open_binding_not_after_success_exit".to_owned(),
                            ]),
                        }
                    } else {
                        OpenObjectAuthority::SuccessBounded {
                            proof_digest: proof_digest(&success_evidence, binding)?,
                        }
                    }
                }
            },
            EvidenceState::SuccessObserved {
                returned_fd: None, ..
            } => OpenObjectAuthority::Ambiguous {
                reason_codes: BTreeSet::from(["successful_open_without_returned_fd".to_owned()]),
            },
            EvidenceState::AttemptObserved
            | EvidenceState::FailureObserved { .. }
            | EvidenceState::PendingObserved { .. } => OpenObjectAuthority::NotSuccessful,
            EvidenceState::Ambiguous { reason_codes } => OpenObjectAuthority::Ambiguous {
                reason_codes: reason_codes.clone(),
            },
        };

        Ok(Self {
            success_evidence,
            binding,
            authority,
        })
    }

    pub fn is_success_authority(&self) -> bool {
        matches!(self.authority, OpenObjectAuthority::SuccessBounded { .. })
    }
}

fn proof_digest(
    success_evidence: &SuccessEvidenceRecord,
    binding: &PostOpenBinding,
) -> Result<String, String> {
    let bytes = serde_json::to_vec(&(success_evidence, binding)).map_err(|error| error.to_string())?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

fn validate_sha256(value: &str, field: &str) -> Result<(), String> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(format!("{field} must use sha256 identity"));
    };
    if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{field} must contain 64 hexadecimal digits"));
    }
    Ok(())
}
