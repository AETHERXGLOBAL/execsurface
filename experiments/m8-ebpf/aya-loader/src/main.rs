use std::path::PathBuf;

use anyhow::{Context, Result};
use aya::{programs::TracePoint, Ebpf};

fn main() -> Result<()> {
    let object = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .context("usage: execsurface-m8-aya-loader <ebpf-object>")?;

    let mut ebpf = Ebpf::load_file(&object)
        .with_context(|| format!("failed to load {}", object.display()))?;
    let program: &mut TracePoint = ebpf
        .program_mut("execsurface_m8_exec")
        .context("missing execsurface_m8_exec program")?
        .try_into()?;
    program.load()?;
    let _link = program.attach("syscalls", "sys_enter_execve")?;

    println!("M8_AYA_ATTACH_PASS");
    Ok(())
}
