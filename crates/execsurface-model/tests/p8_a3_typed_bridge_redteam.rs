use std::collections::BTreeSet;

use execsurface_model::canonical::{CanonicalExecutable, CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::{
    BackendMetadata, CommandOutcome, FileOperation, Observation, ObserverWarning, RawEvent,
    RawEventKind,
};

const ALPHA5_RELEASE_SOURCE: &str = "9e73b925d55557e33de1b0813995609aaefdc037";
const ALPHA5_BINARY_SHA256: &str =
    "11d1f70d3e6bd6526eff4889b90cba4a0ad95ce09d489643e7f3b703e455f646";

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProducerBinding {
    release_source: &'static str,
    binary_sha256: &'static str,
}

fn alpha5_binding() -> ProducerBinding {
    ProducerBinding {
        release_source: ALPHA5_RELEASE_SOURCE,
        binary_sha256: ALPHA5_BINARY_SHA256,
    }
}

fn backend() -> BackendMetadata {
    BackendMetadata {
        name: "linux-ptrace-metadata-v2".to_owned(),
        platform: "linux".to_owned(),
        architecture: "x86_64".to_owned(),
        capabilities: vec![
            "fd_read_write_attribution".to_owned(),
            "descendant_tracking".to_owned(),
        ],
        limitations: vec![
            "file open/create/delete/rename records are syscall attempts; fd read/write records require successful positive-byte I/O".to_owned(),
        ],
    }
}

fn base_observation() -> Observation {
    Observation {
        schema_version: 2,
        backend: backend(),
        complete: true,
        outcome: CommandOutcome {
            exit_code: Some(0),
            signal: None,
        },
        events: vec![],
        warnings: vec![],
    }
}

fn exec_event(sequence: u64, tid: i32, path: &str) -> RawEvent {
    RawEvent {
        sequence,
        tid,
        kind: RawEventKind::ProcessExec {
            path: path.to_owned(),
        },
    }
}

fn write_event(sequence: u64, tid: i32, fd: i32, path: &str) -> RawEvent {
    RawEvent {
        sequence,
        tid,
        kind: RawEventKind::FileDescriptorAccess {
            operation: FileOperation::Write,
            fd,
            path: path.to_owned(),
        },
    }
}

fn canonical_path(path: &str) -> CanonicalPath {
    CanonicalPath {
        value: path.to_owned(),
        class: PathClass::Unknown,
        resolution: PathResolution::KernelFdResolved,
    }
}

fn actor(path: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: CanonicalPath {
            value: path.to_owned(),
            class: PathClass::Unknown,
            resolution: PathResolution::Lexical,
        },
        family: "raw-v2-exec".to_owned(),
    }
}

fn bridge_fd_writes(
    observation: &Observation,
    binding: &ProducerBinding,
) -> Result<Vec<ProofCarryingObservation>, &'static str> {
    if binding != &alpha5_binding() {
        return Err("producer-identity-mismatch");
    }
    if observation.schema_version != 2 {
        return Err("unsupported-raw-schema");
    }
    if observation.backend.name != "linux-ptrace-metadata-v2"
        || observation.backend.platform != "linux"
        || observation.backend.architecture != "x86_64"
    {
        return Err("backend-profile-mismatch");
    }
    if !observation
        .backend
        .capabilities
        .iter()
        .any(|capability| capability == "fd_read_write_attribution")
    {
        return Err("fd-effect-capability-missing");
    }
    if !observation.complete {
        return Err("raw-observation-incomplete");
    }
    if !observation.warnings.is_empty() {
        return Err("raw-observation-warning-present");
    }

    let mut result = Vec::new();
    for event in &observation.events {
        let RawEventKind::FileDescriptorAccess {
            operation: FileOperation::Write,
            path,
            ..
        } = &event.kind
        else {
            continue;
        };

        let prior_exec = observation
            .events
            .iter()
            .filter(|candidate| candidate.tid == event.tid && candidate.sequence < event.sequence)
            .filter_map(|candidate| match &candidate.kind {
                RawEventKind::ProcessExec { path } => Some((candidate.sequence, path.as_str())),
                _ => None,
            })
            .max_by_key(|(sequence, _)| *sequence)
            .ok_or("write-process-binding-missing")?;

        let proposition = Proposition::FileFdEffectObserved {
            actor: Some(actor(prior_exec.1)),
            execution_chain: vec![actor(prior_exec.1)],
            operation: FileOperation::Write,
            target: canonical_path(path),
        };
        let guarantees = EvidenceGuarantees {
            observation_points: BTreeSet::from([
                ObservationPoint::DerivedRuntimeFdState,
                ObservationPoint::SyscallResultPostOperation,
            ]),
            identity_bases: BTreeSet::from([IdentityBasis::RuntimeFdPathCorrelated]),
            temporal_bindings: BTreeSet::from([TemporalBinding::SuccessfulOperationResult]),
            causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
        };

        let mut record = ProofCarryingObservation::new(
            proposition,
            guarantees,
            BackendSemanticProfile {
                name: "linux-ptrace-alpha5-typed-bridge-research".to_owned(),
                semantic_profile_version: 1,
            },
        );
        record.completeness.insert(
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        );
        record.completeness.insert(
            CompletenessDimension::Capability,
            CompletenessState::Complete,
        );
        result.push(record);
    }

    Ok(result)
}

