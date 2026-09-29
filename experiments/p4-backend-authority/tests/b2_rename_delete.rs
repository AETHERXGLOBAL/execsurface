use std::collections::BTreeSet;

use execsurface_p4_backend_authority::b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, TargetProposition,
};
use execsurface_p4_backend_authority::b2_rename_delete::{
    RenameDeleteAuthority, RenameDeleteContext, RenameDeleteRecord,
};

fn digest(ch: char) -> String {
    format!("sha256:{}", ch.to_string().repeat(64))
}

fn actor(tid: i32) -> ActorIdentity {
    ActorIdentity {
        tid,
        process_identity: format!("pid:{tid}"),
        causal_chain_digest: digest('c'),
    }
}

fn rename_context(operation: OperationKind) -> RenameDeleteContext {
    match operation {
        OperationKind::Rename => RenameDeleteContext::Rename {
            operation,
            source: "$WORKSPACE/source".to_owned(),
            target: "$WORKSPACE/target".to_owned(),
            source_dirfd: None,
            target_dirfd: None,
            flags: None,
            flags_classified: true,
        },
        OperationKind::RenameAt => RenameDeleteContext::Rename {
            operation,
            source: "source".to_owned(),
            target: "target".to_owned(),
            source_dirfd: Some(30),
            target_dirfd: Some(31),
            flags: None,
            flags_classified: true,
        },
        OperationKind::RenameAt2 => RenameDeleteContext::Rename {
            operation,
            source: "source".to_owned(),
            target: "target".to_owned(),
            source_dirfd: Some(30),
            target_dirfd: Some(31),
            flags: Some(1),
            flags_classified: true,
        },
        _ => panic!("not rename family"),
    }
}

fn delete_context(operation: OperationKind) -> RenameDeleteContext {
    match operation {
        OperationKind::Unlink => RenameDeleteContext::Delete {
            operation,
            target: "$WORKSPACE/delete-me".to_owned(),
            dirfd: None,
            flags: None,
            flags_classified: true,
        },
        OperationKind::UnlinkAt => RenameDeleteContext::Delete {
            operation,
            target: "delete-me".to_owned(),
            dirfd: Some(32),
            flags: Some(0),
            flags_classified: true,
        },
        OperationKind::Rmdir => RenameDeleteContext::Delete {
            operation,
            target: "$WORKSPACE/remove-dir".to_owned(),
            dirfd: None,
            flags: None,
            flags_classified: true,
        },
        _ => panic!("not delete family"),
    }
}

fn attempt_for(
    context: &RenameDeleteContext,
    actor: ActorIdentity,
    entry_sequence: u64,
) -> AttemptEvidence {
    AttemptEvidence {
        proposition: TargetProposition::FileRenameDelete,
        operation: context.operation(),
        actor,
        entry_sequence,
        argument_digest: context.digest().expect("valid context digest"),
        target_identity: context.target_identity(),
    }
}

fn paired(
    context: &RenameDeleteContext,
    entry_sequence: u64,
    raw_return: i64,
) -> execsurface_p4_backend_authority::b0_success_evidence::SuccessEvidenceRecord {
    let actor = actor(4242);
    let attempt = attempt_for(context, actor.clone(), entry_sequence);
    EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: entry_sequence,
                exit_sequence: entry_sequence + 1,
                raw_return,
            },
            ObservationHealth::healthy(),
        )
        .expect("paired evidence")
}

fn success_record(context: RenameDeleteContext, entry_sequence: u64) -> RenameDeleteRecord {
    let evidence = paired(&context, entry_sequence, 0);
    RenameDeleteRecord::build(evidence, context).expect("B2 record")
}

fn proof(record: &RenameDeleteRecord) -> String {
    match &record.authority {
        RenameDeleteAuthority::SuccessEffectBounded { proof_digest } => proof_digest.clone(),
        other => panic!("expected bounded success, got {other:?}"),
    }
}

#[test]
fn b2_rename_rc_zero_is_success() {
    assert!(success_record(rename_context(OperationKind::Rename), 10).is_success_authority());
}

#[test]
fn b2_renameat_rc_zero_is_success() {
    assert!(success_record(rename_context(OperationKind::RenameAt), 20).is_success_authority());
}

