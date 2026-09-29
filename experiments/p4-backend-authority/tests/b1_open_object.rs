use std::collections::BTreeSet;

use execsurface_p4_backend_authority::b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, TargetProposition,
};
use execsurface_p4_backend_authority::b1_open_object::{
    FdTableRelation, OpenObjectAuthority, OpenObjectRecord, PostOpenBinding,
};

fn digest(ch: char) -> String {
    format!("sha256:{}", ch.to_string().repeat(64))
}

fn actor(tid: i32, ch: char) -> ActorIdentity {
    ActorIdentity {
        tid,
        process_identity: format!("pid:{tid}"),
        causal_chain_digest: digest(ch),
    }
}

fn open_attempt(
    operation: OperationKind,
    actor: ActorIdentity,
    entry_sequence: u64,
    target: &str,
    argument_digest: String,
) -> AttemptEvidence {
    AttemptEvidence {
        proposition: TargetProposition::FileOpenObject,
        operation,
        actor,
        entry_sequence,
        argument_digest,
        target_identity: target.to_owned(),
    }
}

fn successful_open_with_argument_digest(
    operation: OperationKind,
    target: &str,
    fd: i32,
    entry_sequence: u64,
    argument_digest: String,
) -> execsurface_p4_backend_authority::b0_success_evidence::SuccessEvidenceRecord {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(
        operation,
        actor.clone(),
        entry_sequence,
        target,
        argument_digest,
    );
    EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: entry_sequence,
                exit_sequence: entry_sequence + 1,
                raw_return: i64::from(fd),
            },
            ObservationHealth::healthy(),
        )
        .expect("valid B0 success")
}

fn successful_open(
    operation: OperationKind,
    target: &str,
    fd: i32,
    entry_sequence: u64,
) -> execsurface_p4_backend_authority::b0_success_evidence::SuccessEvidenceRecord {
    successful_open_with_argument_digest(operation, target, fd, entry_sequence, digest('a'))
}

fn binding(
    evidence: &execsurface_p4_backend_authority::b0_success_evidence::SuccessEvidenceRecord,
    fd: i32,
    generation: u64,
    sequence: u64,
    object_identity: &str,
) -> PostOpenBinding {
    PostOpenBinding {
        fd,
        fd_generation: generation,
        fd_table_relation: FdTableRelation::KnownIndependent,
        originating_entry_sequence: evidence.attempt.entry_sequence,
        binding_sequence: sequence,
        object_identity: object_identity.to_owned(),
        actor_tid: evidence.attempt.actor.tid,
        causal_chain_digest: evidence.attempt.actor.causal_chain_digest.clone(),
    }
}

fn proof(record: &OpenObjectRecord) -> String {
    match &record.authority {
        OpenObjectAuthority::SuccessBounded { proof_digest } => proof_digest.clone(),
        other => panic!("expected bounded success, got {other:?}"),
    }
}

#[test]
fn b1_successful_open_requires_post_exit_object_binding() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/open", 7, 10);
    let record = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 7, 1, 12, "fd-object:/real/open")),
    )
    .expect("record");
    assert!(record.is_success_authority());
}

#[test]
fn b1_successful_openat_is_supported() {
    let evidence = successful_open(OperationKind::OpenAt, "$WORKSPACE/openat", 8, 20);
    let record = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 8, 1, 22, "fd-object:/real/openat")),
    )
    .expect("record");
    assert!(record.is_success_authority());
}

#[test]
fn b1_successful_supported_openat2_is_supported() {
    let evidence = successful_open(OperationKind::OpenAt2, "$WORKSPACE/openat2", 9, 30);
    let record = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 9, 1, 32, "fd-object:/real/openat2")),
    )
    .expect("record");
    assert!(record.is_success_authority());
}

#[test]
fn b1_success_without_later_io_is_still_representable() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/no-io", 10, 40);
    let record = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 10, 1, 42, "fd-object:/real/no-io")),
    )
    .expect("record");
    assert!(record.is_success_authority());
}

#[test]
fn b1_path_attempt_alone_never_becomes_object_success() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/path-only", 11, 50);
    let record = OpenObjectRecord::build(evidence, None).expect("record");
    assert!(!record.is_success_authority());
    assert!(matches!(
        record.authority,
        OpenObjectAuthority::Ambiguous { .. }
    ));
}

