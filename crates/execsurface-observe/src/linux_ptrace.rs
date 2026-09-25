use std::collections::HashMap;
use std::ffi::c_void;
use std::io;
use std::mem::{size_of, MaybeUninit};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr;

use execsurface_model::{
    BackendMetadata, CommandOutcome, FileOperation, NetworkEndpoint, Observation, ObserverWarning,
    RawEvent, RawEventKind, SpawnMechanism,
};

use crate::{CommandSpec, ObserveError};

const MAX_PATH_BYTES: usize = 4096;
const MAX_SOCKADDR_BYTES: usize = 128;
const PTRACE_GET_SYSCALL_INFO_REQUEST: libc::c_uint = 0x420e;
const PTRACE_SYSCALL_INFO_ENTRY: u8 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
struct PtraceSyscallInfo {
    op: u8,
    pad: [u8; 3],
    arch: u32,
    instruction_pointer: u64,
    stack_pointer: u64,
    data: [u64; 7],
}

impl PtraceSyscallInfo {
    fn entry(&self) -> Option<(u64, [u64; 6])> {
        if self.op != PTRACE_SYSCALL_INFO_ENTRY {
            return None;
        }
        Some((
            self.data[0],
            [
                self.data[1],
                self.data[2],
                self.data[3],
                self.data[4],
                self.data[5],
                self.data[6],
            ],
        ))
    }
}

#[derive(Default)]
struct TraceeState {
    pending_exec: Option<String>,
    newborn: bool,
}

struct Collector {
    observation: Observation,
    sequence: u64,
}

impl Collector {
    fn new() -> Self {
        let backend = BackendMetadata {
            name: "linux-ptrace-metadata-only".to_owned(),
            platform: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
            capabilities: vec![
                "descendant_tracking".to_owned(),
                "exec_path".to_owned(),
                "path_syscall_attempts".to_owned(),
                "connect_destination".to_owned(),
            ],
            limitations: vec![
                "M1 observes selected syscall metadata, not all runtime behavior".to_owned(),
                "file events are path-based syscall attempts; fd read/write attribution is not implemented".to_owned(),
                "ptrace can perturb scheduling and trace-aware programs can behave differently".to_owned(),
                "hostname intent is not inferred from connect(2)".to_owned(),
                "Linux x86_64 only".to_owned(),
            ],
        };
        Self {
            observation: Observation::empty(backend),
            sequence: 0,
        }
    }

    fn event(&mut self, tid: libc::pid_t, kind: RawEventKind) {
        self.sequence += 1;
        self.observation.events.push(RawEvent {
            sequence: self.sequence,
            tid,
            kind,
        });
    }

    fn warning(&mut self, tid: libc::pid_t, code: &str, message: impl Into<String>) {
        self.observation.complete = false;
        self.observation.warnings.push(ObserverWarning {
            code: code.to_owned(),
            tid: Some(tid),
            message: message.into(),
        });
    }
}

pub(super) fn observe(spec: &CommandSpec) -> Result<Observation, ObserveError> {
    let (program, argv) = spec.c_argv()?;
    let mut argv_ptrs: Vec<*const libc::c_char> = argv.iter().map(|arg| arg.as_ptr()).collect();
    argv_ptrs.push(ptr::null());

    // SAFETY: after fork, the child uses only libc tracing/signal/exec/exit calls.
    let child = unsafe { libc::fork() };
    if child < 0 {
        return Err(io::Error::last_os_error().into());
    }

    if child == 0 {
        // SAFETY: child-only path before exec.
        unsafe {
            if libc::ptrace(
                libc::PTRACE_TRACEME,
                0,
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
            ) == -1
            {
                libc::_exit(126);
            }
            if libc::raise(libc::SIGSTOP) != 0 {
                libc::_exit(126);
            }
            libc::execvp(program.as_ptr(), argv_ptrs.as_ptr());
            libc::_exit(127);
        }
    }

    trace_parent(child)
}

