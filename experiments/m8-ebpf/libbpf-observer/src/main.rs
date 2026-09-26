use std::cell::RefCell;
use std::collections::BTreeSet;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{MapCore, MapFlags, RingBufferBuilder};
use serde::Serialize;

#[cfg(unix)]
use std::os::unix::process::{CommandExt, ExitStatusExt};

mod observer {
    include!(concat!(env!("OUT_DIR"), "/observer.skel.rs"));
}

use observer::*;

const EVENT_EXEC: u32 = 1;
const EVENT_SPAWN: u32 = 2;
const EVENT_OPEN: u32 = 3;
const EVENT_EXIT: u32 = 4;
const SPAWN_FORK: u32 = 1;
const SPAWN_VFORK: u32 = 2;
const SPAWN_CLONE: u32 = 3;
const POLL_INTERVAL_MS: u64 = 10;
const QUIESCENCE_GRACE_MS: u64 = 100;
const QUIESCENCE_POLLS: usize = 4;
const CONTROLLED_POST_START_FAILURE_DELAY_MS: u64 = 50;

#[derive(Debug, Clone, Copy)]
struct MetadataEvent {
    kind: u32,
    pid: u32,
    value: u32,
    reserved: u32,
}

#[derive(Debug)]
struct CollectorOptions {
    report_path: PathBuf,
    event_limit: usize,
    consumer_lag_ms: u64,
    lifecycle_timeout_ms: u64,
    inject_decode_error: bool,
    inject_poll_error: bool,
    target: Vec<OsString>,
}

#[derive(Debug, Serialize)]
struct CollectorReport {
    protocol_version: u32,
    backend: BackendReport,
    completeness: String,
    observation_complete: bool,
    root_pid: Option<u32>,
    outcome: OutcomeReport,
    dropped_events: u64,
    consumer_lag_ms: u64,
    lifecycle_timeout_ms: u64,
    lifecycle_drain_complete: bool,
    decode_failure_injected: bool,
    collector_failure: Option<CollectorFailureReport>,
    events: Vec<ReportEvent>,
    warnings: Vec<WarningReport>,
}

#[derive(Debug, Serialize)]
struct BackendReport {
    id: String,
    implementation_version: String,
    platform: String,
    architecture: String,
    kernel_release: Option<String>,
    kernel_btf_readable: bool,
    privacy_profile: String,
    capabilities: Vec<String>,
    unsupported_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct OutcomeReport {
    exit_code: Option<i32>,
    signal: Option<i32>,
}

#[derive(Debug, Serialize)]
struct CollectorFailureReport {
    stage: String,
    message: String,
    target_started: bool,
    target_terminated: bool,
}

#[derive(Debug, Serialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
enum ReportEvent {
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

#[derive(Debug, Serialize)]
struct WarningReport {
    code: String,
    pid: Option<u32>,
    message: String,
}

struct Tracker {
    root_pid: u32,
    known_pids: BTreeSet<u32>,
    active_pids: BTreeSet<u32>,
    pending: Vec<MetadataEvent>,
    events: Vec<ReportEvent>,
    warnings: Vec<WarningReport>,
    sequence: u64,
    event_limit: usize,
    event_limit_hit: bool,
    path_resolution_failed: bool,
    decode_failed: bool,
}

impl Tracker {
    fn new(event_limit: usize) -> Self {
        Self {
            root_pid: 0,
            known_pids: BTreeSet::new(),
            active_pids: BTreeSet::new(),
            pending: Vec::new(),
            events: Vec::new(),
            warnings: Vec::new(),
            sequence: 0,
            event_limit,
            event_limit_hit: false,
            path_resolution_failed: false,
            decode_failed: false,
        }
    }

    fn set_root(&mut self, pid: u32) {
        self.root_pid = pid;
        self.known_pids.insert(pid);
        self.active_pids.insert(pid);
        self.drain_pending_for(pid);
    }

    fn ingest(&mut self, event: MetadataEvent) {
        if self.known_pids.contains(&event.pid) {
            self.process_known(event);
        } else {
            self.pending.push(event);
        }
    }

