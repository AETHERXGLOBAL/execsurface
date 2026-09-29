#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::{c_void, CString};
use std::io;
use std::mem::{size_of, MaybeUninit};
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

    fn is_exit(&self) -> bool {
        self.op == PTRACE_SYSCALL_INFO_EXIT
    }
}

#[derive(Debug, Clone, Copy)]
enum PendingCreation {
    Fork,
    Vfork,
    Clone { flags: Option<u64>, clone3: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relation {
    Shared,
    IndependentCopy,
    Unknown,
}

impl Relation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::IndependentCopy => "independent_copy",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Default)]
struct Summary {
    transitions: usize,
    clone_origin: usize,
    shared: usize,
    independent: usize,
    unknown: usize,
    clone3_read_failures: usize,
    unpaired: usize,
    origin_event_mismatch: usize,
    root_exit: Option<i32>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("census_error={error}");
        std::process::exit(70);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let work_root = env::args()
        .nth(1)
        .ok_or("usage: execsurface-p3-v5c-clone-census <work-root>")?;
    let command = format!("cd '{}' && go test ./... >/dev/null 2>&1", work_root);

    let root = spawn_traced_bash(&command)?;
    let mut status = 0;
    if unsafe { libc::waitpid(root, &mut status, 0) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if !libc::WIFSTOPPED(status) {
        return Err("tracee did not enter initial stop".into());
    }

    let options = libc::PTRACE_O_TRACESYSGOOD
        | libc::PTRACE_O_TRACEFORK
        | libc::PTRACE_O_TRACEVFORK
        | libc::PTRACE_O_TRACECLONE
        | libc::PTRACE_O_TRACEEXEC
        | libc::PTRACE_O_TRACEEXIT
        | libc::PTRACE_O_EXITKILL;
    ptrace_call(
        libc::PTRACE_SETOPTIONS,
        root,
        ptr::null_mut(),
        options as usize as *mut c_void,
    )?;

    let mut tracees = HashSet::from([root]);
    let mut pending: HashMap<libc::pid_t, PendingCreation> = HashMap::new();
    let mut summary = Summary::default();

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
                summary.root_exit = Some(libc::WEXITSTATUS(wait_status));
            }
            tracees.remove(&tid);
            pending.remove(&tid);
            continue;
        }
        if libc::WIFSIGNALED(wait_status) {
            if tid == root {
                summary.root_exit = Some(128 + libc::WTERMSIG(wait_status));
            }
            tracees.remove(&tid);
            pending.remove(&tid);
            continue;
        }
        if !libc::WIFSTOPPED(wait_status) {
            continue;
        }

        let signal = libc::WSTOPSIG(wait_status);
        let event = (wait_status >> 16) as libc::c_int;

        if signal == (libc::SIGTRAP | 0x80) {
            let info = syscall_info(tid)?;
            if let Some((nr, args)) = info.entry() {
                if nr == libc::SYS_fork as u64 {
                    pending.insert(tid, PendingCreation::Fork);
                } else if nr == libc::SYS_vfork as u64 {
                    pending.insert(tid, PendingCreation::Vfork);
                } else if nr == libc::SYS_clone as u64 {
                    pending.insert(
                        tid,
                        PendingCreation::Clone {
                            flags: Some(args[0]),
                            clone3: false,
                        },
                    );
                } else if nr == libc::SYS_clone3 as u64 {
                    let flags = match read_clone3_flags(tid, args[0], args[1]) {
                        Ok(flags) => Some(flags),
                        Err(_) => {
                            summary.clone3_read_failures += 1;
                            None
                        }
                    };
                    pending.insert(
                        tid,
                        PendingCreation::Clone {
                            flags,
                            clone3: true,
                        },
                    );
                }
            } else if info.is_exit() {
                // A successful child-creation syscall is consumed by its ptrace
                // creation event before this syscall-exit stop. Anything left
                // here did not create a child and must not leak into a later event.
                pending.remove(&tid);
            }
            resume_syscall(tid, 0)?;
            continue;
        }

        if signal == libc::SIGTRAP && is_creation_event(event) {
            let child = get_event_message(tid)? as libc::pid_t;
            tracees.insert(child);
            summary.transitions += 1;

            let origin = pending.remove(&tid);
            let (origin_name, flags, relation) = match origin {
                Some(PendingCreation::Fork) => {
                    if event != libc::PTRACE_EVENT_FORK {
                        summary.origin_event_mismatch += 1;
                    }
                    ("fork", None, Relation::IndependentCopy)
                }
                Some(PendingCreation::Vfork) => {
                    if event != libc::PTRACE_EVENT_VFORK {
                        summary.origin_event_mismatch += 1;
                    }
                    ("vfork", None, Relation::IndependentCopy)
                }
                Some(PendingCreation::Clone { flags, clone3 }) => {
                    summary.clone_origin += 1;
                    let relation = match flags {
                        Some(flags) if flags & libc::CLONE_FILES as u64 != 0 => Relation::Shared,
                        Some(_) => Relation::IndependentCopy,
                        None => Relation::Unknown,
                    };
                    (if clone3 { "clone3" } else { "clone" }, flags, relation)
                }
                None => {
                    summary.unpaired += 1;
                    ("unpaired", None, Relation::Unknown)
                }
            };

            match relation {
                Relation::Shared => summary.shared += 1,
                Relation::IndependentCopy => summary.independent += 1,
                Relation::Unknown => summary.unknown += 1,
            }

            println!(
                "transition\tparent={}\tchild={}\tevent={}\torigin={}\tflags={}\trelation={}",
                tid,
                child,
                event_name(event),
                origin_name,
                flags
                    .map(|value| format!("0x{value:016x}"))
                    .unwrap_or_else(|| "-".to_owned()),
                relation.as_str()
            );

            resume_syscall(tid, 0)?;
            continue;
        }

        if signal == libc::SIGTRAP && event == libc::PTRACE_EVENT_EXEC {
            let former_tid = get_event_message(tid)? as libc::pid_t;
            if former_tid != 0 && former_tid != tid {
                tracees.remove(&former_tid);
                pending.remove(&former_tid);
                tracees.insert(tid);
            }
            resume_syscall(tid, 0)?;
            continue;
        }

        if signal == libc::SIGTRAP && event != 0 {
            resume_syscall(tid, 0)?;
            continue;
        }

        // Suppress the synthetic SIGSTOP used for automatically attached
        // children. Forward other real signal-delivery stops.
        let deliver = if signal == libc::SIGSTOP || signal == libc::SIGTRAP {
            0
        } else {
            signal
        };
        resume_syscall(tid, deliver)?;
    }

    println!("summary\ttransitions={}", summary.transitions);
    println!("summary\tclone_origin={}", summary.clone_origin);
    println!("summary\tshared={}", summary.shared);
    println!("summary\tindependent_copy={}", summary.independent);
    println!("summary\tunknown={}", summary.unknown);
    println!(
        "summary\tclone3_read_failures={}",
        summary.clone3_read_failures
    );
    println!("summary\tunpaired={}", summary.unpaired);
    println!(
        "summary\torigin_event_mismatch={}",
        summary.origin_event_mismatch
    );
    println!(
        "summary\troot_exit={}",
        summary
            .root_exit
            .map(|code| code.to_string())
            .unwrap_or_else(|| "missing".to_owned())
    );

    let admissible = summary.root_exit == Some(0)
        && summary.unknown == 0
        && summary.clone3_read_failures == 0
        && summary.unpaired == 0
        && summary.origin_event_mismatch == 0;
    println!(
        "summary\tclassification={}",
        if admissible {
            "C1_RELATION_OBSERVABLE"
        } else {
            "C1_RELATION_PARTIAL_OR_INCOMPLETE"
        }
    );

    Ok(())
}

