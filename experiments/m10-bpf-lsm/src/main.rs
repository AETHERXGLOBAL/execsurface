// SPDX-License-Identifier: Apache-2.0
use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{MapCore, MapFlags, RingBufferBuilder};
use serde::Serialize;

mod file_open {
    include!(concat!(env!("OUT_DIR"), "/file_open.skel.rs"));
}
use file_open::*;

#[derive(Debug, Clone, Serialize)]
struct Event {
    session_id: u64,
    tgid: u32,
    tid: u32,
    inode: u64,
    dev: u32,
    flags: u32,
}

#[derive(Debug, Serialize)]
struct Report {
    protocol: &'static str,
    attachment_available: bool,
    session_registered: bool,
    root_tgid: u32,
    target_exit_code: Option<i32>,
    target_signal: Option<i32>,
    target_stdout: String,
    producer_drops: u64,
    decode_errors: u64,
    events: Vec<Event>,
}

fn decode_event(data: &[u8]) -> Result<Event, String> {
    if data.len() != 32 {
        return Err(format!("unexpected event length {}", data.len()));
    }
    Ok(Event {
        session_id: u64::from_ne_bytes(data[0..8].try_into().unwrap()),
        tgid: u32::from_ne_bytes(data[8..12].try_into().unwrap()),
        tid: u32::from_ne_bytes(data[12..16].try_into().unwrap()),
        inode: u64::from_ne_bytes(data[16..24].try_into().unwrap()),
        dev: u32::from_ne_bytes(data[24..28].try_into().unwrap()),
        flags: u32::from_ne_bytes(data[28..32].try_into().unwrap()),
    })
}

fn read_drop_count(map: &impl MapCore) -> Result<u64, Box<dyn Error>> {
    let key = 0u32.to_ne_bytes();
    let value = map
        .lookup(&key, MapFlags::ANY)?
        .ok_or("missing drop-counter entry")?;
    if value.len() != 8 {
        return Err(format!("unexpected drop-counter size {}", value.len()).into());
    }
    Ok(u64::from_ne_bytes(value[..8].try_into().unwrap()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let mut report_path: Option<PathBuf> = None;
    let mut target = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--report" {
            report_path = Some(PathBuf::from(args.next().ok_or("--report requires a path")?));
        } else if arg == "--" {
            target.extend(args);
            break;
        } else {
            return Err(format!("unexpected argument {:?}", arg).into());
        }
    }
    let report_path = report_path.ok_or("missing --report")?;
    if target.is_empty() {
        return Err("missing target after --".into());
    }

    let builder = FileOpenSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;
    skel.attach()?;

    let events = Rc::new(RefCell::new(Vec::<Event>::new()));
    let decode_errors = Rc::new(RefCell::new(0u64));
    let cb_events = Rc::clone(&events);
    let cb_errors = Rc::clone(&decode_errors);
    let mut ring_builder = RingBufferBuilder::new();
    ring_builder.add(&skel.maps.events, move |data| {
        match decode_event(data) {
            Ok(event) => cb_events.borrow_mut().push(event),
            Err(_) => *cb_errors.borrow_mut() += 1,
        }
        0
    })?;
    let ring = ring_builder.build()?;

    let mut command = Command::new(&target[0]);
    command.args(&target[1..]);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::inherit());
    let mut child = command.spawn()?;
    let root_tgid = child.id();

    let mut wait_status: libc::c_int = 0;
    let waited = unsafe { libc::waitpid(root_tgid as libc::pid_t, &mut wait_status, libc::WUNTRACED) };
    if waited != root_tgid as libc::pid_t || !libc::WIFSTOPPED(wait_status) {
        let _ = child.kill();
        return Err(format!("target failed to enter registration stop: waitpid={waited} status={wait_status:#x}").into());
    }

    let session_id = 0x4d313003u64;
    skel.maps.sessions.update(
        &root_tgid.to_ne_bytes(),
        &session_id.to_ne_bytes(),
        MapFlags::ANY,
    )?;
    let session_registered = true;

    if unsafe { libc::kill(root_tgid as libc::pid_t, libc::SIGCONT) } != 0 {
        let _ = child.kill();
        return Err(std::io::Error::last_os_error().into());
    }

    let status = loop {
        ring.poll(Duration::from_millis(10))?;
        if let Some(status) = child.try_wait()? {
            break status;
        }
    };
    for _ in 0..5 {
        ring.poll(Duration::from_millis(10))?;
        thread::sleep(Duration::from_millis(5));
    }

    let mut target_stdout = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut target_stdout)?;
    }

    let producer_drops = read_drop_count(&skel.maps.dropped)?;
    let _ = skel.maps.sessions.delete(&root_tgid.to_ne_bytes());

    #[cfg(unix)]
    let signal = {
        use std::os::unix::process::ExitStatusExt;
        status.signal()
    };
    #[cfg(not(unix))]
    let signal = None;

    let report = Report {
        protocol: "M10_3_BPF_LSM_FILE_001",
        attachment_available: true,
        session_registered,
        root_tgid,
        target_exit_code: status.code(),
        target_signal: signal,
        target_stdout,
        producer_drops,
        decode_errors: *decode_errors.borrow(),
        events: events.borrow().clone(),
    };
    fs::write(report_path, serde_json::to_vec_pretty(&report)?)?;
    Ok(())
}
