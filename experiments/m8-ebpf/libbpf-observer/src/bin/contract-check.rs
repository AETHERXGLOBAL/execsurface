use std::collections::BTreeSet;
use std::error::Error;
use std::fs;

use execsurface_observe::{experimental_ebpf_backend_descriptor, ObservationCapability};
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: contract-check <collector-report.json>")?;
    let report: Value = serde_json::from_slice(&fs::read(path)?)?;
    let descriptor = experimental_ebpf_backend_descriptor();
    descriptor.validate_capability_partition()?;

    expect_string(&report, "/backend/id", &descriptor.id)?;
    expect_string(&report, "/backend/platform", &descriptor.platform)?;
    expect_string(&report, "/backend/architecture", &descriptor.architecture)?;
    expect_string(
        &report,
        "/backend/privacy_profile",
        &descriptor.privacy_profile,
    )?;

    let actual_supported = string_set(&report, "/backend/capabilities")?;
    let expected_supported = descriptor
        .capabilities
        .iter()
        .map(|capability| serialized_capability(*capability).to_owned())
        .collect::<BTreeSet<_>>();
    if actual_supported != expected_supported {
        return Err(format!(
            "supported capability drift: actual={actual_supported:?} expected={expected_supported:?}"
        )
        .into());
    }

    let actual_unsupported = string_set(&report, "/backend/unsupported_capabilities")?;
    let expected_unsupported = descriptor
        .unsupported_capabilities
        .iter()
        .map(|capability| serialized_capability(*capability).to_owned())
        .collect::<BTreeSet<_>>();
    if actual_unsupported != expected_unsupported {
        return Err(format!(
            "unsupported capability drift: actual={actual_unsupported:?} expected={expected_unsupported:?}"
        )
        .into());
    }

    if report
        .pointer("/observation_complete")
        .and_then(Value::as_bool)
        != Some(false)
    {
        return Err(
            "experimental eBPF report unexpectedly claims observation_complete=true".into(),
        );
    }

    let completeness = report
        .pointer("/completeness")
        .and_then(Value::as_str)
        .ok_or("collector report is missing completeness")?;
    if completeness == "complete" {
        return Err("M8.3c is not authorized to emit completeness=complete".into());
    }

    println!(
        "M8_3C_CENTRAL_CONTRACT_PASS id={} completeness={} supported={} unsupported={}",
        descriptor.id,
        completeness,
        actual_supported.len(),
        actual_unsupported.len()
    );
    Ok(())
}

fn expect_string(report: &Value, pointer: &str, expected: &str) -> Result<(), Box<dyn Error>> {
    let actual = report
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("collector report is missing string at {pointer}"))?;
    if actual != expected {
        return Err(format!(
            "contract drift at {pointer}: actual={actual:?} expected={expected:?}"
        )
        .into());
    }
    Ok(())
}

fn string_set(report: &Value, pointer: &str) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let values = report
        .pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("collector report is missing array at {pointer}"))?;
    let mut set = BTreeSet::new();
    for value in values {
        let item = value
            .as_str()
            .ok_or_else(|| format!("non-string capability at {pointer}"))?;
        if !set.insert(item.to_owned()) {
            return Err(format!("duplicate capability {item:?} at {pointer}").into());
        }
    }
    Ok(set)
}

fn serialized_capability(capability: ObservationCapability) -> &'static str {
    match capability {
        ObservationCapability::ProcessSpawnLineage => "process_spawn_lineage",
        ObservationCapability::ProcessExecOccurrence => "process_exec_occurrence",
        ObservationCapability::ProcessExecPathIdentity => {
            "unconditional_process_exec_path_identity"
        }
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
