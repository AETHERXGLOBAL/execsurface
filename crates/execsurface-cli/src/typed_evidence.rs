use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use execsurface_model::{FileOperation, RawEvent, RawEventKind};
use execsurface_observe::{BackendObservation, CollectionCompleteness, ObservationCapability};
use serde_json::{json, Value};

const EVIDENCE_SCHEMA_VERSION: u32 = 1;
const REPORT_KIND: &str = "typed_observation_evidence";
const STABILITY: &str = "experimental";
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

fn validate_boundary(input: &BackendObservation) -> Result<(), String> {
    input
        .descriptor
        .validate_capability_partition()
        .map_err(|error| format!("invalid backend capability partition: {error}"))?;

    if input.descriptor.id != RECOGNIZED_BACKEND {
        return Err(format!(
            "typed evidence does not recognize backend profile: {}",
            input.descriptor.id
        ));
    }
    if input.descriptor.platform != "linux" || input.descriptor.architecture != "x86_64" {
        return Err("typed evidence supports only the selected Linux x86_64 profile".to_owned());
    }
    if input.descriptor.privacy_profile != RECOGNIZED_PRIVACY_PROFILE {
        return Err("typed evidence privacy-profile mismatch".to_owned());
    }
    if input.observation.schema_version != 2 {
        return Err(format!(
            "typed evidence requires raw observation schema 2, got {}",
            input.observation.schema_version
        ));
    }
    if input.observation.backend.name != input.descriptor.id
        || input.observation.backend.platform != input.descriptor.platform
        || input.observation.backend.architecture != input.descriptor.architecture
    {
        return Err("typed evidence descriptor/raw backend mismatch".to_owned());
    }

    let raw_has_warnings = !input.observation.warnings.is_empty();
    if input.completeness == CollectionCompleteness::Complete
        && (!input.observation.complete || raw_has_warnings)
    {
        return Err("typed evidence refused collection-health laundering".to_owned());
    }
    if input.completeness != CollectionCompleteness::Complete
        && input.observation.complete
        && !raw_has_warnings
    {
        return Err("typed evidence completeness envelope/raw mismatch".to_owned());
    }

    Ok(())
}

fn is_filesystem_kernel_fd_path(path: &str) -> bool {
    path.starts_with('/')
}

fn prior_exec_path(events: &[RawEvent], effect: &RawEvent) -> Option<String> {
    events
        .iter()
        .filter(|event| event.tid == effect.tid && event.sequence < effect.sequence)
        .filter_map(|event| match &event.kind {
            RawEventKind::ProcessExec { path } => Some((event.sequence, path.clone())),
            _ => None,
        })
        .max_by_key(|(sequence, _)| *sequence)
        .map(|(_, path)| path)
}

pub(crate) fn build_report(input: &BackendObservation) -> Result<Value, String> {
    validate_boundary(input)?;

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

        if !is_filesystem_kernel_fd_path(path) {
            continue;
        }

        if !fd_effect_capable {
            return Err(
                "typed evidence found fd-write evidence without fd-effect capability".to_owned(),
            );
        }

        let actor = prior_exec_path(&input.observation.events, event).ok_or_else(|| {
            "typed evidence cannot bind fd-write effect to an executable".to_owned()
        })?;

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

    let warning_codes: BTreeSet<_> = input
        .observation
        .warnings
        .iter()
        .map(|warning| warning.code.clone())
        .collect();

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
        "schema_version": EVIDENCE_SCHEMA_VERSION,
        "report_kind": REPORT_KIND,
        "stability": STABILITY,
        "backend": {
            "profile": input.descriptor.id,
            "implementation_version": input.descriptor.implementation_version,
            "platform": input.descriptor.platform,
            "architecture": input.descriptor.architecture,
            "privacy_profile": input.descriptor.privacy_profile
        },
        "collection_health": {
            "state": health_name(input.completeness),
            "warning_codes": warning_codes.into_iter().collect::<Vec<_>>()
        },
        "unsupported_capabilities": unsupported,
        "effects": effects.into_iter().map(|(_, value)| value).collect::<Vec<_>>(),
        "limitations": limitations,
        "does_not_assert": [
            "policy_verdict",
            "vulnerability_free",
            "exact_transferred_byte_count",
            "file_contents",
            "before_after_state_roots",
            "custody",
            "trusted_time",
            "signature_as_behavioral_authority",
            "causal_source_code_provenance"
        ],
        "raw_observation": &input.observation
    }))
}

