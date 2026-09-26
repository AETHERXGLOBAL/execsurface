use std::mem::MaybeUninit;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{MapCore, MapFlags, RingBufferBuilder};

mod probe {
    include!(concat!(env!("OUT_DIR"), "/probe.skel.rs"));
}

use probe::*;

const ARGV_SENTINEL: &str = "AX_M8_LIBBPF_ARGV_SENTINEL_DO_NOT_PERSIST";
const ENV_SENTINEL: &str = "AX_M8_LIBBPF_ENV_SENTINEL_DO_NOT_PERSIST";
const PRESSURE_EXECS: usize = 768;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = ProbeSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;
    skel.attach()?;

    let seen = Arc::new(AtomicUsize::new(0));
    let seen_cb = Arc::clone(&seen);
    let mut ring_builder = RingBufferBuilder::new();
    ring_builder.add(&skel.maps.events, move |data| {
        if data.len() != 8 {
            return -1;
        }
        let tgid = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        let tid = u32::from_ne_bytes(data[4..8].try_into().unwrap());
        if tgid == 0 || tid == 0 {
            return -1;
        }
        seen_cb.fetch_add(1, Ordering::Relaxed);
        0
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
    let initial_events = seen.load(Ordering::Relaxed);
    if initial_events == 0 {
        return Err("libbpf ring buffer produced no readable exec metadata event".into());
    }

    let before_drops = total_drops(&skel.maps.dropped)?;

    for _ in 0..PRESSURE_EXECS {
        let status = Command::new("/bin/true").status()?;
        if !status.success() {
            return Err("controlled libbpf pressure child failed".into());
        }
    }

    let after_drops = total_drops(&skel.maps.dropped)?;
    let before_drain = seen.load(Ordering::Relaxed);
    ring.poll(Duration::from_millis(50))?;
    let pressure_events = seen.load(Ordering::Relaxed) - before_drain;

    if after_drops <= before_drops {
        return Err(format!(
            "libbpf ring pressure did not produce explicit loss accounting: before={before_drops} after={after_drops}"
        )
        .into());
    }

    println!("M8_LIBBPF_ATTACH_PASS");
    println!("M8_LIBBPF_EVENT_TRANSPORT_PASS events={initial_events}");
    println!(
        "M8_LIBBPF_LOSS_ACCOUNTING_PASS dropped={} pressure_events={pressure_events}",
        after_drops - before_drops
    );
    println!("M8_LIBBPF_PRIVACY_SCHEMA_PASS event_bytes=8 fields=tgid,tid");
    Ok(())
}

fn total_drops<M: MapCore + ?Sized>(map: &M) -> Result<u64, Box<dyn std::error::Error>> {
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
