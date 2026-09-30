#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

pub use execsurface_p4_backend_authority::b0_success_evidence;
#[path = "../src/b3_connect_destination.rs"]
mod b3_connect_destination;

use std::collections::BTreeSet;
use std::process::Command;

use b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, SuccessEvidenceRecord, TargetProposition,
};
use b3_connect_destination::{ConnectAuthority, ConnectContext, ConnectDestination, ConnectRecord};
use sha2::{Digest, Sha256};

const SYSCALL_STOP: i32 = libc::SIGTRAP | 0x80;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Event {
    actor_tid: i32,
    fd: i32,
    entry_seq: u64,
    exit_seq: u64,
    sockaddr_len: u32,
    sockaddr: Vec<u8>,
    raw_return: i64,
}

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_b3-connect-fixture")
}

fn digest_text(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

fn retain_non_live_model_variants_for_target_compilation() {
    // The live ptrace corpus is intentionally bounded to AF_INET. Construct the
    // other frozen model variants here so per-target `-D dead-code` does not
    // mistake research-model reachability for live-backend authority.
    let _ = ConnectDestination::Inet6 {
        address: [0; 16],
        port: 1,
        flowinfo: 0,
        scope_id: 0,
    };
    let _ = ConnectDestination::Unix {
        path_bytes: b"x".to_vec(),
        abstract_namespace: false,
    };
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
    let mut pending: Option<(i32, u32, Vec<u8>, u64)> = None;
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
                let bytes = if (1..=128).contains(&len) {
                    read_bytes(pid, r.rsi, len)
                } else {
                    Vec::new()
                };
                pending = Some((r.rdi as i32, len as u32, bytes, seq));
            } else if !entering {
                if let Some((fd, len, bytes, entry)) = pending.take() {
                    let raw_return = r.rax as i64;
                    unsafe { libc::kill(pid, libc::SIGKILL) };
                    let mut end = 0;
                    unsafe { libc::waitpid(pid, &mut end, 0) };
                    let _ = child.wait();
                    return Event {
                        actor_tid: pid,
                        fd,
                        entry_seq: entry,
                        exit_seq: seq,
                        sockaddr_len: len,
                        sockaddr: bytes,
                        raw_return,
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

fn context_from_event(event: &Event) -> Result<ConnectContext, String> {
    if event.sockaddr_len != std::mem::size_of::<libc::sockaddr_in>() as u32
        || event.sockaddr.len() != std::mem::size_of::<libc::sockaddr_in>()
    {
        return Err("live sockaddr is truncated or outside the bounded AF_INET fixture".to_owned());
    }
    let family = u16::from_ne_bytes([event.sockaddr[0], event.sockaddr[1]]);
    if family != libc::AF_INET as u16 {
        return Err("live sockaddr family is outside the bounded AF_INET fixture".to_owned());
    }
    let context = ConnectContext {
        socket_fd: event.fd,
        destination: ConnectDestination::Inet4 {
            address: [
                event.sockaddr[4],
                event.sockaddr[5],
                event.sockaddr[6],
                event.sockaddr[7],
            ],
            port: u16::from_be_bytes([event.sockaddr[2], event.sockaddr[3]]),
        },
        sockaddr_len: event.sockaddr_len,
        sockaddr_complete: true,
    };
    context.validate()?;
    Ok(context)
}

fn actor_from_event(event: &Event) -> ActorIdentity {
    let process_identity = format!("ptrace-live-tid:{}", event.actor_tid);
    ActorIdentity {
        tid: event.actor_tid,
        causal_chain_digest: digest_text(&process_identity),
        process_identity,
    }
}

fn attempt_from_event(event: &Event, context: &ConnectContext) -> AttemptEvidence {
    AttemptEvidence {
        proposition: TargetProposition::NetworkConnectDestination,
        operation: OperationKind::Connect,
        actor: actor_from_event(event),
        entry_sequence: event.entry_seq,
        argument_digest: context.digest().unwrap(),
        target_identity: context.target_identity().unwrap(),
    }
}

fn exit_from_event(event: &Event) -> ExitEvidence {
    ExitEvidence {
        actor: actor_from_event(event),
        originating_entry_sequence: event.entry_seq,
        exit_sequence: event.exit_seq,
        raw_return: event.raw_return,
    }
}

fn evidence_from_event(
    event: &Event,
    context: &ConnectContext,
    health: ObservationHealth,
) -> SuccessEvidenceRecord {
    EvidenceLedger::default()
        .classify_pair(
            attempt_from_event(event, context),
            exit_from_event(event),
            health,
        )
        .unwrap()
}

fn record_from_event(event: &Event, health: ObservationHealth) -> ConnectRecord {
    let context = context_from_event(event).unwrap();
    let evidence = evidence_from_event(event, &context, health);
    ConnectRecord::build(evidence, context).unwrap()
}

#[test]
fn live_sync_success_binds_actor_entry_fd_destination_and_rc_zero() {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let event = trace("connect", port);
    let context = context_from_event(&event).unwrap();
    let record = record_from_event(&event, ObservationHealth::healthy());

    assert_eq!(event.raw_return, 0);
    assert!(event.fd >= 0);
    assert!(event.actor_tid > 0);
    assert!(event.exit_seq > event.entry_seq);
    assert_eq!(context.socket_fd, event.fd);
    assert_eq!(
        context.destination,
        ConnectDestination::Inet4 {
            address: [127, 0, 0, 1],
            port,
        }
    );
    assert!(record.is_success_authority());
}

#[test]
fn live_negative_connect_remains_explicit_failure() {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let event = trace("failure", port);
    let record = record_from_event(&event, ObservationHealth::healthy());

    assert!(event.raw_return < 0);
    assert!(matches!(
        record.authority,
        ConnectAuthority::FailureObserved { .. }
    ));
    assert!(!record.is_success_authority());
}

#[test]
fn live_nonblocking_connect_proves_einprogress_pending_not_success() {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let event = trace("nonblocking", port);
    let record = record_from_event(&event, ObservationHealth::healthy());

    assert_eq!(event.raw_return, -(libc::EINPROGRESS as i64));
    assert!(matches!(
        record.authority,
        ConnectAuthority::Pending { errno } if errno == libc::EINPROGRESS
    ));
    assert!(!record.is_success_authority());
}

#[test]
fn live_destination_substitution_cannot_preserve_authority() {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let event = trace("connect", port);
    let original = context_from_event(&event).unwrap();
    let evidence = evidence_from_event(&event, &original, ObservationHealth::healthy());
    let mut substituted = original.clone();
    if let ConnectDestination::Inet4 { port, .. } = &mut substituted.destination {
        *port = port.saturating_add(1);
    }

    assert_ne!(original.digest().unwrap(), substituted.digest().unwrap());
    assert!(ConnectRecord::build(evidence, substituted).is_err());
}

#[test]
fn live_fd_substitution_reuse_and_replay_cannot_transfer_authority() {
    let event = trace("fd-reuse", 9);
    let original = context_from_event(&event).unwrap();
    let entry = attempt_from_event(&event, &original);
    let exit = exit_from_event(&event);

    let evidence = EvidenceLedger::default()
        .classify_pair(entry.clone(), exit.clone(), ObservationHealth::healthy())
        .unwrap();
    let mut substituted = original.clone();
    substituted.socket_fd += 1;
    assert!(ConnectRecord::build(evidence, substituted).is_err());

    let mut reused = entry.clone();
    reused.entry_sequence += 2;
    assert_ne!(
        entry.pairing_identity().unwrap(),
        reused.pairing_identity().unwrap()
    );

    let mut ledger = EvidenceLedger::default();
    assert!(ledger
        .classify_pair(entry.clone(), exit.clone(), ObservationHealth::healthy())
        .is_ok());
    assert!(ledger
        .classify_pair(entry, exit, ObservationHealth::healthy())
        .is_err());
}

#[test]
fn live_actor_and_entry_substitution_fail_closed() {
    let event = trace("failure", 9);
    let context = context_from_event(&event).unwrap();
    let entry = attempt_from_event(&event, &context);

    let mut wrong_actor = exit_from_event(&event);
    wrong_actor.actor.tid += 1;
    wrong_actor.actor.process_identity = format!("ptrace-live-tid:{}", wrong_actor.actor.tid);
    wrong_actor.actor.causal_chain_digest = digest_text(&wrong_actor.actor.process_identity);
    let actor_result = EvidenceLedger::default()
        .classify_pair(entry.clone(), wrong_actor, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(
        actor_result.state,
        EvidenceState::Ambiguous { .. }
    ));
    assert!(!actor_result.state.is_success());

    let mut wrong_entry = exit_from_event(&event);
    wrong_entry.originating_entry_sequence += 1;
    let entry_result = EvidenceLedger::default()
        .classify_pair(entry, wrong_entry, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(
        entry_result.state,
        EvidenceState::Ambiguous { .. }
    ));
    assert!(!entry_result.state.is_success());
}

#[test]
fn live_malformed_sockaddr_fails_closed() {
    let event = trace("malformed", 9);

    assert!(event.raw_return < 0);
    assert!(context_from_event(&event).is_err());
}

#[test]
fn live_observer_loss_blocks_success_authority() {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let event = trace("connect", port);
    let health = ObservationHealth {
        complete: false,
        warning_codes: BTreeSet::from(["controlled_live_loss".to_owned()]),
    };
    let record = record_from_event(&event, health);

    assert!(matches!(record.authority, ConnectAuthority::Lost { .. }));
    assert!(!record.is_success_authority());
}

#[test]
fn live_identical_evidence_has_deterministic_proof_identity() {
    use std::net::TcpListener;

    retain_non_live_model_variants_for_target_compilation();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let event = trace("connect", port);
    let first = record_from_event(&event, ObservationHealth::healthy());
    let second = record_from_event(&event, ObservationHealth::healthy());

    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
    let first_digest = match &first.authority {
        ConnectAuthority::SynchronousSuccessBounded { proof_digest } => proof_digest,
        other => panic!("expected bounded success, got {other:?}"),
    };
    let second_digest = match &second.authority {
        ConnectAuthority::SynchronousSuccessBounded { proof_digest } => proof_digest,
        other => panic!("expected bounded success, got {other:?}"),
    };
    assert_eq!(first_digest, second_digest);
    assert!(first_digest.starts_with("sha256:"));
    assert_eq!(first_digest.len(), 71);
}

#[test]
fn live_later_socket_state_cannot_relabel_recorded_failure() {
    use std::net::{TcpListener, TcpStream};

    let closed = TcpListener::bind("127.0.0.1:0").unwrap();
    let closed_port = closed.local_addr().unwrap().port();
    drop(closed);
    let event = trace("failure", closed_port);
    let record = record_from_event(&event, ObservationHealth::healthy());
    let original = record.authority.clone();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let later = TcpStream::connect(listener.local_addr().unwrap());
    assert!(later.is_ok());

    assert_eq!(record.authority, original);
    assert!(matches!(
        record.authority,
        ConnectAuthority::FailureObserved { .. }
    ));
    assert!(!record.is_success_authority());
}
