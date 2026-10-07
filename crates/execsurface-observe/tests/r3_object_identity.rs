#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::os::unix::fs::symlink;
use std::process;

use execsurface_model::{FileOperation, RawEventKind};
use execsurface_observe::{observe_command, CommandSpec};

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_execsurface-fixture")
}

fn divergence_warning(observation: &execsurface_model::Observation) -> bool {
    observation
        .warnings
        .iter()
        .any(|warning| warning.code == "side_effectful_open_identity_divergence")
}

#[test]
fn r3_symlink_truncate_fails_closed_when_open_time_object_identity_diverges() {
    let root =
        std::env::temp_dir().join(format!("execsurface-stage2-r3-symlink-{}", process::id()));
    let workspace = root.join("workspace");
    let target = root.join("target");
    let link = workspace.join("link");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&workspace).expect("create workspace");
    fs::write(&target, b"secret").expect("seed target");
    symlink("../target", &link).expect("create relative symlink");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("stage2-symlink-truncate")
            .arg(link.as_os_str()),
    )
    .expect("observe symlink truncate");

    let lexical = link.to_string_lossy().into_owned();
    let target_len = fs::metadata(&target).expect("target metadata").len();
    let lexical_attempt_retained = observation.events.iter().any(|event| {
        matches!(
            &event.kind,
            RawEventKind::FilePathAccess {
                operation: FileOperation::Open,
                path,
                ..
            } if path == &lexical
        )
    });

    let _ = fs::remove_dir_all(&root);

    assert_eq!(
        target_len, 0,
        "kernel-resolved target must actually be truncated"
    );
    assert!(
        lexical_attempt_retained,
        "lexical open intent must remain visible"
    );
    assert!(
        divergence_warning(&observation),
        "side-effectful lexical/kernel identity divergence must be explicit"
    );
    assert!(
        !observation.complete,
        "raw v2 cannot claim complete authority for an open-time side effect whose resolved object identity is not serialized"
    );
}

#[test]
fn r3_direct_truncate_stays_complete_when_lexical_and_kernel_identity_agree() {
    let path = std::env::temp_dir().join(format!(
        "execsurface-stage2-r3-direct-{}.txt",
        process::id()
    ));
    fs::write(&path, b"secret").expect("seed direct target");

    let observation = observe_command(
        &CommandSpec::new(fixture())
            .arg("stage2-symlink-truncate")
            .arg(path.as_os_str()),
    )
    .expect("observe direct truncate");

    let target_len = fs::metadata(&path).expect("direct target metadata").len();
    let _ = fs::remove_file(&path);

    assert_eq!(target_len, 0, "direct target must be truncated");
    assert!(
        !divergence_warning(&observation),
        "matching lexical/kernel identity must not invent a divergence warning"
    );
    assert!(
        observation.complete,
        "direct side-effectful open with matching object identity must remain complete: {:?}",
        observation.warnings
    );
}