    fn process_known(&mut self, event: MetadataEvent) {
        match event.kind {
            EVENT_SPAWN => {
                let child_pid = event.value;
                if child_pid == 0 {
                    self.warning(
                        Some(event.pid),
                        "invalid_spawn_identity",
                        "spawn event contained zero child pid",
                    );
                    return;
                }
                self.known_pids.insert(child_pid);
                self.active_pids.insert(child_pid);
                let mechanism = match event.reserved {
                    SPAWN_FORK => "fork",
                    SPAWN_VFORK => "vfork",
                    SPAWN_CLONE => "clone",
                    _ => "unknown",
                };
                let sequence = self.next_sequence();
                self.record(ReportEvent::ProcessSpawn {
                    sequence,
                    parent_pid: event.pid,
                    child_pid,
                    mechanism: mechanism.to_owned(),
                });
                self.drain_pending_for(child_pid);
            }
            EVENT_EXEC => {
                let path = resolve_link(format!("/proc/{}/exe", event.pid));
                if path.is_none() {
                    self.path_resolution_failed = true;
                    self.warning(
                        Some(event.pid),
                        "exec_path_resolution_unavailable",
                        "post-exec occurrence was observed but /proc executable identity was unavailable",
                    );
                }
                let sequence = self.next_sequence();
                self.record(ReportEvent::ProcessExecOccurrence {
                    sequence,
                    pid: event.pid,
                    path,
                });
            }
            EVENT_OPEN => {
                let path = resolve_link(format!("/proc/{}/fd/{}", event.pid, event.value));
                if path.is_none() {
                    self.path_resolution_failed = true;
                    self.warning(
                        Some(event.pid),
                        "open_path_resolution_unavailable",
                        format!(
                            "successful open returned fd {}, but live /proc fd identity was unavailable",
                            event.value
                        ),
                    );
                }
                let sequence = self.next_sequence();
                self.record(ReportEvent::SuccessfulOpenIdentity {
                    sequence,
                    pid: event.pid,
                    fd: event.value,
                    path,
                });
            }
            EVENT_EXIT => {
                self.active_pids.remove(&event.pid);
            }
            _ => self.warning(
                Some(event.pid),
                "unknown_event_kind",
                format!("unknown libbpf metadata event kind {}", event.kind),
            ),
        }
    }

    fn drain_pending_for(&mut self, pid: u32) {
        let mut retained = Vec::with_capacity(self.pending.len());
        let pending = std::mem::take(&mut self.pending);
        for event in pending {
            if event.pid == pid {
                self.process_known(event);
            } else {
                retained.push(event);
            }
        }
        self.pending = retained;
    }

    fn next_sequence(&mut self) -> u64 {
        self.sequence = self.sequence.saturating_add(1);
        self.sequence
    }

    fn record(&mut self, event: ReportEvent) {
        if self.events.len() >= self.event_limit {
            if !self.event_limit_hit {
                self.event_limit_hit = true;
                self.warning(
                    None,
                    "event_limit_exceeded",
                    format!(
                        "experimental libbpf event budget {} exceeded; evidence is truncated",
                        self.event_limit
                    ),
                );
            }
            return;
        }
        self.events.push(event);
    }

    fn warning(&mut self, pid: Option<u32>, code: &str, message: impl Into<String>) {
        self.warnings.push(WarningReport {
            code: code.to_owned(),
            pid,
            message: message.into(),
        });
    }

    fn record_decode_failure(&mut self, message: impl Into<String>) {
        self.decode_failed = true;
        self.warning(None, "event_decode_error", message);
    }

