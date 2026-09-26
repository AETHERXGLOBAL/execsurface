use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use aya::{
    maps::{ring_buf::RingBuf, MapData, PerCpuArray},
    programs::TracePoint,
    Ebpf,
};

const EVENT_EXEC: u32 = 1;
const EVENT_FORK: u32 = 2;
const ARGV_SENTINEL: &str = "AX_M8_ARGV_SENTINEL_DO_NOT_PERSIST";
const ENV_SENTINEL: &str = "AX_M8_ENV_SENTINEL_DO_NOT_PERSIST";
const PRESSURE_EXECS: usize = 768;

#[derive(Default)]
struct EventStats {
    total: usize,
    exec: usize,
    fork: usize,
    fork_edges: Vec<(u32, u32)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let object = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: execsurface-m8-aya-loader <ebpf-object>",
            )
        })?;

    let mut ebpf = Ebpf::load_file(&object)?;

    let events_map = ebpf
        .take_map("EVENTS")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing EVENTS map"))?;
    let mut events = RingBuf::try_from(events_map)?;

    let dropped_map = ebpf
        .take_map("DROPPED")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing DROPPED map"))?;
    let dropped = PerCpuArray::<_, u64>::try_from(dropped_map)?;

    let exec_program: &mut TracePoint = ebpf
        .program_mut("execsurface_m8_exec")
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "missing execsurface_m8_exec program",
            )
        })?
        .try_into()?;
    exec_program.load()?;

    // M8.2 diagnostics gate: with privilege available, deliberately request a missing hook.
    // The original kernel/library error is allowed to propagate; it must not become evidence.
    if std::env::var_os("AX_M8_INVALID_ATTACH").is_some() {
        let _ = exec_program.attach("syscalls", "execsurface_m8_missing_tracepoint")?;
        return Err("invalid tracepoint unexpectedly attached".into());
    }

    let _exec_link = exec_program.attach("syscalls", "sys_enter_execve")?;

    // Use syscall-exit lineage rather than task_struct access. This keeps the feasibility
    // program Apache-compatible and avoids depending on GPL-restricted kernel-struct reads.
    attach_spawn_tracepoint(&mut ebpf, "execsurface_m8_clone_exit", "sys_exit_clone")?;
    attach_spawn_tracepoint(&mut ebpf, "execsurface_m8_clone3_exit", "sys_exit_clone3")?;
    attach_spawn_tracepoint(&mut ebpf, "execsurface_m8_fork_exit", "sys_exit_fork")?;
    attach_spawn_tracepoint(&mut ebpf, "execsurface_m8_vfork_exit", "sys_exit_vfork")?;

    // Privacy sentinel: secrets are deliberately present in argv/environment of a controlled
    // child, while the persisted event schema contains numeric process metadata only.
    let status = Command::new("/bin/true")
        .arg(ARGV_SENTINEL)
        .env("AX_M8_SECRET", ENV_SENTINEL)
        .status()?;
    if !status.success() {
        return Err("controlled privacy-sentinel child failed".into());
    }

    thread::sleep(Duration::from_millis(30));
    let privacy_stats = drain_events(&mut events)?;
    if privacy_stats.exec == 0 {
        return Err("attached observer produced no readable exec metadata event".into());
    }

    // Controlled lineage proof: the launched shell must create at least one child. A
    // parent->child edge rooted at the PID returned by spawn is enough to show that userspace
    // can recursively scope descendants without collecting process names or arguments.
    let mut root = Command::new("/bin/sh")
        .arg("-c")
        .arg("/bin/true & wait")
        .spawn()?;
    let root_pid = root.id();
    let root_status = root.wait()?;
    if !root_status.success() {
        return Err("controlled lineage command failed".into());
    }
    thread::sleep(Duration::from_millis(30));
    let lineage_stats = drain_events(&mut events)?;
    let rooted_edges = lineage_stats
        .fork_edges
        .iter()
        .filter(|(parent, _)| *parent == root_pid)
        .count();
    if rooted_edges == 0 {
        return Err(format!(
            "syscall-exit lineage evidence did not contain an edge rooted at launched pid {root_pid}"
        )
        .into());
    }

    let before_drops = total_drops(&dropped)?;

    // Deliberately stop consuming and generate more process events than the 4 KiB feasibility
    // ring can retain. Required result: explicit producer-side loss evidence, not zero loss.
    for _ in 0..PRESSURE_EXECS {
        let status = Command::new("/bin/true").status()?;
        if !status.success() {
            return Err("controlled pressure child failed".into());
        }
    }

    thread::sleep(Duration::from_millis(30));
    let after_drops = total_drops(&dropped)?;
    let pressure_stats = drain_events(&mut events)?;

    if after_drops <= before_drops {
        return Err(format!(
            "ring pressure did not produce explicit loss accounting: before={before_drops} after={after_drops}"
        )
        .into());
    }

    println!("M8_AYA_ATTACH_PASS");
    println!(
        "M8_AYA_EVENT_TRANSPORT_PASS events={}",
        privacy_stats.total + lineage_stats.total
    );
    println!(
        "M8_AYA_LINEAGE_PASS root_pid={root_pid} rooted_edges={rooted_edges} fork_events={}",
        lineage_stats.fork
    );
    println!(
        "M8_AYA_LOSS_ACCOUNTING_PASS dropped={} pressure_events={}",
        after_drops - before_drops,
        pressure_stats.total
    );
    println!(
        "M8_AYA_PRIVACY_SCHEMA_PASS event_bytes=16 fields=kind,pid,related_pid,reserved"
    );
    Ok(())
}

fn attach_spawn_tracepoint(
    ebpf: &mut Ebpf,
    program_name: &str,
    tracepoint_name: &str,
) -> Result<(), Box<dyn Error>> {
    let program: &mut TracePoint = ebpf
        .program_mut(program_name)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("missing {program_name} program"),
            )
        })?
        .try_into()?;
    program.load()?;
    let _ = program.attach("syscalls", tracepoint_name)?;
    Ok(())
}

fn drain_events(events: &mut RingBuf<MapData>) -> Result<EventStats, Box<dyn Error>> {
    let mut stats = EventStats::default();
    while let Some(sample) = events.next() {
        if sample.len() != 16 {
            return Err(format!("unexpected process event size: {}", sample.len()).into());
        }
        let bytes: [u8; 16] = sample.as_ref().try_into()?;
        let kind = u32::from_ne_bytes(bytes[0..4].try_into()?);
        let pid = u32::from_ne_bytes(bytes[4..8].try_into()?);
        let related_pid = u32::from_ne_bytes(bytes[8..12].try_into()?);
        if pid == 0 || related_pid == 0 {
            return Err("invalid zero process identity in process event".into());
        }
        stats.total += 1;
        match kind {
            EVENT_EXEC => stats.exec += 1,
            EVENT_FORK => {
                stats.fork += 1;
                stats.fork_edges.push((pid, related_pid));
            }
            other => return Err(format!("unknown process event kind: {other}").into()),
        }
    }
    Ok(stats)
}

fn total_drops(dropped: &PerCpuArray<MapData, u64>) -> Result<u64, Box<dyn Error>> {
    let values = dropped.get(&0, 0)?;
    Ok(values.iter().copied().sum())
}
