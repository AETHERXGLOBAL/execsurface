use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"))
        .join("probe.skel.rs");

    libbpf_cargo::SkeletonBuilder::new()
        .source("src/bpf/probe.bpf.c")
        .build_and_generate(&out)
        .expect("build and generate libbpf skeleton");

    println!("cargo:rerun-if-changed=src/bpf/probe.bpf.c");
}
