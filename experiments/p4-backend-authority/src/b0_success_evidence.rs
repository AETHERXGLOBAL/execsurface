use std::collections::BTreeSet;

use serde::Serialize;
use sha2::{Digest, Sha256};

const LINUX_EINPROGRESS: i32 = 115;
const MAX_LINUX_ERRNO: i64 = 4095;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetProposition {
    FileOpenObject,
    FileRenameDelete,
    NetworkConnectDestination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    Open,
    OpenAt,
    OpenAt2,
    Rename,
    RenameAt,
    RenameAt2,
    Unlink,
    UnlinkAt,
    Rmdir,
    Connect,
}

impl OperationKind {
    fn proposition(self) -> TargetProposition {
        match self {
            Self::Open | Self::OpenAt | Self::OpenAt2 => TargetProposition::FileOpenObject,
            Self::Rename
            | Self::RenameAt
            | Self::RenameAt2
            | Self::Unlink
            | Self::UnlinkAt
            | Self::Rmdir => TargetProposition::FileRenameDelete,
            Self::Connect => TargetProposition::NetworkConnectDestination,
        }
    }

    fn is_open(self) -> bool {
        matches!(self, Self::Open | Self::OpenAt | Self::OpenAt2)
    }

    fn is_zero_success(self) -> bool {
        matches!(
            self,
            Self::Rename
                | Self::RenameAt
                | Self::RenameAt2
                | Self::Unlink
                | Self::UnlinkAt
                | Self::Rmdir
                | Self::Connect
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ActorIdentity {
    pub tid: i32,
    pub process_identity: String,
    pub causal_chain_digest: String,
}

impl ActorIdentity {
    fn validate(&self) -> Result<(), String> {
        if self.tid <= 0 {
            return Err("actor tid must be positive".to_owned());
        }
        if self.process_identity.trim().is_empty() {
            return Err("process identity must not be empty".to_owned());
        }
        validate_sha256(&self.causal_chain_digest, "causal_chain_digest")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AttemptEvidence {
    pub proposition: TargetProposition,
    pub operation: OperationKind,
    pub actor: ActorIdentity,
    pub entry_sequence: u64,
    pub argument_digest: String,
    pub target_identity: String,
}

impl AttemptEvidence {
    pub fn validate(&self) -> Result<(), String> {
        self.actor.validate()?;
        if self.entry_sequence == 0 {
            return Err("entry sequence must be non-zero".to_owned());
        }
        if self.operation.proposition() != self.proposition {
            return Err("operation does not belong to target proposition".to_owned());
        }
        validate_sha256(&self.argument_digest, "argument_digest")?;
        if self.target_identity.trim().is_empty() {
            return Err("target identity must not be empty".to_owned());
        }
        Ok(())
    }

    pub fn pairing_identity(&self) -> Result<String, String> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|error| error.to_string())?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExitEvidence {
    pub actor: ActorIdentity,
    pub originating_entry_sequence: u64,
    pub exit_sequence: u64,
    pub raw_return: i64,
}

impl ExitEvidence {
    fn validate(&self) -> Result<(), String> {
        self.actor.validate()?;
        if self.originating_entry_sequence == 0 || self.exit_sequence == 0 {
            return Err("entry/exit sequence must be non-zero".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObservationHealth {
    pub complete: bool,
    pub warning_codes: BTreeSet<String>,
}

impl ObservationHealth {
    pub fn healthy() -> Self {
        Self {
            complete: true,
            warning_codes: BTreeSet::new(),
        }
    }

    fn is_healthy(&self) -> bool {
        self.complete && self.warning_codes.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum EvidenceState {
    AttemptObserved,
    SuccessObserved {
        raw_return: i64,
        returned_fd: Option<i32>,
    },
    FailureObserved {
        errno: i32,
    },
    PendingObserved {
        errno: i32,
    },
    Ambiguous {
        reason_codes: BTreeSet<String>,
    },
    Lost {
        reason_codes: BTreeSet<String>,
    },
}

impl EvidenceState {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::SuccessObserved { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SuccessEvidenceRecord {
    pub pairing_identity: String,
    pub attempt: AttemptEvidence,
    pub exit: Option<ExitEvidence>,
    pub health: ObservationHealth,
    pub state: EvidenceState,
}

impl SuccessEvidenceRecord {
    pub fn validate(&self) -> Result<(), String> {
        self.attempt.validate()?;
        validate_sha256(&self.pairing_identity, "pairing_identity")?;
        if self.pairing_identity != self.attempt.pairing_identity()? {
            return Err("pairing identity does not match attempt".to_owned());
        }

        match &self.state {
            EvidenceState::AttemptObserved => {
                if self.exit.is_some() {
                    return Err("attempt-only evidence cannot carry an exit".to_owned());
                }
            }
            EvidenceState::SuccessObserved {
                raw_return,
                returned_fd,
            } => {
                self.require_healthy_matched_exit()?;
                if *raw_return < 0 {
                    return Err("success cannot carry a negative return".to_owned());
                }
                let exit = self.exit.as_ref().expect("matched exit checked");
                if exit.raw_return != *raw_return {
                    return Err("success return does not match exit".to_owned());
                }
                if self.attempt.operation.is_open() {
                    let fd = returned_fd.ok_or_else(|| "successful open requires returned fd".to_owned())?;
                    if i64::from(fd) != *raw_return {
                        return Err("returned fd must equal successful open return".to_owned());
                    }
                } else if self.attempt.operation.is_zero_success() {
                    if *raw_return != 0 || returned_fd.is_some() {
                        return Err("zero-success operation requires rc=0 and no returned fd".to_owned());
                    }
                }
            }
            EvidenceState::FailureObserved { errno } => {
                self.require_healthy_matched_exit()?;
                if *errno <= 0 {
                    return Err("failure errno must be positive".to_owned());
                }
                let exit = self.exit.as_ref().expect("matched exit checked");
                if exit.raw_return != -i64::from(*errno) {
                    return Err("failure errno does not match raw return".to_owned());
                }
            }
            EvidenceState::PendingObserved { errno } => {
                self.require_healthy_matched_exit()?;
                if self.attempt.operation != OperationKind::Connect || *errno != LINUX_EINPROGRESS {
                    return Err("pending state is bounded to connect EINPROGRESS".to_owned());
                }
                let exit = self.exit.as_ref().expect("matched exit checked");
                if exit.raw_return != -i64::from(*errno) {
                    return Err("pending errno does not match raw return".to_owned());
                }
            }
            EvidenceState::Ambiguous { reason_codes } => {
                if reason_codes.is_empty() {
                    return Err("ambiguous evidence requires reason code".to_owned());
                }
            }
            EvidenceState::Lost { reason_codes } => {
                if reason_codes.is_empty() {
                    return Err("lost evidence requires reason code".to_owned());
                }
                if self.health.is_healthy() {
                    return Err("lost state requires incomplete or warning-bearing health".to_owned());
                }
            }
        }
        Ok(())
    }

    fn require_healthy_matched_exit(&self) -> Result<(), String> {
        if !self.health.is_healthy() {
            return Err("success/failure/pending authority requires healthy observation".to_owned());
        }
        let exit = self
            .exit
            .as_ref()
            .ok_or_else(|| "paired result requires syscall exit".to_owned())?;
        exit.validate()?;
        if exit.actor != self.attempt.actor {
            return Err("entry/exit actor mismatch".to_owned());
        }
        if exit.originating_entry_sequence != self.attempt.entry_sequence {
            return Err("entry/exit pairing mismatch".to_owned());
        }
        if exit.exit_sequence <= self.attempt.entry_sequence {
            return Err("exit sequence must follow entry sequence".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct EvidenceLedger {
    consumed_pairings: BTreeSet<String>,
}

impl EvidenceLedger {
    pub fn attempt_only(&self, attempt: AttemptEvidence, health: ObservationHealth) -> Result<SuccessEvidenceRecord, String> {
        let pairing_identity = attempt.pairing_identity()?;
        let record = SuccessEvidenceRecord {
            pairing_identity,
            attempt,
            exit: None,
            health,
            state: EvidenceState::AttemptObserved,
        };
        record.validate()?;
        Ok(record)
    }

    pub fn classify_pair(
        &mut self,
        attempt: AttemptEvidence,
        exit: ExitEvidence,
        health: ObservationHealth,
    ) -> Result<SuccessEvidenceRecord, String> {
        let pairing_identity = attempt.pairing_identity()?;
        let mut record = SuccessEvidenceRecord {
            pairing_identity: pairing_identity.clone(),
            attempt,
            exit: Some(exit),
            health,
            state: EvidenceState::Ambiguous {
                reason_codes: BTreeSet::from(["unclassified_pair".to_owned()]),
            },
        };

        if !record.health.is_healthy() {
            let mut reasons = record.health.warning_codes.clone();
            if !record.health.complete {
                reasons.insert("observation_incomplete".to_owned());
            }
            if reasons.is_empty() {
                reasons.insert("observer_health_not_healthy".to_owned());
            }
            record.state = EvidenceState::Lost { reason_codes: reasons };
            record.validate()?;
            self.consume(&pairing_identity)?;
            return Ok(record);
        }

        let exit = record.exit.as_ref().expect("exit supplied");
        if exit.actor != record.attempt.actor {
            record.state = EvidenceState::Ambiguous {
                reason_codes: BTreeSet::from(["entry_exit_actor_mismatch".to_owned()]),
            };
            record.validate()?;
            self.consume(&pairing_identity)?;
            return Ok(record);
        }
        if exit.originating_entry_sequence != record.attempt.entry_sequence {
            record.state = EvidenceState::Ambiguous {
                reason_codes: BTreeSet::from(["entry_exit_pairing_mismatch".to_owned()]),
            };
            record.validate()?;
            self.consume(&pairing_identity)?;
            return Ok(record);
        }
        if exit.exit_sequence <= record.attempt.entry_sequence {
            record.state = EvidenceState::Ambiguous {
                reason_codes: BTreeSet::from(["non_monotonic_exit_sequence".to_owned()]),
            };
            record.validate()?;
            self.consume(&pairing_identity)?;
            return Ok(record);
        }

        record.state = classify_return(record.attempt.operation, exit.raw_return);
        record.validate()?;
        self.consume(&pairing_identity)?;
        Ok(record)
    }

    fn consume(&mut self, pairing_identity: &str) -> Result<(), String> {
        if !self.consumed_pairings.insert(pairing_identity.to_owned()) {
            return Err("duplicate/replayed syscall pairing rejected".to_owned());
        }
        Ok(())
    }
}

fn classify_return(operation: OperationKind, raw_return: i64) -> EvidenceState {
    if (-MAX_LINUX_ERRNO..=-1).contains(&raw_return) {
        let errno = (-raw_return) as i32;
        if operation == OperationKind::Connect && errno == LINUX_EINPROGRESS {
            return EvidenceState::PendingObserved { errno };
        }
        return EvidenceState::FailureObserved { errno };
    }
    if raw_return < -MAX_LINUX_ERRNO {
        return EvidenceState::Ambiguous {
            reason_codes: BTreeSet::from(["return_outside_linux_errno_contract".to_owned()]),
        };
    }
    if operation.is_open() {
        return match i32::try_from(raw_return) {
            Ok(fd) => EvidenceState::SuccessObserved {
                raw_return,
                returned_fd: Some(fd),
            },
            Err(_) => EvidenceState::Ambiguous {
                reason_codes: BTreeSet::from(["returned_fd_out_of_range".to_owned()]),
            },
        };
    }
    if operation.is_zero_success() && raw_return == 0 {
        return EvidenceState::SuccessObserved {
            raw_return,
            returned_fd: None,
        };
    }
    EvidenceState::Ambiguous {
        reason_codes: BTreeSet::from(["unexpected_nonnegative_return".to_owned()]),
    }
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
