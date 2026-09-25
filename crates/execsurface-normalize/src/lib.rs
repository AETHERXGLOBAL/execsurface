//! Deterministic M2 canonicalization.
//!
//! No baseline, diff or policy semantics live in this crate.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;

use execsurface_model::canonical::{
    CanonicalEffect, CanonicalExecutable, CanonicalNetworkEndpoint, CanonicalPath,
    CanonicalSurface, NormalizationMetadata, OpenIntent, PathClass, PathResolution,
    CANONICAL_SURFACE_SCHEMA_VERSION, NORMALIZATION_PROFILE_VERSION,
};
use execsurface_model::{
    FileOperation, NetworkEndpoint, Observation, RawEventKind, RAW_OBSERVATION_SCHEMA_VERSION,
};

#[derive(Debug, Clone, Default)]
pub struct NormalizationConfig {
    pub workspace: Option<String>,
    pub home: Option<String>,
    pub tmp_roots: Vec<String>,
    pub run_tmp: Option<String>,
    pub caches: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizeError {
    UnsupportedRawSchema(u32),
    IncompleteObservation,
    InvalidRoot { label: String, reason: String },
    AmbiguousRoot { path: String, labels: Vec<String> },
    DuplicateSequence(u64),
    MissingLinuxOpenFlags,
    InvalidFdOperation(FileOperation),
    ExecutionChainTooDeep { tid: i32, limit: usize },
}

impl fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedRawSchema(version) => {
                write!(f, "unsupported raw observation schema version: {version}")
            }
            Self::IncompleteObservation => {
                write!(
                    f,
                    "raw observation is incomplete and cannot form a trusted canonical surface"
                )
            }
            Self::InvalidRoot { label, reason } => {
                write!(f, "invalid semantic root {label}: {reason}")
            }
            Self::AmbiguousRoot { path, labels } => {
                write!(
                    f,
                    "semantic root {path} is assigned to multiple labels: {}",
                    labels.join(", ")
                )
            }
            Self::DuplicateSequence(sequence) => {
                write!(
                    f,
                    "raw observation contains duplicate event sequence {sequence}"
                )
            }
            Self::MissingLinuxOpenFlags => {
                write!(f, "Linux file.open event is missing raw flags required to avoid collapsing access intent")
            }
            Self::InvalidFdOperation(operation) => {
                write!(f, "fd-attributed event has invalid operation: {operation:?}")
            }
            Self::ExecutionChainTooDeep { tid, limit } => {
                write!(f, "execution chain for tid {tid} exceeded fail-closed limit {limit}")
            }
        }
    }
}

impl std::error::Error for NormalizeError {}

#[derive(Debug, Clone)]
struct RootRule {
    physical: String,
    token: String,
    class: PathClass,
    label: String,
}

pub fn canonicalize_path(
    path: &str,
    config: &NormalizationConfig,
) -> Result<CanonicalPath, NormalizeError> {
    let roots = build_root_rules(config)?;
    Ok(canonical_path(path, &roots))
}

pub fn canonicalize_executable(
    path: &str,
    config: &NormalizationConfig,
) -> Result<CanonicalExecutable, NormalizeError> {
    let roots = build_root_rules(config)?;
    Ok(canonical_executable(path, &roots))
}