fn trace_parent(root: libc::pid_t) -> Result<Observation, ObserveError> {
    let mut status = 0;
    if unsafe { libc::waitpid(root, &mut status, 0) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if !libc::WIFSTOPPED(status) {
        return Err(ObserveError::Protocol(
            "tracee did not enter the expected initial stop".to_owned(),
        ));
    }

    let options = libc::PTRACE_O_TRACESYSGOOD
        | libc::PTRACE_O_TRACEFORK
        | libc::PTRACE_O_TRACEVFORK
        | libc::PTRACE_O_TRACECLONE
        | libc::PTRACE_O_TRACEEXEC
        | libc::PTRACE_O_EXITKILL;

    ptrace_call(
        libc::PTRACE_SETOPTIONS,
        root,
        ptr::null_mut(),
        options as usize as *mut c_void,
    )?;

    let mut tracees = HashMap::new();
    tracees.insert(root, TraceeState::default());

    let mut collector = Collector::new();
    let mut root_outcome = CommandOutcome::default();

    resume_syscall(root, 0)?;

    while !tracees.is_empty() {
        let mut wait_status = 0;
        let tid = unsafe { libc::waitpid(-1, &mut wait_status, libc::__WALL) };
        if tid < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            return Err(error.into());
        }

        if libc::WIFEXITED(wait_status) {
            if tid == root {
                root_outcome.exit_code = Some(libc::WEXITSTATUS(wait_status));
            }
            tracees.remove(&tid);
            continue;
        }

        if libc::WIFSIGNALED(wait_status) {
            if tid == root {
                root_outcome.signal = Some(libc::WTERMSIG(wait_status));
            }
            tracees.remove(&tid);
            continue;
        }

        if !libc::WIFSTOPPED(wait_status) {
            continue;
        }

        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_uint;

        if stop_signal == libc::SIGTRAP && event != 0 {
            handle_ptrace_event(tid, event, &mut tracees, &mut collector)?;
            resume_syscall(tid, 0)?;
            continue;
        }

        if stop_signal == (libc::SIGTRAP | 0x80) {
            let info = syscall_info(tid)?;
            if let Some((nr, args)) = info.entry() {
                handle_syscall_entry(tid, nr, args, &mut tracees, &mut collector);
            }
            resume_syscall(tid, 0)?;
            continue;
        }

        let suppress_signal = match tracees.get_mut(&tid) {
            Some(state) if state.newborn => {
                state.newborn = false;
                true
            }
            _ => stop_signal == libc::SIGTRAP,
        };

        resume_syscall(tid, if suppress_signal { 0 } else { stop_signal })?;
    }

    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}

