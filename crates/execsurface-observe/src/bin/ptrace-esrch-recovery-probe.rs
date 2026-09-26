//! M9 diagnostic recovery probe for the narrowly evidenced ptrace ESRCH race.
//!
//! This is not the product observer. It tests one proposed rule only:
//! if PTRACE_SYSCALL returns ESRCH, recover iff an immediate
//! waitpid(tid, __WALL | WNOHANG) returns that same tid in a terminal state.
//! Every other ESRCH remains fatal.

use std::collections::HashMap;
use std::env;
use std::ffi::{c_void, CString, OsStr};
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

#[derive(Debug, Clone, Copy)]
enum ResumeResult {
    Resumed,
    Reaped(i32),
}

fn cstring(value: &OsStr) -> CString {
    CString::new(value.as_bytes()).expect("command contains NUL")
}

fn terminal_status(status: i32) -> bool {
    libc::WIFEXITED(status) || libc::WIFSIGNALED(status)
}

fn ptrace_unit(
    op: &str,
    request: libc::c_uint,
    tid: libc::pid_t,
    address: *mut c_void,
    data: *mut c_void,
) -> io::Result<()> {
    let result = unsafe { libc::ptrace(request, tid, address, data) };
    if result == -1 {
        let error = io::Error::last_os_error();
        eprintln!(
            "M9_RECOVERY_PROBE_FATAL op={op} tid={tid} errno={:?} error={error}",
            error.raw_os_error()
        );
        Err(error)
    } else {
        Ok(())
    }
}

fn resume(tid: libc::pid_t, signal: libc::c_int) -> io::Result<ResumeResult> {
    let result = unsafe {
        libc::ptrace(
            libc::PTRACE_SYSCALL,
            tid,
            ptr::null_mut::<c_void>(),
            signal as usize as *mut c_void,
        )
    };
    if result != -1 {
        return Ok(ResumeResult::Resumed);
    }

    let original = io::Error::last_os_error();
    if original.raw_os_error() != Some(libc::ESRCH) {
        eprintln!(
            "M9_RECOVERY_PROBE_FATAL op=PTRACE_SYSCALL tid={tid} errno={:?} error={original}",
            original.raw_os_error()
        );
        return Err(original);
    }

    let mut status = 0;
    let waited = unsafe { libc::waitpid(tid, &mut status, libc::__WALL | libc::WNOHANG) };
    if waited == tid && terminal_status(status) {
        eprintln!(
            "M9_RECOVERY_PROBE_REAP tid={tid} wait_status=0x{status:08x} exited={} exit_code={} signaled={} signal={}",
            libc::WIFEXITED(status),
            if libc::WIFEXITED(status) { libc::WEXITSTATUS(status) } else { -1 },
            libc::WIFSIGNALED(status),
            if libc::WIFSIGNALED(status) { libc::WTERMSIG(status) } else { 0 },
        );
        return Ok(ResumeResult::Reaped(status));
    }

    let wait_error = io::Error::last_os_error();
    eprintln!(
        "M9_RECOVERY_PROBE_FATAL op=PTRACE_SYSCALL tid={tid} original_errno={:?} wait_result={waited} wait_status=0x{status:08x} wait_errno={:?}",
        original.raw_os_error(),
        wait_error.raw_os_error()
    );
    Err(original)
}

fn syscall_info(tid: libc::pid_t) -> io::Result<PtraceSyscallInfo> {
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
        let error = io::Error::last_os_error();
        eprintln!(
            "M9_RECOVERY_PROBE_FATAL op=PTRACE_GET_SYSCALL_INFO tid={tid} errno={:?} error={error}",
            error.raw_os_error()
        );
        return Err(error);
    }
    Ok(unsafe { info.assume_init() })
}

fn event_message(tid: libc::pid_t) -> io::Result<u64> {
    let mut value: libc::c_ulong = 0;
    ptrace_unit(
        "PTRACE_GETEVENTMSG",
        libc::PTRACE_GETEVENTMSG,
        tid,
        ptr::null_mut(),
        (&mut value as *mut libc::c_ulong).cast::<c_void>(),
    )?;
    Ok(value as u64)
}

fn record_terminal(
    tid: libc::pid_t,
    status: i32,
    root: libc::pid_t,
    tracees: &mut HashMap<libc::pid_t, bool>,
    root_status: &mut Option<i32>,
) {
    if tid == root {
        *root_status = Some(status);
    }
    tracees.remove(&tid);
}

