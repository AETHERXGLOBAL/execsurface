use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let first = thread::spawn(|| -> Result<(), String> {
        let second = thread::spawn(|| {
            thread::sleep(Duration::from_millis(80));
        });
        second
            .join()
            .map_err(|_| "nested thread panicked".to_owned())?;
        thread::sleep(Duration::from_millis(80));
        Ok(())
    });

    first
        .join()
        .map_err(|_| "first thread panicked")?
        .map_err(|error| format!("first thread failed: {error}"))?;

    thread::sleep(Duration::from_millis(120));
    Ok(())
}
