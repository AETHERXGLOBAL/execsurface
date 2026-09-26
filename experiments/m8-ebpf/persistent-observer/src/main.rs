use std::cell::RefCell;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::RawFd;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus};
use std::rc::Rc;
use std::time::{Duration, Instant};

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{MapCore, MapFlags, RingBuffer, RingBufferBuilder};
use serde::Serialize;

mod persistent {
    include!(concat!(env!("OUT_DIR"), "/persistent.skel.rs"));
}

use persistent::*;

const EVENT_EXEC: u32 = 1;
const EVENT_SPAWN: u32 = 2;
const EVENT_EXIT: u32 = 3;
const POLL_INTERVAL_MS: u64 = 10;
const QUIESCENCE_GRACE_MS: u64 = 100;
const QUIESCENCE_POLLS: usize = 4;
const LIFECYCLE_TIMEOUT_MS: u64 = 2_000;
const MAX_SESSION_EVENTS: usize = 100_000;

#[derive(Debug, Clone, Copy)]
struct MetadataEvent {
    epoch: u64,
    kind: u32,
    tid: u32,
    tgid: u32,
    value: u32,
    reserved: u32,
}

#[derive(Debug)]
struct Options {
    report: PathBuf,
    fixture: PathBuf,
}

#[derive(Debug, Clone)]
struct ActiveSession {
    epoch: u64,
    root_tid: u32,
    known: BTreeSet<u32>,
    active: BTreeSet<u32>,
    sequence: u64,
    event_count: usize,
    spawn_count: usize,
    exec_count: usize,
    exit_count: usize,
    stale_epoch_events: u64,
    integrity_errors: u64,
    event_limit_hit: bool,
}

impl ActiveSession {
    fn new(epoch: u64, root_tid: u32) -> Self {
        let mut known = BTreeSet::new();
        let mut active = BTreeSet::new();
        known.insert(root_tid);
        active.insert(root_tid);
        Self {
            epoch,
            root_tid,
            known,
            active,
            sequence: 0,
            event_count: 0,
            spawn_count: 0,
            exec_count: 0,
            exit_count: 0,
            stale_epoch_events: 0,
            integrity_errors: 0,
            event_limit_hit: false,
        }
    }

    fn accept(&mut self, event: MetadataEvent) {
        if event.epoch != self.epoch {
            self.stale_epoch_events = self.stale_epoch_events.saturating_add(1);
            return;
        }
        if self.event_count >= MAX_SESSION_EVENTS {
            self.event_limit_hit = true;
            return;
        }

        self.sequence = self.sequence.saturating_add(1);
        self.event_count = self.event_count.saturating_add(1);

        match event.kind {
            EVENT_SPAWN => {
                if !self.known.contains(&event.tid) || event.value == 0 {
                    self.integrity_errors = self.integrity_errors.saturating_add(1);
                    return;
                }
                self.known.insert(event.value);
                self.active.insert(event.value);
                self.spawn_count = self.spawn_count.saturating_add(1);
            }
            EVENT_EXEC => {
                if !self.known.contains(&event.tid) || event.tgid == 0 {
                    self.integrity_errors = self.integrity_errors.saturating_add(1);
                    return;
                }
                self.exec_count = self.exec_count.saturating_add(1);
            }
            EVENT_EXIT => {
                if !self.known.contains(&event.tid) {
                    self.integrity_errors = self.integrity_errors.saturating_add(1);
                    return;
                }
                self.active.remove(&event.tid);
                self.exit_count = self.exit_count.saturating_add(1);
            }
            _ => self.integrity_errors = self.integrity_errors.saturating_add(1),
        }

        let _ = event.reserved;
    }
}

#[derive(Debug, Default)]
struct Tracker {
    active: Option<ActiveSession>,
    unexpected_without_session: u64,
    decode_errors: u64,
}

impl Tracker {
    fn begin(&mut self, epoch: u64, root_tid: u32) -> Result<(), Box<dyn Error>> {
        if self.active.is_some() {
            return Err("attempted concurrent persistent observation session".into());
        }
        self.active = Some(ActiveSession::new(epoch, root_tid));
        Ok(())
    }