#[test]
fn b2_renameat2_classified_flags_rc_zero_is_success() {
    assert!(success_record(rename_context(OperationKind::RenameAt2), 30).is_success_authority());
}

#[test]
fn b2_unlink_rc_zero_is_success() {
    assert!(success_record(delete_context(OperationKind::Unlink), 40).is_success_authority());
}

#[test]
fn b2_unlinkat_classified_flags_rc_zero_is_success() {
    assert!(success_record(delete_context(OperationKind::UnlinkAt), 50).is_success_authority());
}

#[test]
fn b2_rmdir_rc_zero_is_success() {
    assert!(success_record(delete_context(OperationKind::Rmdir), 60).is_success_authority());
}

#[test]
fn b2_deterministic_rebuild_has_identical_proof_and_bytes() {
    let context = rename_context(OperationKind::RenameAt);
    let evidence = paired(&context, 70, 0);
    let first = RenameDeleteRecord::build(evidence.clone(), context.clone()).expect("first");
    let second = RenameDeleteRecord::build(evidence, context).expect("second");
    assert_eq!(proof(&first), proof(&second));
    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}

#[test]
fn b2_enoent_remains_failure() {
    let context = rename_context(OperationKind::Rename);
    let record = RenameDeleteRecord::build(paired(&context, 80, -2), context).expect("record");
    assert!(matches!(
        record.authority,
        RenameDeleteAuthority::FailureObserved { errno: 2 }
    ));
}

#[test]
fn b2_unexpected_positive_return_is_ambiguous_not_success() {
    let context = delete_context(OperationKind::Unlink);
    let record = RenameDeleteRecord::build(paired(&context, 90, 1), context).expect("record");
    assert!(!record.is_success_authority());
    assert!(matches!(
        record.authority,
        RenameDeleteAuthority::Ambiguous { .. }
    ));
}

#[test]
fn b2_entry_without_exit_is_attempt_only() {
    let context = rename_context(OperationKind::Rename);
    let evidence = EvidenceLedger::default()
        .attempt_only(
            attempt_for(&context, actor(4242), 100),
            ObservationHealth::healthy(),
        )
        .expect("attempt evidence");
    let record = RenameDeleteRecord::build(evidence, context).expect("record");
    assert!(matches!(record.authority, RenameDeleteAuthority::AttemptOnly));
}

#[test]
fn b2_wrong_actor_never_becomes_success() {
    let context = rename_context(OperationKind::Rename);
    let attempt = attempt_for(&context, actor(4242), 110);
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor: actor(9999),
                originating_entry_sequence: 110,
                exit_sequence: 111,
                raw_return: 0,
            },
            ObservationHealth::healthy(),
        )
        .expect("ambiguous evidence");
    let record = RenameDeleteRecord::build(evidence, context).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b2_wrong_originating_entry_never_becomes_success() {
    let context = rename_context(OperationKind::Rename);
    let actor = actor(4242);
    let attempt = attempt_for(&context, actor.clone(), 120);
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 121,
                exit_sequence: 122,
                raw_return: 0,
            },
            ObservationHealth::healthy(),
        )
        .expect("ambiguous evidence");
    let record = RenameDeleteRecord::build(evidence, context).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b2_source_substitution_breaks_context_binding() {
    let context = rename_context(OperationKind::Rename);
    let evidence = paired(&context, 130, 0);
    let mut forged = context.clone();
    if let RenameDeleteContext::Rename { source, .. } = &mut forged {
        *source = "$WORKSPACE/forged-source".to_owned();
    }
    assert!(RenameDeleteRecord::build(evidence, forged).is_err());
}

#[test]
fn b2_target_substitution_breaks_context_binding() {
    let context = rename_context(OperationKind::Rename);
    let evidence = paired(&context, 140, 0);
    let mut forged = context.clone();
    if let RenameDeleteContext::Rename { target, .. } = &mut forged {
        *target = "$WORKSPACE/forged-target".to_owned();
    }
    assert!(RenameDeleteRecord::build(evidence, forged).is_err());
}

