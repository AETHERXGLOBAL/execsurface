use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"))
        .join("file_open.skel.rs");

    libbpf_cargo::SkeletonBuilder::new()
        .source("src/bpf/file_open.bpf.c")
        .clang_args([
            OsStr::new("-I"),
            OsStr::new("/usr/include/x86_64-linux-gnu"),
        ])
        .build_and_generate(&out)
        .expect("build and generate M10.3 BPF-LSM skeleton");

    println!("cargo:rerun-if-changed=src/bpf/file_open.bpf.c");
}