    fn ingest(&mut self, event: MetadataEvent) {
        if let Some(active) = self.active.as_mut() {
            active.accept(event);
        } else {
            self.unexpected_without_session = self.unexpected_without_session.saturating_add(1);
        }
    }

    fn record_decode_error(&mut self) {
        self.decode_errors = self.decode_errors.saturating_add(1);
    }

    fn end(&mut self) -> Result<ActiveSession, Box<dyn Error>> {
        self.active.take().ok_or_else(|| "no active session".into())
    }
}

#[derive(Debug, Serialize)]
struct OneTimeLifecycle {
    open_ms: f64,
    load_ms: f64,
    attach_ms: f64,
    final_detach_ms: f64,
}

#[derive(Debug, Serialize)]
struct Outcome {
    exit_code: Option<i32>,
    signal: Option<i32>,
}

#[derive(Debug, Serialize)]
struct SessionReport {
    epoch: u64,
    root_tid: u32,
    outcome: Outcome,
    elapsed_ms: f64,
    event_count: usize,
    spawn_count: usize,
    exec_count: usize,
    exit_count: usize,
    active_remaining: usize,
    stale_epoch_events: u64,
    integrity_errors: u64,
    decode_error_delta: u64,
    producer_drop_delta: u64,
    routing_error_delta: u64,
    membership_map_empty: bool,
    pending_mechanism_map_empty: bool,
    lifecycle_drain_complete: bool,
    event_limit_hit: bool,
    clean_for_m8_7a: bool,
}

#[derive(Debug, Serialize)]
struct Report {
    protocol_version: u32,
    architecture: &'static str,
    session_model: &'static str,
    kernel_filter: &'static str,
    one_time_lifecycle: OneTimeLifecycle,
    sessions: Vec<SessionReport>,
    unexpected_without_session: u64,
    decode_errors_total: u64,
    final_membership_map_empty: bool,
    final_pending_mechanism_map_empty: bool,
    authority: Authority,
}

#[derive(Debug, Serialize)]
struct Authority {
    ptrace_correctness_reference: bool,
    full_surface_comparable: bool,
    ebpf_pass_authorized: bool,
    product_integration_authorized: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = parse_args()?;

    let total_started = Instant::now();
    let builder = PersistentSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();

    let started = Instant::now();
    let open_skel = builder.open(&mut open_object)?;
    let open_ms = elapsed_ms(started);

    let started = Instant::now();
    let mut skel = open_skel.load()?;
    let load_ms = elapsed_ms(started);

    let started = Instant::now();
    skel.attach()?;
    let attach_ms = elapsed_ms(started);

    let tracker = Rc::new(RefCell::new(Tracker::default()));
    let callback_tracker = Rc::clone(&tracker);
    let mut ring_builder = RingBufferBuilder::new();
    ring_builder.add(&skel.maps.events, move |data| {
        match decode_event(data) {
            Ok(event) => callback_tracker.borrow_mut().ingest(event),
            Err(_) => callback_tracker.borrow_mut().record_decode_error(),
        }
        0
    })?;
    let ring = ring_builder.build()?;

    let mut last_drops = total_counter(&skel.maps.dropped)?;
    let mut last_routing = total_counter(&skel.maps.routing_errors)?;
    let mut sessions = Vec::new();

    for epoch in 1_u64..=2 {
        let report = run_session(
            epoch,
            &options.fixture,
            &skel.maps.task_epoch,
            &skel.maps.pending_spawn_mechanism,
            &skel.maps.dropped,
            &skel.maps.routing_errors,
            &ring,
            &tracker,
            last_drops,
            last_routing,
        )?;
        last_drops = last_drops.saturating_add(report.producer_drop_delta);
        last_routing = last_routing.saturating_add(report.routing_error_delta);
        if !report.clean_for_m8_7a {
            sessions.push(report);
            let final_membership_map_empty = map_empty(&skel.maps.task_epoch);
            let final_pending_mechanism_map_empty = map_empty(&skel.maps.pending_spawn_mechanism);
            return write_failure_report_and_exit(
                options.report,
                open_ms,
                load_ms,
                attach_ms,
                sessions,
                &tracker,
                final_membership_map_empty,
                final_pending_mechanism_map_empty,
                ring,
                skel,
                "persistent session failed M8.7a health gates",
            );
        }
        sessions.push(report);
    }