#[test]
fn b2_operation_family_substitution_is_rejected() {
    let context = rename_context(OperationKind::Rename);
    let evidence = paired(&context, 150, 0);
    assert!(RenameDeleteRecord::build(evidence, delete_context(OperationKind::Unlink)).is_err());
}

#[test]
fn b2_duplicate_replayed_pairing_is_rejected() {
    let context = rename_context(OperationKind::Rename);
    let actor = actor(4242);
    let attempt = attempt_for(&context, actor.clone(), 160);
    let exit = ExitEvidence {
        actor,
        originating_entry_sequence: 160,
        exit_sequence: 161,
        raw_return: 0,
    };
    let mut ledger = EvidenceLedger::default();
    assert!(ledger
        .classify_pair(
            attempt.clone(),
            exit.clone(),
            ObservationHealth::healthy()
        )
        .is_ok());
    assert!(ledger
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .is_err());
}

#[test]
fn b2_non_monotonic_exit_is_ambiguous() {
    let context = rename_context(OperationKind::Rename);
    let actor = actor(4242);
    let attempt = attempt_for(&context, actor.clone(), 170);
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 170,
                exit_sequence: 170,
                raw_return: 0,
            },
            ObservationHealth::healthy(),
        )
        .expect("ambiguous evidence");
    let record = RenameDeleteRecord::build(evidence, context).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b2_unclassified_renameat2_flags_fail_closed() {
    let context = RenameDeleteContext::Rename {
        operation: OperationKind::RenameAt2,
        source: "source".to_owned(),
        target: "target".to_owned(),
        source_dirfd: Some(30),
        target_dirfd: Some(31),
        flags: Some(0x8000_0000),
        flags_classified: false,
    };
    assert!(context.validate().is_err());
}

#[test]
fn b2_unclassified_unlinkat_flags_fail_closed() {
    let context = RenameDeleteContext::Delete {
        operation: OperationKind::UnlinkAt,
        target: "delete-me".to_owned(),
        dirfd: Some(32),
        flags: Some(0x8000_0000),
        flags_classified: false,
    };
    assert!(context.validate().is_err());
}

#[test]
fn b2_lost_observation_never_becomes_success() {
    let context = delete_context(OperationKind::Unlink);
    let actor = actor(4242);
    let attempt = attempt_for(&context, actor.clone(), 180);
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 180,
                exit_sequence: 181,
                raw_return: 0,
            },
            ObservationHealth {
                complete: false,
                warning_codes: BTreeSet::from(["resource_truncation".to_owned()]),
            },
        )
        .expect("lost evidence");
    assert!(matches!(evidence.state, EvidenceState::Lost { .. }));
    let record = RenameDeleteRecord::build(evidence, context).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b2_open_object_evidence_cannot_satisfy_rename_delete() {
    let actor = actor(4242);
    let attempt = AttemptEvidence {
        proposition: TargetProposition::FileOpenObject,
        operation: OperationKind::Open,
        actor: actor.clone(),
        entry_sequence: 190,
        argument_digest: digest('a'),
        target_identity: "$WORKSPACE/input".to_owned(),
    };
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 190,
                exit_sequence: 191,
                raw_return: 3,
            },
            ObservationHealth::healthy(),
        )
        .expect("open evidence");
    assert!(RenameDeleteRecord::build(evidence, delete_context(OperationKind::Unlink)).is_err());
}

#[test]
fn b2_context_digest_changes_when_rename_flags_change() {
    let first = rename_context(OperationKind::RenameAt2);
    let mut second = first.clone();
    if let RenameDeleteContext::Rename { flags, .. } = &mut second {
        *flags = Some(2);
    }
    assert_ne!(first.digest().expect("first"), second.digest().expect("second"));
}

#[test]
fn b2_warning_bearing_observation_never_becomes_success() {
    let context = rename_context(OperationKind::Rename);
    let actor = actor(4242);
    let attempt = attempt_for(&context, actor.clone(), 200);
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 200,
                exit_sequence: 201,
                raw_return: 0,
            },
            ObservationHealth {
                complete: true,
                warning_codes: BTreeSet::from(["observer_warning".to_owned()]),
            },
        )
        .expect("lost evidence");
    let record = RenameDeleteRecord::build(evidence, context).expect("record");
    assert!(!record.is_success_authority());
}
