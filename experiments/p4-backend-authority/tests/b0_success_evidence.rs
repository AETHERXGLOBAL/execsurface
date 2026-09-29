#[path = "../src/b0_success_evidence.rs"]
mod b0_success_evidence;

use std::collections::BTreeSet;

use b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, SuccessEvidenceRecord, TargetProposition,
};

fn digest(ch: char) -> String {
    format!("sha256:{}", ch.to_string().repeat(64))
}

fn actor(tid: i32, name: &str, chain: char) -> ActorIdentity {
    ActorIdentity {
        tid,
        process_identity: name.to_owned(),
        causal_chain_digest: digest(chain),
    }
}

fn attempt(
    proposition: TargetProposition,
    operation: OperationKind,
    tid: i32,
    sequence: u64,
    target: &str,
) -> AttemptEvidence {
    AttemptEvidence {
        proposition,
        operation,
        actor: actor(tid, "/usr/bin/fixture", 'c'),
        entry_sequence: sequence,
        argument_digest: digest('a'),
        target_identity: target.to_owned(),
    }
}

fn exit_for(attempt: &AttemptEvidence, sequence: u64, raw_return: i64) -> ExitEvidence {
    ExitEvidence {
        actor: attempt.actor.clone(),
        originating_entry_sequence: attempt.entry_sequence,
        exit_sequence: sequence,
        raw_return,
    }
}

#[test]
fn b0_operation_must_match_target_proposition() {
    let bad = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Connect,
        11,
        1,
        "127.0.0.1:443",
    );
    assert!(bad.validate().is_err());

    for (operation, sequence, target) in [
        (OperationKind::RenameAt, 2, "$WORKSPACE/a->$WORKSPACE/b"),
        (OperationKind::Unlink, 3, "$WORKSPACE/file"),
        (OperationKind::Rmdir, 4, "$WORKSPACE/dir"),
    ] {
        assert!(attempt(
            TargetProposition::FileRenameDelete,
            operation,
            11,
            sequence,
            target,
        )
        .validate()
        .is_ok());
    }
}

#[test]
fn b0_entry_only_evidence_is_attempt_not_success() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::OpenAt,
        12,
        10,
        "$WORKSPACE/input.txt",
    );
    let record = EvidenceLedger::default()
        .attempt_only(attempt, ObservationHealth::healthy())
        .expect("attempt record");
    assert!(matches!(record.state, EvidenceState::AttemptObserved));
    assert!(!record.state.is_success());
}

#[test]
fn b0_successful_open_requires_paired_nonnegative_returned_fd() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Open,
        13,
        20,
        "$WORKSPACE/input.txt",
    );
    let exit = exit_for(&attempt, 21, 7);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("open success");
    assert!(matches!(
        record.state,
        EvidenceState::SuccessObserved {
            raw_return: 7,
            returned_fd: Some(7)
        }
    ));
}

#[test]
fn b0_failed_open_is_failure_never_success() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::OpenAt,
        14,
        30,
        "$WORKSPACE/missing.txt",
    );
    let exit = exit_for(&attempt, 31, -2);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("open failure");
    assert!(matches!(
        record.state,
        EvidenceState::FailureObserved { errno: 2 }
    ));
    assert!(!record.state.is_success());
}

#[test]
fn b0_rename_rc_zero_is_success() {
    let attempt = attempt(
        TargetProposition::FileRenameDelete,
        OperationKind::RenameAt2,
        15,
        40,
        "$WORKSPACE/a->$WORKSPACE/b",
    );
    let exit = exit_for(&attempt, 41, 0);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("rename success");
    assert!(record.state.is_success());
}

