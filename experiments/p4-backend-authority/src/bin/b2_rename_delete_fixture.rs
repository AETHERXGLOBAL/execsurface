use std::ffi::CString;
use std::io;

const RENAME_NOREPLACE: u32 = 1;

fn cstring(value: &str) -> CString {
    CString::new(value).expect("fixture argument contains NUL")
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

fn require_zero(rc: i64, label: &str) {
    if rc != 0 {
        eprintln!("{label} failed rc={rc}: {}", io::Error::last_os_error());
        std::process::exit(43);
    }
}

fn raw_rename(source: &str, target: &str) -> i64 {
    let source = cstring(source);
    let target = cstring(target);
    unsafe { libc::syscall(libc::SYS_rename, source.as_ptr(), target.as_ptr()) as i64 }
}

fn raw_renameat(source_dirfd: i32, source: &str, target_dirfd: i32, target: &str) -> i64 {
    let source = cstring(source);
    let target = cstring(target);
    unsafe {
        libc::syscall(
            libc::SYS_renameat,
            source_dirfd,
            source.as_ptr(),
            target_dirfd,
            target.as_ptr(),
        ) as i64
    }
}

fn raw_renameat2(
    source_dirfd: i32,
    source: &str,
    target_dirfd: i32,
    target: &str,
    flags: u32,
) -> i64 {
    let source = cstring(source);
    let target = cstring(target);
    unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source_dirfd,
            source.as_ptr(),
            target_dirfd,
            target.as_ptr(),
            flags,
        ) as i64
    }
}

fn raw_unlink(target: &str) -> i64 {
    let target = cstring(target);
    unsafe { libc::syscall(libc::SYS_unlink, target.as_ptr()) as i64 }
}

fn raw_unlinkat(dirfd: i32, target: &str, flags: i32) -> i64 {
    let target = cstring(target);
    unsafe { libc::syscall(libc::SYS_unlinkat, dirfd, target.as_ptr(), flags) as i64 }
}

fn raw_rmdir(target: &str) -> i64 {
    let target = cstring(target);
    unsafe { libc::syscall(libc::SYS_rmdir, target.as_ptr()) as i64 }
}

fn open_dir(path: &str) -> i32 {
    let path = cstring(path);
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        eprintln!("open_dir failed: {}", io::Error::last_os_error());
        std::process::exit(44);
    }
    fd
}

fn main() {
    trace_me_and_stop();

    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).map(String::as_str).unwrap_or("");
    match scenario {
        "rename" => require_zero(raw_rename(&args[2], &args[3]), "rename"),
        "renameat" => {
            let source_dirfd = open_dir(&args[2]);
            let target_dirfd = open_dir(&args[4]);
            require_zero(
                raw_renameat(source_dirfd, &args[3], target_dirfd, &args[5]),
                "renameat",
            );
            unsafe {
                libc::close(source_dirfd);
                libc::close(target_dirfd);
            }
        }
        "renameat2-noreplace" => {
            let rc = raw_renameat2(
                libc::AT_FDCWD,
                &args[2],
                libc::AT_FDCWD,
                &args[3],
                RENAME_NOREPLACE,
            );
            if rc == -1 && io::Error::last_os_error().raw_os_error() == Some(libc::ENOSYS) {
                std::process::exit(45);
            }
            require_zero(rc, "renameat2-noreplace");
        }
        "unlink" => require_zero(raw_unlink(&args[2]), "unlink"),
        "unlinkat" => require_zero(raw_unlinkat(libc::AT_FDCWD, &args[2], 0), "unlinkat"),
        "rmdir" => require_zero(raw_rmdir(&args[2]), "rmdir"),
        "missing-rename" => {
            let rc = raw_rename(&args[2], &args[3]);
            if rc >= 0 {
                eprintln!("missing-rename unexpectedly succeeded");
                std::process::exit(46);
            }
        }
        other => {
            eprintln!("unknown fixture scenario: {other}");
            std::process::exit(47);
        }
    }
}