fn requirement_for(path: &str) -> ProofRequirement {
    ProofRequirement::for_proposition(
        Proposition::FileFdEffectObserved {
            actor: Some(actor("/usr/bin/python3")),
            execution_chain: vec![actor("/usr/bin/python3")],
            operation: FileOperation::Write,
            target: canonical_path(path),
        },
        EvidenceGuarantees {
            observation_points: BTreeSet::from([
                ObservationPoint::DerivedRuntimeFdState,
                ObservationPoint::SyscallResultPostOperation,
            ]),
            identity_bases: BTreeSet::from([IdentityBasis::RuntimeFdPathCorrelated]),
            temporal_bindings: BTreeSet::from([TemporalBinding::SuccessfulOperationResult]),
            causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
        },
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Capability,
        ]),
    )
}

fn positive_write(path: &str) -> Observation {
    let mut observation = base_observation();
    observation.events = vec![
        exec_event(1, 7, "/usr/bin/python3"),
        write_event(2, 7, 3, path),
    ];
    observation
}

#[test]
fn g3_01_positive_control_maps_only_bounded_success_semantics() {
    let records =
        bridge_fd_writes(&positive_write("/tmp/target.bin"), &alpha5_binding()).expect("map");
    assert_eq!(records.len(), 1);
    assert!(records[0].satisfies(&requirement_for("/tmp/target.bin")));

    let json = serde_json::to_string(&records[0]).expect("serialize proof");
    for forbidden in [
        "bytes_transferred",
        "byte_count",
        "beforeRoot",
        "afterRoot",
        "stateRoot",
        "contentSha256",
    ] {
        assert!(
            !json.contains(forbidden),
            "typed bridge must not invent {forbidden}"
        );
    }
}

#[test]
fn g3_02_failed_or_zero_byte_write_without_fd_effect_cannot_be_promoted() {
    let mut observation = base_observation();
    observation.events = vec![exec_event(1, 7, "/usr/bin/python3")];

    let records = bridge_fd_writes(&observation, &alpha5_binding()).expect("map");
    assert!(records.is_empty());
}

#[test]
fn g3_03_short_positive_write_can_only_claim_positive_effect_not_exact_size() {
    // Raw v2 intentionally does not retain the positive byte count. A short
    // positive write therefore maps to the same bounded proposition: one
    // successful positive-byte fd write effect, never an exact-length claim.
    let records =
        bridge_fd_writes(&positive_write("/tmp/short.bin"), &alpha5_binding()).expect("map");
    assert_eq!(records.len(), 1);
    assert!(matches!(
        records[0].proposition,
        Proposition::FileFdEffectObserved { .. }
    ));
    let value = serde_json::to_value(&records[0]).expect("serialize proof");
    assert!(value.get("bytes_transferred").is_none());
    assert!(value.get("byte_count").is_none());
}

#[test]
fn g3_04_repeated_writes_are_not_collapsed_into_one_claim() {
    let mut observation = base_observation();
    observation.events = vec![
        exec_event(1, 7, "/usr/bin/python3"),
        write_event(2, 7, 3, "/tmp/target.bin"),
        write_event(3, 7, 3, "/tmp/target.bin"),
    ];

    let records = bridge_fd_writes(&observation, &alpha5_binding()).expect("map");
    assert_eq!(records.len(), 2);
}

