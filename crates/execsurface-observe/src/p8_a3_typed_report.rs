use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

use execsurface_model::{FileOperation, RawEvent, RawEventKind};
use serde_json::{json, Value};

use super::*;

const REPORT_SCHEMA: &str = "execsurface-research-typed-report-v0";
const RECOGNIZED_BACKEND: &str = "linux-ptrace-metadata-v2";
const RECOGNIZED_PRIVACY_PROFILE: &str = "metadata-only-v1";

fn capability_name(capability: ObservationCapability) -> &'static str {
    match capability {
        ObservationCapability::ProcessSpawnLineage => "process_spawn_lineage",
        ObservationCapability::ProcessExecOccurrence => "process_exec_occurrence",
        ObservationCapability::ProcessExecPathIdentity => "process_exec_path_identity",
        ObservationCapability::ProcessExit => "process_exit",
        ObservationCapability::PathAccessIntent => "path_access_intent",
        ObservationCapability::SuccessfulOpenFdIdentity => "successful_open_fd_identity",
        ObservationCapability::OpenPathIdentity => "open_path_identity",
        ObservationCapability::FdReadWriteEffect => "fd_read_write_effect",
        ObservationCapability::FdDupCloseLifecycle => "fd_dup_close_lifecycle",
        ObservationCapability::ForkFdInheritance => "fork_fd_inheritance",
        ObservationCapability::CloseOnExec => "close_on_exec",
        ObservationCapability::RenameDeleteEffects => "rename_delete_effects",
        ObservationCapability::NetworkConnectDestination => "network_connect_destination",
        ObservationCapability::TraceTimeRelativePath => "trace_time_relative_path",
        ObservationCapability::CausalExecutableChain => "causal_executable_chain",
        ObservationCapability::LossTruncationVisibility => "loss_truncation_visibility",
    }
}

fn health_name(completeness: CollectionCompleteness) -> &'static str {
    match completeness {
        CollectionCompleteness::Complete => "complete",
        CollectionCompleteness::IncompleteLoss => "incomplete_loss",
        CollectionCompleteness::IncompleteLimit => "incomplete_limit",
        CollectionCompleteness::IncompleteCapability => "incomplete_capability",
        CollectionCompleteness::IncompleteAmbiguity => "incomplete_ambiguity",
        CollectionCompleteness::Error => "error",
    }
}

fn validate_internal_boundary(input: &BackendObservation) -> Result<(), &'static str> {
    input
        .descriptor
        .validate_capability_partition()
        .map_err(|_| "invalid-capability-partition")?;

    if input.descriptor.id != RECOGNIZED_BACKEND {
        return Err("unknown-backend-profile");
    }
    if input.descriptor.platform != "linux" || input.descriptor.architecture != "x86_64" {
        return Err("unsupported-platform-profile");
    }
    if input.descriptor.privacy_profile != RECOGNIZED_PRIVACY_PROFILE {
        return Err("privacy-profile-mismatch");
    }
    if input.observation.schema_version != 2 {
        return Err("unsupported-raw-schema");
    }
    if input.observation.backend.name != input.descriptor.id
        || input.observation.backend.platform != input.descriptor.platform
        || input.observation.backend.architecture != input.descriptor.architecture
    {
        return Err("descriptor-raw-profile-mismatch");
    }

    let raw_has_warnings = !input.observation.warnings.is_empty();
    if input.completeness == CollectionCompleteness::Complete
        && (!input.observation.complete || raw_has_warnings)
    {
        return Err("health-laundering");
    }
    if input.completeness != CollectionCompleteness::Complete
        && input.observation.complete
        && !raw_has_warnings
    {
        return Err("health-envelope-raw-mismatch");
    }

    Ok(())
}

fn prior_exec_path(events: &[RawEvent], write: &RawEvent) -> Option<String> {
    events
        .iter()
        .filter(|event| event.tid == write.tid && event.sequence < write.sequence)
        .filter_map(|event| match &event.kind {
            RawEventKind::ProcessExec { path } => Some((event.sequence, path.clone())),
            _ => None,
        })
        .max_by_key(|(sequence, _)| *sequence)
        .map(|(_, path)| path)
}