fn spawn_traced_bash(command: &str) -> Result<libc::pid_t, Box<dyn std::error::Error>> {
    let program = CString::new("/bin/bash")?;
    let arg0 = CString::new("/bin/bash")?;
    let arg1 = CString::new("-lc")?;
    let arg2 = CString::new(command)?;
    let argv = [arg0.as_ptr(), arg1.as_ptr(), arg2.as_ptr(), ptr::null()];

    let child = unsafe { libc::fork() };
    if child < 0 {
        return Err(io::Error::last_os_error().into());
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
    Ok(child)
}

fn is_creation_event(event: libc::c_int) -> bool {
    matches!(
        event,
        libc::PTRACE_EVENT_FORK | libc::PTRACE_EVENT_VFORK | libc::PTRACE_EVENT_CLONE
    )
}

fn event_name(event: libc::c_int) -> &'static str {
    match event {
        libc::PTRACE_EVENT_FORK => "fork_event",
        libc::PTRACE_EVENT_VFORK => "vfork_event",
        libc::PTRACE_EVENT_CLONE => "clone_event",
        _ => "other_event",
    }
}

fn read_clone3_flags(tid: libc::pid_t, address: u64, size: u64) -> io::Result<u64> {
    if address == 0 || size < 8 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "clone3 flags field unavailable",
        ));
    }
    peek_word(tid, address).map(|word| word as u64)
}

fn peek_word(tid: libc::pid_t, address: u64) -> io::Result<libc::c_long> {
    unsafe {
        *libc::__errno_location() = 0;
    }
    let value = unsafe {
        libc::ptrace(
            libc::PTRACE_PEEKDATA,
            tid,
            address as usize as *mut c_void,
            ptr::null_mut::<c_void>(),
        )
    };
    let errno = unsafe { *libc::__errno_location() };
    if value == -1 && errno != 0 {
        Err(io::Error::from_raw_os_error(errno))
    } else {
        Ok(value)
    }
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
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { info.assume_init() })
}

fn get_event_message(tid: libc::pid_t) -> io::Result<libc::c_ulong> {
    let mut message: libc::c_ulong = 0;
    let result = unsafe {
        libc::ptrace(
            libc::PTRACE_GETEVENTMSG,
            tid,
            ptr::null_mut::<c_void>(),
            &mut message as *mut _ as *mut c_void,
        )
    };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(message)
    }
}

fn resume_syscall(tid: libc::pid_t, signal: libc::c_int) -> io::Result<()> {
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
) -> io::Result<()> {
    let result = unsafe { libc::ptrace(request, tid, address, data) };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