#[test]
fn g3_05_warning_blocks_typed_promotion_even_when_target_exit_is_zero() {
    let mut observation = positive_write("/tmp/target.bin");
    observation.complete = false;
    observation.warnings.push(ObserverWarning {
        code: "controlled_warning".to_owned(),
        tid: Some(7),
        message: "controlled red-team warning".to_owned(),
    });

    assert_eq!(
        bridge_fd_writes(&observation, &alpha5_binding()),
        Err("raw-observation-incomplete")
    );
}

#[test]
fn g3_06_truncation_or_incomplete_observation_is_fail_closed() {
    let mut observation = positive_write("/tmp/target.bin");
    observation.complete = false;
    observation.warnings.push(ObserverWarning {
        code: "event_limit_exceeded".to_owned(),
        tid: None,
        message: "controlled truncation".to_owned(),
    });

    assert!(bridge_fd_writes(&observation, &alpha5_binding()).is_err());
}

#[test]
fn g3_07_path_substitution_does_not_satisfy_bound_requirement() {
    let records =
        bridge_fd_writes(&positive_write("/tmp/other.bin"), &alpha5_binding()).expect("map");
    assert_eq!(records.len(), 1);
    assert!(!records[0].satisfies(&requirement_for("/tmp/target.bin")));
}

#[test]
fn g3_08_process_binding_mismatch_is_rejected() {
    let mut observation = base_observation();
    observation.events = vec![
        exec_event(1, 8, "/usr/bin/python3"),
        write_event(2, 7, 3, "/tmp/target.bin"),
    ];

    assert_eq!(
        bridge_fd_writes(&observation, &alpha5_binding()),
        Err("write-process-binding-missing")
    );
}

#[test]
fn g3_09_shared_fd_ambiguity_cannot_be_laundered_into_complete_health() {
    let mut observation = positive_write("/tmp/target.bin");
    observation.complete = false;
    observation.warnings.push(ObserverWarning {
        code: "shared_fd_table_ambiguity".to_owned(),
        tid: None,
        message: "controlled shared-fd ambiguity".to_owned(),
    });

    assert_eq!(
        bridge_fd_writes(&observation, &alpha5_binding()),
        Err("raw-observation-incomplete")
    );
}

#[test]
fn g3_10_binary_or_source_identity_mismatch_is_rejected() {
    let wrong_source = ProducerBinding {
        release_source: "wrong-source",
        binary_sha256: ALPHA5_BINARY_SHA256,
    };
    let wrong_binary = ProducerBinding {
        release_source: ALPHA5_RELEASE_SOURCE,
        binary_sha256: "wrong-binary",
    };
    let observation = positive_write("/tmp/target.bin");

    assert_eq!(
        bridge_fd_writes(&observation, &wrong_source),
        Err("producer-identity-mismatch")
    );
    assert_eq!(
        bridge_fd_writes(&observation, &wrong_binary),
        Err("producer-identity-mismatch")
    );
}

#[test]
fn g3_11_backend_name_cannot_upgrade_authority() {
    let mut observation = positive_write("/tmp/target.bin");
    observation.backend.name = "trusted-super-backend".to_owned();

    assert_eq!(
        bridge_fd_writes(&observation, &alpha5_binding()),
        Err("backend-profile-mismatch")
    );
}

#[test]
fn g3_12_external_snapshots_cannot_change_or_strengthen_bridge_output() {
    let observation = positive_write("/tmp/target.bin");
    let first = bridge_fd_writes(&observation, &alpha5_binding()).expect("map");
    let second = bridge_fd_writes(&observation, &alpha5_binding()).expect("map");

    // Deliberately distinct untrusted external snapshot material is not an
    // input to the bridge and therefore cannot affect the proof record.
    let external_snapshot_a = b"before";
    let external_snapshot_b = b"substituted-after";
    assert_ne!(
        external_snapshot_a.as_slice(),
        external_snapshot_b.as_slice()
    );
    assert_eq!(first, second);

    let json = serde_json::to_string(&first).expect("serialize proof");
    assert!(!json.contains("preStateDigest"));
    assert!(!json.contains("postStateDigest"));
}

#[test]
fn g3_13_warning_list_cannot_be_ignored_when_complete_is_true() {
    let mut observation = positive_write("/tmp/target.bin");
    observation.warnings.push(ObserverWarning {
        code: "unexpected_health_signal".to_owned(),
        tid: Some(7),
        message: "complete=true with warning must still fail closed in bridge".to_owned(),
    });

    assert_eq!(
        bridge_fd_writes(&observation, &alpha5_binding()),
        Err("raw-observation-warning-present")
    );
}
