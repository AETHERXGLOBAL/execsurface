use std::error::Error;
use std::io;
use std::path::PathBuf;

use aya::{programs::TracePoint, Ebpf};

fn main() -> Result<(), Box<dyn Error>> {
    let object = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "usage: execsurface-m8-aya-loader <ebpf-object>"))?;

    let mut ebpf = Ebpf::load_file(&object)?;
    let program: &mut TracePoint = ebpf
        .program_mut("execsurface_m8_exec")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing execsurface_m8_exec program"))?
        .try_into()?;
    program.load()?;
    let _link = program.attach("syscalls", "sys_enter_execve")?;

    println!("M8_AYA_ATTACH_PASS");
    Ok(())
}