pub fn canonicalize(
    observation: &Observation,
    config: &NormalizationConfig,
) -> Result<CanonicalSurface, NormalizeError> {
    if observation.schema_version != RAW_OBSERVATION_SCHEMA_VERSION {
        return Err(NormalizeError::UnsupportedRawSchema(
            observation.schema_version,
        ));
    }
    if !observation.complete {
        return Err(NormalizeError::IncompleteObservation);
    }

    let roots = build_root_rules(config)?;
    let normalization = NormalizationMetadata {
        profile_version: NORMALIZATION_PROFILE_VERSION,
        semantic_roots: semantic_root_labels(&roots),
    };

    let mut events = observation.events.iter().collect::<Vec<_>>();
    events.sort_by_key(|event| event.sequence);

    let mut seen_sequences = HashSet::new();
    for event in &events {
        if !seen_sequences.insert(event.sequence) {
            return Err(NormalizeError::DuplicateSequence(event.sequence));
        }
    }

    let mut processes: HashMap<i32, CanonicalProcessState> = HashMap::new();
    let mut effects = BTreeSet::new();

    for event in events {
        match &event.kind {
            RawEventKind::ProcessSpawn {
                child_tid,
                mechanism,
            } => {
                let state = processes.get(&event.tid).cloned().unwrap_or_default();
                let actor = state.current.clone();
                processes.insert(*child_tid, state);
                effects.insert(CanonicalEffect::ProcessSpawn {
                    actor,
                    mechanism: *mechanism,
                });
            }
            RawEventKind::ProcessExec { path } => {
                let executable = canonical_executable(path, &roots);
                let state = processes.entry(event.tid).or_default();
                let from = state.current.replace(executable.clone());
                if state.execution_chain.last() != Some(&executable) {
                    if state.execution_chain.len() >= MAX_EXECUTION_CHAIN {
                        return Err(NormalizeError::ExecutionChainTooDeep {
                            tid: event.tid,
                            limit: MAX_EXECUTION_CHAIN,
                        });
                    }
                    state.execution_chain.push(executable.clone());
                }
                effects.insert(CanonicalEffect::ProcessExec { from, executable });
            }
            RawEventKind::FilePathAccess {
                operation,
                path,
                flags,
            } => {
                let state = processes.get(&event.tid).cloned().unwrap_or_default();
                let open_intent = match operation {
                    FileOperation::Open => Some(linux_open_intent(
                        observation.backend.platform.as_str(),
                        *flags,
                        0,
                    )?),
                    FileOperation::Create
                    | FileOperation::Delete
                    | FileOperation::Read
                    | FileOperation::Write => None,
                };
                effects.insert(CanonicalEffect::FilePathAccess {
                    actor: state.current,
                    execution_chain: state.execution_chain,
                    operation: *operation,
                    target: canonical_path(path, &roots),
                    open_intent,
                });
            }
            RawEventKind::FileOpenAt2 {
                path,
                flags,
                resolve,
            } => {
                let state = processes.get(&event.tid).cloned().unwrap_or_default();
                effects.insert(CanonicalEffect::FilePathAccess {
                    actor: state.current,
                    execution_chain: state.execution_chain,
                    operation: FileOperation::Open,
                    target: canonical_path(path, &roots),
                    open_intent: Some(linux_open_intent(
                        observation.backend.platform.as_str(),
                        Some(*flags),
                        *resolve,
                    )?),
                });
            }
            RawEventKind::FileDescriptorAccess {
                operation,
                path,
                ..
            } => {
                if !matches!(operation, FileOperation::Read | FileOperation::Write) {
                    return Err(NormalizeError::InvalidFdOperation(*operation));
                }
                let state = processes.get(&event.tid).cloned().unwrap_or_default();
                effects.insert(CanonicalEffect::FilePathAccess {
                    actor: state.current,
                    execution_chain: state.execution_chain,
                    operation: *operation,
                    target: canonical_kernel_fd_path(path, &roots),
                    open_intent: None,
                });
            }
            RawEventKind::FileRename { from, to } => {
                let state = processes.get(&event.tid).cloned().unwrap_or_default();
                effects.insert(CanonicalEffect::FileRename {
                    actor: state.current,
                    execution_chain: state.execution_chain,
                    from: canonical_path(from, &roots),
                    to: canonical_path(to, &roots),
                });
            }
            RawEventKind::NetworkConnectAttempt { endpoint } => {
                let state = processes.get(&event.tid).cloned().unwrap_or_default();
                effects.insert(CanonicalEffect::NetworkConnectAttempt {
                    actor: state.current,
                    execution_chain: state.execution_chain,
                    endpoint: canonical_endpoint(endpoint, &roots),
                });
            }
        }
    }

    Ok(CanonicalSurface {
        schema_version: CANONICAL_SURFACE_SCHEMA_VERSION,
        normalization,
        effects: effects.into_iter().collect(),
    })
}

const MAX_EXECUTION_CHAIN: usize = 32;

#[derive(Debug, Clone, Default)]
struct CanonicalProcessState {
    current: Option<CanonicalExecutable>,
    execution_chain: Vec<CanonicalExecutable>,
}