fn main() -> io::Result<()> {
    let mut args = env::args_os().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|arg| arg == "--") {
        args.remove(0);
    }
    if args.is_empty() {
        eprintln!("usage: ptrace-esrch-recovery-probe -- PROGRAM [ARG ...]");
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
        eprintln!("M9_RECOVERY_PROBE_FATAL initial_stop status=0x{initial_status:08x}");
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
    )?;

    let mut tracees = HashMap::from([(child, false)]);
    let mut root_status = None;
    let mut reaped_on_esrch = 0_u64;
    let mut waits = 0_u64;
    let mut syscall_stops = 0_u64;
    let mut events = 0_u64;
    let mut last_syscall_nr: HashMap<libc::pid_t, u64> = HashMap::new();

    match resume(child, 0)? {
        ResumeResult::Resumed => {}
        ResumeResult::Reaped(status) => {
            reaped_on_esrch += 1;
            record_terminal(child, status, child, &mut tracees, &mut root_status);
        }
    }

    while !tracees.is_empty() {
        let mut status = 0;
        let tid = unsafe { libc::waitpid(-1, &mut status, libc::__WALL) };
        if tid < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            return Err(error);
        }
        waits += 1;

        if libc::WIFEXITED(status) || libc::WIFSIGNALED(status) {
            record_terminal(tid, status, child, &mut tracees, &mut root_status);
            last_syscall_nr.remove(&tid);
            continue;
        }
        if !libc::WIFSTOPPED(status) {
            continue;
        }

        let signal = libc::WSTOPSIG(status);
        let event = ((status as u32) >> 16) as libc::c_int;

        if signal == libc::SIGTRAP && event != 0 {
            events += 1;
            if matches!(
                event,
                libc::PTRACE_EVENT_FORK | libc::PTRACE_EVENT_VFORK | libc::PTRACE_EVENT_CLONE
            ) {
                let new_tid = event_message(tid)? as libc::pid_t;
                tracees.entry(new_tid).or_insert(true);
            }
            match resume(tid, 0)? {
                ResumeResult::Resumed => {}
                ResumeResult::Reaped(terminal) => {
                    reaped_on_esrch += 1;
                    record_terminal(tid, terminal, child, &mut tracees, &mut root_status);
                    last_syscall_nr.remove(&tid);
                }
            }
            continue;
        }

        if signal == (libc::SIGTRAP | 0x80) {
            syscall_stops += 1;
            let info = syscall_info(tid)?;
            match info.op {
                PTRACE_SYSCALL_INFO_ENTRY => {
                    last_syscall_nr.insert(tid, info.data[0]);
                }
                PTRACE_SYSCALL_INFO_EXIT => {}
                _ => {}
            }
            match resume(tid, 0)? {
                ResumeResult::Resumed => {}
                ResumeResult::Reaped(terminal) => {
                    let nr = last_syscall_nr.get(&tid).copied();
                    eprintln!("M9_RECOVERY_PROBE_TERMINAL_AFTER_SYSCALL tid={tid} nr={nr:?}");
                    reaped_on_esrch += 1;
                    record_terminal(tid, terminal, child, &mut tracees, &mut root_status);
                    last_syscall_nr.remove(&tid);
                }
            }
            continue;
        }

        let newborn = tracees.get_mut(&tid).is_some_and(|flag| {
            let value = *flag;
            *flag = false;
            value
        });
        let forwarded = if newborn || signal == libc::SIGTRAP { 0 } else { signal };
        match resume(tid, forwarded)? {
            ResumeResult::Resumed => {}
            ResumeResult::Reaped(terminal) => {
                reaped_on_esrch += 1;
                record_terminal(tid, terminal, child, &mut tracees, &mut root_status);
                last_syscall_nr.remove(&tid);
            }
        }
    }

    let Some(root_status) = root_status else {
        eprintln!("M9_RECOVERY_PROBE_FATAL root_terminal_status_missing");
        std::process::exit(66);
    };
    if !libc::WIFEXITED(root_status) || libc::WEXITSTATUS(root_status) != 0 {
        eprintln!("M9_RECOVERY_PROBE_FATAL root_status=0x{root_status:08x}");
        std::process::exit(67);
    }

    eprintln!(
        "M9_RECOVERY_PROBE_COMPLETE root={child} root_exit=0 waits={waits} syscall_stops={syscall_stops} ptrace_events={events} terminal_esrch_reaps={reaped_on_esrch}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_status_accepts_normal_exit() {
        assert!(terminal_status(0));
        assert!(terminal_status(7 << 8));
    }

    #[test]
    fn terminal_status_accepts_signal_exit() {
        assert!(terminal_status(libc::SIGTERM));
    }

    #[test]
    fn terminal_status_rejects_syscall_stop() {
        assert!(!terminal_status(0x0000_857f));
    }
}
