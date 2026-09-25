use std::env;
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::FileExt;
use std::process::Command;
use std::sync::Arc;
use std::thread;

#[repr(C)]
struct OpenHow {
    flags: u64,
    mode: u64,
    resolve: u64,
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("noop") => {}
        Some("spawn") => {
            let status = Command::new("/bin/true").status().expect("spawn /bin/true");
            assert!(status.success());
        }
        Some("file") => {
            let path = args.next().expect("file path");
            let _ = fs::read(path).expect("read fixture file");
        }
        Some("file-rw") => {
            let path = args.next().expect("file path");
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect("open fixture file");
            let mut byte = [0_u8; 1];
            file.read_exact(&mut byte).expect("read fixture byte");
            file.seek(SeekFrom::End(0)).expect("seek end");
            file.write_all(b"x").expect("write fixture byte");
            file.flush().expect("flush");
        }
        Some("relative-file") => {
            let dir = args.next().expect("directory");
            let name = args.next().expect("relative file name");
            env::set_current_dir(dir).expect("chdir");
            let _ = fs::read(name).expect("read relative fixture file");
        }
        Some("dup-read") => {
            let path = args.next().expect("file path");
            let file = File::open(path).expect("open");
            let duplicated = unsafe { libc::dup(file.as_raw_fd()) };
            assert!(duplicated >= 0);
            let mut duplicated = unsafe { File::from_raw_fd(duplicated) };
            let mut byte = [0_u8; 1];
            duplicated
                .read_exact(&mut byte)
                .expect("read duplicated fd");
        }
        Some("fork-read") => {
            let path = args.next().expect("file path");
            let file = File::open(path).expect("open");
            let fd = file.as_raw_fd();
            let child = unsafe { libc::fork() };
            assert!(child >= 0);
            if child == 0 {
                let mut byte = [0_u8; 1];
                let result = unsafe { libc::read(fd, byte.as_mut_ptr().cast(), 1) };
                unsafe { libc::_exit(if result == 1 { 0 } else { 1 }) };
            }
            let mut status = 0;
            assert_eq!(unsafe { libc::waitpid(child, &mut status, 0) }, child);
            assert!(libc::WIFEXITED(status));
            assert_eq!(libc::WEXITSTATUS(status), 0);
        }
        Some("thread-read") => {
            let path = args.next().expect("file path");
            let count: usize = args.next().expect("thread count").parse().expect("count");
            let file = Arc::new(File::open(path).expect("open"));
            let mut handles = Vec::new();
            for _ in 0..count {
                let file = Arc::clone(&file);
                handles.push(thread::spawn(move || {
                    let mut byte = [0_u8; 1];
                    file.read_at(&mut byte, 0).expect("pread shared fd");
                }));
            }
            for handle in handles {
                handle.join().expect("thread");
            }
        }
        Some("burst") => {
            let dir = args.next().expect("directory");
            let count: usize = args.next().expect("count").parse().expect("count");
            fs::create_dir_all(&dir).expect("create burst dir");
            for index in 0..count {
                let path = format!("{dir}/event-{index}.txt");
                fs::write(&path, b"x").expect("burst write");
                let _ = fs::read(&path).expect("burst read");
            }
        }
        Some("openat2") => {
            let path = args.next().expect("file path");
            let path = CString::new(path).expect("cstring");
            let how = OpenHow {
                flags: libc::O_RDONLY as u64,
                mode: 0,
                resolve: 0,
            };
            let fd = unsafe {
                libc::syscall(
                    libc::SYS_openat2,
                    libc::AT_FDCWD,
                    path.as_ptr(),
                    &how as *const OpenHow,
                    std::mem::size_of::<OpenHow>(),
                )
            } as i32;
            assert!(
                fd >= 0,
                "openat2 failed: {}",
                std::io::Error::last_os_error()
            );
            unsafe {
                libc::close(fd);
            }
        }
        Some("fault-path") => {
            let result = unsafe {
                libc::syscall(
                    libc::SYS_openat,
                    libc::AT_FDCWD,
                    1_usize as *const libc::c_char,
                    libc::O_RDONLY,
                    0,
                )
            };
            assert_eq!(result, -1);
        }
        Some("network") => {
            let address = args.next().expect("socket address");
            let _stream = TcpStream::connect(address).expect("connect fixture listener");
        }
        other => panic!("unknown fixture mode: {other:?}"),
    }
}
