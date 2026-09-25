//! Deterministic, content-addressed baseline lockfiles.
//!
//! The baseline records an accepted canonical execution surface. It contains
//! no policy and no candidate-vs-baseline diff semantics.

use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use execsurface_model::canonical::{
    CanonicalExecutable, CanonicalSurface, CANONICAL_SURFACE_SCHEMA_VERSION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const LOCK_SCHEMA_VERSION: u32 = 2;
pub const DIGEST_FORMAT_VERSION: u32 = 2;
pub const DEFAULT_LOCKFILE_NAME: &str = "execsurface.lock.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineLock {
    pub schema_version: u32,
    pub baseline_digest: String,
    pub payload: BaselinePayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselinePayload {
    pub digest_format_version: u32,
    pub tool: ToolIdentity,
    pub command: CommandIdentity,
    pub platform: PlatformIdentity,
    pub observer: ObserverIdentity,
    pub canonical_surface: CanonicalSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolIdentity {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandIdentity {
    pub executable: CanonicalExecutable,
    pub argument_count: u32,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformIdentity {
    pub os: String,
    pub architecture: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObserverIdentity {
    pub name: String,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Serialize)]
struct DigestEnvelope<'a> {
    schema_version: u32,
    payload: &'a BaselinePayload,
}

#[derive(Debug)]
pub enum BaselineError {
    UnsupportedLockSchema(u32),
    UnsupportedDigestFormat(u32),
    UnsupportedCanonicalSchema(u32),
    DigestMismatch { expected: String, actual: String },
    Serialization(serde_json::Error),
    Io(io::Error),
    TargetExists(PathBuf),
    TemporaryPathExhausted(PathBuf),
}

impl fmt::Display for BaselineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLockSchema(version) => {
                write!(f, "unsupported lock schema version: {version}")
            }
            Self::UnsupportedDigestFormat(version) => {
                write!(f, "unsupported digest format version: {version}")
            }
            Self::UnsupportedCanonicalSchema(version) => {
                write!(f, "unsupported canonical surface schema version: {version}")
            }
            Self::DigestMismatch { expected, actual } => {
                write!(
                    f,
                    "baseline digest mismatch: expected {expected}, calculated {actual}"
                )
            }
            Self::Serialization(error) => write!(f, "baseline serialization error: {error}"),
            Self::Io(error) => write!(f, "baseline I/O error: {error}"),
            Self::TargetExists(path) => {
                write!(f, "lockfile already exists: {}", path.display())
            }
            Self::TemporaryPathExhausted(path) => write!(
                f,
                "could not reserve a temporary lockfile beside {}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for BaselineError {}

impl From<serde_json::Error> for BaselineError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

impl From<io::Error> for BaselineError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl BaselinePayload {
    pub fn new(
        tool: ToolIdentity,
        command: CommandIdentity,
        platform: PlatformIdentity,
        observer: ObserverIdentity,
        canonical_surface: CanonicalSurface,
    ) -> Self {
        Self {
            digest_format_version: DIGEST_FORMAT_VERSION,
            tool,
            command,
            platform,
            observer,
            canonical_surface,
        }
    }
}

pub fn build_lock(mut payload: BaselinePayload) -> Result<BaselineLock, BaselineError> {
    validate_payload_versions(&payload)?;
    normalize_payload_order(&mut payload);
    let baseline_digest = digest_payload(&payload)?;
    Ok(BaselineLock {
        schema_version: LOCK_SCHEMA_VERSION,
        baseline_digest,
        payload,
    })
}

pub fn verify_lock(lock: &BaselineLock) -> Result<(), BaselineError> {
    if lock.schema_version != LOCK_SCHEMA_VERSION {
        return Err(BaselineError::UnsupportedLockSchema(lock.schema_version));
    }
    validate_payload_versions(&lock.payload)?;

    let actual = digest_payload(&lock.payload)?;
    if actual != lock.baseline_digest {
        return Err(BaselineError::DigestMismatch {
            expected: lock.baseline_digest.clone(),
            actual,
        });
    }
    Ok(())
}

pub fn parse_and_verify(bytes: &[u8]) -> Result<BaselineLock, BaselineError> {
    let lock: BaselineLock = serde_json::from_slice(bytes)?;
    verify_lock(&lock)?;
    Ok(lock)
}

pub fn serialize_lock(lock: &BaselineLock) -> Result<Vec<u8>, BaselineError> {
    verify_lock(lock)?;
    let mut bytes = serde_json::to_vec_pretty(lock)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn digest_input_bytes(payload: &BaselinePayload) -> Result<Vec<u8>, BaselineError> {
    validate_payload_versions(payload)?;
    Ok(serde_json::to_vec(&DigestEnvelope {
        schema_version: LOCK_SCHEMA_VERSION,
        payload,
    })?)
}

pub fn digest_payload(payload: &BaselinePayload) -> Result<String, BaselineError> {
    let bytes = digest_input_bytes(payload)?;
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    Ok(format!("sha256:{hex}"))
}

pub fn write_lockfile(
    path: &Path,
    lock: &BaselineLock,
    overwrite: bool,
) -> Result<(), BaselineError> {
    let bytes = serialize_lock(lock)?;
    atomic_write(path, &bytes, overwrite)
}

fn validate_payload_versions(payload: &BaselinePayload) -> Result<(), BaselineError> {
    if payload.digest_format_version != DIGEST_FORMAT_VERSION {
        return Err(BaselineError::UnsupportedDigestFormat(
            payload.digest_format_version,
        ));
    }
    if payload.canonical_surface.schema_version != CANONICAL_SURFACE_SCHEMA_VERSION {
        return Err(BaselineError::UnsupportedCanonicalSchema(
            payload.canonical_surface.schema_version,
        ));
    }
    Ok(())
}

fn normalize_payload_order(payload: &mut BaselinePayload) {
    payload.observer.capabilities.sort();
    payload.observer.capabilities.dedup();
    payload.observer.limitations.sort();
    payload.observer.limitations.dedup();

    payload
        .canonical_surface
        .normalization
        .semantic_roots
        .sort();
    payload
        .canonical_surface
        .normalization
        .semantic_roots
        .dedup();

    payload.canonical_surface.effects.sort();
    payload.canonical_surface.effects.dedup();
}

fn atomic_write(path: &Path, bytes: &[u8], overwrite: bool) -> Result<(), BaselineError> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let filename = path
        .file_name()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "lockfile path has no filename")
        })?
        .to_string_lossy();

    let mut temp = None;
    for attempt in 0..128_u32 {
        let candidate = parent.join(format!(
            ".{filename}.tmp.{}.{}",
            std::process::id(),
            attempt
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temp = Some((candidate, file));
                break;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(BaselineError::Io(error)),
        }
    }

    let (temp_path, mut file) =
        temp.ok_or_else(|| BaselineError::TemporaryPathExhausted(path.to_path_buf()))?;

    let result = (|| -> Result<(), BaselineError> {
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);

        if overwrite {
            fs::rename(&temp_path, path)?;
        } else {
            match fs::hard_link(&temp_path, path) {
                Ok(()) => {
                    fs::remove_file(&temp_path)?;
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    return Err(BaselineError::TargetExists(path.to_path_buf()));
                }
                Err(error) => return Err(BaselineError::Io(error)),
            }
        }

        File::open(parent)?.sync_all()?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_model::canonical::{
        CanonicalEffect, CanonicalNetworkEndpoint, CanonicalPath, NormalizationMetadata, PathClass,
        PathResolution,
    };

    fn fixture_payload() -> BaselinePayload {
        BaselinePayload::new(
            ToolIdentity {
                name: "execsurface".to_owned(),
                version: "0.0.1".to_owned(),
            },
            CommandIdentity {
                executable: CanonicalExecutable {
                    path: CanonicalPath {
                        value: "/usr/bin/python3".to_owned(),
                        class: PathClass::System,
                        resolution: PathResolution::Lexical,
                    },
                    family: "python3".to_owned(),
                },
                argument_count: 2,
                label: None,
            },
            PlatformIdentity {
                os: "linux".to_owned(),
                architecture: "x86_64".to_owned(),
            },
            ObserverIdentity {
                name: "linux-ptrace-metadata-only".to_owned(),
                capabilities: vec![
                    "connect_destination".to_owned(),
                    "descendant_tracking".to_owned(),
                ],
                limitations: vec!["example limitation".to_owned()],
            },
            CanonicalSurface {
                schema_version: 2,
                normalization: NormalizationMetadata {
                    profile_version: 2,
                    semantic_roots: vec!["home".to_owned(), "workspace".to_owned()],
                },
                effects: vec![],
            },
        )
    }

    #[test]
    fn sha256_matches_standard_abc_vector() {
        let digest = Sha256::digest(b"abc");
        assert_eq!(
            format!("{digest:x}"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn digest_serialization_vector_is_frozen() {
        let payload = fixture_payload();
        let bytes = digest_input_bytes(&payload).expect("digest bytes");
        let expected = concat!(
            "{\"schema_version\":2,\"payload\":{",
            "\"digest_format_version\":2,",
            "\"tool\":{\"name\":\"execsurface\",\"version\":\"0.0.1\"},",
            "\"command\":{\"executable\":{\"path\":{\"value\":\"/usr/bin/python3\",",
            "\"class\":\"system\",\"resolution\":\"lexical\"},\"family\":\"python3\"},",
            "\"argument_count\":2,\"label\":null},",
            "\"platform\":{\"os\":\"linux\",\"architecture\":\"x86_64\"},",
            "\"observer\":{\"name\":\"linux-ptrace-metadata-only\",",
            "\"capabilities\":[\"connect_destination\",\"descendant_tracking\"],",
            "\"limitations\":[\"example limitation\"]},",
            "\"canonical_surface\":{\"schema_version\":2,",
            "\"normalization\":{\"profile_version\":2,",
            "\"semantic_roots\":[\"home\",\"workspace\"]},\"effects\":[]}}}"
        );
        assert_eq!(bytes, expected.as_bytes());
        assert_eq!(
            digest_payload(&payload).expect("digest"),
            "sha256:34c6428da9ce58aebd17fe47470c154d1f2eb99aff2d37ba0282a678f7dcb067"
        );
    }

    #[test]
    fn controlled_effect_change_changes_digest() {
        let first = build_lock(fixture_payload()).expect("first");

        let mut second_payload = fixture_payload();
        second_payload
            .canonical_surface
            .effects
            .push(CanonicalEffect::NetworkConnectAttempt {
                actor: None,
                execution_chain: vec![],
                endpoint: CanonicalNetworkEndpoint::Inet {
                    ip: "192.0.2.10".to_owned(),
                    port: 443,
                },
            });
        let second = build_lock(second_payload).expect("second");

        assert_ne!(first.baseline_digest, second.baseline_digest);
    }

    #[test]
    fn unordered_observer_metadata_is_normalized_before_hashing() {
        let first = build_lock(fixture_payload()).expect("first");

        let mut second_payload = fixture_payload();
        second_payload.observer.capabilities.reverse();
        second_payload
            .observer
            .capabilities
            .push("connect_destination".to_owned());
        let second = build_lock(second_payload).expect("second");

        assert_eq!(first.baseline_digest, second.baseline_digest);
        assert_eq!(
            serialize_lock(&first).expect("first bytes"),
            serialize_lock(&second).expect("second bytes")
        );
    }

    #[test]
    fn corrupted_digest_is_rejected() {
        let mut lock = build_lock(fixture_payload()).expect("lock");
        lock.baseline_digest = "sha256:deadbeef".to_owned();
        assert!(matches!(
            verify_lock(&lock),
            Err(BaselineError::DigestMismatch { .. })
        ));
    }

    #[test]
    fn atomic_writer_refuses_silent_overwrite() {
        let dir =
            std::env::temp_dir().join(format!("execsurface-baseline-unit-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create dir");
        let path = dir.join("lock.json");
        let _ = fs::remove_file(&path);

        let lock = build_lock(fixture_payload()).expect("lock");
        write_lockfile(&path, &lock, false).expect("first write");
        assert!(matches!(
            write_lockfile(&path, &lock, false),
            Err(BaselineError::TargetExists(_))
        ));
        write_lockfile(&path, &lock, true).expect("explicit overwrite");

        let bytes = fs::read(&path).expect("read");
        parse_and_verify(&bytes).expect("verify");
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(&dir);
    }
    #[test]
    fn legacy_lock_schema_is_rejected_instead_of_reinterpreted() {
        let mut lock = build_lock(fixture_payload()).expect("lock");
        lock.schema_version = 1;
        assert!(matches!(
            verify_lock(&lock),
            Err(BaselineError::UnsupportedLockSchema(1))
        ));
    }
}
