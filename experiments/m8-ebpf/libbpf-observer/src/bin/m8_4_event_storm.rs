use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let threads = args
        .next()
        .ok_or("usage: m8_4_event_storm THREADS ITERATIONS")?
        .parse::<usize>()?;
    let iterations = args
        .next()
        .ok_or("usage: m8_4_event_storm THREADS ITERATIONS")?
        .parse::<usize>()?;
    if args.next().is_some() || threads == 0 || iterations == 0 {
        return Err(
            "threads and iterations must be positive and no extra arguments are allowed".into(),
        );
    }

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        handles.push(thread::spawn(move || -> Result<(), String> {
            let path = b"/dev/null\0";
            for _ in 0..iterations {
                let fd = unsafe {
                    libc::openat(
                        libc::AT_FDCWD,
                        path.as_ptr().cast::<libc::c_char>(),
                        libc::O_RDONLY | libc::O_CLOEXEC,
                    )
                };
                if fd < 0 {
                    return Err(std::io::Error::last_os_error().to_string());
                }
                if unsafe { libc::close(fd) } != 0 {
                    return Err(std::io::Error::last_os_error().to_string());
                }
            }
            Ok(())
        }));
    }

    for handle in handles {
        handle
            .join()
            .map_err(|_| "event-storm worker panicked")?
            .map_err(|error| format!("event-storm worker failed: {error}"))?;
    }

    Ok(())
}
