use std::cell::RefCell;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::mem::MaybeUninit;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::RingBufferBuilder;

mod probe {
    include!(concat!(env!("OUT_DIR"), "/probe.skel.rs"));
}

use probe::*;

const EVENT_EXEC: u32 = 1;
const EVENT_FILE_OPEN: u32 = 3;
const SECRET_SENTINEL: &str = "AX_M8_3_PATH_BRIDGE_SECRET_DO_NOT_PERSIST";

#[derive(Debug, Clone, Copy)]
struct MetadataEvent {
    kind: u32,
    pid: u32,
    value: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    match std::env::args().nth(1).as_deref() {
        Some("--hold-open-child") => return deterministic_open_child(true),
        Some("--short-open-child") => return deterministic_open_child(false),
        _ => {}
    }

    let builder = ProbeSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;
    skel.attach()?;

    let events = Rc::new(RefCell::new(Vec::<MetadataEvent>::new()));
    let callback_events = Rc::clone(&events);
    let mut ring_builder = RingBufferBuilder::new();
    ring_builder.add(&skel.maps.events, move |data| match decode_event(data) {
        Ok(event) => {
            callback_events.borrow_mut().push(event);
            0
        }
        Err(message) => {
            eprintln!("M8_3_PATH_BRIDGE_DECODE_ERROR {message}");
            -1
        }
    })?;
    let ring = ring_builder.build()?;

    let mut held_exec = Command::new("/bin/sleep")
        .arg("2")
        .env("AX_M8_3_SECRET", SECRET_SENTINEL)
        .spawn()?;
    let held_exec_pid = held_exec.id();
    poll_for(&ring, &events, EVENT_EXEC, held_exec_pid)?;
    let held_exec_path = fs::read_link(format!("/proc/{held_exec_pid}/exe"))?;
    if held_exec_path.file_name() != Some(OsStr::new("sleep")) {
        return Err(format!(
            "held exec resolved to stale or wrong identity: {}",
            held_exec_path.display()
        )
        .into());
    }
    println!(
        "M8_3_EXEC_HELD_RESOLUTION_PASS pid={held_exec_pid} path={}",
        held_exec_path.display()
    );
    let _ = held_exec.kill();
    let _ = held_exec.wait();

    let mut short_exec = Command::new("/bin/true")
        .env("AX_M8_3_SECRET", SECRET_SENTINEL)
        .spawn()?;
    let short_exec_pid = short_exec.id();
    let short_status = short_exec.wait()?;
    if !short_status.success() {
        return Err("short exec fixture failed".into());
    }
    poll_for(&ring, &events, EVENT_EXEC, short_exec_pid)?;
    if let Ok(path) = fs::read_link(format!("/proc/{short_exec_pid}/exe")) {
        return Err(format!(
            "reaped exec unexpectedly resolved; race harness is not valid: {}",
            path.display()
        )
        .into());
    }
    println!("M8_3_EXEC_REAPED_RESOLUTION_UNAVAILABLE_PASS pid={short_exec_pid}");

    let probe_exe = std::env::current_exe()?;
    let mut held_open = Command::new(&probe_exe)
        .arg("--hold-open-child")
        .env("AX_M8_3_SECRET", SECRET_SENTINEL)
        .spawn()?;
    let held_open_pid = held_open.id();
    let (held_fd, held_path) = resolve_live_open(&ring, &events, held_open_pid)?;
    if held_path != std::path::PathBuf::from("/dev/null") {
        return Err(format!(
            "held open resolved to unexpected target: {}",
            held_path.display()
        )
        .into());
    }
    println!(
        "M8_3_OPEN_HELD_RESOLUTION_PASS pid={held_open_pid} fd={held_fd} path={}",
        held_path.display()
    );
    let _ = held_open.kill();
    let _ = held_open.wait();

    let mut short_open = Command::new(&probe_exe)
        .arg("--short-open-child")
        .env("AX_M8_3_SECRET", SECRET_SENTINEL)
        .spawn()?;
    let short_open_pid = short_open.id();
    let short_open_status = short_open.wait()?;
    if !short_open_status.success() {
        return Err("short open fixture failed".into());
    }
    let short_event = poll_for(&ring, &events, EVENT_FILE_OPEN, short_open_pid)?;
    if let Ok(path) = fs::read_link(format!("/proc/{short_open_pid}/fd/{}", short_event.value)) {
        return Err(format!(
            "reaped fd unexpectedly resolved; race harness is not valid: {}",
            path.display()
        )
        .into());
    }
    println!(
        "M8_3_OPEN_REAPED_RESOLUTION_UNAVAILABLE_PASS pid={short_open_pid} fd={}",
        short_event.value
    );

    println!("M8_3_PATH_BRIDGE_CONDITIONAL_FAIL_CLOSED_PASS");
    Ok(())
}

fn deterministic_open_child(hold: bool) -> Result<(), Box<dyn Error>> {
    let path = b"/dev/null\0";
    let fd = unsafe {
        libc::openat(
            libc::AT_FDCWD,
            path.as_ptr().cast::<libc::c_char>(),
            libc::O_RDONLY,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if hold {
        std::thread::sleep(Duration::from_secs(2));
    }
    let rc = unsafe { libc::close(fd) };
    if rc != 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn poll_for(
    ring: &libbpf_rs::RingBuffer<'_>,
    events: &Rc<RefCell<Vec<MetadataEvent>>>,
    kind: u32,
    pid: u32,
) -> Result<MetadataEvent, Box<dyn Error>> {
    for _ in 0..80 {
        ring.poll(Duration::from_millis(25))?;
        if let Some(event) = events
            .borrow()
            .iter()
            .copied()
            .find(|event| event.kind == kind && event.pid == pid)
        {
            return Ok(event);
        }
    }
    Err(format!("timed out waiting for event kind={kind} pid={pid}").into())
}

fn resolve_live_open(
    ring: &libbpf_rs::RingBuffer<'_>,
    events: &Rc<RefCell<Vec<MetadataEvent>>>,
    pid: u32,
) -> Result<(u32, std::path::PathBuf), Box<dyn Error>> {
    for _ in 0..80 {
        ring.poll(Duration::from_millis(25))?;
        let candidates = events
            .borrow()
            .iter()
            .copied()
            .filter(|event| event.kind == EVENT_FILE_OPEN && event.pid == pid)
            .collect::<Vec<_>>();
        for event in candidates {
            if let Ok(path) = fs::read_link(format!("/proc/{pid}/fd/{}", event.value)) {
                if path == std::path::PathBuf::from("/dev/null") {
                    return Ok((event.value, path));
                }
            }
        }
    }
    Err(format!("timed out resolving held /dev/null fd for pid={pid}").into())
}

fn decode_event(data: &[u8]) -> Result<MetadataEvent, String> {
    if data.len() != 16 {
        return Err(format!("unexpected event size: {}", data.len()));
    }
    let kind = u32::from_ne_bytes(data[0..4].try_into().map_err(|_| "kind")?);
    let pid = u32::from_ne_bytes(data[4..8].try_into().map_err(|_| "pid")?);
    let value = u32::from_ne_bytes(data[8..12].try_into().map_err(|_| "value")?);
    if pid == 0 {
        return Err("zero process identity".to_owned());
    }
    if !matches!(kind, EVENT_EXEC | EVENT_FILE_OPEN) {
        return Err(format!("unknown metadata event kind: {kind}"));
    }
    Ok(MetadataEvent { kind, pid, value })
}
