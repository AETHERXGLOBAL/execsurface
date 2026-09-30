pub use execsurface_p4_backend_authority::b0_success_evidence;
#[path = "../src/b3_connect_destination.rs"]
mod b3_connect_destination;

use std::collections::BTreeSet;

use b3_connect_destination::{ConnectAuthority, ConnectContext, ConnectDestination, ConnectRecord};
use execsurface_p4_backend_authority::b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, SuccessEvidenceRecord, TargetProposition,
};
use execsurface_p4_backend_authority::b1_open_object::{
    FdTableRelation, OpenObjectAuthority, OpenObjectRecord, PostOpenBinding,
};
use execsurface_p4_backend_authority::b2_rename_delete::{
    RenameDeleteAuthority, RenameDeleteContext, RenameDeleteRecord,
};
use sha2::{Digest, Sha256};

fn digest_text(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

fn actor(label: &str, tid: i32) -> ActorIdentity {
    let process_identity = format!("cross-proposition:{label}");
    ActorIdentity {
        tid,
        causal_chain_digest: digest_text(&process_identity),
        process_identity,
    }
}

fn exit_for(attempt: &AttemptEvidence, raw_return: i64) -> ExitEvidence {
    ExitEvidence {
        actor: attempt.actor.clone(),
        originating_entry_sequence: attempt.entry_sequence,
        exit_sequence: attempt.entry_sequence + 1,
        raw_return,
    }
}

fn classify(
    attempt: &AttemptEvidence,
    raw_return: i64,
    health: ObservationHealth,
) -> SuccessEvidenceRecord {
    EvidenceLedger::default()
        .classify_pair(attempt.clone(), exit_for(attempt, raw_return), health)
        .unwrap()
}

fn open_attempt(entry_sequence: u64, fd: i32) -> AttemptEvidence {
    AttemptEvidence {
        proposition: TargetProposition::FileOpenObject,
        operation: OperationKind::Open,
        actor: actor("open", 4101),
        entry_sequence,
        argument_digest: digest_text("open:/tmp/cross-object"),
        target_identity: format!("/tmp/cross-object@fd:{fd}"),
    }
}

fn open_binding(attempt: &AttemptEvidence, fd: i32, generation: u64) -> PostOpenBinding {
    PostOpenBinding {
        fd,
        fd_generation: generation,
        fd_table_relation: FdTableRelation::KnownIndependent,
        originating_entry_sequence: attempt.entry_sequence,
        binding_sequence: attempt.entry_sequence + 2,
        object_identity: "dev:cross:ino:1".to_owned(),
        actor_tid: attempt.actor.tid,
        causal_chain_digest: attempt.actor.causal_chain_digest.clone(),
    }
}

fn rename_context() -> RenameDeleteContext {
    RenameDeleteContext::Rename {
        operation: OperationKind::Rename,
        source: "/tmp/cross-source".to_owned(),
        target: "/tmp/cross-target".to_owned(),
        source_dirfd: None,
        target_dirfd: None,
        flags: None,
        flags_classified: true,
    }
}

fn rename_attempt(entry_sequence: u64) -> AttemptEvidence {
    let context = rename_context();
    AttemptEvidence {
        proposition: TargetProposition::FileRenameDelete,
        operation: OperationKind::Rename,
        actor: actor("rename", 4201),
        entry_sequence,
        argument_digest: context.digest().unwrap(),
        target_identity: context.target_identity(),
    }
}

fn connect_context(fd: i32) -> ConnectContext {
    ConnectContext {
        socket_fd: fd,
        destination: ConnectDestination::Inet4 {
            address: [127, 0, 0, 1],
            port: 443,
        },
        sockaddr_len: 16,
        sockaddr_complete: true,
    }
}

fn connect_attempt(entry_sequence: u64, fd: i32) -> AttemptEvidence {
    let context = connect_context(fd);
    AttemptEvidence {
        proposition: TargetProposition::NetworkConnectDestination,
        operation: OperationKind::Connect,
        actor: actor("connect", 4301),
        entry_sequence,
        argument_digest: context.digest().unwrap(),
        target_identity: context.target_identity().unwrap(),
    }
}

fn retain_non_live_b3_variants_for_target_compilation() {
    let _ = ConnectDestination::Inet6 {
        address: [0; 16],
        port: 443,
        flowinfo: 0,
        scope_id: 0,
    };
    let _ = ConnectDestination::Unix {
        path_bytes: b"cross.sock".to_vec(),
        abstract_namespace: false,
    };
}

fn open_proof(record: &OpenObjectRecord) -> &str {
    match &record.authority {
        OpenObjectAuthority::SuccessBounded { proof_digest } => proof_digest,
        other => panic!("expected open success proof, got {other:?}"),
    }
}

fn rename_proof(record: &RenameDeleteRecord) -> &str {
    match &record.authority {
        RenameDeleteAuthority::SuccessEffectBounded { proof_digest } => proof_digest,
        other => panic!("expected rename success proof, got {other:?}"),
    }
}

fn connect_proof(record: &ConnectRecord) -> &str {
    match &record.authority {
        ConnectAuthority::SynchronousSuccessBounded { proof_digest } => proof_digest,
        other => panic!("expected connect success proof, got {other:?}"),
    }
}

#[test]
fn cross_01_entry_only_never_becomes_success() {
    let open = open_attempt(10, 7);
    let open_evidence = EvidenceLedger::default()
        .attempt_only(open.clone(), ObservationHealth::healthy())
        .unwrap();
    let open_record = OpenObjectRecord::build(open_evidence, Some(open_binding(&open, 7, 1))).unwrap();
    assert!(!open_record.is_success_authority());

    let rename = rename_attempt(20);
    let rename_evidence = EvidenceLedger::default()
        .attempt_only(rename, ObservationHealth::healthy())
        .unwrap();
    let rename_record = RenameDeleteRecord::build(rename_evidence, rename_context()).unwrap();
    assert!(!rename_record.is_success_authority());

    let connect = connect_attempt(30, 9);
    let connect_evidence = EvidenceLedger::default()
        .attempt_only(connect, ObservationHealth::healthy())
        .unwrap();
    let connect_record = ConnectRecord::build(connect_evidence, connect_context(9)).unwrap();
    assert!(!connect_record.is_success_authority());
}

#[test]
fn cross_02_failure_never_becomes_success() {
    let open = open_attempt(40, 7);
    let open_record = OpenObjectRecord::build(
        classify(&open, -2, ObservationHealth::healthy()),
        Some(open_binding(&open, 7, 1)),
    )
    .unwrap();
    assert!(!open_record.is_success_authority());

    let rename = rename_attempt(50);
    let rename_record = RenameDeleteRecord::build(
        classify(&rename, -2, ObservationHealth::healthy()),
        rename_context(),
    )
    .unwrap();
    assert!(!rename_record.is_success_authority());

    let connect = connect_attempt(60, 9);
    let connect_record = ConnectRecord::build(
        classify(&connect, -111, ObservationHealth::healthy()),
        connect_context(9),
    )
    .unwrap();
    assert!(!connect_record.is_success_authority());
}

#[test]
fn cross_03_einprogress_remains_pending_and_cannot_satisfy_file_propositions() {
    let connect = connect_attempt(70, 9);
    let pending = classify(&connect, -115, ObservationHealth::healthy());
    let connect_record = ConnectRecord::build(pending.clone(), connect_context(9)).unwrap();
    assert!(matches!(
        connect_record.authority,
        ConnectAuthority::Pending { errno: 115 }
    ));
    assert!(!connect_record.is_success_authority());
    assert!(OpenObjectRecord::build(pending.clone(), None).is_err());
    assert!(RenameDeleteRecord::build(pending, rename_context()).is_err());
}

#[test]
fn cross_04_actor_and_entry_substitution_fail_closed() {
    let open = open_attempt(80, 7);
    let mut wrong_actor = exit_for(&open, 7);
    wrong_actor.actor = actor("forged-open", 4999);
    let open_evidence = EvidenceLedger::default()
        .classify_pair(open.clone(), wrong_actor, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(open_evidence.state, EvidenceState::Ambiguous { .. }));
    let open_record = OpenObjectRecord::build(open_evidence, Some(open_binding(&open, 7, 1))).unwrap();
    assert!(!open_record.is_success_authority());

    let rename = rename_attempt(90);
    let mut wrong_entry = exit_for(&rename, 0);
    wrong_entry.originating_entry_sequence += 1;
    let rename_evidence = EvidenceLedger::default()
        .classify_pair(rename, wrong_entry, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(rename_evidence.state, EvidenceState::Ambiguous { .. }));
    let rename_record = RenameDeleteRecord::build(rename_evidence, rename_context()).unwrap();
    assert!(!rename_record.is_success_authority());

    let connect = connect_attempt(100, 9);
    let mut wrong_connect_actor = exit_for(&connect, 0);
    wrong_connect_actor.actor = actor("forged-connect", 4998);
    let connect_evidence = EvidenceLedger::default()
        .classify_pair(connect, wrong_connect_actor, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(connect_evidence.state, EvidenceState::Ambiguous { .. }));
    let connect_record = ConnectRecord::build(connect_evidence, connect_context(9)).unwrap();
    assert!(!connect_record.is_success_authority());
}

#[test]
fn cross_05_duplicate_replay_is_rejected_without_authority_inflation() {
    let open = open_attempt(110, 7);
    let open_exit = exit_for(&open, 7);
    let mut open_ledger = EvidenceLedger::default();
    assert!(open_ledger
        .classify_pair(open.clone(), open_exit.clone(), ObservationHealth::healthy())
        .is_ok());
    assert!(open_ledger
        .classify_pair(open, open_exit, ObservationHealth::healthy())
        .is_err());

    let rename = rename_attempt(120);
    let rename_exit = exit_for(&rename, 0);
    let mut rename_ledger = EvidenceLedger::default();
    assert!(rename_ledger
        .classify_pair(rename.clone(), rename_exit.clone(), ObservationHealth::healthy())
        .is_ok());
    assert!(rename_ledger
        .classify_pair(rename, rename_exit, ObservationHealth::healthy())
        .is_err());

    let connect = connect_attempt(130, 9);
    let connect_exit = exit_for(&connect, 0);
    let mut connect_ledger = EvidenceLedger::default();
    assert!(connect_ledger
        .classify_pair(connect.clone(), connect_exit.clone(), ObservationHealth::healthy())
        .is_ok());
    assert!(connect_ledger
        .classify_pair(connect, connect_exit, ObservationHealth::healthy())
        .is_err());
}

#[test]
fn cross_06_fd_reuse_requires_new_object_socket_and_pairing_identity() {
    let first_open = open_attempt(140, 7);
    let first_open_evidence = classify(&first_open, 7, ObservationHealth::healthy());
    let first_open_record = OpenObjectRecord::build(
        first_open_evidence,
        Some(open_binding(&first_open, 7, 1)),
    )
    .unwrap();
    assert!(first_open_record.is_success_authority());

    let mut second_open = first_open.clone();
    second_open.entry_sequence = 150;
    let second_open_evidence = classify(&second_open, 7, ObservationHealth::healthy());
    let second_open_record = OpenObjectRecord::build(
        second_open_evidence,
        Some(open_binding(&second_open, 7, 2)),
    )
    .unwrap();
    assert!(second_open_record.is_success_authority());
    assert_ne!(
        first_open.pairing_identity().unwrap(),
        second_open.pairing_identity().unwrap()
    );
    assert_ne!(open_proof(&first_open_record), open_proof(&second_open_record));

    let first_connect = connect_attempt(160, 9);
    let first_connect_evidence = classify(&first_connect, 0, ObservationHealth::healthy());
    let first_connect_record =
        ConnectRecord::build(first_connect_evidence.clone(), connect_context(9)).unwrap();
    assert!(first_connect_record.is_success_authority());

    let mut second_connect = first_connect.clone();
    second_connect.entry_sequence = 170;
    let second_connect_record = ConnectRecord::build(
        classify(&second_connect, 0, ObservationHealth::healthy()),
        connect_context(9),
    )
    .unwrap();
    assert_ne!(
        first_connect.pairing_identity().unwrap(),
        second_connect.pairing_identity().unwrap()
    );
    assert_ne!(
        connect_proof(&first_connect_record),
        connect_proof(&second_connect_record)
    );
    assert!(ConnectRecord::build(first_connect_evidence, connect_context(10)).is_err());
}

#[test]
fn cross_07_observer_loss_blocks_success_across_all_propositions() {
    let lost = ObservationHealth {
        complete: false,
        warning_codes: BTreeSet::from(["cross_controlled_loss".to_owned()]),
    };

    let open = open_attempt(180, 7);
    let open_record = OpenObjectRecord::build(
        classify(&open, 7, lost.clone()),
        Some(open_binding(&open, 7, 1)),
    )
    .unwrap();
    assert!(matches!(open_record.authority, OpenObjectAuthority::Lost { .. }));
    assert!(!open_record.is_success_authority());

    let rename = rename_attempt(190);
    let rename_record =
        RenameDeleteRecord::build(classify(&rename, 0, lost.clone()), rename_context()).unwrap();
    assert!(matches!(rename_record.authority, RenameDeleteAuthority::Lost { .. }));
    assert!(!rename_record.is_success_authority());

    let connect = connect_attempt(200, 9);
    let connect_record =
        ConnectRecord::build(classify(&connect, 0, lost), connect_context(9)).unwrap();
    assert!(matches!(connect_record.authority, ConnectAuthority::Lost { .. }));
    assert!(!connect_record.is_success_authority());
}

#[test]
fn cross_08_pathname_or_binding_without_success_exit_cannot_create_open_object_success() {
    let open = open_attempt(210, 7);
    let attempted = EvidenceLedger::default()
        .attempt_only(open.clone(), ObservationHealth::healthy())
        .unwrap();
    let record = OpenObjectRecord::build(attempted, Some(open_binding(&open, 7, 1))).unwrap();
    assert!(matches!(record.authority, OpenObjectAuthority::NotSuccessful));
    assert!(!record.is_success_authority());
}

#[test]
fn cross_09_success_for_one_proposition_is_rejected_by_the_other_builders() {
    let open = open_attempt(220, 7);
    let open_success = classify(&open, 7, ObservationHealth::healthy());
    assert!(RenameDeleteRecord::build(open_success.clone(), rename_context()).is_err());
    assert!(ConnectRecord::build(open_success, connect_context(9)).is_err());

    let rename = rename_attempt(230);
    let rename_success = classify(&rename, 0, ObservationHealth::healthy());
    assert!(OpenObjectRecord::build(rename_success.clone(), None).is_err());
    assert!(ConnectRecord::build(rename_success, connect_context(9)).is_err());

    let connect = connect_attempt(240, 9);
    let connect_success = classify(&connect, 0, ObservationHealth::healthy());
    assert!(OpenObjectRecord::build(connect_success.clone(), None).is_err());
    assert!(RenameDeleteRecord::build(connect_success, rename_context()).is_err());
}

#[test]
fn cross_10_identical_evidence_reconstructs_identical_bytes_and_proof_digests() {
    retain_non_live_b3_variants_for_target_compilation();

    let open = open_attempt(250, 7);
    let open_evidence = classify(&open, 7, ObservationHealth::healthy());
    let binding = open_binding(&open, 7, 1);
    let open_a = OpenObjectRecord::build(open_evidence.clone(), Some(binding.clone())).unwrap();
    let open_b = OpenObjectRecord::build(open_evidence, Some(binding)).unwrap();
    assert_eq!(serde_json::to_vec(&open_a).unwrap(), serde_json::to_vec(&open_b).unwrap());
    assert_eq!(open_proof(&open_a), open_proof(&open_b));

    let rename = rename_attempt(260);
    let rename_evidence = classify(&rename, 0, ObservationHealth::healthy());
    let rename_a = RenameDeleteRecord::build(rename_evidence.clone(), rename_context()).unwrap();
    let rename_b = RenameDeleteRecord::build(rename_evidence, rename_context()).unwrap();
    assert_eq!(serde_json::to_vec(&rename_a).unwrap(), serde_json::to_vec(&rename_b).unwrap());
    assert_eq!(rename_proof(&rename_a), rename_proof(&rename_b));

    let connect = connect_attempt(270, 9);
    let connect_evidence = classify(&connect, 0, ObservationHealth::healthy());
    let connect_a = ConnectRecord::build(connect_evidence.clone(), connect_context(9)).unwrap();
    let connect_b = ConnectRecord::build(connect_evidence, connect_context(9)).unwrap();
    assert_eq!(serde_json::to_vec(&connect_a).unwrap(), serde_json::to_vec(&connect_b).unwrap());
    assert_eq!(connect_proof(&connect_a), connect_proof(&connect_b));
}

#[test]
fn cross_11_causal_chain_substitution_cannot_preserve_success_authority() {
    let open = open_attempt(280, 7);
    let open_evidence = classify(&open, 7, ObservationHealth::healthy());
    let mut forged_binding = open_binding(&open, 7, 1);
    forged_binding.causal_chain_digest = digest_text("forged-causal-chain");
    let forged_open = OpenObjectRecord::build(open_evidence, Some(forged_binding)).unwrap();
    assert!(matches!(forged_open.authority, OpenObjectAuthority::Ambiguous { .. }));
    assert!(!forged_open.is_success_authority());

    let connect = connect_attempt(290, 9);
    let mut forged_exit = exit_for(&connect, 0);
    forged_exit.actor.causal_chain_digest = digest_text("forged-connect-causal-chain");
    let forged_connect_evidence = EvidenceLedger::default()
        .classify_pair(connect, forged_exit, ObservationHealth::healthy())
        .unwrap();
    assert!(matches!(
        forged_connect_evidence.state,
        EvidenceState::Ambiguous { .. }
    ));
    assert!(!forged_connect_evidence.state.is_success());
}

#[test]
fn cross_12_pairing_identity_is_domain_separated_by_proposition_and_operation() {
    let common_actor = actor("domain-separation", 4401);
    let common_digest = digest_text("identical-argument-bytes");
    let common_target = "identical-target-text".to_owned();

    let open = AttemptEvidence {
        proposition: TargetProposition::FileOpenObject,
        operation: OperationKind::Open,
        actor: common_actor.clone(),
        entry_sequence: 300,
        argument_digest: common_digest.clone(),
        target_identity: common_target.clone(),
    };
    let rename = AttemptEvidence {
        proposition: TargetProposition::FileRenameDelete,
        operation: OperationKind::Rename,
        actor: common_actor.clone(),
        entry_sequence: 300,
        argument_digest: common_digest.clone(),
        target_identity: common_target.clone(),
    };
    let connect = AttemptEvidence {
        proposition: TargetProposition::NetworkConnectDestination,
        operation: OperationKind::Connect,
        actor: common_actor,
        entry_sequence: 300,
        argument_digest: common_digest,
        target_identity: common_target,
    };

    let identities = BTreeSet::from([
        open.pairing_identity().unwrap(),
        rename.pairing_identity().unwrap(),
        connect.pairing_identity().unwrap(),
    ]);
    assert_eq!(identities.len(), 3);
}
