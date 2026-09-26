//! Diagnostic-only M9 ptrace lifecycle probe.
//!
//! This binary intentionally does not change ExecSurface product semantics. It mirrors the
//! parent/descendant ptrace control flow closely enough to identify which ptrace request returns
//! ESRCH on process-dense external workloads.

use std::collections::HashMap;
use std::env;
use std::ffi::{c_void, CString, OsStr};
use std::fs;
use std::io;
use std::mem::{size_of, MaybeUninit};
use std::os::unix::ffi::OsStrExt;
use std::ptr;

const PTRACE_GET_SYSCALL_INFO_REQUEST: libc::c_uint = 0x420e;
const PTRACE_SYSCALL_INFO_ENTRY: u8 = 1;
const PTRACE_SYSCALL_INFO_EXIT: u8 = 2;

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

fn cstring(value: &OsStr) -> CString {
    CString::new(value.as_bytes()).expect("command contains NUL")
}

fn proc_exists(tid: libc::pid_t) -> bool {
    std::path::Path::new(&format!("/proc/{tid}")).exists()
}

fn proc_snapshot(tid: libc::pid_t) -> String {
    let Ok(status) = fs::read_to_string(format!("/proc/{tid}/status")) else {
        return "status=unreadable".to_owned();
    };
    let mut fields = Vec::new();
    for key in ["State:", "Tgid:", "Pid:", "PPid:", "TracerPid:"] {
        if let Some(line) = status.lines().find(|line| line.starts_with(key)) {
            fields.push(line.replace('\t', " "));
        }
    }
    fields.join(";")
}

fn fail(op: &str, tid: libc::pid_t, status: i32) -> io::Error {
    let error = io::Error::last_os_error();
    eprintln!(
        "M9_PTRACE_PROBE_ERROR op={op} tid={tid} errno={:?} error={} wait_status=0x{status:08x} proc_exists={} proc_status={}",
        error.raw_os_error(),
        error,
        proc_exists(tid),
        proc_snapshot(tid)
    );
    error
}

fn ptrace_unit(
    op: &str,
    request: libc::c_uint,
    tid: libc::pid_t,
    address: *mut c_void,
    data: *mut c_void,
    status: i32,
) -> io::Result<()> {
    let result = unsafe { libc::ptrace(request, tid, address, data) };
    if result == -1 {
        Err(fail(op, tid, status))
    } else {
        Ok(())
    }
}

fn resume(tid: libc::pid_t, signal: libc::c_int, status: i32) -> io::Result<()> {
    ptrace_unit(
        "PTRACE_SYSCALL",
        libc::PTRACE_SYSCALL,
        tid,
        ptr::null_mut(),
        signal as usize as *mut c_void,
        status,
    )
}

fn syscall_info(tid: libc::pid_t, status: i32) -> io::Result<PtraceSyscallInfo> {
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
        return Err(fail("PTRACE_GET_SYSCALL_INFO", tid, status));
    }
    Ok(unsafe { info.assume_init() })
}

fn event_message(tid: libc::pid_t, status: i32) -> io::Result<u64> {
    let mut value: libc::c_ulong = 0;
    ptrace_unit(
        "PTRACE_GETEVENTMSG",
        libc::PTRACE_GETEVENTMSG,
        tid,
        ptr::null_mut(),
        (&mut value as *mut libc::c_ulong).cast::<c_void>(),
        status,
    )?;
    Ok(value as u64)
}

