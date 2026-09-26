use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use execsurface_model::{Observation, RawEventKind, SpawnMechanism};
use execsurface_observe::{
    experimental_ebpf_backend_descriptor, reference_backend_descriptor, ObservationCapability,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ParityVerdict {
    Equivalent,
    ReferenceSubset,
    CandidateSubset,
    RepresentationDifference,
    NonComparable,
    Contradicted,
    BlockedIncomplete,
}

#[derive(Debug, Serialize)]
struct Assessment {
    semantic_class: String,
    verdict: ParityVerdict,
    detail: String,
}

#[derive(Debug, Serialize)]
struct ParityReport {
    schema_version: u32,
    reference_backend: String,
    candidate_backend: String,
    reference_complete: bool,
    candidate_completeness: String,
    candidate_observation_complete: bool,
    full_surface_comparable: bool,
    ebpf_pass_authorized: bool,
    assessments: Vec<Assessment>,
}

#[derive(Debug, Deserialize)]
struct CandidateReport {
    backend: CandidateBackend,
    completeness: String,
    observation_complete: bool,
    root_pid: Option<u32>,
    #[serde(default)]
    collector_failure: Option<serde_json::Value>,
    events: Vec<CandidateEvent>,
}

#[derive(Debug, Deserialize)]
struct CandidateBackend {
    id: String,
    capabilities: Vec<String>,
    unsupported_capabilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
enum CandidateEvent {
    ProcessSpawn {
        sequence: u64,
        parent_pid: u32,
        child_pid: u32,
        mechanism: String,
    },
    ProcessExecOccurrence {
        sequence: u64,
        pid: u32,
        path: Option<String>,
    },
    SuccessfulOpenIdentity {
        sequence: u64,
        pid: u32,
        fd: u32,
        path: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Projection {
    spawn_edges: Vec<String>,
    exec_roles: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let (reference_path, candidate_path, output_path) = parse_args()?;
    let reference: Observation = serde_json::from_slice(&fs::read(reference_path)?)?;
    let candidate: CandidateReport = serde_json::from_slice(&fs::read(candidate_path)?)?;

    let report = compare(&reference, &candidate)?;
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    fs::write(output_path, bytes)?;
    Ok(())
}

fn parse_args() -> Result<(PathBuf, PathBuf, PathBuf), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let reference = args
        .next()
        .map(PathBuf::from)
        .ok_or("usage: execsurface-m8-parity-harness REFERENCE_JSON CANDIDATE_JSON OUTPUT_JSON")?;
    let candidate = args
        .next()
        .map(PathBuf::from)
        .ok_or("missing candidate report path")?;
    let output = args
        .next()
        .map(PathBuf::from)
        .ok_or("missing output report path")?;
    if args.next().is_some() {
        return Err("unexpected extra parity-harness argument".into());
    }
    Ok((reference, candidate, output))
}

fn compare(reference: &Observation, candidate: &CandidateReport) -> Result<ParityReport, Box<dyn Error>> {
    let reference_descriptor = reference_backend_descriptor();
    let candidate_descriptor = experimental_ebpf_backend_descriptor();
    reference_descriptor.validate_capability_partition()?;
    candidate_descriptor.validate_capability_partition()?;

    if reference.backend.name != reference_descriptor.id {
        return Err(format!(
            "reference backend mismatch: report={} expected={}",
            reference.backend.name, reference_descriptor.id
        )
        .into());
    }
    if candidate.backend.id != candidate_descriptor.id {
        return Err(format!(
            "candidate backend mismatch: report={} expected={}",
            candidate.backend.id, candidate_descriptor.id
        )
        .into());
    }

    validate_candidate_capabilities(&candidate.backend, &candidate_descriptor.capabilities, &candidate_descriptor.unsupported_capabilities)?;

    let hard_blocker = !reference.complete || candidate_hard_blocked(candidate);
    let mut assessments = Vec::new();

    if hard_blocker {
        for capability in candidate_descriptor.capabilities.iter().copied() {
            assessments.push(Assessment {
                semantic_class: capability_name(capability).to_owned(),
                verdict: ParityVerdict::BlockedIncomplete,
                detail: format!(
                    "selected-class parity blocked by evidence health: reference_complete={} candidate_completeness={} collector_failure={}",
                    reference.complete,
                    candidate.completeness,
                    candidate.collector_failure.is_some()
                ),
            });
        }
    } else {
        let reference_projection = project_reference(reference)?;
        let candidate_projection = project_candidate(candidate)?;

        assessments.push(Assessment {
            semantic_class: capability_name(ObservationCapability::ProcessSpawnLineage).to_owned(),
            verdict: if reference_projection.spawn_edges == candidate_projection.spawn_edges {
                ParityVerdict::Equivalent
            } else {
                ParityVerdict::Contradicted
            },
            detail: format!(
                "reference_edges={:?}; candidate_edges={:?}",
                reference_projection.spawn_edges, candidate_projection.spawn_edges
            ),
        });

        assessments.push(Assessment {
            semantic_class: capability_name(ObservationCapability::ProcessExecOccurrence).to_owned(),
            verdict: if reference_projection.exec_roles == candidate_projection.exec_roles {
                ParityVerdict::Equivalent
            } else {
                ParityVerdict::Contradicted
            },
            detail: format!(
                "reference_exec_roles={:?}; candidate_exec_roles={:?}",
                reference_projection.exec_roles, candidate_projection.exec_roles
            ),
        });

        assessments.push(Assessment {
            semantic_class: capability_name(ObservationCapability::SuccessfulOpenFdIdentity).to_owned(),
            verdict: ParityVerdict::RepresentationDifference,
            detail: "ptrace proves successful open into its internal FD table but raw Observation v2 does not serialize a dedicated successful-open identity event; eBPF does, so raw evidence is not yet equivalent".to_owned(),
        });

        assessments.push(Assessment {
            semantic_class: capability_name(ObservationCapability::LossTruncationVisibility).to_owned(),
            verdict: ParityVerdict::RepresentationDifference,
            detail: "both backends expose fail-closed health semantics, but transport-specific loss accounting is not treated as event-level equivalence; controlled incomplete evidence is tested separately".to_owned(),
        });
    }

    for capability in candidate_descriptor.unsupported_capabilities.iter().copied() {
        assessments.push(Assessment {
            semantic_class: capability_name(capability).to_owned(),
            verdict: ParityVerdict::NonComparable,
            detail: "candidate backend explicitly declares this semantic class unsupported".to_owned(),
        });
    }

    assessments.sort_by(|a, b| a.semantic_class.cmp(&b.semantic_class));

    Ok(ParityReport {
        schema_version: 1,
        reference_backend: reference.backend.name.clone(),
        candidate_backend: candidate.backend.id.clone(),
        reference_complete: reference.complete,
        candidate_completeness: candidate.completeness.clone(),
        candidate_observation_complete: candidate.observation_complete,
        full_surface_comparable: false,
        ebpf_pass_authorized: false,
        assessments,
    })
}

fn validate_candidate_capabilities(
    backend: &CandidateBackend,
    expected_supported: &[ObservationCapability],
    expected_unsupported: &[ObservationCapability],
) -> Result<(), Box<dyn Error>> {
    let actual_supported = backend.capabilities.iter().cloned().collect::<BTreeSet<_>>();
    let actual_unsupported = backend
        .unsupported_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_supported = expected_supported
        .iter()
        .copied()
        .map(capability_name)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected_unsupported = expected_unsupported
        .iter()
        .copied()
        .map(capability_name)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    if actual_supported != expected_supported {
        return Err(format!(
            "candidate supported capability drift: actual={actual_supported:?} expected={expected_supported:?}"
        )
        .into());
    }
    if actual_unsupported != expected_unsupported {
        return Err(format!(
            "candidate unsupported capability drift: actual={actual_unsupported:?} expected={expected_unsupported:?}"
        )
        .into());
    }
    Ok(())
}

fn candidate_hard_blocked(candidate: &CandidateReport) -> bool {
    candidate.collector_failure.is_some()
        || matches!(
            candidate.completeness.as_str(),
            "incomplete_loss"
                | "incomplete_limit"
                | "incomplete_decode"
                | "incomplete_collector"
                | "incomplete_lifecycle"
        )
}

fn project_reference(reference: &Observation) -> Result<Projection, Box<dyn Error>> {
    let root_tid = reference
        .events
        .first()
        .map(|event| event.tid)
        .ok_or("reference report contains no events")?;
    let mut roles = BTreeMap::new();
    roles.insert(root_tid, "root".to_owned());
    let mut counters = BTreeMap::<(String, String), u32>::new();
    let mut spawn_edges = Vec::new();
    let mut exec_roles = Vec::new();

    for event in &reference.events {
        match &event.kind {
            RawEventKind::ProcessSpawn {
                child_tid,
                mechanism,
            } => {
                let parent_role = roles
                    .get(&event.tid)
                    .cloned()
                    .ok_or_else(|| format!("reference spawn parent {} has no structural role", event.tid))?;
                let mechanism = spawn_name(*mechanism).to_owned();
                let counter = counters
                    .entry((parent_role.clone(), mechanism.clone()))
                    .or_insert(0);
                *counter = counter.saturating_add(1);
                let child_role = format!("{parent_role}/{mechanism}#{}", *counter);
                roles.insert(*child_tid, child_role.clone());
                spawn_edges.push(format!("{parent_role}|{mechanism}|{child_role}"));
            }
            RawEventKind::ProcessExec { .. } => {
                let role = roles
                    .get(&event.tid)
                    .cloned()
                    .ok_or_else(|| format!("reference exec tid {} has no structural role", event.tid))?;
                exec_roles.push(role);
            }
            _ => {}
        }
    }

    Ok(Projection {
        spawn_edges,
        exec_roles,
    })
}

fn project_candidate(candidate: &CandidateReport) -> Result<Projection, Box<dyn Error>> {
    let root_pid = candidate.root_pid.ok_or("candidate report has no root_pid")?;
    let mut roles = BTreeMap::new();
    roles.insert(root_pid, "root".to_owned());
    let mut counters = BTreeMap::<(String, String), u32>::new();
    let mut spawn_edges = Vec::new();
    let mut exec_roles = Vec::new();

    let mut events = candidate.events.iter().collect::<Vec<_>>();
    events.sort_by_key(|event| candidate_sequence(event));

    for event in events {
        match event {
            CandidateEvent::ProcessSpawn {
                parent_pid,
                child_pid,
                mechanism,
                ..
            } => {
                let parent_role = roles
                    .get(parent_pid)
                    .cloned()
                    .ok_or_else(|| format!("candidate spawn parent {parent_pid} has no structural role"))?;
                let counter = counters
                    .entry((parent_role.clone(), mechanism.clone()))
                    .or_insert(0);
                *counter = counter.saturating_add(1);
                let child_role = format!("{parent_role}/{mechanism}#{}", *counter);
                roles.insert(*child_pid, child_role.clone());
                spawn_edges.push(format!("{parent_role}|{mechanism}|{child_role}"));
            }
            CandidateEvent::ProcessExecOccurrence { pid, path, .. } => {
                let _path_is_not_parity_evidence = path.as_deref();
                let role = roles
                    .get(pid)
                    .cloned()
                    .ok_or_else(|| format!("candidate exec pid {pid} has no structural role"))?;
                exec_roles.push(role);
            }
            CandidateEvent::SuccessfulOpenIdentity { pid, fd, path, .. } => {
                let _successful_open_metadata_is_not_raw_parity_yet = (pid, fd, path.as_deref());
            }
        }
    }

    Ok(Projection {
        spawn_edges,
        exec_roles,
    })
}

fn candidate_sequence(event: &CandidateEvent) -> u64 {
    match event {
        CandidateEvent::ProcessSpawn { sequence, .. }
        | CandidateEvent::ProcessExecOccurrence { sequence, .. }
        | CandidateEvent::SuccessfulOpenIdentity { sequence, .. } => *sequence,
    }
}

fn spawn_name(mechanism: SpawnMechanism) -> &'static str {
    match mechanism {
        SpawnMechanism::Fork => "fork",
        SpawnMechanism::Vfork => "vfork",
        SpawnMechanism::Clone => "clone",
    }
}

fn capability_name(capability: ObservationCapability) -> &'static str {
    match capability {
        ObservationCapability::ProcessSpawnLineage => "process_spawn_lineage",
        ObservationCapability::ProcessExecOccurrence => "process_exec_occurrence",
        ObservationCapability::ProcessExecPathIdentity => "unconditional_process_exec_path_identity",
        ObservationCapability::ProcessExit => "process_exit",
        ObservationCapability::PathAccessIntent => "path_access_intent",
        ObservationCapability::SuccessfulOpenFdIdentity => "successful_open_fd_identity",
        ObservationCapability::OpenPathIdentity => "unconditional_open_path_identity",
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

#[allow(dead_code)]
fn _retain_all_verdict_variants_for_schema_stability() -> [ParityVerdict; 7] {
    [
        ParityVerdict::Equivalent,
        ParityVerdict::ReferenceSubset,
        ParityVerdict::CandidateSubset,
        ParityVerdict::RepresentationDifference,
        ParityVerdict::NonComparable,
        ParityVerdict::Contradicted,
        ParityVerdict::BlockedIncomplete,
    ]
}