    fn active_descendant_count(&self) -> usize {
        self.active_pids
            .iter()
            .filter(|pid| **pid != self.root_pid)
            .count()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = parse_args()?;

    let builder = ObserverSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = match builder.open(&mut open_object) {
        Ok(value) => value,
        Err(error) => return fail_before_target(&options, "open", error.to_string()),
    };
    let mut skel = match open_skel.load() {
        Ok(value) => value,
        Err(error) => return fail_before_target(&options, "load", error.to_string()),
    };
    if let Err(error) = skel.attach() {
        return fail_before_target(&options, "attach", error.to_string());
    }

    let tracker = Rc::new(RefCell::new(Tracker::new(options.event_limit)));
    let callback_tracker = Rc::clone(&tracker);
    let mut ring_builder = RingBufferBuilder::new();
    if let Err(error) = ring_builder.add(&skel.maps.events, move |data| match decode_event(data) {
        Ok(event) => {
            callback_tracker.borrow_mut().ingest(event);
            0
        }
        Err(message) => {
            callback_tracker.borrow_mut().record_decode_failure(message);
            0
        }
    }) {
        return fail_before_target(&options, "ring_buffer_register", error.to_string());
    }
    let ring = match ring_builder.build() {
        Ok(value) => value,
        Err(error) => return fail_before_target(&options, "ring_buffer_build", error.to_string()),
    };

    if options.inject_decode_error {
        tracker.borrow_mut().warning(
            None,
            "decode_failure_injected",
            "M8.4c controlled malformed-event decode failure was injected",
        );
        tracker
            .borrow_mut()
            .record_decode_failure("controlled malformed metadata event for M8.4c");
    }

    let mut command = Command::new(&options.target[0]);
    command.args(&options.target[1..]);
    #[cfg(unix)]
    command.process_group(0);
    let mut child = match command.spawn() {
        Ok(value) => value,
        Err(error) => return fail_before_target(&options, "target_spawn", error.to_string()),
    };
    let root_pid = child.id();
    tracker.borrow_mut().set_root(root_pid);

    if options.inject_poll_error {
        thread::sleep(Duration::from_millis(
            CONTROLLED_POST_START_FAILURE_DELAY_MS,
        ));
        return fail_after_target(
            &options,
            "poll",
            "controlled M8.4d post-start poll failure".to_owned(),
            &mut child,
            root_pid,
            None,
            &tracker,
        );
    }

    if options.consumer_lag_ms > 0 {
        tracker.borrow_mut().warning(
            None,
            "consumer_lag_injected",
            format!(
                "M8.4 controlled consumer lag of {} ms was injected before ring-buffer polling",
                options.consumer_lag_ms
            ),
        );
        thread::sleep(Duration::from_millis(options.consumer_lag_ms));
    }

    let status = loop {
        if let Err(error) = ring.poll(Duration::from_millis(POLL_INTERVAL_MS)) {
            return fail_after_target(
                &options,
                "poll",
                error.to_string(),
                &mut child,
                root_pid,
                None,
                &tracker,
            );
        }
        if let Some(status) = child.try_wait()? {
            break status;
        }
    };
    let outcome = outcome_report(&status);

    let drain_started = Instant::now();
    let drain_timeout = Duration::from_millis(options.lifecycle_timeout_ms);
    let mut last_sequence = tracker.borrow().sequence;
    let mut idle_polls = 0usize;
    let mut lifecycle_drain_complete = false;

    loop {
        if drain_started.elapsed() >= drain_timeout {
            let active_descendants = tracker.borrow().active_descendant_count();
            tracker.borrow_mut().warning(
                None,
                "lifecycle_drain_timeout",
                format!(
                    "post-root-exit lifecycle drain timed out after {} ms with {} active known descendants",
                    options.lifecycle_timeout_ms, active_descendants
                ),
            );
            break;
        }

        if let Err(error) = ring.poll(Duration::from_millis(POLL_INTERVAL_MS)) {
            return fail_after_target(
                &options,
                "post_root_poll",
                error.to_string(),
                &mut child,
                root_pid,
                Some(outcome.clone()),
                &tracker,
            );
        }
        let (sequence, active_descendants) = {
            let state = tracker.borrow();
            (state.sequence, state.active_descendant_count())
        };

        if sequence == last_sequence {
            idle_polls = idle_polls.saturating_add(1);
        } else {
            idle_polls = 0;
            last_sequence = sequence;
        }

        if active_descendants == 0
            && drain_started.elapsed() >= Duration::from_millis(QUIESCENCE_GRACE_MS)
            && idle_polls >= QUIESCENCE_POLLS
        {
            lifecycle_drain_complete = true;
            break;
        }
    }

    let dropped_events = match total_drops(&skel.maps.dropped) {
        Ok(value) => value,
        Err(error) => {
            return fail_after_target(
                &options,
                "drop_counter_read",
                error.to_string(),
                &mut child,
                root_pid,
                Some(outcome.clone()),
                &tracker,
            )
        }
    };
    let mut state = tracker.borrow_mut();
    state.warning(
        None,
        "experimental_backend_partial_capability",
        "M8.4 libbpf evidence remains intentionally incomplete relative to the ptrace reference and is not PASS-authorized",
    );

    let completeness = if dropped_events > 0 {
        state.warning(
            None,
            "producer_event_loss",
            format!("libbpf producer drop counter reported {dropped_events} lost events"),
        );
        "incomplete_loss"
    } else if state.decode_failed {
        "incomplete_decode"
    } else if !lifecycle_drain_complete {
        "incomplete_lifecycle"
    } else if state.event_limit_hit {
        "incomplete_limit"
    } else {
        "incomplete_capability"
    };

    if state.path_resolution_failed {
        state.warning(
            None,
            "conditional_path_resolution",
            "one or more numeric events could not be promoted to path identity; no path was guessed",
        );
    }

    let report = CollectorReport {
        protocol_version: 1,
        backend: backend_report(),
        completeness: completeness.to_owned(),
        observation_complete: false,
        root_pid: Some(root_pid),
        outcome,
        dropped_events,
        consumer_lag_ms: options.consumer_lag_ms,
        lifecycle_timeout_ms: options.lifecycle_timeout_ms,
        lifecycle_drain_complete,
        decode_failure_injected: options.inject_decode_error,
        collector_failure: None,
        events: std::mem::take(&mut state.events),
        warnings: std::mem::take(&mut state.warnings),
    };
    drop(state);

    write_report(&options.report_path, &report)?;
    eprintln!(
        "M8_4_LIBBPF_OBSERVATION_PASS root_pid={:?} completeness={} events={} dropped={} lifecycle_drain_complete={}",
        report.root_pid,
        report.completeness,
        report.events.len(),
        report.dropped_events,
        report.lifecycle_drain_complete
    );
    Ok(())
}

fn fail_before_target(
    options: &CollectorOptions,
    stage: &str,
    message: String,
) -> Result<(), Box<dyn Error>> {
    let report = CollectorReport {
        protocol_version: 1,
        backend: backend_report(),
        completeness: "incomplete_collector".to_owned(),
        observation_complete: false,
        root_pid: None,
        outcome: OutcomeReport {
            exit_code: None,
            signal: None,
        },
        dropped_events: 0,
        consumer_lag_ms: options.consumer_lag_ms,
        lifecycle_timeout_ms: options.lifecycle_timeout_ms,
        lifecycle_drain_complete: false,
        decode_failure_injected: options.inject_decode_error,
        collector_failure: Some(CollectorFailureReport {
            stage: stage.to_owned(),
            message: message.clone(),
            target_started: false,
            target_terminated: false,
        }),
        events: Vec::new(),
        warnings: vec![WarningReport {
            code: "collector_stage_failure".to_owned(),
            pid: None,
            message: format!("experimental libbpf collector failed during {stage}: {message}"),
        }],
    };
    write_report(&options.report_path, &report)?;
    Err(format!("experimental libbpf collector failed during {stage}: {message}").into())
}

fn fail_after_target(
    options: &CollectorOptions,
    stage: &str,
    message: String,
    child: &mut Child,
    root_pid: u32,
    known_outcome: Option<OutcomeReport>,
    tracker: &Rc<RefCell<Tracker>>,
) -> Result<(), Box<dyn Error>> {
    let mut target_terminated = false;
    let mut termination_warning = None;

    let outcome = if let Some(outcome) = known_outcome {
        outcome
    } else {
        match child.try_wait() {
            Ok(Some(status)) => outcome_report(&status),
            Ok(None) => {
                #[cfg(unix)]
                {
                    let group = -(root_pid as i32);
                    let rc = unsafe { libc::kill(group, libc::SIGKILL) };
                    if rc == 0 {
                        target_terminated = true;
                    } else {
                        termination_warning = Some(format!(
                            "failed to terminate target process group after collector failure: {}",
                            std::io::Error::last_os_error()
                        ));
                        if child.kill().is_ok() {
                            target_terminated = true;
                        }
                    }
                }
                #[cfg(not(unix))]
                {
                    match child.kill() {
                        Ok(()) => target_terminated = true,
                        Err(error) => {
                            termination_warning = Some(format!(
                                "failed to terminate target after collector failure: {error}"
                            ));
                        }
                    }
                }

                match child.wait() {
                    Ok(status) => outcome_report(&status),
                    Err(error) => {
                        termination_warning = Some(format!(
                            "failed to reap target after collector failure: {error}"
                        ));
                        OutcomeReport {
                            exit_code: None,
                            signal: None,
                        }
                    }
                }
            }
            Err(error) => {
                termination_warning = Some(format!(
                    "failed to inspect target after collector failure: {error}"
                ));
                OutcomeReport {
                    exit_code: None,
                    signal: None,
                }
            }
        }
    };

    let mut state = tracker.borrow_mut();
    state.warning(
        None,
        "collector_stage_failure",
        format!("experimental libbpf collector failed during {stage}: {message}"),
    );
    if target_terminated {
        state.warning(
            Some(root_pid),
            "target_terminated_after_collector_failure",
            "target process group was terminated and root was reaped after post-start collector failure",
        );
    }
    if let Some(warning) = termination_warning {
        state.warning(Some(root_pid), "target_termination_uncertain", warning);
    }

    let report = CollectorReport {
        protocol_version: 1,
        backend: backend_report(),
        completeness: "incomplete_collector".to_owned(),
        observation_complete: false,
        root_pid: Some(root_pid),
        outcome,
        dropped_events: 0,
        consumer_lag_ms: options.consumer_lag_ms,
        lifecycle_timeout_ms: options.lifecycle_timeout_ms,
        lifecycle_drain_complete: false,
        decode_failure_injected: options.inject_decode_error,
        collector_failure: Some(CollectorFailureReport {
            stage: stage.to_owned(),
            message: message.clone(),
            target_started: true,
            target_terminated,
        }),
        events: std::mem::take(&mut state.events),
        warnings: std::mem::take(&mut state.warnings),
    };
    drop(state);

    write_report(&options.report_path, &report)?;
    Err(format!("experimental libbpf collector failed during {stage}: {message}").into())
}

fn parse_args() -> Result<CollectorOptions, Box<dyn Error>> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    let mut report_path = None;
    let mut event_limit = 100_000usize;
    let mut consumer_lag_ms = 0_u64;
    let mut lifecycle_timeout_ms = 2_000_u64;
    let mut inject_decode_error = false;
    let mut inject_poll_error = false;
    let mut index = 0usize;

