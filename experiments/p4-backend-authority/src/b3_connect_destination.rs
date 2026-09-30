use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::b0_success_evidence::{
    EvidenceState, OperationKind, SuccessEvidenceRecord, TargetProposition,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "family", rename_all = "snake_case")]
pub enum ConnectDestination {
    Inet4 { address: [u8; 4], port: u16 },
    Inet6 {
        address: [u8; 16],
        port: u16,
        flowinfo: u32,
        scope_id: u32,
    },
    Unix { path_bytes: Vec<u8>, abstract_namespace: bool },
}

impl ConnectDestination {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Inet4 { port, .. } | Self::Inet6 { port, .. } if *port == 0 => {
                Err("connect destination port must be non-zero".to_owned())
            }
            Self::Unix { path_bytes, .. } if path_bytes.is_empty() => {
                Err("unix destination must preserve non-empty path bytes".to_owned())
            }
            _ => Ok(()),
        }
    }

    pub fn identity(&self) -> Result<String, String> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|error| error.to_string())?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectContext {
    pub socket_fd: i32,
    pub destination: ConnectDestination,
    pub sockaddr_len: u32,
    pub sockaddr_complete: bool,
}

impl ConnectContext {
    pub fn validate(&self) -> Result<(), String> {
        if self.socket_fd < 0 {
            return Err("connect requires non-negative socket fd".to_owned());
        }
        if self.sockaddr_len == 0 || !self.sockaddr_complete {
            return Err("connect requires complete non-empty sockaddr evidence".to_owned());
        }
        self.destination.validate()
    }

    pub fn digest(&self) -> Result<String, String> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|error| error.to_string())?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }

    pub fn target_identity(&self) -> Result<String, String> {
        Ok(format!("fd:{}@{}", self.socket_fd, self.destination.identity()?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "authority", rename_all = "snake_case")]
pub enum ConnectAuthority {
    SynchronousSuccessBounded { proof_digest: String },
    Pending { errno: i32 },
    AttemptOnly,
    FailureObserved { errno: i32 },
    Ambiguous { reason_codes: std::collections::BTreeSet<String> },
    Lost { reason_codes: std::collections::BTreeSet<String> },
}

impl ConnectAuthority {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::SynchronousSuccessBounded { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectRecord {
    pub success_evidence: SuccessEvidenceRecord,
    pub context: ConnectContext,
    pub authority: ConnectAuthority,
}

impl ConnectRecord {
    pub fn build(
        success_evidence: SuccessEvidenceRecord,
        context: ConnectContext,
    ) -> Result<Self, String> {
        success_evidence.validate()?;
        context.validate()?;

        if success_evidence.attempt.proposition != TargetProposition::NetworkConnectDestination {
            return Err("B3 accepts only P4.NET.CONNECT_DESTINATION evidence".to_owned());
        }
        if success_evidence.attempt.operation != OperationKind::Connect {
            return Err("B3 accepts only connect operation evidence".to_owned());
        }
        if success_evidence.attempt.argument_digest != context.digest()? {
            return Err("B3 connect-context digest mismatch".to_owned());
        }
        if success_evidence.attempt.target_identity != context.target_identity()? {
            return Err("B3 socket-fd/destination identity mismatch".to_owned());
        }

        let authority = match &success_evidence.state {
            EvidenceState::AttemptObserved => ConnectAuthority::AttemptOnly,
            EvidenceState::SuccessObserved { raw_return, returned_fd } => {
                if *raw_return != 0 || returned_fd.is_some() {
                    return Err("connect success requires synchronous rc=0 and no returned fd".to_owned());
                }
                ConnectAuthority::SynchronousSuccessBounded {
                    proof_digest: proof_digest(&success_evidence, &context)?,
                }
            }
            EvidenceState::PendingObserved { errno } => ConnectAuthority::Pending { errno: *errno },
            EvidenceState::FailureObserved { errno } => {
                ConnectAuthority::FailureObserved { errno: *errno }
            }
            EvidenceState::Ambiguous { reason_codes } => ConnectAuthority::Ambiguous {
                reason_codes: reason_codes.clone(),
            },
            EvidenceState::Lost { reason_codes } => ConnectAuthority::Lost {
                reason_codes: reason_codes.clone(),
            },
        };

        Ok(Self { success_evidence, context, authority })
    }

    pub fn is_success_authority(&self) -> bool {
        self.authority.is_success()
    }
}

fn proof_digest(
    success_evidence: &SuccessEvidenceRecord,
    context: &ConnectContext,
) -> Result<String, String> {
    let bytes = serde_json::to_vec(&(success_evidence, context)).map_err(|error| error.to_string())?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}
