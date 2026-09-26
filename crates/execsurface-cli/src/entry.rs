use std::collections::BTreeSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use execsurface_observe::{
    experimental_ebpf_backend_descriptor, observe_command, CommandSpec, ObservationCapability,
};
use serde_json::Value;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

mod legacy {
    include!("main.rs");

    pub(super) fn run_legacy_entry() -> std::process::ExitCode {
        main()
    }
}

const EXPERIMENTAL_BACKEND: &str = "experimental-libbpf";
const EXPECTED_PROTOCOL_VERSION: u64 = 1;

fn main() -> ExitCode {
    let args = env::args_os().collect::<Vec<_>>();
    match maybe_run_explicit_observe(&args) {
        Ok(Some(())) => ExitCode::SUCCESS,
        Ok(None) => legacy::run_legacy_entry(),
        Err(message) => {
            eprintln!("ExecSurface: ERROR");
            eprintln!("execsurface: {message}");
            ExitCode::from(2)
        }
    }
}

fn maybe_run_explicit_observe(args: &[OsString]) -> Result<Option<()>, String> {
    if args.get(1).is_none_or(|arg| arg != "observe")
        || args.get(2).is_none_or(|arg| arg != "--backend")
    {
        return Ok(None);
    }

    let parsed = parse_explicit_observe(&args[2..])?;
    match parsed.backend.as_str() {
        "ptrace" => {
            if parsed.collector.is_some() {
                return Err(
                    "--collector is only valid with --backend experimental-libbpf".to_owned(),
                );
            }
            run_explicit_ptrace(parsed.target)?;
        }
        EXPERIMENTAL_BACKEND => {
            let collector = parsed.collector.ok_or_else(|| {
                "--backend experimental-libbpf requires an explicit --collector PATH".to_owned()
            })?;
            run_experimental_libbpf(&collector, &parsed.target)?;
        }
        other => {
            return Err(format!(
                "unsupported observe backend `{other}`; supported explicit backends are `ptrace` and `{EXPERIMENTAL_BACKEND}`"
            ));
        }
    }

    Ok(Some(()))
}

struct ExplicitObserveArgs {
    backend: String,
    collector: Option<PathBuf>,
    target: Vec<OsString>,
}

fn parse_explicit_observe(args: &[OsString]) -> Result<ExplicitObserveArgs, String> {
    let backend = args
        .get(1)
        .ok_or_else(|| "--backend requires a value".to_owned())?
        .to_string_lossy()
        .into_owned();
    let mut collector = None;
    let mut index = 2usize;

    while index < args.len() {
        if args[index] == "--" {
            let target = args[index + 1..].to_vec();
            if target.is_empty() {
                return Err("missing target command after `--`".to_owned());
            }
            return Ok(ExplicitObserveArgs {
                backend,
                collector,
                target,
            });
        }

        if args[index] == "--collector" {
            if collector.is_some() {
                return Err("duplicate --collector option".to_owned());
            }
            let path = args
                .get(index + 1)
                .ok_or_else(|| "--collector requires a path".to_owned())?;
            collector = Some(PathBuf::from(path));
            index += 2;
            continue;
        }

        return Err(format!(
            "unknown explicit observe option: {}",
            args[index].to_string_lossy()
        ));
    }

    Err("expected `--` before the target command".to_owned())
}

fn run_explicit_ptrace(target: Vec<OsString>) -> Result<(), String> {
    let program = target
        .first()
        .cloned()
        .ok_or_else(|| "missing target command".to_owned())?;
    let spec = CommandSpec::new(program).args(target.into_iter().skip(1));
    let observation = observe_command(&spec).map_err(|error| error.to_string())?;
    let json = serde_json::to_string_pretty(&observation)
        .map_err(|error| format!("cannot serialize observation: {error}"))?;
    println!("{json}");
    Ok(())
}

fn run_experimental_libbpf(collector: &Path, target: &[OsString]) -> Result<(), String> {
    if target.is_empty() {
        return Err("missing target command".to_owned());
    }

    let temp = PrivateReportDir::create()?;
    let status = Command::new(collector)
        .arg("--report")
        .arg(temp.report_path())
        .arg("--")
        .arg(&target[0])
        .args(&target[1..])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| {
            format!(
                "cannot launch experimental libbpf collector {}: {error}; no ptrace fallback attempted",
                collector.display()
            )
        })?;

    if !status.success() {
        return Err(format!(
            "experimental libbpf collector {} failed with status {status}; no ptrace fallback attempted",
            collector.display()
        ));
    }

    let bytes = fs::read(temp.report_path()).map_err(|error| {
        format!(
            "experimental libbpf collector completed without a readable report: {error}; no ptrace fallback attempted"
        )
    })?;
    let report: Value = serde_json::from_slice(&bytes).map_err(|error| {
        format!("experimental libbpf collector report is invalid JSON: {error}")
    })?;
    validate_experimental_report(&report)?;

    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| format!("cannot serialize validated experimental report: {error}"))?;
    println!("{json}");
    Ok(())
}