fn semantic_root_labels(roots: &[RootRule]) -> Vec<String> {
    roots
        .iter()
        .map(|root| root.label.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn build_root_rules(config: &NormalizationConfig) -> Result<Vec<RootRule>, NormalizeError> {
    let mut roots = Vec::new();

    if let Some(path) = &config.workspace {
        roots.push(root_rule(
            path,
            "$WORKSPACE",
            PathClass::Workspace,
            "workspace",
        )?);
    }
    if let Some(path) = &config.home {
        roots.push(root_rule(path, "$HOME", PathClass::Home, "home")?);
    }
    for path in &config.tmp_roots {
        roots.push(root_rule(path, "$TMP", PathClass::Temp, "tmp")?);
    }
    if let Some(path) = &config.run_tmp {
        roots.push(root_rule(path, "$RUN_TMP", PathClass::RunTemp, "run_tmp")?);
    }
    for (name, path) in &config.caches {
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(NormalizeError::InvalidRoot {
                label: format!("cache:{name}"),
                reason: "cache name must use only ASCII letters, digits, '.', '-' or '_'"
                    .to_owned(),
            });
        }
        roots.push(root_rule(
            path,
            &format!("$CACHE:{name}"),
            PathClass::Cache,
            &format!("cache:{name}"),
        )?);
    }

    let mut by_path: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for root in &roots {
        by_path
            .entry(root.physical.clone())
            .or_default()
            .push(root.label.clone());
    }
    for (path, labels) in by_path {
        let distinct = labels.iter().collect::<BTreeSet<_>>();
        if distinct.len() > 1 {
            return Err(NormalizeError::AmbiguousRoot { path, labels });
        }
    }

    if let Some(home) = roots.iter().find(|root| root.label == "home") {
        for root in roots.iter().filter(|root| root.label != "home") {
            if is_sensitive_path_under_home(&root.physical, &home.physical) {
                return Err(NormalizeError::InvalidRoot {
                    label: root.label.clone(),
                    reason: "semantic roots may not shadow credential-sensitive paths under $HOME"
                        .to_owned(),
                });
            }
        }
    }

    roots.sort_by(|left, right| {
        right
            .physical
            .len()
            .cmp(&left.physical.len())
            .then_with(|| left.token.cmp(&right.token))
    });
    Ok(roots)
}

fn root_rule(
    path: &str,
    token: &str,
    class: PathClass,
    label: &str,
) -> Result<RootRule, NormalizeError> {
    if !path.starts_with('/') {
        return Err(NormalizeError::InvalidRoot {
            label: label.to_owned(),
            reason: "root must be absolute".to_owned(),
        });
    }
    if has_parent_traversal(path) {
        return Err(NormalizeError::InvalidRoot {
            label: label.to_owned(),
            reason: "root must not contain '..' path traversal".to_owned(),
        });
    }
    let physical = clean_lexical_path(path);
    Ok(RootRule {
        physical,
        token: token.to_owned(),
        class,
        label: label.to_owned(),
    })
}

fn canonical_executable(path: &str, roots: &[RootRule]) -> CanonicalExecutable {
    let path = canonical_path(path, roots);
    let family = path
        .value
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or(path.value.as_str())
        .to_owned();
    CanonicalExecutable { path, family }
}

fn canonical_path(path: &str, roots: &[RootRule]) -> CanonicalPath {
    if !path.starts_with('/') {
        return CanonicalPath {
            value: clean_lexical_path(path),
            class: PathClass::Unknown,
            resolution: PathResolution::RelativeUnresolved,
        };
    }

    if has_parent_traversal(path) {
        return CanonicalPath {
            value: clean_lexical_path(path),
            class: PathClass::Unknown,
            resolution: PathResolution::ContainsParentTraversal,
        };
    }

    let cleaned = clean_lexical_path(path);

    for root in roots {
        if let Some(suffix) = root_suffix(&cleaned, &root.physical) {
            let value = if suffix.is_empty() {
                root.token.clone()
            } else {
                format!("{}{suffix}", root.token)
            };
            let class = classify_tokenized_path(&value, root.class);
            return CanonicalPath {
                value,
                class,
                resolution: PathResolution::Lexical,
            };
        }
    }

    CanonicalPath {
        class: classify_absolute_path(&cleaned),
        value: cleaned,
        resolution: PathResolution::Lexical,
    }
}

fn canonical_kernel_fd_path(path: &str, roots: &[RootRule]) -> CanonicalPath {
    let mut canonical = canonical_path(path, roots);
    canonical.resolution = PathResolution::KernelFdResolved;
    canonical
}

fn root_suffix<'a>(path: &'a str, root: &str) -> Option<&'a str> {
    if path == root {
        return Some("");
    }
    if root == "/" {
        return Some(path);
    }
    path.strip_prefix(root)
        .filter(|suffix| suffix.starts_with('/'))
}

