pub use execsurface_p4_backend_authority::{
    completeness_map, AdapterRecord, AuthorityState, EvidenceReference, PropositionCompleteness,
};

#[path = "../src/ptrace_v2.rs"]
mod ptrace_v2;

use execsurface_model::semantics_v3::Proposition;
use execsurface_model::{
    BackendMetadata, CommandOutcome, FileOperation, Observation, ObserverWarning, RawEvent,
    RawEventKind, SpawnMechanism, RAW_OBSERVATION_SCHEMA_VERSION,
};
use execsurface_normalize::NormalizationConfig;
use ptrace_v2::{map_ptrace_v2, PtraceMapping};

fn backend() -> BackendMetadata {
    BackendMetadata {
        name: "linux-ptrace-metadata-v2".to_owned(),
        platform: "linux".to_owned(),
        architecture: "x86_64".to_owned(),
        capabilities: vec!["test".to_owned()],
        limitations: vec![],
    }
}

fn observation(events: Vec<RawEvent>) -> Observation {
    Observation {
        schema_version: RAW_OBSERVATION_SCHEMA_VERSION,
        backend: backend(),
        complete: true,
        outcome: CommandOutcome {
            exit_code: Some(0),
            signal: None,
        },
        events,
        warnings: vec![],
    }
}

fn event(sequence: u64, tid: i32, kind: RawEventKind) -> RawEvent {
    RawEvent {
        sequence,
        tid,
        kind,
    }
}

fn mapped(events: Vec<RawEvent>) -> PtraceMapping {
    map_ptrace_v2(&observation(events), &NormalizationConfig::default()).expect("map ptrace")
}

fn lineage_record(mapping: &PtraceMapping) -> &AdapterRecord {
    mapping
        .records
        .iter()
        .find(|record| record.proposition_id == "P4.CAUSAL.EXEC_LINEAGE")
        .expect("lineage record")
}

fn fd_record(mapping: &PtraceMapping) -> &AdapterRecord {
    mapping
        .records
        .iter()
        .find(|record| record.proposition_id == "P4.FD.IO_ATTRIBUTION")
        .expect("fd record")
}

#[test]
fn a1_3_path_toctou_cannot_launder_attempt_into_object_authority() {
    let mapping = mapped(vec![event(
        1,
        10,
        RawEventKind::FilePathAccess {
            operation: FileOperation::Open,
            path: "/tmp/same-name".to_owned(),
            flags: Some(0),
        },
    )]);

    assert!(mapping.capability_gaps.contains("P4.FILE.OPEN_OBJECT"));
    assert!(mapping.records.iter().any(|record| {
        record.proposition_id == "P4.PATH.ACCESS_ATTEMPT"
            && record.authority == AuthorityState::AttemptOnly
    }));
    assert!(!mapping.records.iter().any(|record| {
        record.proposition_id == "P4.FILE.OPEN_OBJECT"
            || matches!(
                record.proof.proposition,
                Proposition::FileOpenObjectObserved { .. }
            )
    }));
}

#[test]
fn a1_3_missing_successful_open_evidence_remains_capability_gap_not_negative_proof() {
    let mapping = mapped(vec![]);

    assert!(mapping.capability_gaps.contains("P4.FILE.OPEN_OBJECT"));
    assert!(!mapping
        .records
        .iter()
        .any(|record| record.proposition_id == "P4.FILE.OPEN_OBJECT"));
}

#[test]
fn a1_3_no_success_exec_event_cannot_create_exec_success() {
    let mapping = mapped(vec![event(
        1,
        10,
        RawEventKind::FilePathAccess {
            operation: FileOperation::Open,
            path: "/bin/true".to_owned(),
            flags: Some(0),
        },
    )]);

    assert!(!mapping
        .records
        .iter()
        .any(|record| record.proposition_id == "P4.EXEC.SUCCESS"));
}