fn lexical_absolute(path: &Path) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| format!("cannot resolve current directory: {error}"))?
            .join(path)
    };

    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::Normal(value) => normalized.push(value),
        }
    }
    Ok(normalized)
}

fn path_identity(path: &Path) -> Result<PathBuf, String> {
    if path.exists() {
        return std::fs::canonicalize(path)
            .map_err(|error| format!("cannot resolve path {}: {error}", path.display()));
    }

    let absolute = lexical_absolute(path)?;
    let parent = absolute.parent().unwrap_or_else(|| Path::new("/"));
    let resolved_parent = std::fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
    let name = absolute
        .file_name()
        .ok_or_else(|| format!("evidence output path has no file name: {}", path.display()))?;
    Ok(resolved_parent.join(name))
}

fn ensure_output_path_disjoint(path: &Path, input: &BackendObservation) -> Result<(), String> {
    let output = path_identity(path)?;

    for event in &input.observation.events {
        let RawEventKind::FileDescriptorAccess {
            operation: FileOperation::Write,
            path: observed_path,
            ..
        } = &event.kind
        else {
            continue;
        };

        if !is_filesystem_kernel_fd_path(observed_path) {
            continue;
        }

        let observed = path_identity(Path::new(observed_path))?;
        if observed == output {
            return Err(format!(
                "evidence output path overlaps observed workload path: {}",
                path.display()
            ));
        }
    }

    Ok(())
}

