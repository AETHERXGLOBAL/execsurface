use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "fork-exec".to_owned());

    match mode.as_str() {
        "fork-exec" => run_fork_exec(),
        "vfork-exec" => run_vfork_exec(),
        "nested-thread-fork" => run_nested_thread_fork(),
        other => Err(format!("unknown fixture mode: {other}").into()),
    }
}

fn open_probe() -> Result<File, Box<dyn Error>> {
    let mut file = File::open("/dev/zero")?;
    let mut byte = [0_u8; 1];
    file.read_exact(&mut byte)?;
    Ok(file)
}

fn run_fork_exec() -> Result<(), Box<dyn Error>> {
    let file = open_probe()?;
    fork_exec_true()?;

    // Keep the root and the opened descriptor alive briefly so the eBPF
    // userspace resolver can establish identities without relying on a race.
    thread::sleep(Duration::from_millis(120));
    drop(file);
    Ok(())
}

fn run_vfork_exec() -> Result<(), Box<dyn Error>> {
    let file = open_probe()?;
    vfork_exec_true()?;
    thread::sleep(Duration::from_millis(120));
    drop(file);
    Ok(())
}

fn run_nested_thread_fork() -> Result<(), Box<dyn Error>> {
    let file = open_probe()?;
    let worker = thread::spawn(|| -> Result<(), String> {
        fork_exec_true().map_err(|error| error.to_string())?;
        // Keep the non-root creator task alive briefly after its child exits;
        // this makes parent-role evidence deterministic without depending on
        // scheduler timing.
        thread::sleep(Duration::from_millis(80));
        Ok(())
    });

    worker
        .join()
        .map_err(|_| "nested-thread worker panicked")?
        .map_err(|error| format!("nested-thread worker failed: {error}"))?;

    thread::sleep(Duration::from_millis(120));
    drop(file);
    Ok(())
}

fn fork_exec_true() -> Result<(), Box<dyn Error>> {
    let child = unsafe { libc::fork() };
    if child < 0 {
        return Err(std::io::Error::last_os_error().into());
    }

    if child == 0 {
        let path = CString::new("/bin/true")?;
        let arg0 = CString::new("true")?;
        unsafe {
            libc::execl(path.as_ptr(), arg0.as_ptr(), ptr::null::<libc::c_char>());
            libc::_exit(127);
        }
    }

    wait_clean_child(child)
}

fn vfork_exec_true() -> Result<(), Box<dyn Error>> {
    // Prepare every object before vfork. The child performs only async-safe C
    // calls before exec/_exit and does not touch Rust-owned state.
    let path = CString::new("/bin/true")?;
    let arg0 = CString::new("true")?;
    let child = unsafe { libc::vfork() };
    if child < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if child == 0 {
        unsafe {
            libc::execl(path.as_ptr(), arg0.as_ptr(), ptr::null::<libc::c_char>());
            libc::_exit(127);
        }
    }
    wait_clean_child(child)
}

fn wait_clean_child(child: libc::pid_t) -> Result<(), Box<dyn Error>> {
    let mut status = 0_i32;
    let waited = unsafe { libc::waitpid(child, &mut status, 0) };
    if waited != child {
        return Err(std::io::Error::last_os_error().into());
    }
    if !libc::WIFEXITED(status) || libc::WEXITSTATUS(status) != 0 {
        return Err(format!("child did not exit cleanly: status={status}").into());
    }
    Ok(())
}
