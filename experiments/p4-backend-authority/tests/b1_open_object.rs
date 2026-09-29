use std::collections::BTreeSet;

use execsurface_p4_backend_authority::b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, EvidenceState, ExitEvidence, ObservationHealth,
    OperationKind, TargetProposition,
};
use execsurface_p4_backend_authority::b1_open_object::{
    OpenObjectAuthority, OpenObjectRecord, PostOpenBinding,
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

fn open_attempt(actor: ActorIdentity, entry_sequence: u64, target: &str) -> AttemptEvidence {
    AttemptEvidence {
        proposition: TargetProposition::FileOpenObject,
        operation: OperationKind::OpenAt,
        actor,
        entry_sequence,
        argument_digest: digest('a'),
        target_identity: target.to_owned(),
    }
}

fn successful_open(target: &str, fd: i32) -> execsurface_p4_backend_authority::b0_success_evidence::SuccessEvidenceRecord {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(actor.clone(), 10, target);
    EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 10,
                exit_sequence: 11,
                raw_return: i64::from(fd),
            },
            ObservationHealth::healthy(),
        )
        .expect("valid B0 success")
}

fn binding(fd: i32, sequence: u64, object_identity: &str) -> PostOpenBinding {
    PostOpenBinding {
        fd,
        binding_sequence: sequence,
        object_identity: object_identity.to_owned(),
        actor_tid: 4242,
        causal_chain_digest: digest('c'),
    }
}

#[test]
fn b1_success_requires_post_open_binding_for_returned_fd() {
    let record = OpenObjectRecord::build(
        successful_open("$WORKSPACE/requested", 7),
        Some(binding(7, 12, "fd-object:/real/target")),
    )
    .expect("record");
    assert!(record.is_success_authority());
}

#[test]
fn b1_success_without_later_io_is_still_representable() {
    let record = OpenObjectRecord::build(
        successful_open("$WORKSPACE/no-later-io", 8),
        Some(binding(8, 12, "fd-object:/real/no-later-io")),
    )
    .expect("record");
    assert!(matches!(
        record.authority,
        OpenObjectAuthority::SuccessBounded { .. }
    ));
}

#[test]
fn b1_path_attempt_alone_never_becomes_open_object_success() {
    let record = OpenObjectRecord::build(successful_open("$WORKSPACE/requested", 9), None)
        .expect("record");
    assert!(!record.is_success_authority());
    assert!(matches!(record.authority, OpenObjectAuthority::Ambiguous { .. }));
}

#[test]
fn b1_fd_binding_mismatch_is_ambiguous() {
    let record = OpenObjectRecord::build(
        successful_open("$WORKSPACE/requested", 10),
        Some(binding(11, 12, "fd-object:/other")),
    )
    .expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_actor_substitution_cannot_acquire_authority() {
    let mut forged = binding(12, 12, "fd-object:/real/target");
    forged.actor_tid = 9999;
    let record = OpenObjectRecord::build(successful_open("$WORKSPACE/requested", 12), Some(forged))
        .expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_causal_chain_substitution_cannot_acquire_authority() {
    let mut forged = binding(13, 12, "fd-object:/real/target");
    forged.causal_chain_digest = digest('f');
    let record = OpenObjectRecord::build(successful_open("$WORKSPACE/requested", 13), Some(forged))
        .expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_binding_must_follow_successful_syscall_exit() {
    let record = OpenObjectRecord::build(
        successful_open("$WORKSPACE/requested", 14),
        Some(binding(14, 11, "fd-object:/real/target")),
    )
    .expect("record");
    assert!(!record.is_success_authority());
}

#[test]
fn b1_path_toctou_does_not_define_success_object_identity() {
    let first = OpenObjectRecord::build(
        successful_open("$WORKSPACE/same-request", 15),
        Some(binding(15, 12, "fd-object:/real/A")),
    )
    .expect("first");
    let second = OpenObjectRecord::build(
        successful_open("$WORKSPACE/same-request", 15),
        Some(binding(15, 12, "fd-object:/real/B")),
    )
    .expect("second");
    let first_digest = match first.authority {
        OpenObjectAuthority::SuccessBounded { proof_digest } => proof_digest,
        _ => panic!("expected bounded success"),
    };
    let second_digest = match second.authority {
        OpenObjectAuthority::SuccessBounded { proof_digest } => proof_digest,
        _ => panic!("expected bounded success"),
    };
    assert_ne!(first_digest, second_digest);
}

#[test]
fn b1_failed_open_never_becomes_success_even_with_forged_binding() {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(actor.clone(), 20, "$WORKSPACE/missing");
    let failed = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 20,
                exit_sequence: 21,
                raw_return: -2,
            },
            ObservationHealth::healthy(),
        )
        .expect("failure evidence");
    assert!(matches!(failed.state, EvidenceState::FailureObserved { .. }));
    let record = OpenObjectRecord::build(failed, Some(binding(5, 22, "fd-object:/forged")))
        .expect("record");
    assert!(matches!(record.authority, OpenObjectAuthority::NotSuccessful));
}

#[test]
fn b1_lost_observation_never_becomes_success() {
    let actor = actor(4242, 'c');
    let attempt = open_attempt(actor.clone(), 30, "$WORKSPACE/input");
    let lost = EvidenceLedger::default()
        .classify_pair(
            attempt,
            ExitEvidence {
                actor,
                originating_entry_sequence: 30,
                exit_sequence: 31,
                raw_return: 6,
            },
            ObservationHealth {
                complete: false,
                warning_codes: BTreeSet::from(["resource_truncation".to_owned()]),
            },
        )
        .expect("lost evidence");
    let record = OpenObjectRecord::build(lost, Some(binding(6, 32, "fd-object:/target")))
        .expect("record");
    assert!(matches!(record.authority, OpenObjectAuthority::Lost { .. }));
}

#[test]
fn b1_binding_and_proof_serialization_are_deterministic() {
    let first = OpenObjectRecord::build(
        successful_open("$WORKSPACE/input", 16),
        Some(binding(16, 12, "fd-object:/target")),
    )
    .expect("first");
    let second = OpenObjectRecord::build(
        successful_open("$WORKSPACE/input", 16),
        Some(binding(16, 12, "fd-object:/target")),
    )
    .expect("second");
    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}