fn main() -> io::Result<()> {
    let mut args = env::args_os().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|arg| arg == "--") {
        args.remove(0);
    }
    if args.is_empty() {
        eprintln!("usage: ptrace-esrch-probe -- PROGRAM [ARG ...]");
        std::process::exit(64);
    }

    let program = cstring(&args[0]);
    let cargs = args.iter().map(|arg| cstring(arg)).collect::<Vec<_>>();
    let mut argv = cargs.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();
    argv.push(ptr::null());

    let child = unsafe { libc::fork() };
    if child < 0 {
        return Err(io::Error::last_os_error());
    }
    if child == 0 {
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
            libc::execvp(program.as_ptr(), argv.as_ptr());
            libc::_exit(127);
        }
    }

    let mut initial_status = 0;
    if unsafe { libc::waitpid(child, &mut initial_status, 0) } < 0 {
        return Err(io::Error::last_os_error());
    }
    if !libc::WIFSTOPPED(initial_status) {
        eprintln!("M9_PTRACE_PROBE_PROTOCOL initial stop missing status=0x{initial_status:08x}");
        std::process::exit(65);
    }

    let options = libc::PTRACE_O_TRACESYSGOOD
        | libc::PTRACE_O_TRACEFORK
        | libc::PTRACE_O_TRACEVFORK
        | libc::PTRACE_O_TRACECLONE
        | libc::PTRACE_O_TRACEEXEC
        | libc::PTRACE_O_EXITKILL;
    ptrace_unit(
        "PTRACE_SETOPTIONS",
        libc::PTRACE_SETOPTIONS,
        child,
        ptr::null_mut(),
        options as usize as *mut c_void,
        initial_status,
    )?;

    let mut tracees = HashMap::from([(child, false)]);
    let mut last_syscall_nr: HashMap<libc::pid_t, u64> = HashMap::new();
    let mut waits: u64 = 0;
    let mut syscall_stops: u64 = 0;
    let mut ptrace_events: u64 = 0;
    resume(child, 0, initial_status)?;

    while !tracees.is_empty() {
        let mut status = 0;
        let tid = unsafe { libc::waitpid(-1, &mut status, libc::__WALL) };
        if tid < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            eprintln!("M9_PTRACE_PROBE_ERROR op=waitpid errno={:?} error={error}", error.raw_os_error());
            return Err(error);
        }
        waits += 1;

        if libc::WIFEXITED(status) || libc::WIFSIGNALED(status) {
            tracees.remove(&tid);
            last_syscall_nr.remove(&tid);
            continue;
        }
        if !libc::WIFSTOPPED(status) {
            continue;
        }

        let signal = libc::WSTOPSIG(status);
        let event = ((status as u32) >> 16) as libc::c_int;

        if signal == libc::SIGTRAP && event != 0 {
            ptrace_events += 1;
            if matches!(
                event,
                libc::PTRACE_EVENT_FORK | libc::PTRACE_EVENT_VFORK | libc::PTRACE_EVENT_CLONE
            ) {
                let new_tid = event_message(tid, status)? as libc::pid_t;
                tracees.entry(new_tid).or_insert(true);
            }
            if let Err(error) = resume(tid, 0, status) {
                eprintln!(
                    "M9_PTRACE_PROBE_CONTEXT kind=event tid={tid} event={event} tracked={} {}",
                    tracees.contains_key(&tid),
                    proc_snapshot(tid)
                );
                return Err(error);
            }
            continue;
        }

        if signal == (libc::SIGTRAP | 0x80) {
            syscall_stops += 1;
            let info = syscall_info(tid, status)?;
            let (phase, nr) = match info.op {
                PTRACE_SYSCALL_INFO_ENTRY => {
                    let nr = info.data[0];
                    last_syscall_nr.insert(tid, nr);
                    ("entry", Some(nr))
                }
                PTRACE_SYSCALL_INFO_EXIT => ("exit", last_syscall_nr.get(&tid).copied()),
                _ => ("other", last_syscall_nr.get(&tid).copied()),
            };
            if let Err(error) = resume(tid, 0, status) {
                eprintln!(
                    "M9_PTRACE_PROBE_CONTEXT kind=syscall tid={tid} phase={phase} nr={nr:?} info_op={} ip=0x{:x} sp=0x{:x} tracked={} {}",
                    info.op,
                    info.instruction_pointer,
                    info.stack_pointer,
                    tracees.contains_key(&tid),
                    proc_snapshot(tid)
                );
                return Err(error);
            }
            continue;
        }

        let newborn = tracees.get_mut(&tid).is_some_and(|flag| {
            let was_newborn = *flag;
            *flag = false;
            was_newborn
        });
        let forwarded = if newborn || signal == libc::SIGTRAP { 0 } else { signal };
        if let Err(error) = resume(tid, forwarded, status) {
            eprintln!(
                "M9_PTRACE_PROBE_CONTEXT kind=signal tid={tid} signal={signal} newborn={newborn} tracked={} {}",
                tracees.contains_key(&tid),
                proc_snapshot(tid)
            );
            return Err(error);
        }
    }

    eprintln!(
        "M9_PTRACE_PROBE_COMPLETE root={child} waits={waits} syscall_stops={syscall_stops} ptrace_events={ptrace_events}"
    );
    Ok(())
}