#[test]
fn b1_fd_binding_mismatch_is_ambiguous() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/fd", 12, 60);
    let record = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 13, 1, 62, "fd-object:/other")),
    )
    .expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_originating_entry_mismatch_is_ambiguous() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/entry", 14, 70);
    let mut forged = binding(&evidence, 14, 1, 72, "fd-object:/entry");
    forged.originating_entry_sequence += 1;
    let record = OpenObjectRecord::build(evidence, Some(forged)).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_actor_substitution_cannot_acquire_authority() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/actor", 15, 80);
    let mut forged = binding(&evidence, 15, 1, 82, "fd-object:/actor");
    forged.actor_tid = 9999;
    let record = OpenObjectRecord::build(evidence, Some(forged)).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_causal_chain_substitution_cannot_acquire_authority() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/chain", 16, 90);
    let mut forged = binding(&evidence, 16, 1, 92, "fd-object:/chain");
    forged.causal_chain_digest = digest('f');
    let record = OpenObjectRecord::build(evidence, Some(forged)).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_binding_must_follow_successful_syscall_exit() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/order", 17, 100);
    let record = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 17, 1, 101, "fd-object:/order")),
    )
    .expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_zero_fd_generation_is_rejected() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/gen", 18, 110);
    let result = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 18, 0, 112, "fd-object:/gen")),
    );
    assert!(result.is_err());
}

#[test]
fn b1_empty_object_identity_is_rejected() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/object", 19, 120);
    let result =
        OpenObjectRecord::build(evidence.clone(), Some(binding(&evidence, 19, 1, 122, "")));
    assert!(result.is_err());
}

#[test]
fn b1_path_toctou_changes_proof_identity() {
    let first = successful_open(OperationKind::Open, "$WORKSPACE/request-a", 20, 130);
    let first_record = OpenObjectRecord::build(
        first.clone(),
        Some(binding(&first, 20, 1, 132, "fd-object:/real/target")),
    )
    .expect("first");
    let second = successful_open(OperationKind::Open, "$WORKSPACE/request-b", 20, 140);
    let second_record = OpenObjectRecord::build(
        second.clone(),
        Some(binding(&second, 20, 1, 142, "fd-object:/real/target")),
    )
    .expect("second");
    assert_ne!(proof(&first_record), proof(&second_record));
}

#[test]
fn b1_fd_reuse_with_new_generation_changes_proof_identity() {
    let first = successful_open(OperationKind::Open, "$WORKSPACE/first", 21, 150);
    let first_record = OpenObjectRecord::build(
        first.clone(),
        Some(binding(&first, 21, 1, 152, "fd-object:/first")),
    )
    .expect("first");
    let second = successful_open(OperationKind::Open, "$WORKSPACE/second", 21, 160);
    let second_record = OpenObjectRecord::build(
        second.clone(),
        Some(binding(&second, 21, 2, 162, "fd-object:/second")),
    )
    .expect("second");
    assert_ne!(proof(&first_record), proof(&second_record));
}

#[test]
fn b1_same_fd_and_object_with_new_generation_changes_proof_identity() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/generation", 22, 170);
    let first = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 22, 1, 172, "fd-object:/same")),
    )
    .expect("first generation");
    let second = OpenObjectRecord::build(
        evidence.clone(),
        Some(binding(&evidence, 22, 2, 172, "fd-object:/same")),
    )
    .expect("second generation");
    assert_ne!(proof(&first), proof(&second));
}

#[test]
fn b1_failed_open_never_becomes_success_even_with_binding() {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(
        OperationKind::Open,
        actor.clone(),
        180,
        "$WORKSPACE/missing",
        digest('a'),
    );
    let failed = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 180,
                exit_sequence: 181,
                raw_return: -2,
            },
            ObservationHealth::healthy(),
        )
        .expect("failure evidence");
    assert!(matches!(
        failed.state,
        EvidenceState::FailureObserved { .. }
    ));
    let fake_binding = binding(&failed, 5, 1, 182, "fd-object:/forged");
    let record = OpenObjectRecord::build(failed, Some(fake_binding)).expect("record");
    assert!(matches!(
        record.authority,
        OpenObjectAuthority::NotSuccessful
    ));
}

#[test]
fn b1_lost_observation_never_becomes_success() {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(
        OperationKind::Open,
        actor.clone(),
        190,
        "$WORKSPACE/lost",
        digest('a'),
    );
    let lost = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 190,
                exit_sequence: 191,
                raw_return: 23,
            },
            ObservationHealth {
                complete: false,
                warning_codes: BTreeSet::from(["resource_truncation".to_owned()]),
            },
        )
        .expect("lost evidence");
    let fake_binding = binding(&lost, 23, 1, 192, "fd-object:/target");
    let record = OpenObjectRecord::build(lost, Some(fake_binding)).expect("record");
    assert!(!record.is_success_authority());
    assert!(matches!(record.authority, OpenObjectAuthority::Lost { .. }));
}

