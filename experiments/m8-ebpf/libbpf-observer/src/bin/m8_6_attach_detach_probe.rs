use std::error::Error;
use std::mem::MaybeUninit;
use std::time::{Duration, Instant};

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use serde::Serialize;

mod observer {
    include!(concat!(env!("OUT_DIR"), "/observer.skel.rs"));
}

use observer::*;

#[derive(Debug, Serialize)]
struct Sample {
    repetition: usize,
    open_ms: f64,
    load_ms: f64,
    attach_ms: f64,
    detach_ms: f64,
    total_ms: f64,
}

#[derive(Debug, Serialize)]
struct Evidence {
    schema_version: u32,
    claim_scope: &'static str,
    samples: Vec<Sample>,
}

fn elapsed_ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1_000.0
}

fn main() -> Result<(), Box<dyn Error>> {
    let samples = std::env::args()
        .nth(1)
        .map(|value| value.parse::<usize>())
        .transpose()?
        .unwrap_or(7);
    if samples == 0 {
        return Err("sample count must be greater than zero".into());
    }

    let mut evidence = Vec::with_capacity(samples);
    for repetition in 0..samples {
        let total_started = Instant::now();
        let builder = ObserverSkelBuilder::default();

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

        // Do not run a target here. This probe isolates lifecycle setup/teardown
        // cost for the exact skeleton used by the experimental observer.
        std::thread::sleep(Duration::from_millis(5));

        let started = Instant::now();
        drop(skel);
        let detach_ms = elapsed_ms(started);
        let total_ms = elapsed_ms(total_started);

        evidence.push(Sample {
            repetition,
            open_ms,
            load_ms,
            attach_ms,
            detach_ms,
            total_ms,
        });
    }

    let output = Evidence {
        schema_version: 1,
        claim_scope: "isolated open/load/attach/detach timing for the current per-invocation libbpf skeleton on this exact tested host",
        samples: evidence,
    };
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
