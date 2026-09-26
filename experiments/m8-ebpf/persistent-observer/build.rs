use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let out =
        PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR")).join("persistent.skel.rs");

    let mut clang_args = vec![
        OsStr::new("-I"),
        OsStr::new("/usr/include/x86_64-linux-gnu"),
    ];
    if std::env::var_os("M8_7_FAULT_SMALL_TASK_MAP").is_some() {
        clang_args.push(OsStr::new("-DM8_7_TASK_EPOCH_MAX_ENTRIES=8"));
    }

    libbpf_cargo::SkeletonBuilder::new()
        .source("src/bpf/persistent.bpf.c")
        .clang_args(clang_args)
        .build_and_generate(&out)
        .expect("build and generate persistent libbpf observer skeleton");

    println!("cargo:rerun-if-changed=src/bpf/persistent.bpf.c");
    println!("cargo:rerun-if-env-changed=M8_7_FAULT_SMALL_TASK_MAP");
}
