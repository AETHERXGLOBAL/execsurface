use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"))
        .join("probe.skel.rs");

    // Ubuntu/Debian keep architecture UAPI headers such as asm/types.h under the
    // multiarch include directory. clang -target bpf does not add that host path
    // automatically, so the experiment declares it explicitly instead of relying
    // on ambient compiler behavior.
    libbpf_cargo::SkeletonBuilder::new()
        .source("src/bpf/probe.bpf.c")
        .clang_args([
            OsStr::new("-I"),
            OsStr::new("/usr/include/x86_64-linux-gnu"),
        ])
        .build_and_generate(&out)
        .expect("build and generate libbpf skeleton");

    println!("cargo:rerun-if-changed=src/bpf/probe.bpf.c");
}
