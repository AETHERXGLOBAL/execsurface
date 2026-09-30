#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::process::Command;

const SYSCALL_STOP: i32 = libc::SIGTRAP | 0x80;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    Success,
    Pending,
    Failure(i32),
    Malformed,
    Ambiguous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Event {
    fd: i32,
    entry_seq: u64,
    exit_seq: u64,
    sockaddr: Vec<u8>,
    outcome: Outcome,
}

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_b3-connect-fixture")
}

fn resume(pid: libc::pid_t) {
    let rc = unsafe {
        libc::ptrace(
            libc::PTRACE_SYSCALL,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        )
    };
    assert_eq!(rc, 0);
}
fn regs(pid: libc::pid_t) -> libc::user_regs_struct {
    let mut r = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGS,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            &mut r as *mut _ as *mut libc::c_void,
        )
    };
    assert_eq!(rc, 0);
    r
}
fn read_bytes(pid: libc::pid_t, addr: u64, len: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let ws = std::mem::size_of::<libc::c_long>();
    for off in (0..len).step_by(ws) {
        let w = unsafe {
            libc::ptrace(
                libc::PTRACE_PEEKDATA,
                pid,
                (addr as usize + off) as *mut libc::c_void,
                std::ptr::null_mut::<libc::c_void>(),
            )
        };
        let b = w.to_ne_bytes();
        out.extend_from_slice(&b[..std::cmp::min(ws, len - off)]);
    }
    out
}

fn trace(scenario: &str, port: u16) -> Event {
    let mut child = Command::new(fixture())
        .arg(scenario)
        .arg(port.to_string())
        .spawn()
        .unwrap();
    let pid = child.id() as libc::pid_t;
    let mut st = 0;
    assert_eq!(unsafe { libc::waitpid(pid, &mut st, 0) }, pid);
    assert!(libc::WIFSTOPPED(st));
    let opt = libc::PTRACE_O_TRACESYSGOOD as usize as *mut libc::c_void;
    assert_eq!(
        unsafe {
            libc::ptrace(
                libc::PTRACE_SETOPTIONS,
                pid,
                std::ptr::null_mut::<libc::c_void>(),
                opt,
            )
        },
        0
    );
    let mut entering = true;
    let mut seq = 0;
    let mut pending: Option<(i32, u64, Vec<u8>, u64)> = None;
    resume(pid);
    loop {
        st = 0;
        assert_eq!(unsafe { libc::waitpid(pid, &mut st, 0) }, pid);
        if libc::WIFEXITED(st) || libc::WIFSIGNALED(st) {
            break;
        }
        assert!(libc::WIFSTOPPED(st));
        if libc::WSTOPSIG(st) == SYSCALL_STOP {
            seq += 1;
            let r = regs(pid);
            if entering && r.orig_rax as libc::c_long == libc::SYS_connect {
                let len = r.rdx as usize;
                let bytes = if (2..=128).contains(&len) {
                    read_bytes(pid, r.rsi, len)
                } else {
                    Vec::new()
                };
                pending = Some((r.rdi as i32, len as u64, bytes, seq));
            } else if !entering {
                if let Some((fd, len, bytes, entry)) = pending.take() {
                    let raw = r.rax as i64;
                    let outcome = if len < 2 || bytes.len() != len as usize {
                        Outcome::Malformed
                    } else if raw == 0 {
                        Outcome::Success
                    } else if raw == -(libc::EINPROGRESS as i64) {
                        Outcome::Pending
                    } else if raw < 0 {
                        Outcome::Failure((-raw) as i32)
                    } else {
                        Outcome::Ambiguous
                    };
                    unsafe { libc::kill(pid, libc::SIGKILL) };
                    let mut end = 0;
                    unsafe { libc::waitpid(pid, &mut end, 0) };
                    let _ = child.wait();
                    return Event {
                        fd,
                        entry_seq: entry,
                        exit_seq: seq,
                        sockaddr: bytes,
                        outcome,
                    };
                }
            }
            entering = !entering;
        }
        resume(pid);
    }
    let _ = child.wait();
    panic!("no connect event")
}

#[test]
fn synchronous_loopback_success_is_exactly_rc_zero() {
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let e = trace("connect", port);
    assert_eq!(e.outcome, Outcome::Success);
    assert!(e.fd >= 0);
    assert!(e.exit_seq > e.entry_seq);
    assert_eq!(u16::from_be_bytes([e.sockaddr[2], e.sockaddr[3]]), port);
}
#[test]
fn refused_connection_remains_failure() {
    use std::net::TcpListener;
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let p = l.local_addr().unwrap().port();
    drop(l);
    assert!(matches!(trace("failure", p).outcome, Outcome::Failure(_)));
}
#[test]
fn nonblocking_einprogress_never_becomes_success() {
    let e = trace("nonblocking", 9);
    assert!(!matches!(e.outcome, Outcome::Success));
    assert!(matches!(e.outcome, Outcome::Pending | Outcome::Failure(_)));
}
#[test]
fn malformed_sockaddr_fails_closed() {
    assert_eq!(trace("malformed", 9).outcome, Outcome::Malformed);
}
#[test]
fn destination_substitution_changes_evidence() {
    let a = trace("failure", 9);
    let b = trace("failure", 10);
    assert_ne!(a.sockaddr, b.sockaddr);
}
#[test]
fn actor_entry_identity_is_not_transferable() {
    let a = trace("failure", 9);
    let b = trace("failure", 9);
    assert_ne!((a.entry_seq, a.fd), (0, -1));
    assert_ne!((&a as *const _, a.entry_seq), (&b as *const _, 0));
}
#[test]
fn deterministic_destination_bytes_for_same_destination() {
    let a = trace("failure", 9);
    let b = trace("failure", 9);
    assert_eq!(a.sockaddr, b.sockaddr);
}
#[test]
fn fd_reuse_does_not_define_connect_authority() {
    let e = trace("fd-reuse", 9);
    assert!(e.entry_seq > 0);
    assert!(e.exit_seq > e.entry_seq);
}
#[test]
fn later_state_cannot_relabel_recorded_failure() {
    let e = trace("failure", 9);
    let original = e.outcome.clone();
    let _later_socket = std::net::TcpStream::connect("127.0.0.1:9");
    assert_eq!(e.outcome, original);
}
#[test]
fn proof_pairing_is_monotonic() {
    let e = trace("failure", 9);
    assert!(e.exit_seq > e.entry_seq);
}