struct PrivateReportDir {
    dir: PathBuf,
    report: PathBuf,
}

impl PrivateReportDir {
    fn create() -> Result<Self, String> {
        let base = env::temp_dir();
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock is before UNIX epoch: {error}"))?
            .as_nanos();

        for attempt in 0..32u32 {
            let dir = base.join(format!("execsurface-m8-3c2-{pid}-{nanos}-{attempt}"));
            match fs::create_dir(&dir) {
                Ok(()) => {
                    #[cfg(unix)]
                    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).map_err(
                        |error| {
                            let _ = fs::remove_dir(&dir);
                            format!("cannot restrict experimental report directory: {error}")
                        },
                    )?;
                    let report = dir.join("collector-report.json");
                    return Ok(Self { dir, report });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(format!(
                        "cannot create private experimental report directory: {error}"
                    ));
                }
            }
        }

        Err("cannot allocate a unique private experimental report directory".to_owned())
    }

    fn report_path(&self) -> &Path {
        &self.report
    }
}

impl Drop for PrivateReportDir {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.report);
        let _ = fs::remove_dir(&self.dir);
    }
}

fn validate_experimental_report(report: &Value) -> Result<(), String> {
    let protocol = report
        .get("protocol_version")
        .and_then(Value::as_u64)
        .ok_or_else(|| "experimental report is missing numeric protocol_version".to_owned())?;
    if protocol != EXPECTED_PROTOCOL_VERSION {
        return Err(format!(
            "unsupported experimental collector protocol version {protocol}; expected {EXPECTED_PROTOCOL_VERSION}"
        ));
    }

    let descriptor = experimental_ebpf_backend_descriptor();
    descriptor
        .validate_capability_partition()
        .map_err(|error| format!("central experimental backend contract is invalid: {error}"))?;

    expect_string(report, "/backend/id", &descriptor.id)?;
    expect_string(report, "/backend/platform", &descriptor.platform)?;
    expect_string(report, "/backend/architecture", &descriptor.architecture)?;
    expect_string(
        report,
        "/backend/privacy_profile",
        &descriptor.privacy_profile,
    )?;

    let actual_supported = string_set(report, "/backend/capabilities")?;
    let expected_supported = descriptor
        .capabilities
        .iter()
        .map(|capability| capability_wire_name(*capability).to_owned())
        .collect::<BTreeSet<_>>();
    if actual_supported != expected_supported {
        return Err(format!(
            "experimental collector supported-capability drift: actual={actual_supported:?} expected={expected_supported:?}"
        ));
    }

    let actual_unsupported = string_set(report, "/backend/unsupported_capabilities")?;
    let expected_unsupported = descriptor
        .unsupported_capabilities
        .iter()
        .map(|capability| capability_wire_name(*capability).to_owned())
        .collect::<BTreeSet<_>>();
    if actual_unsupported != expected_unsupported {
        return Err(format!(
            "experimental collector unsupported-capability drift: actual={actual_unsupported:?} expected={expected_unsupported:?}"
        ));
    }

    if report.get("observation_complete").and_then(Value::as_bool) != Some(false) {
        return Err(
            "experimental collector report must declare observation_complete=false until a later authority gate"
                .to_owned(),
        );
    }

    let completeness = report
        .get("completeness")
        .and_then(Value::as_str)
        .ok_or_else(|| "experimental report is missing completeness".to_owned())?;
    if !matches!(
        completeness,
        "incomplete_capability" | "incomplete_loss" | "incomplete_limit"
    ) {
        return Err(format!(
            "experimental collector completeness `{completeness}` is not authorized by M8.3c2"
        ));
    }

    let root_pid = report
        .get("root_pid")
        .and_then(Value::as_u64)
        .ok_or_else(|| "experimental report is missing root_pid".to_owned())?;
    if root_pid == 0 {
        return Err("experimental report contains zero root_pid".to_owned());
    }

    let dropped = report
        .get("dropped_events")
        .and_then(Value::as_u64)
        .ok_or_else(|| "experimental report is missing dropped_events".to_owned())?;
    if dropped > 0 && completeness != "incomplete_loss" {
        return Err(format!(
            "experimental report records {dropped} dropped events without incomplete_loss"
        ));
    }

    if !report.get("events").is_some_and(Value::is_array) {
        return Err("experimental report is missing events array".to_owned());
    }
    if !report.get("warnings").is_some_and(Value::is_array) {
        return Err("experimental report is missing warnings array".to_owned());
    }
    if !report.get("outcome").is_some_and(Value::is_object) {
        return Err("experimental report is missing outcome object".to_owned());
    }

    Ok(())
}