#[test]
fn a1_3_unknown_clone_relation_blocks_dependent_fd_authority() {
    let mapping = mapped(vec![
        event(
            1,
            10,
            RawEventKind::ProcessSpawn {
                child_tid: 11,
                mechanism: SpawnMechanism::Clone,
            },
        ),
        event(
            2,
            10,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Read,
                fd: 3,
                path: "/tmp/input".to_owned(),
            },
        ),
    ]);

    let relation = mapping
        .records
        .iter()
        .find(|record| record.proposition_id == "P4.FDTABLE.RELATION")
        .expect("fd-table relation");
    assert_eq!(relation.authority, AuthorityState::Ambiguous);
    assert_eq!(relation.completeness, PropositionCompleteness::Incomplete);

    let io = fd_record(&mapping);
    assert_eq!(io.authority, AuthorityState::Ambiguous);
    assert_eq!(io.completeness, PropositionCompleteness::Incomplete);
}

#[test]
fn a1_3_observer_loss_stops_effect_authority() {
    let mut input = observation(vec![
        event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/usr/bin/cat".to_owned(),
            },
        ),
        event(
            2,
            10,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Read,
                fd: 3,
                path: "/tmp/input".to_owned(),
            },
        ),
    ]);
    input.complete = false;
    input.warnings.push(ObserverWarning {
        code: "resource_limit".to_owned(),
        tid: Some(10),
        message: "controlled adversarial loss".to_owned(),
    });

    let mapping = map_ptrace_v2(&input, &NormalizationConfig::default()).expect("map loss");
    assert_eq!(mapping.records.len(), 1);
    assert_eq!(mapping.records[0].proposition_id, "P4.OBSERVER.HEALTH_LOSS");
    assert_eq!(mapping.records[0].authority, AuthorityState::Lost);
    assert_eq!(
        mapping.records[0].completeness,
        PropositionCompleteness::Incomplete
    );
}

#[test]
fn a1_3_causal_chain_substitution_changes_proposition_identity() {
    let first = mapped(vec![
        event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/bin/bash".to_owned(),
            },
        ),
        event(
            2,
            10,
            RawEventKind::ProcessExec {
                path: "/usr/bin/cat".to_owned(),
            },
        ),
        event(
            3,
            10,
            RawEventKind::FilePathAccess {
                operation: FileOperation::Read,
                path: "/tmp/input".to_owned(),
                flags: None,
            },
        ),
    ]);
    let substituted = mapped(vec![
        event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/bin/sh".to_owned(),
            },
        ),
        event(
            2,
            10,
            RawEventKind::ProcessExec {
                path: "/usr/bin/cat".to_owned(),
            },
        ),
        event(
            3,
            10,
            RawEventKind::FilePathAccess {
                operation: FileOperation::Read,
                path: "/tmp/input".to_owned(),
                flags: None,
            },
        ),
    ]);

    let first_lineage = lineage_record(&first);
    let substituted_lineage = lineage_record(&substituted);
    assert_eq!(first_lineage.authority, AuthorityState::DerivedBounded);
    assert_eq!(
        substituted_lineage.authority,
        AuthorityState::DerivedBounded
    );
    assert_ne!(
        first_lineage.proof.proposition,
        substituted_lineage.proof.proposition
    );
}

#[test]
fn a1_3_wrong_actor_same_target_does_not_reuse_fd_proposition_identity() {
    let cat = mapped(vec![
        event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/usr/bin/cat".to_owned(),
            },
        ),
        event(
            2,
            10,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Read,
                fd: 3,
                path: "/tmp/input".to_owned(),
            },
        ),
    ]);
    let shell = mapped(vec![
        event(
            1,
            10,
            RawEventKind::ProcessExec {
                path: "/bin/sh".to_owned(),
            },
        ),
        event(
            2,
            10,
            RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Read,
                fd: 3,
                path: "/tmp/input".to_owned(),
            },
        ),
    ]);

    let cat_record = fd_record(&cat);
    let shell_record = fd_record(&shell);
    assert_eq!(cat_record.authority, AuthorityState::Direct);
    assert_eq!(shell_record.authority, AuthorityState::Direct);
    assert_ne!(cat_record.proof.proposition, shell_record.proof.proposition);
}

#[test]
fn a1_3_unsupported_no_event_never_becomes_supported_negative_proof() {
    let mapping = mapped(vec![]);

    assert!(mapping.capability_gaps.contains("P4.FILE.OPEN_OBJECT"));
    assert!(!mapping.records.iter().any(|record| {
        record.proposition_id == "P4.FILE.OPEN_OBJECT"
            && record.authority == AuthorityState::Direct
            && record.completeness == PropositionCompleteness::Complete
    }));
}
