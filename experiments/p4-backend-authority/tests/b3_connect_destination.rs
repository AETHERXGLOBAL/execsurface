pub use execsurface_p4_backend_authority::b0_success_evidence;
#[path = "../src/b3_connect_destination.rs"]
mod b3_connect_destination;

use std::collections::BTreeSet;

use b3_connect_destination::{ConnectAuthority, ConnectContext, ConnectDestination, ConnectRecord};
use b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, TargetProposition,
};

fn actor(tid: i32) -> ActorIdentity {
    ActorIdentity {
        tid,
        process_identity: format!("pid:{tid}:start:1"),
        causal_chain_digest: format!("sha256:{}", "a".repeat(64)),
    }
}

fn context(fd: i32, port: u16) -> ConnectContext {
    ConnectContext {
        socket_fd: fd,
        destination: ConnectDestination::Inet4 { address: [127, 0, 0, 1], port },
        sockaddr_len: 16,
        sockaddr_complete: true,
    }
}

fn attempt(ctx: &ConnectContext, sequence: u64, tid: i32) -> AttemptEvidence {
    AttemptEvidence {
        proposition: TargetProposition::NetworkConnectDestination,
        operation: OperationKind::Connect,
        actor: actor(tid),
        entry_sequence: sequence,
        argument_digest: ctx.digest().unwrap(),
        target_identity: ctx.target_identity().unwrap(),
    }
}

fn exit(entry: &AttemptEvidence, sequence: u64, rc: i64) -> ExitEvidence {
    ExitEvidence {
        actor: entry.actor.clone(),
        originating_entry_sequence: entry.entry_sequence,
        exit_sequence: sequence,
        raw_return: rc,
    }
}

fn classify(ctx: &ConnectContext, entry_seq: u64, rc: i64) -> ConnectRecord {
    let entry = attempt(ctx, entry_seq, 101);
    let result = EvidenceLedger::default()
        .classify_pair(entry.clone(), exit(&entry, entry_seq + 1, rc), ObservationHealth::healthy())
        .unwrap();
    ConnectRecord::build(result, ctx.clone()).unwrap()
}

#[test]
fn b3_sync_zero_is_bounded_success() {
    assert!(classify(&context(3, 8080), 1, 0).is_success_authority());
}

#[test]
fn b3_einprogress_is_pending_not_success() {
    let record = classify(&context(3, 8080), 10, -115);
    assert!(matches!(record.authority, ConnectAuthority::Pending { errno: 115 }));
    assert!(!record.is_success_authority());
}

#[test]
fn b3_negative_errno_is_failure() {
    let record = classify(&context(3, 8080), 20, -111);
    assert!(matches!(record.authority, ConnectAuthority::FailureObserved { errno: 111 }));
}

#[test]
fn b3_entry_only_is_attempt_only() {
    let ctx = context(3, 8080);
    let evidence = EvidenceLedger::default()
        .attempt_only(attempt(&ctx, 30, 101), ObservationHealth::healthy())
        .unwrap();
    let record = ConnectRecord::build(evidence, ctx).unwrap();
    assert!(matches!(record.authority, ConnectAuthority::AttemptOnly));
}

