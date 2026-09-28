use std::fs;
use std::process;

use execsurface_observe::{observe_command, CommandSpec};

#[test]
fn argv_value_sentinel_is_not_captured_in_observation() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let sentinel = format!(
        "M11_ARGV_SECRET_{}_DO_NOT_CAPTURE_7f6b3a9c",
        process::id()
    );

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("noop")
            .arg(sentinel.as_str()),
    )
    .expect("observe noop with private argv sentinel");

    let encoded = serde_json::to_string(&observation).expect("serialize observation");
    assert!(
        !encoded.contains(&sentinel),
        "metadata-only observation must not capture argv values"
    );
}

#[test]
fn file_content_sentinel_is_not_captured_in_observation() {
    let fixture = env!("CARGO_BIN_EXE_execsurface-fixture");
    let path = std::env::temp_dir().join(format!(
        "execsurface-m11-privacy-{}-fixture.txt",
        process::id()
    ));
    let sentinel = format!(
        "M11_FILE_CONTENT_SECRET_{}_DO_NOT_CAPTURE_2c91e5d4",
        process::id()
    );
    fs::write(&path, sentinel.as_bytes()).expect("write privacy fixture");

    let observation = observe_command(
        &CommandSpec::new(fixture)
            .arg("file")
            .arg(path.as_os_str()),
    )
    .expect("observe file-content privacy fixture");

    let _ = fs::remove_file(&path);

    let encoded = serde_json::to_string(&observation).expect("serialize observation");
    assert!(
        !encoded.contains(&sentinel),
        "metadata-only observation must not capture file content"
    );
}
