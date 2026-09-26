//! Linux metadata-only observation backend.
//!
//! On Linux x86_64 the reference implementation uses ptrace and reads only
//! selected metadata pointers. It never dereferences argv or envp.
//!
//! M8 introduces an internal backend boundary before adding alternative
//! collectors. The public observation semantics remain unchanged: ptrace is
//! still the default backend and the correctness reference.

use std::collections::BTreeSet;
use std::ffi::{CString, OsStr, OsString};
use std::fmt;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::sync::Mutex;

use execsurface_model::Observation;

pub const DEFAULT_EVENT_LIMIT: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObserveOptions {
    pub event_limit: usize,
}

impl Default for ObserveOptions {
    fn default() -> Self {
        Self {
            event_limit: DEFAULT_EVENT_LIMIT,
        }
    }
}

#[derive(Clone)]
pub struct CommandSpec {
    program: OsString,
    args: Vec<OsString>,
}

impl CommandSpec {
    pub fn new(program: impl Into<OsString>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    fn c_argv(&self) -> Result<(CString, Vec<CString>), ObserveError> {
        let program = cstring_from_os(&self.program)?;
        let mut argv = Vec::with_capacity(self.args.len() + 1);
        argv.push(cstring_from_os(&self.program)?);
        for arg in &self.args {
            argv.push(cstring_from_os(arg)?);
        }
        Ok((program, argv))
    }
}

fn cstring_from_os(value: &OsStr) -> Result<CString, ObserveError> {
    CString::new(value.as_bytes()).map_err(|_| {
        ObserveError::InvalidCommand("command contains an interior NUL byte".to_owned())
    })
}

#[derive(Debug)]
pub enum ObserveError {
    UnsupportedPlatform(&'static str),
    InvalidCommand(String),
    Os(io::Error),
    Protocol(String),
}

impl fmt::Display for ObserveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform(message) => write!(f, "unsupported platform: {message}"),
            Self::InvalidCommand(message) => write!(f, "invalid command: {message}"),
            Self::Os(error) => write!(f, "observer OS error: {error}"),
            Self::Protocol(message) => write!(f, "observer protocol error: {message}"),
        }
    }
}

impl std::error::Error for ObserveError {}

impl From<io::Error> for ObserveError {
    fn from(value: io::Error) -> Self {
        Self::Os(value)
    }
}

/// Typed observation capability vocabulary introduced by M8.3.
///
/// This is deliberately more granular than a single "eBPF supported" bit. A
/// backend must explicitly partition the whole vocabulary into supported and
/// unsupported sets for its declared contract version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObservationCapability {
    ProcessSpawnLineage,
    ProcessExec,
    ProcessExit,
    PathAccessIntent,
    SuccessfulOpenFdIdentity,
    FdReadWriteEffect,
    FdDupCloseLifecycle,
    ForkFdInheritance,
    CloseOnExec,
    RenameDeleteEffects,
    NetworkConnectDestination,
    TraceTimeRelativePath,
    CausalExecutableChain,
    LossTruncationVisibility,
}

pub const ALL_OBSERVATION_CAPABILITIES: [ObservationCapability; 14] = [
    ObservationCapability::ProcessSpawnLineage,
    ObservationCapability::ProcessExec,
    ObservationCapability::ProcessExit,
    ObservationCapability::PathAccessIntent,
    ObservationCapability::SuccessfulOpenFdIdentity,
    ObservationCapability::FdReadWriteEffect,
    ObservationCapability::FdDupCloseLifecycle,
    ObservationCapability::ForkFdInheritance,
    ObservationCapability::CloseOnExec,
    ObservationCapability::RenameDeleteEffects,
    ObservationCapability::NetworkConnectDestination,
    ObservationCapability::TraceTimeRelativePath,
    ObservationCapability::CausalExecutableChain,
    ObservationCapability::LossTruncationVisibility,
];

/// Machine-readable identity and capability declaration for one backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendDescriptor {
    pub id: String,
    pub implementation_version: String,
    pub platform: String,
    pub architecture: String,
    pub kernel_release: Option<String>,
    pub privacy_profile: String,
    pub capabilities: Vec<ObservationCapability>,
    pub unsupported_capabilities: Vec<ObservationCapability>,
}

impl BackendDescriptor {
    /// Prove that supported and unsupported sets form an exact, disjoint
    /// partition of the declared capability universe.
    pub fn validate_capability_partition(&self) -> Result<(), String> {
        let supported: BTreeSet<_> = self.capabilities.iter().copied().collect();
        let unsupported: BTreeSet<_> = self.unsupported_capabilities.iter().copied().collect();
        let universe: BTreeSet<_> = ALL_OBSERVATION_CAPABILITIES.iter().copied().collect();

        if supported.len() != self.capabilities.len() {
            return Err("backend descriptor repeats a supported capability".to_owned());
        }
        if unsupported.len() != self.unsupported_capabilities.len() {
            return Err("backend descriptor repeats an unsupported capability".to_owned());
        }
        if !supported.is_disjoint(&unsupported) {
            return Err("backend capability is both supported and unsupported".to_owned());
        }

        let declared: BTreeSet<_> = supported.union(&unsupported).copied().collect();
        if declared != universe {
            return Err(
                "backend descriptor does not classify the full capability universe".to_owned(),
            );
        }

        Ok(())
    }
}