    while index < args.len() {
        if args[index] == "--" {
            let target = args[index + 1..].to_vec();
            if target.is_empty() {
                return Err("missing target command after --".into());
            }
            return Ok(CollectorOptions {
                report_path: report_path.ok_or("--report is required")?,
                event_limit,
                consumer_lag_ms,
                lifecycle_timeout_ms,
                inject_decode_error,
                inject_poll_error,
                target,
            });
        }
        if args[index] == "--report" {
            let value = args.get(index + 1).ok_or("--report requires a path")?;
            report_path = Some(PathBuf::from(value));
            index += 2;
            continue;
        }
        if args[index] == "--event-limit" {
            let value = args
                .get(index + 1)
                .ok_or("--event-limit requires a value")?;
            event_limit = value
                .to_string_lossy()
                .parse::<usize>()
                .map_err(|_| "invalid --event-limit")?;
            if event_limit == 0 {
                return Err("--event-limit must be greater than zero".into());
            }
            index += 2;
            continue;
        }
        if args[index] == "--consumer-lag-ms" {
            let value = args
                .get(index + 1)
                .ok_or("--consumer-lag-ms requires a value")?;
            consumer_lag_ms = value
                .to_string_lossy()
                .parse::<u64>()
                .map_err(|_| "invalid --consumer-lag-ms")?;
            index += 2;
            continue;
        }
        if args[index] == "--lifecycle-timeout-ms" {
            let value = args
                .get(index + 1)
                .ok_or("--lifecycle-timeout-ms requires a value")?;
            lifecycle_timeout_ms = value
                .to_string_lossy()
                .parse::<u64>()
                .map_err(|_| "invalid --lifecycle-timeout-ms")?;
            if lifecycle_timeout_ms == 0 {
                return Err("--lifecycle-timeout-ms must be greater than zero".into());
            }
            index += 2;
            continue;
        }
        if args[index] == "--inject-decode-error" {
            inject_decode_error = true;
            index += 1;
            continue;
        }
        if args[index] == "--inject-poll-error" {
            inject_poll_error = true;
            index += 1;
            continue;
        }
        return Err(format!("unknown observer option: {}", args[index].to_string_lossy()).into());
    }