pub(crate) fn write_report(path: &Path, input: &BackendObservation) -> Result<(), String> {
    ensure_output_path_disjoint(path, input)?;
    let report = build_report(input)?;
    let mut bytes = serde_json::to_vec_pretty(&report)
        .map_err(|error| format!("cannot serialize typed evidence report: {error}"))?;
    bytes.push(b'\n');
    std::fs::write(path, bytes).map_err(|error| {
        format!(
            "cannot write typed evidence report {}: {error}",
            path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use execsurface_model::{
        BackendMetadata, CommandOutcome, Observation, ObserverWarning, RawEvent, SpawnMechanism,
    };
    use execsurface_observe::{reference_backend_descriptor, BackendDescriptor};

    use super::*;

    fn raw_backend(descriptor: &BackendDescriptor) -> BackendMetadata {
        BackendMetadata {
            name: descriptor.id.clone(),
            platform: descriptor.platform.clone(),
            architecture: descriptor.architecture.clone(),
            capabilities: vec!["fd_read_write_attribution".to_owned()],
            limitations: vec!["metadata-only test input".to_owned()],
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

    fn complete_input(path: &str) -> BackendObservation {
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
    fn evidence_envelope_contains_same_raw_object_and_typed_effect() {
        let input = complete_input("/tmp/target.bin");
        let report = build_report(&input).expect("report");
        assert_eq!(
            report["raw_observation"],
            serde_json::to_value(&input.observation).expect("raw value")
        );
        assert_eq!(
            report["effects"][0]["target"],
            report["raw_observation"]["events"][1]["path"]
        );
        assert_eq!(
            report["effects"][0]["guarantees"]["temporal_bindings"][0],
            "successful_operation_result"
        );
    }

    #[test]
    fn report_is_deterministic_for_identical_input() {
        let input = complete_input("/tmp/target.bin");
        let first = build_report(&input).expect("first");
        let second = build_report(&input).expect("second");
        assert_eq!(
            serde_json::to_vec(&first).expect("first bytes"),
            serde_json::to_vec(&second).expect("second bytes")
        );
    }

    #[test]
    fn incomplete_ambiguity_is_preserved_not_laundered() {
        let mut input = complete_input("/tmp/target.bin");
        input.observation.complete = false;
        input.observation.warnings.push(ObserverWarning {
            code: "shared_fd_table_ambiguity".to_owned(),
            tid: None,
            message: "controlled".to_owned(),
        });
        input.completeness = CollectionCompleteness::IncompleteAmbiguity;
        let report = build_report(&input).expect("report");
        assert_eq!(report["collection_health"]["state"], "incomplete_ambiguity");
    }

    #[test]
    fn kernel_pseudo_fd_write_is_retained_raw_but_not_promoted_to_file_effect() {
        let input = complete_input("pipe:[12345]");
        let report = build_report(&input).expect("report");

        assert!(report["effects"].as_array().expect("effects").is_empty());
        assert_eq!(
            report["raw_observation"]["events"][1]["kind"]["path"],
            "pipe:[12345]"
        );
    }

    #[test]
    fn complete_envelope_cannot_launder_raw_warning() {
        let mut input = complete_input("/tmp/target.bin");
        input.observation.warnings.push(ObserverWarning {
            code: "controlled_warning".to_owned(),
            tid: None,
            message: "controlled".to_owned(),
        });
        assert!(build_report(&input).is_err());
    }

    #[test]
    fn backend_name_cannot_upgrade_authority() {
        let mut input = complete_input("/tmp/target.bin");
        input.descriptor.id = "trusted-super-backend".to_owned();
        input.observation.backend.name = input.descriptor.id.clone();
        assert!(build_report(&input).is_err());
    }

    #[test]
    fn missing_fd_capability_rejects_retained_write() {
        let mut input = complete_input("/tmp/target.bin");
        input
            .descriptor
            .capabilities
            .retain(|capability| *capability != ObservationCapability::FdReadWriteEffect);
        input
            .descriptor
            .unsupported_capabilities
            .push(ObservationCapability::FdReadWriteEffect);
        assert!(build_report(&input).is_err());
    }

    #[test]
    fn process_binding_mismatch_is_rejected() {
        let mut input = complete_input("/tmp/target.bin");
        input.observation.events = vec![
            exec_event(1, 8, "/usr/bin/python3"),
            write_event(2, 7, "/tmp/target.bin"),
        ];
        assert!(build_report(&input).is_err());
    }

    #[test]
    fn repeated_write_effects_remain_distinct() {
        let mut input = complete_input("/tmp/target.bin");
        input
            .observation
            .events
            .push(write_event(3, 7, "/tmp/target.bin"));
        let report = build_report(&input).expect("report");
        assert_eq!(report["effects"].as_array().expect("effects").len(), 2);
    }

    #[test]
    fn forbidden_claims_are_not_fields() {
        let report = build_report(&complete_input("/tmp/target.bin")).expect("report");
        let mut keys = BTreeSet::new();
        collect_object_keys(&report, &mut keys);
        for forbidden in [
            "verdict",
            "bytes_transferred",
            "byte_count",
            "beforeRoot",
            "afterRoot",
            "preStateDigest",
            "postStateDigest",
        ] {
            assert!(!keys.contains(forbidden), "forbidden field {forbidden}");
        }
    }

    #[test]
    fn workload_written_output_path_is_rejected_without_overwrite() {
        let path = std::env::temp_dir().join(format!(
            "execsurface-p8-a3-collision-{}",
            std::process::id()
        ));
        std::fs::write(&path, b"WORKLOAD-STATE").expect("seed workload bytes");
        let input = complete_input(path.to_string_lossy().as_ref());

        let error = write_report(&path, &input).expect_err("collision must fail");
        assert!(error.contains("evidence output path overlaps observed workload path"));
        assert_eq!(
            std::fs::read(&path).expect("read preserved workload bytes"),
            b"WORKLOAD-STATE"
        );
        let _ = std::fs::remove_file(path);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_alias_to_workload_written_path_is_rejected() {
        use std::os::unix::fs::symlink;

        let root = std::env::temp_dir().join(format!(
            "execsurface-p8-a3-collision-alias-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("create collision dir");
        let target = root.join("workload.bin");
        let alias = root.join("evidence-alias.json");
        std::fs::write(&target, b"WORKLOAD-STATE").expect("seed workload bytes");
        symlink(&target, &alias).expect("create alias");

        let input = complete_input(target.to_string_lossy().as_ref());
        let error = write_report(&alias, &input).expect_err("alias collision must fail");
        assert!(error.contains("evidence output path overlaps observed workload path"));
        assert_eq!(
            std::fs::read(&target).expect("read preserved workload bytes"),
            b"WORKLOAD-STATE"
        );

        let _ = std::fs::remove_file(alias);
        let _ = std::fs::remove_file(target);
        let _ = std::fs::remove_dir(root);
    }

    #[test]
    fn output_path_failure_is_explicit() {
        let root = std::env::temp_dir().join(format!(
            "execsurface-p8-a3-output-dir-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("temp dir");
        let result = write_report(&root, &complete_input("/tmp/target.bin"));
        assert!(result.is_err());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn unrelated_spawn_does_not_create_a_write_effect() {
        let mut input = complete_input("/tmp/target.bin");
        input.observation.events.push(RawEvent {
            sequence: 3,
            tid: 7,
            kind: RawEventKind::ProcessSpawn {
                child_tid: 9,
                mechanism: SpawnMechanism::Fork,
            },
        });
        let report = build_report(&input).expect("report");
        assert_eq!(report["effects"].as_array().expect("effects").len(), 1);
    }
}