fn classify_tokenized_path(value: &str, default: PathClass) -> PathClass {
    if value == "$HOME/.ssh"
        || value.starts_with("$HOME/.ssh/")
        || value == "$HOME/.aws"
        || value.starts_with("$HOME/.aws/")
        || value == "$HOME/.config/gcloud"
        || value.starts_with("$HOME/.config/gcloud/")
    {
        return PathClass::CredentialSensitive;
    }
    default
}

fn is_sensitive_path_under_home(path: &str, home: &str) -> bool {
    root_suffix(path, home).is_some_and(|suffix| {
        suffix == "/.ssh"
            || suffix.starts_with("/.ssh/")
            || suffix == "/.aws"
            || suffix.starts_with("/.aws/")
            || suffix == "/.config/gcloud"
            || suffix.starts_with("/.config/gcloud/")
    })
}

fn classify_absolute_path(path: &str) -> PathClass {
    if path == "/dev" || path.starts_with("/dev/") {
        PathClass::Device
    } else if [
        "/bin", "/sbin", "/usr", "/lib", "/lib64", "/etc", "/opt", "/proc", "/sys",
    ]
    .iter()
    .any(|root| path == *root || path.starts_with(&format!("{root}/")))
    {
        PathClass::System
    } else {
        PathClass::OutsideDeclaredRoots
    }
}

fn canonical_endpoint(endpoint: &NetworkEndpoint, roots: &[RootRule]) -> CanonicalNetworkEndpoint {
    match endpoint {
        NetworkEndpoint::Inet { ip, port } => CanonicalNetworkEndpoint::Inet {
            ip: ip.clone(),
            port: *port,
        },
        NetworkEndpoint::Inet6 { ip, port } => CanonicalNetworkEndpoint::Inet6 {
            ip: ip.clone(),
            port: *port,
        },
        NetworkEndpoint::Unix { path } => CanonicalNetworkEndpoint::Unix {
            path: path.as_deref().map(|path| canonical_path(path, roots)),
        },
        NetworkEndpoint::Other { family } => CanonicalNetworkEndpoint::Other { family: *family },
    }
}

fn linux_open_intent(
    platform: &str,
    flags: Option<u64>,
    resolve_flags: u64,
) -> Result<OpenIntent, NormalizeError> {
    if platform != "linux" {
        return Ok(OpenIntent {
            read: false,
            write: false,
            create: false,
            truncate: false,
            append: false,
            path_only: false,
            resolve_flags,
            other_flags: flags.unwrap_or_default(),
        });
    }

    let flags = flags.ok_or(NormalizeError::MissingLinuxOpenFlags)?;
    let flags_i32 = flags as i32;
    let path_only = flags_i32 & libc::O_PATH != 0;
    let access = flags_i32 & libc::O_ACCMODE;

    let (read, write) = if path_only {
        (false, false)
    } else if access == libc::O_WRONLY {
        (false, true)
    } else if access == libc::O_RDWR {
        (true, true)
    } else {
        (true, false)
    };

    let known_mask =
        (libc::O_ACCMODE | libc::O_CREAT | libc::O_TRUNC | libc::O_APPEND | libc::O_PATH) as u64;

    Ok(OpenIntent {
        read,
        write,
        create: flags_i32 & libc::O_CREAT != 0,
        truncate: flags_i32 & libc::O_TRUNC != 0,
        append: flags_i32 & libc::O_APPEND != 0,
        path_only,
        resolve_flags,
        other_flags: flags & !known_mask,
    })
}

fn has_parent_traversal(path: &str) -> bool {
    path.split('/').any(|segment| segment == "..")
}