    // Flush any delayed event after the second accepted drain. With no active
    // session, any epoch-tagged event is explicit cross-session contamination.
    ring.poll(Duration::from_millis(QUIESCENCE_GRACE_MS))?;

    let final_membership_map_empty = map_empty(&skel.maps.task_epoch);
    let final_pending_mechanism_map_empty = map_empty(&skel.maps.pending_spawn_mechanism);
    let unexpected_without_session = tracker.borrow().unexpected_without_session;
    let decode_errors_total = tracker.borrow().decode_errors;

    let detach_started = Instant::now();
    drop(ring);
    drop(skel);
    let final_detach_ms = elapsed_ms(detach_started);

    let report = Report {
        protocol_version: 1,
        architecture: "persistent-attached-libbpf-feasibility-v1",
        session_model: "single-active-session-epoch-v1",
        kernel_filter: "task-tid-to-u64-epoch",
        one_time_lifecycle: OneTimeLifecycle {
            open_ms,
            load_ms,
            attach_ms,
            final_detach_ms,
        },
        sessions,
        unexpected_without_session,
        decode_errors_total,
        final_membership_map_empty,
        final_pending_mechanism_map_empty,
        authority: Authority {
            ptrace_correctness_reference: true,
            full_surface_comparable: false,
            ebpf_pass_authorized: false,
            product_integration_authorized: false,
        },
    };

    write_report(&options.report, &report)?;

    if unexpected_without_session != 0
        || decode_errors_total != 0
        || !final_membership_map_empty
        || !final_pending_mechanism_map_empty
        || report
            .sessions
            .iter()
            .any(|session| !session.clean_for_m8_7a)
    {
        return Err("M8.7a persistent observer evidence failed final isolation gates".into());
    }