fn expect_string(report: &Value, pointer: &str, expected: &str) -> Result<(), String> {
    let actual = report
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("experimental report is missing string at {pointer}"))?;
    if actual != expected {
        return Err(format!(
            "experimental collector contract drift at {pointer}: actual={actual:?} expected={expected:?}"
        ));
    }
    Ok(())
}

fn string_set(report: &Value, pointer: &str) -> Result<BTreeSet<String>, String> {
    let values = report
        .pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("experimental report is missing array at {pointer}"))?;
    let mut set = BTreeSet::new();
    for value in values {
        let item = value
            .as_str()
            .ok_or_else(|| format!("non-string capability at {pointer}"))?;
        if !set.insert(item.to_owned()) {
            return Err(format!("duplicate capability {item:?} at {pointer}"));
        }
    }
    Ok(set)
}

fn capability_wire_name(capability: ObservationCapability) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn os(value: &str) -> OsString {
        OsString::from(value)
    }

    fn valid_report() -> Value {
        let descriptor = experimental_ebpf_backend_descriptor();
        let capabilities = descriptor
            .capabilities
            .iter()
            .map(|capability| capability_wire_name(*capability))
            .collect::<Vec<_>>();
        let unsupported = descriptor
            .unsupported_capabilities
            .iter()
            .map(|capability| capability_wire_name(*capability))
            .collect::<Vec<_>>();
        json!({
            "protocol_version": 1,
            "backend": {
                "id": descriptor.id,
                "implementation_version": "test",
                "platform": descriptor.platform,
                "architecture": descriptor.architecture,
                "kernel_release": "test",
                "kernel_btf_readable": true,
                "privacy_profile": descriptor.privacy_profile,
                "capabilities": capabilities,
                "unsupported_capabilities": unsupported
            },
            "completeness": "incomplete_capability",
            "observation_complete": false,
            "root_pid": 42,
            "outcome": {"exit_code": 0, "signal": null},
            "dropped_events": 0,
            "events": [],
            "warnings": []
        })
    }

    #[test]
    fn experimental_parser_requires_explicit_collector() {
        let args = vec![
            os("--backend"),
            os(EXPERIMENTAL_BACKEND),
            os("--"),
            os("/bin/true"),
        ];
        let parsed = parse_explicit_observe(&args).expect("parse");
        assert_eq!(parsed.backend, EXPERIMENTAL_BACKEND);
        assert!(parsed.collector.is_none());
    }

    #[test]
    fn parser_accepts_explicit_collector_and_target() {
        let args = vec![
            os("--backend"),
            os(EXPERIMENTAL_BACKEND),
            os("--collector"),
            os("/tmp/collector"),
            os("--"),
            os("/bin/true"),
            os("arg"),
        ];
        let parsed = parse_explicit_observe(&args).expect("parse");
        assert_eq!(parsed.collector, Some(PathBuf::from("/tmp/collector")));
        assert_eq!(parsed.target, vec![os("/bin/true"), os("arg")]);
    }

    #[test]
    fn central_contract_accepts_incomplete_experimental_report() {
        validate_experimental_report(&valid_report()).expect("valid report");
    }

    #[test]
    fn report_cannot_claim_complete_authority() {
        let mut report = valid_report();
        report["observation_complete"] = Value::Bool(true);
        assert!(validate_experimental_report(&report).is_err());

        let mut report = valid_report();
        report["completeness"] = Value::String("complete".to_owned());
        assert!(validate_experimental_report(&report).is_err());
    }

    #[test]
    fn report_rejects_backend_or_capability_drift() {
        let mut report = valid_report();
        report["backend"]["id"] = Value::String("fake-backend".to_owned());
        assert!(validate_experimental_report(&report).is_err());

        let mut report = valid_report();
        report["backend"]["capabilities"] = json!(["process_exec_occurrence"]);
        assert!(validate_experimental_report(&report).is_err());
    }

    #[test]
    fn dropped_events_require_loss_completeness() {
        let mut report = valid_report();
        report["dropped_events"] = json!(2);
        assert!(validate_experimental_report(&report).is_err());
        report["completeness"] = Value::String("incomplete_loss".to_owned());
        validate_experimental_report(&report).expect("loss report");
    }
}