fn clean_lexical_path(path: &str) -> String {
    let absolute = path.starts_with('/');
    let mut segments = Vec::new();
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        segments.push(segment);
    }

    let body = segments.join("/");
    if absolute {
        if body.is_empty() {
            "/".to_owned()
        } else {
            format!("/{body}")
        }
    } else if body.is_empty() {
        ".".to_owned()
    } else {
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_model::{BackendMetadata, CommandOutcome, RawEvent, SpawnMechanism};

    fn observation(events: Vec<RawEvent>) -> Observation {
        Observation {
            schema_version: RAW_OBSERVATION_SCHEMA_VERSION,
            backend: BackendMetadata {
                name: "test".to_owned(),
                platform: "linux".to_owned(),
                architecture: "x86_64".to_owned(),
                capabilities: vec![],
                limitations: vec![],
            },
            complete: true,
            outcome: CommandOutcome::default(),
            events,
            warnings: vec![],
        }
    }

    fn config_a() -> NormalizationConfig {
        NormalizationConfig {
            workspace: Some("/home/runner/work/repo".to_owned()),
            home: Some("/home/runner".to_owned()),
            tmp_roots: vec!["/tmp".to_owned()],
            run_tmp: Some("/tmp/run-A".to_owned()),
            caches: BTreeMap::from([("cargo".to_owned(), "/home/runner/.cargo".to_owned())]),
        }
    }

    fn config_b() -> NormalizationConfig {
        NormalizationConfig {
            workspace: Some("/builds/project/repo".to_owned()),
            home: Some("/home/ci".to_owned()),
            tmp_roots: vec!["/var/tmp".to_owned()],
            run_tmp: Some("/var/tmp/run-B".to_owned()),
            caches: BTreeMap::from([("cargo".to_owned(), "/home/ci/.cargo".to_owned())]),
        }
    }

    #[test]
    fn same_logical_behavior_survives_pid_sequence_root_and_interleaving_variance() {
        let first = observation(vec![
            RawEvent {
                sequence: 1,
                tid: 10,
                kind: RawEventKind::ProcessExec {
                    path: "/usr/bin/python3".to_owned(),
                },
            },
            RawEvent {
                sequence: 2,
                tid: 10,
                kind: RawEventKind::FilePathAccess {
                    operation: FileOperation::Open,
                    path: "/home/runner/work/repo/data/input.txt".to_owned(),
                    flags: Some(libc::O_RDONLY as u64),
                },
            },
            RawEvent {
                sequence: 3,
                tid: 10,
                kind: RawEventKind::ProcessSpawn {
                    child_tid: 20,
                    mechanism: SpawnMechanism::Fork,
                },
            },
            RawEvent {
                sequence: 4,
                tid: 20,
                kind: RawEventKind::ProcessExec {
                    path: "/tmp/run-A/helper".to_owned(),
                },
            },
            RawEvent {
                sequence: 5,
                tid: 20,
                kind: RawEventKind::NetworkConnectAttempt {
                    endpoint: NetworkEndpoint::Inet {
                        ip: "127.0.0.1".to_owned(),
                        port: 443,
                    },
                },
            },
        ]);

        let second = observation(vec![
            RawEvent {
                sequence: 100,
                tid: 700,
                kind: RawEventKind::ProcessExec {
                    path: "/usr/bin/python3".to_owned(),
                },
            },
            RawEvent {
                sequence: 120,
                tid: 700,
                kind: RawEventKind::ProcessSpawn {
                    child_tid: 900,
                    mechanism: SpawnMechanism::Fork,
                },
            },
            RawEvent {
                sequence: 130,
                tid: 900,
                kind: RawEventKind::ProcessExec {
                    path: "/var/tmp/run-B/helper".to_owned(),
                },
            },
            RawEvent {
                sequence: 140,
                tid: 900,
                kind: RawEventKind::NetworkConnectAttempt {
                    endpoint: NetworkEndpoint::Inet {
                        ip: "127.0.0.1".to_owned(),
                        port: 443,
                    },
                },
            },
            RawEvent {
                sequence: 150,
                tid: 700,
                kind: RawEventKind::FilePathAccess {
                    operation: FileOperation::Open,
                    path: "/builds/project/repo/data/input.txt".to_owned(),
                    flags: Some(libc::O_RDONLY as u64),
                },
            },
        ]);

        assert_eq!(
            canonicalize(&first, &config_a()).expect("first"),
            canonicalize(&second, &config_b()).expect("second")
        );
    }

    #[test]
    fn explicit_run_root_normalization_preserves_security_relevant_suffix() {
        let curl = canonical_path(
            "/tmp/run-A/plugin/curl",
            &build_root_rules(&config_a()).unwrap(),
        );
        let ssh = canonical_path(
            "/tmp/run-A/plugin/ssh",
            &build_root_rules(&config_a()).unwrap(),
        );

        assert_eq!(curl.value, "$RUN_TMP/plugin/curl");
        assert_eq!(ssh.value, "$RUN_TMP/plugin/ssh");
        assert_ne!(curl, ssh);
    }

    #[test]
    fn credential_path_remains_specific_and_sensitive() {
        let path = canonical_path(
            "/home/runner/.ssh/config",
            &build_root_rules(&config_a()).unwrap(),
        );
        assert_eq!(path.value, "$HOME/.ssh/config");
        assert_eq!(path.class, PathClass::CredentialSensitive);
    }

    #[test]
    fn semantic_root_cannot_shadow_credential_namespace() {
        let mut config = config_a();
        config
            .caches
            .insert("aws".to_owned(), "/home/runner/.aws".to_owned());
        assert!(matches!(
            canonicalize(&observation(vec![]), &config),
            Err(NormalizeError::InvalidRoot { label, .. }) if label == "cache:aws"
        ));
    }

    #[test]
    fn relative_and_parent_traversal_paths_are_not_guessed() {
        let roots = build_root_rules(&config_a()).unwrap();
        let relative = canonical_path("../secrets", &roots);
        let traversal = canonical_path("/home/runner/work/repo/../.ssh/config", &roots);

        assert_eq!(relative.resolution, PathResolution::RelativeUnresolved);
        assert_eq!(relative.class, PathClass::Unknown);
        assert_eq!(
            traversal.resolution,
            PathResolution::ContainsParentTraversal
        );
        assert_eq!(traversal.class, PathClass::Unknown);
    }

    #[test]
    fn root_matching_respects_path_component_boundaries() {
        let roots = build_root_rules(&config_a()).unwrap();
        let path = canonical_path("/home/runner/work/repository/file", &roots);
        assert_eq!(path.class, PathClass::Home);
        assert_eq!(path.value, "$HOME/work/repository/file");
    }

    #[test]
    fn duplicate_raw_effects_collapse_deterministically() {
        let events = vec![
            RawEvent {
                sequence: 1,
                tid: 1,
                kind: RawEventKind::ProcessExec {
                    path: "/usr/bin/python3".to_owned(),
                },
            },
            RawEvent {
                sequence: 2,
                tid: 1,
                kind: RawEventKind::NetworkConnectAttempt {
                    endpoint: NetworkEndpoint::Inet {
                        ip: "192.0.2.10".to_owned(),
                        port: 443,
                    },
                },
            },
            RawEvent {
                sequence: 3,
                tid: 1,
                kind: RawEventKind::NetworkConnectAttempt {
                    endpoint: NetworkEndpoint::Inet {
                        ip: "192.0.2.10".to_owned(),
                        port: 443,
                    },
                },
            },
        ];

        let surface = canonicalize(&observation(events), &NormalizationConfig::default()).unwrap();
        let connects = surface
            .effects
            .iter()
            .filter(|effect| matches!(effect, CanonicalEffect::NetworkConnectAttempt { .. }))
            .count();
        assert_eq!(connects, 1);
    }

    #[test]
    fn remote_destination_port_remains_significant() {
        let roots = build_root_rules(&NormalizationConfig::default()).unwrap();
        let a = canonical_endpoint(
            &NetworkEndpoint::Inet {
                ip: "192.0.2.1".to_owned(),
                port: 443,
            },
            &roots,
        );
        let b = canonical_endpoint(
            &NetworkEndpoint::Inet {
                ip: "192.0.2.1".to_owned(),
                port: 8443,
            },
            &roots,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn linux_open_access_mode_is_not_collapsed() {
        let read = linux_open_intent("linux", Some(libc::O_RDONLY as u64), 0).unwrap();
        let write = linux_open_intent("linux", Some(libc::O_WRONLY as u64), 0).unwrap();
        assert_ne!(read, write);
        assert!(read.read);
        assert!(write.write);
    }

    #[test]
    fn incomplete_observation_is_rejected() {
        let mut raw = observation(vec![]);
        raw.complete = false;
        assert_eq!(
            canonicalize(&raw, &NormalizationConfig::default()),
            Err(NormalizeError::IncompleteObservation)
        );
    }

    #[test]
    fn duplicate_sequence_is_rejected() {
        let raw = observation(vec![
            RawEvent {
                sequence: 1,
                tid: 1,
                kind: RawEventKind::ProcessExec {
                    path: "/bin/true".to_owned(),
                },
            },
            RawEvent {
                sequence: 1,
                tid: 2,
                kind: RawEventKind::ProcessExec {
                    path: "/bin/false".to_owned(),
                },
            },
        ]);
        assert_eq!(
            canonicalize(&raw, &NormalizationConfig::default()),
            Err(NormalizeError::DuplicateSequence(1))
        );
    }

    #[test]
    fn same_physical_root_cannot_have_conflicting_semantics() {
        let config = NormalizationConfig {
            workspace: Some("/x".to_owned()),
            home: Some("/x".to_owned()),
            ..NormalizationConfig::default()
        };
        assert!(matches!(
            canonicalize(&observation(vec![]), &config),
            Err(NormalizeError::AmbiguousRoot { .. })
        ));
    }

    #[test]
    fn fd_attributed_read_is_kernel_resolved_and_keeps_execution_chain() {
        let raw = observation(vec![
            RawEvent {
                sequence: 1,
                tid: 10,
                kind: RawEventKind::ProcessExec {
                    path: "/bin/sh".to_owned(),
                },
            },
            RawEvent {
                sequence: 2,
                tid: 10,
                kind: RawEventKind::ProcessSpawn {
                    child_tid: 20,
                    mechanism: SpawnMechanism::Fork,
                },
            },
            RawEvent {
                sequence: 3,
                tid: 20,
                kind: RawEventKind::ProcessExec {
                    path: "/usr/bin/cat".to_owned(),
                },
            },
            RawEvent {
                sequence: 4,
                tid: 20,
                kind: RawEventKind::FileDescriptorAccess {
                    operation: FileOperation::Read,
                    fd: 3,
                    path: "/home/runner/work/repo/data.txt".to_owned(),
                },
            },
        ]);

        let surface = canonicalize(&raw, &config_a()).expect("canonicalize");
        let effect = surface
            .effects
            .iter()
            .find(|effect| matches!(
                effect,
                CanonicalEffect::FilePathAccess {
                    operation: FileOperation::Read,
                    ..
                }
            ))
            .expect("read effect");

        match effect {
            CanonicalEffect::FilePathAccess {
                execution_chain,
                target,
                ..
            } => {
                assert_eq!(
                    execution_chain
                        .iter()
                        .map(|exec| exec.family.as_str())
                        .collect::<Vec<_>>(),
                    vec!["sh", "cat"]
                );
                assert_eq!(target.value, "$WORKSPACE/data.txt");
                assert_eq!(target.resolution, PathResolution::KernelFdResolved);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn openat2_resolve_flags_remain_semantically_visible() {
        let raw = observation(vec![
            RawEvent {
                sequence: 1,
                tid: 1,
                kind: RawEventKind::ProcessExec {
                    path: "/bin/demo".to_owned(),
                },
            },
            RawEvent {
                sequence: 2,
                tid: 1,
                kind: RawEventKind::FileOpenAt2 {
                    path: "/home/runner/work/repo/input".to_owned(),
                    flags: libc::O_RDONLY as u64,
                    resolve: 0x08,
                },
            },
        ]);
        let surface = canonicalize(&raw, &config_a()).expect("canonicalize");
        assert!(surface.effects.iter().any(|effect| matches!(
            effect,
            CanonicalEffect::FilePathAccess {
                open_intent: Some(OpenIntent { resolve_flags: 0x08, .. }),
                ..
            }
        )));
    }

    #[test]
    fn execution_chain_limit_fails_closed() {
        let mut events = Vec::new();
        for index in 0..=MAX_EXECUTION_CHAIN {
            events.push(RawEvent {
                sequence: index as u64 + 1,
                tid: 1,
                kind: RawEventKind::ProcessExec {
                    path: format!("/bin/exec-{index}"),
                },
            });
        }
        assert!(matches!(
            canonicalize(&observation(events), &NormalizationConfig::default()),
            Err(NormalizeError::ExecutionChainTooDeep { limit, .. })
                if limit == MAX_EXECUTION_CHAIN
        ));
    }

}
