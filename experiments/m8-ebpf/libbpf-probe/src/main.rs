use std::cell::RefCell;
use std::error::Error;
use std::mem::MaybeUninit;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{MapCore, MapFlags, RingBufferBuilder};

mod probe {
    include!(concat!(env!("OUT_DIR"), "/probe.skel.rs"));
}

use probe::*;

const EVENT_EXEC: u32 = 1;
const EVENT_FORK: u32 = 2;
const ARGV_SENTINEL: &str = "AX_M8_LIBBPF_ARGV_SENTINEL_DO_NOT_PERSIST";
const ENV_SENTINEL: &str = "AX_M8_LIBBPF_ENV_SENTINEL_DO_NOT_PERSIST";
const PRESSURE_EXECS: usize = 768;

#[derive(Default)]
struct EventStats {
    total: usize,
    exec: usize,
    fork: usize,
    fork_edges: Vec<(u32, u32)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = ProbeSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;
    skel.attach()?;

    let stats = Rc::new(RefCell::new(EventStats::default()));
    let callback_stats = Rc::clone(&stats);
    let mut ring_builder = RingBufferBuilder::new();
    ring_builder.add(&skel.maps.events, move |data| {
        match record_event(data, &mut callback_stats.borrow_mut()) {
            Ok(()) => 0,
            Err(message) => {
                eprintln!("M8_LIBBPF_EVENT_DECODE_ERROR {message}");
                -1
            }
        }
    })?;
    let ring = ring_builder.build()?;

    let status = Command::new("/bin/true")
        .arg(ARGV_SENTINEL)
        .env("AX_M8_LIBBPF_SECRET", ENV_SENTINEL)
        .status()?;
    if !status.success() {
        return Err("controlled libbpf privacy-sentinel child failed".into());
    }
    ring.poll(Duration::from_millis(50))?;
    if stats.borrow().exec == 0 {
        return Err("libbpf ring buffer produced no readable exec metadata event".into());
    }

    let mut root = Command::new("/bin/sh")
        .arg("-c")
        .arg("/bin/true & wait")
        .spawn()?;
    let root_pid = root.id();
    let root_status = root.wait()?;
    if !root_status.success() {
        return Err("controlled libbpf lineage command failed".into());
    }
    ring.poll(Duration::from_millis(50))?;
    let rooted_edges = stats
        .borrow()
        .fork_edges
        .iter()
        .filter(|(parent, _)| *parent == root_pid)
        .count();
    if rooted_edges == 0 {
        return Err(format!(
            "libbpf fork evidence did not contain an edge rooted at launched pid {root_pid}"
        )
        .into());
    }

    let before_drops = total_drops(&skel.maps.dropped)?;
    for _ in 0..PRESSURE_EXECS {
        let status = Command::new("/bin/true").status()?;
        if !status.success() {
            return Err("controlled libbpf pressure child failed".into());
        }
    }
    std::thread::sleep(Duration::from_millis(30));
    let after_drops = total_drops(&skel.maps.dropped)?;
    if after_drops <= before_drops {
        return Err(format!(
            "libbpf ring pressure did not produce explicit loss accounting: before={before_drops} after={after_drops}"
        )
        .into());
    }
    ring.consume()?;

    let snapshot = stats.borrow();
    println!("M8_LIBBPF_ATTACH_PASS");
    println!("M8_LIBBPF_EVENT_TRANSPORT_PASS events={}", snapshot.total);
    println!(
        "M8_LIBBPF_LINEAGE_PASS root_pid={root_pid} rooted_edges={rooted_edges} fork_events={}",
        snapshot.fork
    );
    println!(
        "M8_LIBBPF_LOSS_ACCOUNTING_PASS dropped={}",
        after_drops - before_drops
    );
    println!("M8_LIBBPF_PRIVACY_SCHEMA_PASS event_bytes=16 fields=kind,pid,related_pid,reserved");
    Ok(())
}

fn record_event(data: &[u8], stats: &mut EventStats) -> Result<(), String> {
    if data.len() != 16 {
        return Err(format!("unexpected process event size: {}", data.len()));
    }
    let kind = u32::from_ne_bytes(data[0..4].try_into().map_err(|_| "kind")?);
    let pid = u32::from_ne_bytes(data[4..8].try_into().map_err(|_| "pid")?);
    let related_pid = u32::from_ne_bytes(data[8..12].try_into().map_err(|_| "related_pid")?);
    if pid == 0 || related_pid == 0 {
        return Err("zero process identity".to_owned());
    }
    stats.total += 1;
    match kind {
        EVENT_EXEC => stats.exec += 1,
        EVENT_FORK => {
            stats.fork += 1;
            stats.fork_edges.push((pid, related_pid));
        }
        other => return Err(format!("unknown process event kind: {other}")),
    }
    Ok(())
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