#[test]
fn b3_actor_substitution_cannot_be_success() {
    let ctx = context(3, 8080);
    let entry = attempt(&ctx, 40, 101);
    let mut wrong = exit(&entry, 41, 0);
    wrong.actor = actor(202);
    let evidence = EvidenceLedger::default()
        .classify_pair(entry, wrong, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(evidence.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b3_entry_sequence_substitution_cannot_be_success() {
    let ctx = context(3, 8080);
    let entry = attempt(&ctx, 50, 101);
    let mut wrong = exit(&entry, 52, 0);
    wrong.originating_entry_sequence = 49;
    let evidence = EvidenceLedger::default()
        .classify_pair(entry, wrong, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(evidence.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b3_destination_substitution_is_rejected() {
    let original = context(3, 8080);
    let changed = context(3, 8081);
    let entry = attempt(&original, 60, 101);
    let evidence = EvidenceLedger::default()
        .classify_pair(entry.clone(), exit(&entry, 61, 0), ObservationHealth::healthy())
        .unwrap();
    assert!(ConnectRecord::build(evidence, changed).is_err());
}

#[test]
fn b3_fd_substitution_is_rejected() {
    let original = context(3, 8080);
    let changed = context(4, 8080);
    let entry = attempt(&original, 70, 101);
    let evidence = EvidenceLedger::default()
        .classify_pair(entry.clone(), exit(&entry, 71, 0), ObservationHealth::healthy())
        .unwrap();
    assert!(ConnectRecord::build(evidence, changed).is_err());
}

#[test]
fn b3_replay_is_rejected() {
    let ctx = context(3, 8080);
    let entry = attempt(&ctx, 80, 101);
    let result = exit(&entry, 81, 0);
    let mut ledger = EvidenceLedger::default();
    assert!(ledger.classify_pair(entry.clone(), result.clone(), ObservationHealth::healthy()).is_ok());
    assert!(ledger.classify_pair(entry, result, ObservationHealth::healthy()).is_err());
}

#[test]
fn b3_non_monotonic_exit_is_ambiguous() {
    let ctx = context(3, 8080);
    let entry = attempt(&ctx, 90, 101);
    let evidence = EvidenceLedger::default()
        .classify_pair(entry.clone(), exit(&entry, 90, 0), ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(evidence.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b3_incomplete_sockaddr_is_rejected() {
    let mut ctx = context(3, 8080);
    ctx.sockaddr_complete = false;
    assert!(ctx.validate().is_err());
}

#[test]
fn b3_empty_unix_sockaddr_is_rejected() {
    let ctx = ConnectContext {
        socket_fd: 3,
        destination: ConnectDestination::Unix { path_bytes: vec![], abstract_namespace: false },
        sockaddr_len: 2,
        sockaddr_complete: true,
    };
    assert!(ctx.validate().is_err());
}

#[test]
fn b3_observer_loss_remains_lost() {
    let ctx = context(3, 8080);
    let entry = attempt(&ctx, 100, 101);
    let health = ObservationHealth {
        complete: false,
        warning_codes: BTreeSet::from(["controlled_loss".to_owned()]),
    };
    let evidence = EvidenceLedger::default()
        .classify_pair(entry.clone(), exit(&entry, 101, 0), health)
        .unwrap();
    let record = ConnectRecord::build(evidence, ctx).unwrap();
    assert!(matches!(record.authority, ConnectAuthority::Lost { .. }));
}

#[test]
fn b3_unexpected_positive_return_is_ambiguous() {
    let record = classify(&context(3, 8080), 110, 1);
    assert!(matches!(record.authority, ConnectAuthority::Ambiguous { .. }));
}

#[test]
fn b3_file_proposition_cannot_be_laundered_into_connect() {
    let ctx = context(3, 8080);
    let mut entry = attempt(&ctx, 120, 101);
    entry.proposition = TargetProposition::FileOpenObject;
    assert!(entry.validate().is_err());
}

#[test]
fn b3_same_evidence_is_deterministic() {
    let first = classify(&context(3, 8080), 130, 0);
    let second = classify(&context(3, 8080), 130, 0);
    assert_eq!(serde_json::to_vec(&first).unwrap(), serde_json::to_vec(&second).unwrap());
}

#[test]
fn b3_fd_reuse_requires_new_pairing_identity() {
    let ctx = context(3, 8080);
    let first = attempt(&ctx, 140, 101);
    let second = attempt(&ctx, 142, 101);
    assert_ne!(first.pairing_identity().unwrap(), second.pairing_identity().unwrap());
}

#[test]
fn b3_same_fd_different_destination_changes_identity() {
    let first = context(3, 8080);
    let second = context(3, 8081);
    assert_ne!(first.target_identity().unwrap(), second.target_identity().unwrap());
    assert_ne!(first.digest().unwrap(), second.digest().unwrap());
}

#[test]
fn b3_ipv6_scope_is_part_of_identity() {
    let first = ConnectContext {
        socket_fd: 3,
        destination: ConnectDestination::Inet6 {
            address: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            port: 8080,
            flowinfo: 0,
            scope_id: 0,
        },
        sockaddr_len: 28,
        sockaddr_complete: true,
    };
    let mut second = first.clone();
    if let ConnectDestination::Inet6 { scope_id, .. } = &mut second.destination {
        *scope_id = 1;
    }
    assert_ne!(first.digest().unwrap(), second.digest().unwrap());
}

#[test]
fn b3_pending_never_reports_success() {
    let pending = classify(&context(3, 8080), 150, -115);
    assert!(!pending.is_success_authority());
}
