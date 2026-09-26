//! Linux metadata-only observation backend.
//!
//! On Linux x86_64 the reference implementation uses ptrace and reads only
//! selected metadata pointers. It never dereferences argv or envp.
//!
//! M8 introduces an internal backend boundary before adding alternative
//! collectors. The public observation semantics remain unchanged: ptrace is
//! still the only enabled backend and the correctness reference.

use std::ffi::{CString, OsStr, OsString};
use std::fmt;
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

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux_ptrace;

/// Internal collection boundary introduced by M8.
///
/// Collection mechanism is deliberately kept behind this contract so that a
/// future eBPF backend can be evaluated without changing canonicalization,
/// baseline, diff, policy, or verdict semantics. Backends are not assumed to
/// be evidence-equivalent; comparability remains an explicit higher-level
/// decision.
trait ObservationBackend {
    fn observe(
        &self,
        spec: &CommandSpec,
        options: ObserveOptions,
    ) -> Result<Observation, ObserveError>;
}

/// Native Linux ptrace remains the sole enabled backend and the correctness
/// reference under the accepted M6.5 decision.
struct PtraceBackend;

impl ObservationBackend for PtraceBackend {
    fn observe(
        &self,
        spec: &CommandSpec,
        options: ObserveOptions,
    ) -> Result<Observation, ObserveError> {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            linux_ptrace::observe(spec, options)
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

    // M8.1 intentionally preserves the existing behavior exactly: callers do
    // not select a backend yet, and ptrace remains the only enabled path.
    PTRACE_BACKEND.observe(spec, options)
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
}
