//! Backend-independent canonical execution-surface model.
//!
//! These types deliberately exclude PIDs, TIDs, timestamps and raw event
//! sequence numbers. M2 canonical surfaces are not baseline lockfiles.

use serde::{Deserialize, Serialize};

use crate::{FileOperation, SpawnMechanism};

pub const CANONICAL_SURFACE_SCHEMA_VERSION: u32 = 1;
pub const NORMALIZATION_PROFILE_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSurface {
    pub schema_version: u32,
    pub normalization: NormalizationMetadata,
    pub effects: Vec<CanonicalEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizationMetadata {
    pub profile_version: u32,
    pub semantic_roots: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CanonicalExecutable {
    pub path: CanonicalPath,
    pub family: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CanonicalPath {
    pub value: String,
    pub class: PathClass,
    pub resolution: PathResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathClass {
    Workspace,
    Home,
    CredentialSensitive,
    Temp,
    RunTemp,
    Cache,
    System,
    Device,
    OutsideDeclaredRoots,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathResolution {
    Lexical,
    RelativeUnresolved,
    ContainsParentTraversal,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum CanonicalEffect {
    ProcessSpawn {
        actor: Option<CanonicalExecutable>,
        mechanism: SpawnMechanism,
    },
    ProcessExec {
        from: Option<CanonicalExecutable>,
        executable: CanonicalExecutable,
    },
    FilePathAccess {
        actor: Option<CanonicalExecutable>,
        operation: FileOperation,
        target: CanonicalPath,
        open_intent: Option<OpenIntent>,
    },
    FileRename {
        actor: Option<CanonicalExecutable>,
        from: CanonicalPath,
        to: CanonicalPath,
    },
    NetworkConnectAttempt {
        actor: Option<CanonicalExecutable>,
        endpoint: CanonicalNetworkEndpoint,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OpenIntent {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub truncate: bool,
    pub append: bool,
    pub path_only: bool,
    pub other_flags: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "address_family", rename_all = "snake_case")]
pub enum CanonicalNetworkEndpoint {
    Inet { ip: String, port: u16 },
    Inet6 { ip: String, port: u16 },
    Unix { path: Option<CanonicalPath> },
    Other { family: u16 },
}
