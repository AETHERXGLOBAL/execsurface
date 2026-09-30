use std::ffi::CString;
use std::io;
use std::mem;

fn trace_me_and_stop() {
    unsafe {
        if libc::ptrace(libc::PTRACE_TRACEME, 0, std::ptr::null_mut::<libc::c_void>(), std::ptr::null_mut::<libc::c_void>()) != 0 {
            eprintln!("PTRACE_TRACEME failed: {}", io::Error::last_os_error());
            std::process::exit(42);
        }
        libc::raise(libc::SIGSTOP);
    }
}

fn socket(nonblocking: bool) -> i32 {
    let mut ty = libc::SOCK_STREAM | libc::SOCK_CLOEXEC;
    if nonblocking { ty |= libc::SOCK_NONBLOCK; }
    let fd = unsafe { libc::socket(libc::AF_INET, ty, 0) };
    if fd < 0 { eprintln!("socket failed: {}", io::Error::last_os_error()); std::process::exit(41); }
    fd
}

fn addr(port: u16) -> libc::sockaddr_in {
    libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: libc::in_addr { s_addr: u32::from_ne_bytes([127,0,0,1]).to_be() },
        sin_zero: [0; 8],
    }
}

fn raw_connect(fd: i32, address: &libc::sockaddr_in) -> i64 {
    unsafe { libc::syscall(libc::SYS_connect, fd, address as *const _ as *const libc::sockaddr, mem::size_of::<libc::sockaddr_in>() as libc::socklen_t) as i64 }
}

fn malformed_connect(fd: i32, address: &libc::sockaddr_in) -> i64 {
    unsafe { libc::syscall(libc::SYS_connect, fd, address as *const _ as *const libc::sockaddr, 1usize) as i64 }
}

fn main() {
    trace_me_and_stop();
    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).map(String::as_str).unwrap_or("");
    let port: u16 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(9);
    match scenario {
        "connect" => { let fd=socket(false); let rc=raw_connect(fd,&addr(port)); unsafe { libc::close(fd); } if rc != 0 { std::process::exit(45); } }
        "failure" => { let fd=socket(false); let rc=raw_connect(fd,&addr(port)); unsafe { libc::close(fd); } if rc >= 0 { std::process::exit(46); } }
        "nonblocking" => { let fd=socket(true); let rc=raw_connect(fd,&addr(port)); unsafe { libc::close(fd); } if rc < 0 { let e=io::Error::last_os_error().raw_os_error().unwrap_or(0); if e != libc::EINPROGRESS && e != libc::ECONNREFUSED { std::process::exit(47); } } }
        "malformed" => { let fd=socket(false); let rc=malformed_connect(fd,&addr(port)); unsafe { libc::close(fd); } if rc >= 0 { std::process::exit(48); } }
        "fd-reuse" => { let first=socket(false); unsafe { libc::close(first); } let second=socket(false); let _=raw_connect(second,&addr(port)); unsafe { libc::close(second); } }
        other => { let _ = CString::new(other); eprintln!("unknown fixture scenario: {other}"); std::process::exit(44); }
    }
}