fn build_typed_report(input: &BackendObservation) -> Result<Value, &'static str> {
    validate_internal_boundary(input)?;

    let fd_effect_capable = input
        .descriptor
        .capabilities
        .contains(&ObservationCapability::FdReadWriteEffect);

    let mut effects = Vec::new();
    for event in &input.observation.events {
        let RawEventKind::FileDescriptorAccess {
            operation: FileOperation::Write,
            path,
            ..
        } = &event.kind
        else {
            continue;
        };

        if !fd_effect_capable {
            return Err("fd-effect-capability-mismatch");
        }

        let actor = prior_exec_path(&input.observation.events, event)
            .ok_or("write-process-binding-missing")?;

        effects.push((
            event.sequence,
            json!({
                "proposition": "file_fd_write_effect_observed",
                "actor": actor,
                "target": path,
                "guarantees": {
                    "observation_points": [
                        "derived_runtime_fd_state",
                        "syscall_result_post_operation"
                    ],
                    "identity_bases": ["runtime_fd_path_correlated"],
                    "temporal_bindings": ["successful_operation_result"],
                    "causal_bindings": ["state_machine_correlated"]
                }
            }),
        ));
    }
    effects.sort_by_key(|(sequence, _)| *sequence);

    let mut warning_codes = BTreeSet::new();
    for warning in &input.observation.warnings {
        warning_codes.insert(warning.code.clone());
    }

    let mut unsupported: Vec<_> = input
        .descriptor
        .unsupported_capabilities
        .iter()
        .copied()
        .map(capability_name)
        .collect();
    unsupported.sort_unstable();

    let mut limitations = input.observation.backend.limitations.clone();
    limitations.sort();
    limitations.dedup();

    Ok(json!({
        "schema": REPORT_SCHEMA,
        "backend": {
            "profile": input.descriptor.id,
            "implementation_version": input.descriptor.implementation_version,
            "platform": input.descriptor.platform,
            "architecture": input.descriptor.architecture,
            "privacy_profile": input.descriptor.privacy_profile
        },
        "collection_health": {
            "state": health_name(input.completeness),
            "pass_eligible": input.completeness.pass_eligible()
                && input.observation.complete
                && warning_codes.is_empty(),
            "warning_codes": warning_codes.into_iter().collect::<Vec<_>>()
        },
        "unsupported_capabilities": unsupported,
        "effects": effects.into_iter().map(|(_, value)| value).collect::<Vec<_>>(),
        "limitations": limitations,
        "does_not_assert": [
            "exact_transferred_byte_count",
            "file_contents",
            "before_after_state_roots",
            "custody",
            "trusted_time",
            "signature_as_behavioral_authority",
            "causal_source_code_provenance"
        ]
    }))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use execsurface_model::{
        BackendMetadata, CommandOutcome, ObserverWarning, RawEvent, RawEventKind,
    };

    use super::*;

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_path(label: &str) -> PathBuf {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "execsurface-p8-a3-report-{label}-{}-{id}",
            std::process::id()
        ))
    }

    fn raw_backend(descriptor: &BackendDescriptor) -> BackendMetadata {
        BackendMetadata {
            name: descriptor.id.clone(),
            platform: descriptor.platform.clone(),
            architecture: descriptor.architecture.clone(),
            capabilities: vec!["fd_read_write_attribution".to_owned()],
            limitations: vec!["metadata-only controlled research input".to_owned()],
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

    fn write_event(sequence: u64, tid: i32, path: &str) -> RawEvent {
        RawEvent {
            sequence,
            tid,
            kind: RawEventKind::FileDescriptorAccess {
                operation: FileOperation::Write,
                fd: 3,
                path: path.to_owned(),
            },
        }
    }

    fn synthetic_complete(path: &str) -> BackendObservation {
        let descriptor = reference_backend_descriptor();
        BackendObservation {
            observation: Observation {
                schema_version: 2,
                backend: raw_backend(&descriptor),
                complete: true,
                outcome: CommandOutcome {
                    exit_code: Some(0),
                    signal: None,
                },
                events: vec![
                    exec_event(1, 7, "/usr/bin/python3"),
                    write_event(2, 7, path),
                ],
                warnings: vec![],
            },
            descriptor,
            completeness: CollectionCompleteness::Complete,
        }
    }

    fn collect_object_keys(value: &Value, keys: &mut BTreeSet<String>) {
        match value {
            Value::Object(map) => {
                for (key, nested) in map {
                    keys.insert(key.clone());
                    collect_object_keys(nested, keys);
                }
            }
            Value::Array(values) => {
                for nested in values {
                    collect_object_keys(nested, keys);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn p8_a3_r0_r1_report_is_deterministic_and_bounded() {
        let input = synthetic_complete("/tmp/target.bin");
        let first = build_typed_report(&input).expect("first report");
        let second = build_typed_report(&input).expect("second report");
        assert_eq!(
            serde_json::to_string(&first).expect("serialize first"),
            serde_json::to_string(&second).expect("serialize second")
        );
        assert_eq!(first["schema"], REPORT_SCHEMA);
        assert_eq!(first["collection_health"]["state"], "complete");
        assert_eq!(first["collection_health"]["pass_eligible"], true);
        assert_eq!(first["effects"].as_array().expect("effects").len(), 1);

        let mut object_keys = BTreeSet::new();
        collect_object_keys(&first, &mut object_keys);
        for forbidden in [
            "bytes_transferred",
            "byte_count",
            "preStateDigest",
            "postStateDigest",
            "beforeRoot",
            "afterRoot",
        ] {
            assert!(
                !object_keys.contains(forbidden),
                "must not invent field {forbidden}"
            );
        }
    }

    #[test]
    fn p8_a3_r1_incomplete_ambiguity_is_typed_and_not_pass_eligible() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input.observation.complete = false;
        input.observation.warnings.push(ObserverWarning {
            code: "shared_fd_table_ambiguity".to_owned(),
            tid: None,
            message: "controlled ambiguity".to_owned(),
        });
        input.completeness = CollectionCompleteness::IncompleteAmbiguity;

        let report = build_typed_report(&input).expect("bounded incomplete report");
        assert_eq!(report["collection_health"]["state"], "incomplete_ambiguity");
        assert_eq!(report["collection_health"]["pass_eligible"], false);
        assert_eq!(
            report["collection_health"]["warning_codes"][0],
            "shared_fd_table_ambiguity"
        );
    }

    #[test]
    fn p8_a3_r1_unsupported_fd_effect_capability_is_explicit() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input.observation.events.clear();
        input
            .descriptor
            .capabilities
            .retain(|capability| *capability != ObservationCapability::FdReadWriteEffect);
        input
            .descriptor
            .unsupported_capabilities
            .push(ObservationCapability::FdReadWriteEffect);

        let report = build_typed_report(&input).expect("unsupported report");
        assert!(report["unsupported_capabilities"]
            .as_array()
            .expect("unsupported")
            .iter()
            .any(|value| value == "fd_read_write_effect"));
        assert!(report["effects"].as_array().expect("effects").is_empty());
    }

    #[test]
    fn p8_a3_r2_warning_or_incomplete_state_cannot_be_laundered() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input.observation.complete = false;
        input.observation.warnings.push(ObserverWarning {
            code: "event_limit_exceeded".to_owned(),
            tid: None,
            message: "controlled loss".to_owned(),
        });
        input.completeness = CollectionCompleteness::Complete;

        assert_eq!(build_typed_report(&input), Err("health-laundering"));
    }

    #[test]
    fn p8_a3_r2_backend_name_cannot_upgrade_authority() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input.descriptor.id = "trusted-super-backend".to_owned();
        input.observation.backend.name = input.descriptor.id.clone();

        assert_eq!(build_typed_report(&input), Err("unknown-backend-profile"));
    }

    #[test]
    fn p8_a3_r2_missing_capability_rejects_conflicting_fd_effect() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input
            .descriptor
            .capabilities
            .retain(|capability| *capability != ObservationCapability::FdReadWriteEffect);
        input
            .descriptor
            .unsupported_capabilities
            .push(ObservationCapability::FdReadWriteEffect);

        assert_eq!(
            build_typed_report(&input),
            Err("fd-effect-capability-mismatch")
        );
    }

    #[test]
    fn p8_a3_r2_process_binding_mismatch_is_rejected() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input.observation.events = vec![
            exec_event(1, 8, "/usr/bin/python3"),
            write_event(2, 7, "/tmp/target.bin"),
        ];

        assert_eq!(
            build_typed_report(&input),
            Err("write-process-binding-missing")
        );
    }

    #[test]
    fn p8_a3_r2_repeated_effects_remain_distinct() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input
            .observation
            .events
            .push(write_event(3, 7, "/tmp/target.bin"));

        let report = build_typed_report(&input).expect("report");
        assert_eq!(report["effects"].as_array().expect("effects").len(), 2);
    }

    #[test]
    fn p8_a3_r2_profile_mismatch_is_rejected() {
        let mut input = synthetic_complete("/tmp/target.bin");
        input.descriptor.privacy_profile = "metadata-plus-content".to_owned();

        assert_eq!(build_typed_report(&input), Err("privacy-profile-mismatch"));
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[test]
    fn p8_a3_r3_live_native_boundary_preserves_typed_health_and_write_effect() {
        let target = temp_path("live-target");
        let command = format!("printf bridge > {}", target.display());
        let observed = PTRACE_BACKEND
            .observe(
                &CommandSpec::new("/bin/sh").arg("-c").arg(command),
                ObserveOptions::default(),
            )
            .expect("native ptrace observation");

        let report = build_typed_report(&observed).expect("typed report");
        assert_eq!(report["collection_health"]["state"], "complete");
        assert_eq!(report["collection_health"]["pass_eligible"], true);

        let effects = report["effects"].as_array().expect("effects");
        assert!(effects.iter().any(|effect| {
            effect["proposition"] == "file_fd_write_effect_observed"
                && effect["target"] == target.to_string_lossy().as_ref()
                && effect["guarantees"]["temporal_bindings"][0] == "successful_operation_result"
        }));

        let _ = fs::remove_file(target);
    }
}