    eprintln!(
        "M8_7A_PERSISTENT_SESSION_PASS sessions={} startup_ms={:.3} final_detach_ms={:.3} total_ms={:.3}",
        report.sessions.len(),
        open_ms + load_ms + attach_ms,
        final_detach_ms,
        total_started.elapsed().as_secs_f64() * 1_000.0
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_session<M: MapCore + ?Sized>(
    epoch: u64,
    fixture: &PathBuf,
    task_epoch: &M,
    pending_spawn_mechanism: &M,
    dropped: &M,
    routing_errors: &M,
    ring: &RingBuffer<'_>,
    tracker: &Rc<RefCell<Tracker>>,
    expected_drops: u64,
    expected_routing: u64,
) -> Result<SessionReport, Box<dyn Error>> {
    if epoch == 0 {
        return Err("epoch zero is reserved".into());
    }
    if !map_empty(task_epoch) || !map_empty(pending_spawn_mechanism) {
        return Err("persistent kernel routing state is not empty before session".into());
    }

    ring.poll(Duration::ZERO)?;
    let unexpected_before = tracker.borrow().unexpected_without_session;
    let decode_before = tracker.borrow().decode_errors;
    let drops_before = total_counter(dropped)?;
    let routing_before = total_counter(routing_errors)?;
    if drops_before != expected_drops || routing_before != expected_routing {
        return Err("global producer/routing counters changed while no session was active".into());
    }

    let (mut child, release_fd) = spawn_blocked(fixture)?;
    let root_tid = child.id();
    task_epoch.update(&root_tid.to_ne_bytes(), &epoch.to_ne_bytes(), MapFlags::ANY)?;
    tracker.borrow_mut().begin(epoch, root_tid)?;

    if let Err(error) = release_target(release_fd) {
        terminate_process_group(&mut child, root_tid);
        let _ = task_epoch.delete(&root_tid.to_ne_bytes());
        let _ = tracker.borrow_mut().end();
        return Err(error.into());
    }

    let session_started = Instant::now();
    let status = loop {
        ring.poll(Duration::from_millis(POLL_INTERVAL_MS))?;
        if let Some(status) = child.try_wait()? {
            break status;
        }
    };

    let drain_started = Instant::now();
    let mut last_sequence = tracker
        .borrow()
        .active
        .as_ref()
        .ok_or("active session disappeared before drain")?
        .sequence;
    let mut idle_polls = 0usize;
    let mut lifecycle_drain_complete = false;

    while drain_started.elapsed() < Duration::from_millis(LIFECYCLE_TIMEOUT_MS) {
        ring.poll(Duration::from_millis(POLL_INTERVAL_MS))?;
        let (sequence, active_count) = {
            let state = tracker.borrow();
            let active = state
                .active
                .as_ref()
                .ok_or("active session disappeared during drain")?;
            (active.sequence, active.active.len())
        };

        if sequence == last_sequence {
            idle_polls = idle_polls.saturating_add(1);
        } else {
            last_sequence = sequence;
            idle_polls = 0;
        }

        if active_count == 0
            && drain_started.elapsed() >= Duration::from_millis(QUIESCENCE_GRACE_MS)
            && idle_polls >= QUIESCENCE_POLLS
        {
            lifecycle_drain_complete = true;
            break;
        }
    }

    let drops_after = total_counter(dropped)?;
    let routing_after = total_counter(routing_errors)?;
    let producer_drop_delta = drops_after.saturating_sub(drops_before);
    let routing_error_delta = routing_after.saturating_sub(routing_before);
    let decode_error_delta = tracker.borrow().decode_errors.saturating_sub(decode_before);
    let session = tracker.borrow_mut().end()?;
    let membership_map_empty = map_empty(task_epoch);
    let pending_mechanism_map_empty = map_empty(pending_spawn_mechanism);
    let unexpected_after = tracker.borrow().unexpected_without_session;

    let clean_for_m8_7a = lifecycle_drain_complete
        && session.active.is_empty()
        && session.stale_epoch_events == 0
        && session.integrity_errors == 0
        && !session.event_limit_hit
        && producer_drop_delta == 0
        && routing_error_delta == 0
        && decode_error_delta == 0
        && unexpected_after == unexpected_before
        && membership_map_empty
        && pending_mechanism_map_empty
        && session.exec_count >= 2
        && session.spawn_count >= 1
        && session.exit_count >= 2;

    Ok(SessionReport {
        epoch,
        root_tid: session.root_tid,
        outcome: outcome(&status),
        elapsed_ms: elapsed_ms(session_started),
        event_count: session.event_count,
        spawn_count: session.spawn_count,
        exec_count: session.exec_count,
        exit_count: session.exit_count,
        active_remaining: session.active.len(),
        stale_epoch_events: session.stale_epoch_events,
        integrity_errors: session.integrity_errors,
        decode_error_delta,
        producer_drop_delta,
        routing_error_delta,
        membership_map_empty,
        pending_mechanism_map_empty,
        lifecycle_drain_complete,
        event_limit_hit: session.event_limit_hit,
        clean_for_m8_7a,
    })
}

#[allow(clippy::too_many_arguments)]
fn write_failure_report_and_exit(
    report_path: PathBuf,
    open_ms: f64,
    load_ms: f64,
    attach_ms: f64,
    sessions: Vec<SessionReport>,
    tracker: &Rc<RefCell<Tracker>>,
    final_membership_map_empty: bool,
    final_pending_mechanism_map_empty: bool,
    ring: RingBuffer<'_>,
    skel: PersistentSkel<'_>,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    let unexpected_without_session = tracker.borrow().unexpected_without_session;
    let decode_errors_total = tracker.borrow().decode_errors;
    let detach_started = Instant::now();
    drop(ring);
    drop(skel);
    let final_detach_ms = elapsed_ms(detach_started);

    let report = Report {
        protocol_version: 1,
        architecture: "persistent-attached-libbpf-feasibility-v1",
        session_model: "single-active-session-epoch-v1",
        kernel_filter: "task-tid-to-u64-epoch",
        one_time_lifecycle: OneTimeLifecycle {
            open_ms,
            load_ms,
            attach_ms,
            final_detach_ms,
        },
        sessions,
        unexpected_without_session,
        decode_errors_total,
        final_membership_map_empty,
        final_pending_mechanism_map_empty,
        authority: Authority {
            ptrace_correctness_reference: true,
            full_surface_comparable: false,
            ebpf_pass_authorized: false,
            product_integration_authorized: false,
        },
    };
    write_report(&report_path, &report)?;
    Err(message.into())
}

fn spawn_blocked(fixture: &PathBuf) -> Result<(Child, RawFd), Box<dyn Error>> {
    let mut fds = [-1_i32; 2];
    if unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let read_fd = fds[0];
    let write_fd = fds[1];

    let mut command = Command::new(fixture);
    command.arg("barrier").arg(read_fd.to_string());
    unsafe {
        command.pre_exec(move || {
            if libc::setpgid(0, 0) != 0 {
                return Err(io::Error::last_os_error());
            }
            libc::close(write_fd);

            let flags = libc::fcntl(read_fd, libc::F_GETFD);
            if flags < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::fcntl(read_fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }

    match command.spawn() {
        Ok(child) => {
            unsafe {
                libc::close(read_fd);
            }
            Ok((child, write_fd))
        }
        Err(error) => {
            unsafe {
                libc::close(read_fd);
                libc::close(write_fd);
            }
            Err(error.into())
        }
    }
}

fn release_target(write_fd: RawFd) -> Result<(), &'static str> {
    let byte = [1_u8];
    let rc = unsafe { libc::write(write_fd, byte.as_ptr().cast(), 1) };
    unsafe {
        libc::close(write_fd);
    }
    if rc == 1 {
        Ok(())
    } else {
        Err("failed to release root-registration launch barrier")
    }
}

fn terminate_process_group(child: &mut Child, root_tid: u32) {
    unsafe {
        libc::kill(-(root_tid as i32), libc::SIGKILL);
    }
    let _ = child.wait();
}

fn parse_args() -> Result<Options, Box<dyn Error>> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    let mut report = None;
    let mut fixture = None;
    let mut index = 0usize;
    while index < args.len() {
        match args[index].to_string_lossy().as_ref() {
            "--report" => {
                report = Some(PathBuf::from(
                    args.get(index + 1).ok_or("--report requires a path")?,
                ));
                index += 2;
            }
            "--fixture" => {
                fixture = Some(PathBuf::from(
                    args.get(index + 1).ok_or("--fixture requires a path")?,
                ));
                index += 2;
            }
            other => return Err(format!("unknown option: {other}").into()),
        }
    }
    Ok(Options {
        report: report.ok_or("--report is required")?,
        fixture: fixture.ok_or("--fixture is required")?,
    })
}

fn decode_event(data: &[u8]) -> Result<MetadataEvent, &'static str> {
    if data.len() != 32 {
        return Err("unexpected persistent metadata event size");
    }
    Ok(MetadataEvent {
        epoch: u64::from_ne_bytes(data[0..8].try_into().map_err(|_| "epoch")?),
        kind: u32::from_ne_bytes(data[8..12].try_into().map_err(|_| "kind")?),
        tid: u32::from_ne_bytes(data[12..16].try_into().map_err(|_| "tid")?),
        tgid: u32::from_ne_bytes(data[16..20].try_into().map_err(|_| "tgid")?),
        value: u32::from_ne_bytes(data[20..24].try_into().map_err(|_| "value")?),
        reserved: u32::from_ne_bytes(data[24..28].try_into().map_err(|_| "reserved")?),
    })
}

fn total_counter<M: MapCore + ?Sized>(map: &M) -> Result<u64, Box<dyn Error>> {
    let key = 0_u32.to_ne_bytes();
    let values = map
        .lookup_percpu(&key, MapFlags::ANY)?
        .ok_or("missing persistent observer counter")?;
    let mut total = 0_u64;
    for value in values {
        let bytes: [u8; 8] = value
            .as_slice()
            .try_into()
            .map_err(|_| "unexpected persistent observer counter size")?;
        total = total.saturating_add(u64::from_ne_bytes(bytes));
    }
    Ok(total)
}

fn map_empty<M: MapCore + ?Sized>(map: &M) -> bool {
    map.keys().next().is_none()
}

fn outcome(status: &ExitStatus) -> Outcome {
    Outcome {
        exit_code: status.code(),
        signal: status.signal(),
    }
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1_000.0
}

fn write_report(path: &PathBuf, report: &Report) -> Result<(), Box<dyn Error>> {
    let mut bytes = serde_json::to_vec_pretty(report)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}