    Err("missing -- target command".into())
}

fn backend_report() -> BackendReport {
    BackendReport {
        id: "linux-libbpf-metadata-experimental-v1".to_owned(),
        implementation_version: "m8.4d-experimental-v1".to_owned(),
        platform: "linux".to_owned(),
        architecture: "x86_64".to_owned(),
        kernel_release: fs::read_to_string("/proc/sys/kernel/osrelease")
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty()),
        kernel_btf_readable: Path::new("/sys/kernel/btf/vmlinux").is_file(),
        privacy_profile: "metadata-only-v1".to_owned(),
        capabilities: vec![
            "process_spawn_lineage".to_owned(),
            "process_exec_occurrence".to_owned(),
            "successful_open_fd_identity".to_owned(),
            "loss_truncation_visibility".to_owned(),
        ],
        unsupported_capabilities: vec![
            "unconditional_process_exec_path_identity".to_owned(),
            "process_exit".to_owned(),
            "path_access_intent".to_owned(),
            "unconditional_open_path_identity".to_owned(),
            "fd_read_write_effect".to_owned(),
            "fd_dup_close_lifecycle".to_owned(),
            "fork_fd_inheritance".to_owned(),
            "close_on_exec".to_owned(),
            "rename_delete_effects".to_owned(),
            "network_connect_destination".to_owned(),
            "trace_time_relative_path".to_owned(),
            "causal_executable_chain".to_owned(),
        ],
    }
}

fn outcome_report(status: &ExitStatus) -> OutcomeReport {
    OutcomeReport {
        exit_code: status.code(),
        #[cfg(unix)]
        signal: status.signal(),
        #[cfg(not(unix))]
        signal: None,
    }
}

fn resolve_link(path: String) -> Option<String> {
    fs::read_link(path)
        .ok()
        .map(|value| value.to_string_lossy().into_owned())
}

fn write_report(path: &Path, report: &CollectorReport) -> Result<(), Box<dyn Error>> {
    let mut bytes = serde_json::to_vec_pretty(report)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

fn decode_event(data: &[u8]) -> Result<MetadataEvent, String> {
    if data.len() != 16 {
        return Err(format!("unexpected metadata event size: {}", data.len()));
    }
    let kind = u32::from_ne_bytes(data[0..4].try_into().map_err(|_| "kind")?);
    let pid = u32::from_ne_bytes(data[4..8].try_into().map_err(|_| "pid")?);
    let value = u32::from_ne_bytes(data[8..12].try_into().map_err(|_| "value")?);
    let reserved = u32::from_ne_bytes(data[12..16].try_into().map_err(|_| "reserved")?);
    if pid == 0 {
        return Err("zero process identity".to_owned());
    }
    if !matches!(kind, EVENT_EXEC | EVENT_SPAWN | EVENT_OPEN | EVENT_EXIT) {
        return Err(format!("unknown metadata event kind: {kind}"));
    }
    Ok(MetadataEvent {
        kind,
        pid,
        value,
        reserved,
    })
}

fn total_drops<M: MapCore + ?Sized>(map: &M) -> Result<u64, Box<dyn Error>> {
    let key = 0_u32.to_ne_bytes();
    let values = map
        .lookup_percpu(&key, MapFlags::ANY)?
        .ok_or("missing libbpf dropped counter")?;

    let mut total = 0_u64;
    for value in values {
        let bytes: [u8; 8] = value
            .as_slice()
            .try_into()
            .map_err(|_| "unexpected libbpf dropped-counter value size")?;
        total = total.saturating_add(u64::from_ne_bytes(bytes));
    }
    Ok(total)
}
