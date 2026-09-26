use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("/dev/zero")?;
    let mut byte = [0_u8; 1];
    file.read_exact(&mut byte)?;

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

    let mut status = 0_i32;
    let waited = unsafe { libc::waitpid(child, &mut status, 0) };
    if waited != child {
        return Err(std::io::Error::last_os_error().into());
    }
    if !libc::WIFEXITED(status) || libc::WEXITSTATUS(status) != 0 {
        return Err(format!("child did not exit cleanly: status={status}").into());
    }

    // Keep the root and the opened descriptor alive briefly so the eBPF
    // userspace resolver can establish identities without relying on a race.
    thread::sleep(Duration::from_millis(120));
    drop(file);
    Ok(())
}
