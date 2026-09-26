use std::mem::MaybeUninit;

use libbpf_rs::skel::{OpenSkel, SkelBuilder};

mod probe {
    include!(concat!(env!("OUT_DIR"), "/probe.skel.rs"));
}

use probe::*;

fn main() -> Result<(), libbpf_rs::Error> {
    let builder = ProbeSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;
    skel.attach()?;
    println!("M8_LIBBPF_ATTACH_PASS");
    Ok(())
}
