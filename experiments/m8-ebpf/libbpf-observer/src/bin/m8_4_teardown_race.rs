use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let delay_ms = std::env::args()
        .nth(1)
        .ok_or("usage: m8_4_teardown_race DELAY_MS")?
        .parse::<u64>()?;

    let pid = unsafe { libc::fork() };
    if pid < 0 {
        return Err(std::io::Error::last_os_error().into());
    }

    if pid == 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
        let path = b"/dev/null\0";
        let fd = unsafe {
            libc::openat(
                libc::AT_FDCWD,
                path.as_ptr().cast::<libc::c_char>(),
                libc::O_RDONLY | libc::O_CLOEXEC,
            )
        };
        if fd >= 0 {
            unsafe {
                libc::close(fd);
            }
            unsafe {
                libc::_exit(0);
            }
        }
        unsafe {
            libc::_exit(2);
        }
    }

    Ok(())
}