#[test]
fn b0_unexpected_positive_rename_return_is_ambiguous() {
    let attempt = attempt(
        TargetProposition::FileRenameDelete,
        OperationKind::Rename,
        16,
        50,
        "$WORKSPACE/a->$WORKSPACE/b",
    );
    let exit = exit_for(&attempt, 51, 1);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("ambiguous positive rename return");
    assert!(matches!(record.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b0_immediate_connect_rc_zero_is_success() {
    let attempt = attempt(
        TargetProposition::NetworkConnectDestination,
        OperationKind::Connect,
        17,
        60,
        "127.0.0.1:443",
    );
    let exit = exit_for(&attempt, 61, 0);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("connect success");
    assert!(record.state.is_success());
}

#[test]
fn b0_einprogress_is_pending_never_success() {
    let attempt = attempt(
        TargetProposition::NetworkConnectDestination,
        OperationKind::Connect,
        18,
        70,
        "203.0.113.1:443",
    );
    let exit = exit_for(&attempt, 71, -115);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("pending connect");
    assert!(matches!(
        record.state,
        EvidenceState::PendingObserved { errno: 115 }
    ));
    assert!(!record.state.is_success());
}

#[test]
fn b0_connect_failure_remains_failure() {
    let attempt = attempt(
        TargetProposition::NetworkConnectDestination,
        OperationKind::Connect,
        19,
        80,
        "127.0.0.1:1",
    );
    let exit = exit_for(&attempt, 81, -111);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("connect failure");
    assert!(matches!(
        record.state,
        EvidenceState::FailureObserved { errno: 111 }
    ));
}

#[test]
fn b0_wrong_actor_cannot_acquire_success_authority() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::OpenAt2,
        20,
        90,
        "$WORKSPACE/input.txt",
    );
    let mut exit = exit_for(&attempt, 91, 5);
    exit.actor = actor(21, "/usr/bin/attacker", 'd');
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("actor mismatch is explicit ambiguity");
    assert!(matches!(record.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b0_wrong_entry_sequence_cannot_acquire_success_authority() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Open,
        22,
        100,
        "$WORKSPACE/input.txt",
    );
    let mut exit = exit_for(&attempt, 101, 3);
    exit.originating_entry_sequence = 99;
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("pairing mismatch is explicit ambiguity");
    assert!(matches!(record.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b0_non_monotonic_exit_sequence_is_ambiguous() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Open,
        23,
        110,
        "$WORKSPACE/input.txt",
    );
    let exit = exit_for(&attempt, 110, 4);
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("non-monotonic pairing is explicit ambiguity");
    assert!(matches!(record.state, EvidenceState::Ambiguous { .. }));
}

#[test]
fn b0_warning_bearing_observation_is_lost_not_success() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Open,
        24,
        120,
        "$WORKSPACE/input.txt",
    );
    let exit = exit_for(&attempt, 121, 8);
    let health = ObservationHealth {
        complete: true,
        warning_codes: BTreeSet::from(["resource_truncation".to_owned()]),
    };
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, health)
        .expect("warning-bearing pair");
    assert!(matches!(record.state, EvidenceState::Lost { .. }));
}

#[test]
fn b0_incomplete_observation_is_lost_not_success() {
    let attempt = attempt(
        TargetProposition::FileRenameDelete,
        OperationKind::UnlinkAt,
        25,
        130,
        "$WORKSPACE/file",
    );
    let exit = exit_for(&attempt, 131, 0);
    let health = ObservationHealth {
        complete: false,
        warning_codes: BTreeSet::new(),
    };
    let record = EvidenceLedger::default()
        .classify_pair(attempt, exit, health)
        .expect("incomplete pair");
    assert!(matches!(record.state, EvidenceState::Lost { .. }));
}

#[test]
fn b0_duplicate_replayed_pairing_is_rejected() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Open,
        26,
        140,
        "$WORKSPACE/input.txt",
    );
    let exit = exit_for(&attempt, 141, 6);
    let mut ledger = EvidenceLedger::default();
    ledger
        .classify_pair(attempt.clone(), exit.clone(), ObservationHealth::healthy())
        .expect("first pair");
    let error = ledger
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect_err("replay must fail");
    assert!(error.contains("duplicate/replayed"));
}

#[test]
fn b0_pairing_identity_and_serialization_are_deterministic() {
    let attempt = attempt(
        TargetProposition::NetworkConnectDestination,
        OperationKind::Connect,
        27,
        150,
        "192.0.2.10:443",
    );
    assert_eq!(
        attempt.pairing_identity().expect("first pairing"),
        attempt.pairing_identity().expect("second pairing")
    );
    let exit = exit_for(&attempt, 151, 0);
    let first = EvidenceLedger::default()
        .classify_pair(attempt.clone(), exit.clone(), ObservationHealth::healthy())
        .expect("first record");
    let second = EvidenceLedger::default()
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .expect("second record");
    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}

#[test]
fn b0_forged_negative_success_record_is_rejected() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::Open,
        28,
        160,
        "$WORKSPACE/input.txt",
    );
    let exit = exit_for(&attempt, 161, -2);
    let record = SuccessEvidenceRecord {
        pairing_identity: attempt.pairing_identity().expect("pairing"),
        attempt,
        exit: Some(exit),
        health: ObservationHealth::healthy(),
        state: EvidenceState::SuccessObserved {
            raw_return: -2,
            returned_fd: Some(2),
        },
    };
    assert!(record.validate().is_err());
}

#[test]
fn b0_forged_open_fd_identity_mismatch_is_rejected() {
    let attempt = attempt(
        TargetProposition::FileOpenObject,
        OperationKind::OpenAt,
        29,
        170,
        "$WORKSPACE/input.txt",
    );
    let exit = exit_for(&attempt, 171, 9);
    let record = SuccessEvidenceRecord {
        pairing_identity: attempt.pairing_identity().expect("pairing"),
        attempt,
        exit: Some(exit),
        health: ObservationHealth::healthy(),
        state: EvidenceState::SuccessObserved {
            raw_return: 9,
            returned_fd: Some(10),
        },
    };
    assert!(record.validate().is_err());
}