#[test]
fn b1_entry_exit_actor_substitution_never_reaches_success() {
    let expected_actor = actor(4242, 'c');
    let attempt = open_attempt(
        OperationKind::Open,
        expected_actor.clone(),
        200,
        "$WORKSPACE/substitute",
        digest('a'),
    );
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor: actor(9999, 'c'),
                originating_entry_sequence: 200,
                exit_sequence: 201,
                raw_return: 24,
            },
            ObservationHealth::healthy(),
        )
        .expect("ambiguous evidence");
    assert!(matches!(evidence.state, EvidenceState::Ambiguous { .. }));
    let record = OpenObjectRecord::build(evidence, None).expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_duplicate_pairing_is_rejected_before_object_authority() {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(
        OperationKind::Open,
        actor.clone(),
        210,
        "$WORKSPACE/replay",
        digest('a'),
    );
    let exit = ExitEvidence {
        actor,
        originating_entry_sequence: 210,
        exit_sequence: 211,
        raw_return: 25,
    };
    let mut ledger = EvidenceLedger::default();
    assert!(ledger
        .classify_pair(attempt.clone(), exit.clone(), ObservationHealth::healthy())
        .is_ok());
    assert!(ledger
        .classify_pair(attempt, exit, ObservationHealth::healthy())
        .is_err());
}

#[test]
fn b1_deterministic_rebuild_has_identical_proof_and_bytes() {
    let evidence = successful_open(OperationKind::OpenAt, "$WORKSPACE/stable", 26, 220);
    let object_binding = binding(&evidence, 26, 7, 222, "fd-object:/stable");
    let first = OpenObjectRecord::build(evidence.clone(), Some(object_binding.clone()))
        .expect("first proof");
    let second = OpenObjectRecord::build(evidence, Some(object_binding)).expect("second proof");
    assert_eq!(proof(&first), proof(&second));
    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}

#[test]
fn b1_rejects_non_open_proposition() {
    let actor = actor(4242, 'c');
    let attempt = AttemptEvidence {
        proposition: TargetProposition::FileRenameDelete,
        operation: OperationKind::Rename,
        actor: actor.clone(),
        entry_sequence: 230,
        argument_digest: digest('a'),
        target_identity: "$WORKSPACE/a->$WORKSPACE/b".to_owned(),
    };
    let evidence = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 230,
                exit_sequence: 231,
                raw_return: 0,
            },
            ObservationHealth::healthy(),
        )
        .expect("rename evidence");
    assert!(OpenObjectRecord::build(evidence, None).is_err());
}

#[test]
fn b1_unknown_fd_table_relation_blocks_success_authority() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/unknown-fdtable", 27, 240);
    let mut object_binding = binding(&evidence, 27, 1, 242, "fd-object:/unknown-fdtable");
    object_binding.fd_table_relation = FdTableRelation::Unknown;
    let record = OpenObjectRecord::build(evidence, Some(object_binding)).expect("record");
    assert!(!record.is_success_authority());
    assert!(matches!(
        record.authority,
        OpenObjectAuthority::Ambiguous { .. }
    ));
}

#[test]
fn b1_known_shared_certified_fd_table_can_remain_bounded() {
    let evidence = successful_open(OperationKind::Open, "$WORKSPACE/shared-certified", 28, 250);
    let mut object_binding = binding(&evidence, 28, 1, 252, "fd-object:/shared-certified");
    object_binding.fd_table_relation = FdTableRelation::KnownSharedCertified;
    let record = OpenObjectRecord::build(evidence, Some(object_binding)).expect("record");
    assert!(record.is_success_authority());
}

#[test]
fn b1_argument_context_digest_substitution_changes_proof_identity() {
    let first = successful_open_with_argument_digest(
        OperationKind::OpenAt,
        "$WORKSPACE/same-target",
        29,
        260,
        digest('a'),
    );
    let first_record = OpenObjectRecord::build(
        first.clone(),
        Some(binding(&first, 29, 1, 262, "fd-object:/same-target")),
    )
    .expect("first");
    let second = successful_open_with_argument_digest(
        OperationKind::OpenAt,
        "$WORKSPACE/same-target",
        29,
        260,
        digest('b'),
    );
    let second_record = OpenObjectRecord::build(
        second.clone(),
        Some(binding(&second, 29, 1, 262, "fd-object:/same-target")),
    )
    .expect("second");
    assert_ne!(proof(&first_record), proof(&second_record));
}
