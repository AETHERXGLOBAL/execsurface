use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::b0_success_evidence::{
    EvidenceState, OperationKind, SuccessEvidenceRecord, TargetProposition,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RenameDeleteContext {
    Rename {
        operation: OperationKind,
        source: String,
        target: String,
        source_dirfd: Option<i64>,
        target_dirfd: Option<i64>,
        flags: Option<u32>,
        flags_classified: bool,
    },
    Delete {
        operation: OperationKind,
        target: String,
        dirfd: Option<i64>,
        flags: Option<u32>,
        flags_classified: bool,
    },
}

impl RenameDeleteContext {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Rename {
                operation,
                source,
                target,
                flags,
                flags_classified,
                ..
            } => {
                if !matches!(
                    operation,
                    OperationKind::Rename | OperationKind::RenameAt | OperationKind::RenameAt2
                ) {
                    return Err("rename context requires rename-family operation".to_owned());
                }
                if source.trim().is_empty() || target.trim().is_empty() {
                    return Err("rename context requires source and target".to_owned());
                }
                if matches!(operation, OperationKind::RenameAt2)
                    && (flags.is_none() || !flags_classified)
                {
                    return Err("renameat2 requires classified flags".to_owned());
                }
                if !matches!(operation, OperationKind::RenameAt2) && flags.is_some() {
                    return Err("rename flags are valid only for renameat2".to_owned());
                }
            }
            Self::Delete {
                operation,
                target,
                flags,
                flags_classified,
                ..
            } => {
                if !matches!(
                    operation,
                    OperationKind::Unlink | OperationKind::UnlinkAt | OperationKind::Rmdir
                ) {
                    return Err("delete context requires delete-family operation".to_owned());
                }
                if target.trim().is_empty() {
                    return Err("delete context requires target".to_owned());
                }
                if matches!(operation, OperationKind::UnlinkAt)
                    && (flags.is_none() || !flags_classified)
                {
                    return Err("unlinkat requires classified flags".to_owned());
                }
                if !matches!(operation, OperationKind::UnlinkAt) && flags.is_some() {
                    return Err("delete flags are valid only for unlinkat".to_owned());
                }
            }
        }
        Ok(())
    }

    pub fn operation(&self) -> OperationKind {
        match self {
            Self::Rename { operation, .. } | Self::Delete { operation, .. } => *operation,
        }
    }

    pub fn target_identity(&self) -> String {
        match self {
            Self::Rename { source, target, .. } => format!("{source}->{target}"),
            Self::Delete { target, .. } => target.clone(),
        }
    }

    pub fn digest(&self) -> Result<String, String> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|error| error.to_string())?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "authority", rename_all = "snake_case")]
pub enum RenameDeleteAuthority {
    SuccessEffectBounded {
        proof_digest: String,
    },
    AttemptOnly,
    FailureObserved {
        errno: i32,
    },
    Ambiguous {
        reason_codes: std::collections::BTreeSet<String>,
    },
    Lost {
        reason_codes: std::collections::BTreeSet<String>,
    },
}

impl RenameDeleteAuthority {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::SuccessEffectBounded { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenameDeleteRecord {
    pub success_evidence: SuccessEvidenceRecord,
    pub context: RenameDeleteContext,
    pub authority: RenameDeleteAuthority,
}

impl RenameDeleteRecord {
    pub fn build(
        success_evidence: SuccessEvidenceRecord,
        context: RenameDeleteContext,
    ) -> Result<Self, String> {
        success_evidence.validate()?;
        context.validate()?;

        if success_evidence.attempt.proposition != TargetProposition::FileRenameDelete {
            return Err("B2 accepts only P4.FILE.RENAME_DELETE evidence".to_owned());
        }
        if success_evidence.attempt.operation != context.operation() {
            return Err("B2 operation/context mismatch".to_owned());
        }
        if success_evidence.attempt.argument_digest != context.digest()? {
            return Err("B2 operation-context digest mismatch".to_owned());
        }
        if success_evidence.attempt.target_identity != context.target_identity() {
            return Err("B2 source/target identity mismatch".to_owned());
        }

        let authority = match &success_evidence.state {
            EvidenceState::AttemptObserved => RenameDeleteAuthority::AttemptOnly,
            EvidenceState::FailureObserved { errno } => {
                RenameDeleteAuthority::FailureObserved { errno: *errno }
            }
            EvidenceState::Ambiguous { reason_codes } => RenameDeleteAuthority::Ambiguous {
                reason_codes: reason_codes.clone(),
            },
            EvidenceState::Lost { reason_codes } => RenameDeleteAuthority::Lost {
                reason_codes: reason_codes.clone(),
            },
            EvidenceState::PendingObserved { .. } => {
                return Err("rename/delete cannot use pending authority".to_owned())
            }
            EvidenceState::SuccessObserved {
                raw_return,
                returned_fd,
            } => {
                if *raw_return != 0 || returned_fd.is_some() {
                    return Err("rename/delete success requires rc=0 and no returned fd".to_owned());
                }
                RenameDeleteAuthority::SuccessEffectBounded {
                    proof_digest: proof_digest(&success_evidence, &context)?,
                }
            }
        };

        Ok(Self {
            success_evidence,
            context,
            authority,
        })
    }

    pub fn is_success_authority(&self) -> bool {
        self.authority.is_success()
    }
}

fn proof_digest(
    success_evidence: &SuccessEvidenceRecord,
    context: &RenameDeleteContext,
) -> Result<String, String> {
    let bytes =
        serde_json::to_vec(&(success_evidence, context)).map_err(|error| error.to_string())?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}