fn handle_ptrace_event(
    tid: libc::pid_t,
    event: libc::c_uint,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    collector: &mut Collector,
) -> Result<(), ObserveError> {
    match event {
        libc::PTRACE_EVENT_FORK | libc::PTRACE_EVENT_VFORK | libc::PTRACE_EVENT_CLONE => {
            let child_tid = get_event_message(tid)? as libc::pid_t;
            let mechanism = match event {
                libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,
                libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,
                _ => SpawnMechanism::Clone,
            };
            tracees.entry(child_tid).or_insert(TraceeState {
                pending_exec: None,
                newborn: true,
            });
            collector.event(
                tid,
                RawEventKind::ProcessSpawn {
                    child_tid,
                    mechanism,
                },
            );
        }
        libc::PTRACE_EVENT_EXEC => {
            let state = tracees.entry(tid).or_default();
            match state.pending_exec.take() {
                Some(path) => collector.event(tid, RawEventKind::ProcessExec { path }),
                None => collector.warning(
                    tid,
                    "exec_without_path",
                    "PTRACE_EVENT_EXEC was observed without a readable pending exec pathname",
                ),
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_syscall_entry(
    tid: libc::pid_t,
    nr: u64,
    args: [u64; 6],
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    collector: &mut Collector,
) {
    let nr = nr as libc::c_long;

    if nr == libc::SYS_execve {
        record_exec_path(tid, args[0], tracees, collector);
        return;
    }
    if nr == libc::SYS_execveat {
        record_exec_path(tid, args[1], tracees, collector);
        return;
    }

    if nr == libc::SYS_open {
        record_file_path(tid, FileOperation::Open, args[0], Some(args[1]), collector);
        return;
    }
    if nr == libc::SYS_openat {
        record_file_path(tid, FileOperation::Open, args[1], Some(args[2]), collector);
        return;
    }
    if nr == libc::SYS_creat {
        record_file_path(tid, FileOperation::Create, args[0], None, collector);
        return;
    }
    if nr == libc::SYS_unlink {
        record_file_path(tid, FileOperation::Delete, args[0], None, collector);
        return;
    }
    if nr == libc::SYS_unlinkat {
        record_file_path(
            tid,
            FileOperation::Delete,
            args[1],
            Some(args[2]),
            collector,
        );
        return;
    }
    if nr == libc::SYS_rename {
        record_rename(tid, args[0], args[1], collector);
        return;
    }
    if nr == libc::SYS_renameat {
        record_rename(tid, args[1], args[3], collector);
        return;
    }
    if nr == libc::SYS_renameat2 {
        record_rename(tid, args[1], args[3], collector);
        return;
    }
    if nr == libc::SYS_connect {
        record_connect(tid, args[1], args[2], collector);
    }
}

fn record_exec_path(
    tid: libc::pid_t,
    address: u64,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    collector: &mut Collector,
) {
    match read_c_string(tid, address, MAX_PATH_BYTES) {
        Ok(path) => {
            tracees.entry(tid).or_default().pending_exec = Some(path);
        }
        Err(error) => collector.warning(tid, "exec_path_unreadable", error.to_string()),
    }
}

fn record_file_path(
    tid: libc::pid_t,
    operation: FileOperation,
    address: u64,
    flags: Option<u64>,
    collector: &mut Collector,
) {
    match read_c_string(tid, address, MAX_PATH_BYTES) {
        Ok(path) => collector.event(
            tid,
            RawEventKind::FilePathAccess {
                operation,
                path,
                flags,
            },
        ),
        Err(error) => collector.warning(tid, "file_path_unreadable", error.to_string()),
    }
}

fn record_rename(tid: libc::pid_t, from: u64, to: u64, collector: &mut Collector) {
    let from = read_c_string(tid, from, MAX_PATH_BYTES);
    let to = read_c_string(tid, to, MAX_PATH_BYTES);
    match (from, to) {
        (Ok(from), Ok(to)) => collector.event(tid, RawEventKind::FileRename { from, to }),
        (Err(error), _) | (_, Err(error)) => {
            collector.warning(tid, "rename_path_unreadable", error.to_string())
        }
    }
}

fn record_connect(tid: libc::pid_t, address: u64, length: u64, collector: &mut Collector) {
    let length = usize::try_from(length)
        .unwrap_or(MAX_SOCKADDR_BYTES)
        .min(MAX_SOCKADDR_BYTES);

    match read_memory(tid, address, length).and_then(|bytes| parse_sockaddr(&bytes)) {
        Ok(endpoint) => collector.event(tid, RawEventKind::NetworkConnectAttempt { endpoint }),
        Err(error) => collector.warning(tid, "connect_target_unreadable", error.to_string()),
    }
}

fn syscall_info(tid: libc::pid_t) -> Result<PtraceSyscallInfo, ObserveError> {
    let mut info = MaybeUninit::<PtraceSyscallInfo>::zeroed();
    let result = unsafe {
        libc::ptrace(
            PTRACE_GET_SYSCALL_INFO_REQUEST,
            tid,
            size_of::<PtraceSyscallInfo>() as *mut c_void,
            info.as_mut_ptr() as *mut c_void,
        )
    };
    if result == -1 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(unsafe { info.assume_init() })
}

fn get_event_message(tid: libc::pid_t) -> Result<u64, ObserveError> {
    let mut value: libc::c_ulong = 0;
    ptrace_call(
        libc::PTRACE_GETEVENTMSG,
        tid,
        ptr::null_mut(),
        (&mut value as *mut libc::c_ulong).cast::<c_void>(),
    )?;
    Ok(value as u64)
}

fn resume_syscall(tid: libc::pid_t, signal: libc::c_int) -> Result<(), ObserveError> {
    ptrace_call(
        libc::PTRACE_SYSCALL,
        tid,
        ptr::null_mut(),
        signal as usize as *mut c_void,
    )
}

fn ptrace_call(
    request: libc::c_uint,
    tid: libc::pid_t,
    address: *mut c_void,
    data: *mut c_void,
) -> Result<(), ObserveError> {
    let result = unsafe { libc::ptrace(request, tid, address, data) };
    if result == -1 {
        Err(io::Error::last_os_error().into())
    } else {
        Ok(())
    }
}

fn read_c_string(tid: libc::pid_t, address: u64, max_len: usize) -> Result<String, ObserveError> {
    if address == 0 {
        return Err(ObserveError::Protocol("null string pointer".to_owned()));
    }

    let mut bytes = Vec::new();

    while bytes.len() < max_len {
        let word = peek_word(tid, address + bytes.len() as u64)?;
        for byte in word.to_ne_bytes() {
            if byte == 0 {
                return Ok(String::from_utf8_lossy(&bytes).into_owned());
            }
            bytes.push(byte);
            if bytes.len() == max_len {
                break;
            }
        }
    }

    Err(ObserveError::Protocol(format!(
        "string exceeded metadata limit of {max_len} bytes"
    )))
}

fn read_memory(tid: libc::pid_t, address: u64, len: usize) -> Result<Vec<u8>, ObserveError> {
    if address == 0 {
        return Err(ObserveError::Protocol("null memory pointer".to_owned()));
    }
    let mut bytes = Vec::with_capacity(len);
    while bytes.len() < len {
        let word = peek_word(tid, address + bytes.len() as u64)?;
        let word_bytes = word.to_ne_bytes();
        let remaining = len - bytes.len();
        bytes.extend_from_slice(&word_bytes[..remaining.min(word_bytes.len())]);
    }
    Ok(bytes)
}

fn peek_word(tid: libc::pid_t, address: u64) -> Result<libc::c_long, ObserveError> {
    set_errno(0);
    let result = unsafe {
        libc::ptrace(
            libc::PTRACE_PEEKDATA,
            tid,
            address as usize as *mut c_void,
            ptr::null_mut::<c_void>(),
        )
    };
    let error = io::Error::last_os_error();
    if result == -1 && error.raw_os_error() != Some(0) {
        return Err(error.into());
    }
    Ok(result)
}

fn set_errno(value: libc::c_int) {
    unsafe {
        *libc::__errno_location() = value;
    }
}

fn parse_sockaddr(bytes: &[u8]) -> Result<NetworkEndpoint, ObserveError> {
    if bytes.len() < 2 {
        return Err(ObserveError::Protocol(
            "sockaddr shorter than address-family field".to_owned(),
        ));
    }

    let family = u16::from_ne_bytes([bytes[0], bytes[1]]);
    match family as libc::c_int {
        libc::AF_INET => {
            if bytes.len() < 8 {
                return Err(ObserveError::Protocol(
                    "AF_INET sockaddr is truncated".to_owned(),
                ));
            }
            let port = u16::from_be_bytes([bytes[2], bytes[3]]);
            let ip = Ipv4Addr::new(bytes[4], bytes[5], bytes[6], bytes[7]);
            Ok(NetworkEndpoint::Inet {
                ip: ip.to_string(),
                port,
            })
        }
        libc::AF_INET6 => {
            if bytes.len() < 24 {
                return Err(ObserveError::Protocol(
                    "AF_INET6 sockaddr is truncated".to_owned(),
                ));
            }
            let port = u16::from_be_bytes([bytes[2], bytes[3]]);
            let mut octets = [0_u8; 16];
            octets.copy_from_slice(&bytes[8..24]);
            Ok(NetworkEndpoint::Inet6 {
                ip: Ipv6Addr::from(octets).to_string(),
                port,
            })
        }
        libc::AF_UNIX => {
            let path_bytes = bytes
                .get(2..)
                .unwrap_or_default()
                .iter()
                .copied()
                .take_while(|byte| *byte != 0)
                .collect::<Vec<_>>();
            let path = if path_bytes.is_empty() {
                None
            } else {
                Some(String::from_utf8_lossy(&path_bytes).into_owned())
            };
            Ok(NetworkEndpoint::Unix { path })
        }
        _ => Ok(NetworkEndpoint::Other { family }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ipv4_sockaddr_without_payload_data() {
        let bytes = [2, 0, 0x01, 0xbb, 127, 0, 0, 1];
        assert_eq!(
            parse_sockaddr(&bytes).expect("parse"),
            NetworkEndpoint::Inet {
                ip: "127.0.0.1".to_owned(),
                port: 443,
            }
        );
    }

    #[test]
    fn parses_ipv6_sockaddr_without_payload_data() {
        let mut bytes = vec![0_u8; 28];
        bytes[0..2].copy_from_slice(&(libc::AF_INET6 as u16).to_ne_bytes());
        bytes[2..4].copy_from_slice(&443_u16.to_be_bytes());
        bytes[23] = 1;
        assert_eq!(
            parse_sockaddr(&bytes).expect("parse"),
            NetworkEndpoint::Inet6 {
                ip: "::1".to_owned(),
                port: 443,
            }
        );
    }
}