/// Explicit health state at the backend-to-core handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionCompleteness {
    Complete,
    IncompleteLoss,
    IncompleteLimit,
    IncompleteCapability,
    Error,
}

impl CollectionCompleteness {
    pub fn pass_eligible(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Internal collection result. The legacy public observer functions continue
/// returning `Observation`; M8.3 uses this envelope at the backend boundary so
/// collection health cannot be confused with absence of drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendObservation {
    pub descriptor: BackendDescriptor,
    pub completeness: CollectionCompleteness,
    pub observation: Observation,
}

fn kernel_release() -> Option<String> {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn ptrace_backend_descriptor() -> BackendDescriptor {
    BackendDescriptor {
        id: "linux-ptrace-metadata-v2".to_owned(),
        implementation_version: env!("CARGO_PKG_VERSION").to_owned(),
        platform: std::env::consts::OS.to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
        kernel_release: kernel_release(),
        privacy_profile: "metadata-only-v1".to_owned(),
        capabilities: vec![
            ObservationCapability::ProcessSpawnLineage,
            ObservationCapability::ProcessExec,
            ObservationCapability::PathAccessIntent,
            ObservationCapability::SuccessfulOpenFdIdentity,
            ObservationCapability::FdReadWriteEffect,
            ObservationCapability::FdDupCloseLifecycle,
            ObservationCapability::ForkFdInheritance,
            ObservationCapability::CloseOnExec,
            ObservationCapability::RenameDeleteEffects,
            ObservationCapability::NetworkConnectDestination,
            ObservationCapability::TraceTimeRelativePath,
            ObservationCapability::LossTruncationVisibility,
        ],
        unsupported_capabilities: vec![
            ObservationCapability::ProcessExit,
            ObservationCapability::CausalExecutableChain,
        ],
    }
}

/// Descriptor for the selected M8.3 libbpf-rs path before product integration.
///
/// The supported subset is intentionally limited to semantics actually proved
/// by M8.2. Additional classes must move from unsupported to supported only
/// after their own evidence gate.
pub fn experimental_ebpf_backend_descriptor() -> BackendDescriptor {
    BackendDescriptor {
        id: "linux-libbpf-metadata-experimental-v1".to_owned(),
        implementation_version: "m8.3-experimental-v1".to_owned(),
        platform: "linux".to_owned(),
        architecture: "x86_64".to_owned(),
        kernel_release: kernel_release(),
        privacy_profile: "metadata-only-v1".to_owned(),
        capabilities: vec![
            ObservationCapability::ProcessSpawnLineage,
            ObservationCapability::ProcessExec,
            ObservationCapability::SuccessfulOpenFdIdentity,
            ObservationCapability::LossTruncationVisibility,
        ],
        unsupported_capabilities: vec![
            ObservationCapability::ProcessExit,
            ObservationCapability::PathAccessIntent,
            ObservationCapability::FdReadWriteEffect,
            ObservationCapability::FdDupCloseLifecycle,
            ObservationCapability::ForkFdInheritance,
            ObservationCapability::CloseOnExec,
            ObservationCapability::RenameDeleteEffects,
            ObservationCapability::NetworkConnectDestination,
            ObservationCapability::TraceTimeRelativePath,
            ObservationCapability::CausalExecutableChain,
        ],
    }
}

pub fn reference_backend_descriptor() -> BackendDescriptor {
    ptrace_backend_descriptor()
}

fn classify_observation_completeness(observation: &Observation) -> CollectionCompleteness {
    if observation.complete {
        return CollectionCompleteness::Complete;
    }

    if observation
        .warnings
        .iter()
        .any(|warning| warning.code == "event_limit_exceeded")
    {
        CollectionCompleteness::IncompleteLimit
    } else {
        // Existing ptrace warnings represent a known semantic/capability gap.
        // M8.4 will further refine transport/lifecycle loss classification for
        // the eBPF implementation.
        CollectionCompleteness::IncompleteCapability
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux_ptrace;

/// Internal collection boundary introduced by M8.
///
/// Collection mechanism is deliberately kept behind this contract so that an
/// eBPF backend can be evaluated without changing canonicalization, baseline,
/// diff, policy, or verdict semantics. Backends are not assumed to be
/// evidence-equivalent; comparability remains an explicit higher-level
/// decision.
trait ObservationBackend {
    fn descriptor(&self) -> BackendDescriptor;

    fn observe(
        &self,
        spec: &CommandSpec,
        options: ObserveOptions,
    ) -> Result<BackendObservation, ObserveError>;
}

/// Native Linux ptrace remains the default backend and the correctness
/// reference under the accepted M6.5 decision.
struct PtraceBackend;

impl ObservationBackend for PtraceBackend {
    fn descriptor(&self) -> BackendDescriptor {
        ptrace_backend_descriptor()
    }

    fn observe(
        &self,
        spec: &CommandSpec,
        options: ObserveOptions,
    ) -> Result<BackendObservation, ObserveError> {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            let observation = linux_ptrace::observe(spec, options)?;
            let descriptor = self.descriptor();
            descriptor
                .validate_capability_partition()
                .map_err(ObserveError::Protocol)?;

            if observation.backend.name != descriptor.id {
                return Err(ObserveError::Protocol(format!(
                    "ptrace observation backend identity mismatch: model={} descriptor={}",
                    observation.backend.name, descriptor.id
                )));
            }

            let completeness = classify_observation_completeness(&observation);
            Ok(BackendObservation {
                descriptor,
                completeness,
                observation,
            })
        }

        #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
        {
            let _ = (spec, options);
            Err(ObserveError::UnsupportedPlatform(
                "current observer supports Linux x86_64 only",
            ))
        }
    }
}

static PTRACE_BACKEND: PtraceBackend = PtraceBackend;
static OBSERVE_LOCK: Mutex<()> = Mutex::new(());

pub fn observe_command(spec: &CommandSpec) -> Result<Observation, ObserveError> {
    observe_command_with_options(spec, ObserveOptions::default())
}

pub fn observe_command_with_options(
    spec: &CommandSpec,
    options: ObserveOptions,
) -> Result<Observation, ObserveError> {
    let _session_guard = OBSERVE_LOCK.lock().map_err(|_| {
        ObserveError::Protocol("observer session serialization lock was poisoned".to_owned())
    })?;

    // Preserve the existing public behavior: callers still select no backend,
    // ptrace remains the default/reference path, and the return type is the
    // existing Observation schema. The richer M8 handoff remains internal
    // until backend selection semantics are separately accepted.
    Ok(PTRACE_BACKEND.observe(spec, options)?.observation)
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use std::os::unix::ffi::OsStringExt;

    #[test]
    fn invalid_command_metadata_returns_explicit_error() {
        let invalid = OsString::from_vec(b"bad\0program".to_vec());
        let result = observe_command(&CommandSpec::new(invalid));
        assert!(matches!(result, Err(ObserveError::InvalidCommand(_))));
    }

    #[test]
    fn default_event_budget_is_fail_closed_and_finite() {
        let options = ObserveOptions::default();
        assert_eq!(options.event_limit, DEFAULT_EVENT_LIMIT);
        assert!(options.event_limit >= 100_000);
        assert!(options.event_limit < usize::MAX);
    }

    #[test]
    fn ptrace_descriptor_partitions_every_capability() {
        let descriptor = reference_backend_descriptor();
        assert_eq!(descriptor.id, "linux-ptrace-metadata-v2");
        descriptor
            .validate_capability_partition()
            .expect("ptrace capability partition must be total and disjoint");
    }

    #[test]
    fn experimental_libbpf_descriptor_is_explicitly_partial() {
        let descriptor = experimental_ebpf_backend_descriptor();
        descriptor
            .validate_capability_partition()
            .expect("libbpf capability partition must be total and disjoint");

        assert!(descriptor
            .capabilities
            .contains(&ObservationCapability::ProcessSpawnLineage));
        assert!(descriptor
            .capabilities
            .contains(&ObservationCapability::ProcessExec));
        assert!(descriptor
            .capabilities
            .contains(&ObservationCapability::SuccessfulOpenFdIdentity));
        assert!(descriptor
            .capabilities
            .contains(&ObservationCapability::LossTruncationVisibility));
        assert!(descriptor
            .unsupported_capabilities
            .contains(&ObservationCapability::NetworkConnectDestination));
        assert!(descriptor
            .unsupported_capabilities
            .contains(&ObservationCapability::FdReadWriteEffect));
    }

    #[test]
    fn incomplete_observation_is_never_pass_eligible() {
        let mut observation = Observation::empty(execsurface_model::BackendMetadata {
            name: "linux-ptrace-metadata-v2".to_owned(),
            platform: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
            capabilities: Vec::new(),
            limitations: Vec::new(),
        });
        observation.complete = false;
        observation
            .warnings
            .push(execsurface_model::ObserverWarning {
                code: "event_limit_exceeded".to_owned(),
                tid: None,
                message: "controlled test".to_owned(),
            });

        let completeness = classify_observation_completeness(&observation);
        assert_eq!(completeness, CollectionCompleteness::IncompleteLimit);
        assert!(!completeness.pass_eligible());
        assert!(CollectionCompleteness::Complete.pass_eligible());
    }
}
