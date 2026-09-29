use std::ffi::CString;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct OpenHow {
    flags: u64,
    mode: u64,
    resolve: u64,
}

fn cstring(value: &str) -> CString {
    CString::new(value).expect("fixture argument contains NUL")
}

fn raw_open(path: &str) -> i64 {
    let path = cstring(path);
    unsafe { libc::syscall(libc::SYS_open, path.as_ptr(), libc::O_RDONLY | libc::O_CLOEXEC) as i64 }
}

fn raw_openat(dirfd: i32, path: &str) -> i64 {
    let path = cstring(path);
    unsafe {
        libc::syscall(
            libc::SYS_openat,
            dirfd,
            path.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC,
            0,
        ) as i64
    }
}

fn raw_openat2(path: &str) -> i64 {
    let path = cstring(path);
    let how = OpenHow {
        flags: (libc::O_RDONLY | libc::O_CLOEXEC) as u64,
        mode: 0,
        resolve: 0,
    };
    unsafe {
        libc::syscall(
            libc::SYS_openat2,
            libc::AT_FDCWD,
            path.as_ptr(),
            &how as *const OpenHow,
            std::mem::size_of::<OpenHow>(),
        ) as i64
    }
}

fn close_fd(fd: i64) {
    if fd >= 0 {
        unsafe {
            libc::close(fd as i32);
        }
    }
}

fn require_success(fd: i64, label: &str) -> i64 {
    if fd >= 0 {
        fd
    } else {
        eprintln!("{label} failed: {}", io::Error::last_os_error());
        std::process::exit(41);
    }
}

fn trace_me_and_stop() {
    unsafe {
        let rc = libc::ptrace(
            libc::PTRACE_TRACEME,
            0,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        );
        if rc != 0 {
            eprintln!("PTRACE_TRACEME failed: {}", io::Error::last_os_error());
            std::process::exit(42);
        }
        libc::raise(libc::SIGSTOP);
    }
}

fn main() {
    trace_me_and_stop();

    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).map(String::as_str).unwrap_or("");
    match scenario {
        "open" => {
            let fd = require_success(raw_open(&args[2]), "open");
            close_fd(fd);
        }
        "openat-cwd" => {
            let fd = require_success(raw_openat(libc::AT_FDCWD, &args[2]), "openat-cwd");
            close_fd(fd);
        }
        "openat-dir" => {
            let dir_path = cstring(&args[2]);
            let dirfd = unsafe {
                libc::syscall(
                    libc::SYS_open,
                    dir_path.as_ptr(),
                    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
                ) as i64
            };
            let dirfd = require_success(dirfd, "open-directory");
            let fd = require_success(raw_openat(dirfd as i32, &args[3]), "openat-dir");
            close_fd(fd);
            close_fd(dirfd);
        }
        "openat2" => {
            let fd = require_success(raw_openat2(&args[2]), "openat2");
            close_fd(fd);
        }
        "immediate-close" => {
            let fd = require_success(raw_open(&args[2]), "immediate-close");
            close_fd(fd);
        }
        "missing" => {
            let fd = raw_open(&args[2]);
            if fd >= 0 {
                close_fd(fd);
                eprintln!("missing-path fixture unexpectedly opened target");
                std::process::exit(43);
            }
        }
        "fd-reuse" => {
            let first = require_success(raw_open(&args[2]), "fd-reuse-first");
            close_fd(first);
            let second = require_success(raw_open(&args[3]), "fd-reuse-second");
            close_fd(second);
        }
        "threaded" => {
            let (tx, rx) = mpsc::channel();
            let worker = thread::spawn(move || {
                tx.send(()).expect("signal worker ready");
                thread::sleep(Duration::from_millis(900));
            });
            rx.recv().expect("worker ready");
            let fd = require_success(raw_open(&args[2]), "threaded-open");
            close_fd(fd);
            worker.join().expect("worker join");
        }
        other => {
            eprintln!("unknown fixture scenario: {other}");
            std::process::exit(44);
        }
    }
}