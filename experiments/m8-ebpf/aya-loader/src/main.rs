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

const ARGV_SENTINEL: &str = "AX_M8_ARGV_SENTINEL_DO_NOT_PERSIST";
const ENV_SENTINEL: &str = "AX_M8_ENV_SENTINEL_DO_NOT_PERSIST";
const PRESSURE_EXECS: usize = 768;

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

    let program: &mut TracePoint = ebpf
        .program_mut("execsurface_m8_exec")
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "missing execsurface_m8_exec program",
            )
        })?
        .try_into()?;
    program.load()?;
    let _link = program.attach("syscalls", "sys_enter_execve")?;

    // Privacy sentinel: secrets are deliberately present in argv/environment of a controlled
    // child, while the eBPF event schema contains only numeric process metadata.
    let status = Command::new("/bin/true")
        .arg(ARGV_SENTINEL)
        .env("AX_M8_SECRET", ENV_SENTINEL)
        .status()?;
    if !status.success() {
        return Err("controlled privacy-sentinel child failed".into());
    }

    thread::sleep(Duration::from_millis(30));
    let initial_events = drain_events(&mut events)?;
    if initial_events == 0 {
        return Err("attached observer produced no readable exec metadata event".into());
    }

    let before_drops = total_drops(&dropped)?;

    // Deliberately stop consuming and generate more execs than the 4 KiB feasibility ring can
    // retain. The required result is not zero loss; it is explicit producer-side loss evidence.
    for _ in 0..PRESSURE_EXECS {
        let status = Command::new("/bin/true").status()?;
        if !status.success() {
            return Err("controlled pressure child failed".into());
        }
    }

    thread::sleep(Duration::from_millis(30));
    let after_drops = total_drops(&dropped)?;
    let pressure_events = drain_events(&mut events)?;

    if after_drops <= before_drops {
        return Err(format!(
            "ring pressure did not produce explicit loss accounting: before={before_drops} after={after_drops}"
        )
        .into());
    }

    println!("M8_AYA_ATTACH_PASS");
    println!("M8_AYA_EVENT_TRANSPORT_PASS events={initial_events}");
    println!(
        "M8_AYA_LOSS_ACCOUNTING_PASS dropped={} pressure_events={pressure_events}",
        after_drops - before_drops
    );
    println!("M8_AYA_PRIVACY_SCHEMA_PASS event_bytes=8 fields=tgid,tid");
    Ok(())
}

fn drain_events(events: &mut RingBuf<MapData>) -> Result<usize, Box<dyn Error>> {
    let mut seen = 0usize;
    while let Some(sample) = events.next() {
        if sample.len() != 8 {
            return Err(format!("unexpected exec event size: {}", sample.len()).into());
        }
        let bytes: [u8; 8] = sample.as_ref().try_into()?;
        let tgid = u32::from_ne_bytes(bytes[0..4].try_into()?);
        let tid = u32::from_ne_bytes(bytes[4..8].try_into()?);
        if tgid == 0 || tid == 0 {
            return Err("invalid zero process identity in exec event".into());
        }
        seen += 1;
    }
    Ok(seen)
}

fn total_drops(dropped: &PerCpuArray<MapData, u64>) -> Result<u64, Box<dyn Error>> {
    let values = dropped.get(&0, 0)?;
    Ok(values.iter().copied().sum())
}
