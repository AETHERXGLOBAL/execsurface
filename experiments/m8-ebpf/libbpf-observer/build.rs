use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR")).join("observer.skel.rs");

    libbpf_cargo::SkeletonBuilder::new()
        .source("src/bpf/observer.bpf.c")
        .clang_args([
            OsStr::new("-I"),
            OsStr::new("/usr/include/x86_64-linux-gnu"),
        ])
        .build_and_generate(&out)
        .expect("build and generate libbpf observer skeleton");

    println!("cargo:rerun-if-changed=src/bpf/observer.bpf.c");
}
