use std::env;

use execsurface_model::{RawEventKind, SpawnMechanism};
use execsurface_observe::{observe_command, CommandSpec};

fn main() {
    let work_root = env::args()
        .nth(1)
        .expect("usage: diagnostic <work-root>");
    let command = format!("cd '{}' && go test ./... >/dev/null 2>&1", work_root);

    let observation = observe_command(
        &CommandSpec::new("/bin/bash")
            .arg("-lc")
            .arg(command),
    )
    .expect("observer execution failed");

    let clone_spawns = observation
        .events
        .iter()
        .filter(|event| {
            matches!(
                &event.kind,
                RawEventKind::ProcessSpawn {
                    mechanism: SpawnMechanism::Clone,
                    ..
                }
            )
        })
        .count();

    println!("complete={}", observation.complete);
    println!("event_count={}", observation.events.len());
    println!("clone_spawn_count={clone_spawns}");
    println!("warning_count={}", observation.warnings.len());

    for (index, warning) in observation.warnings.iter().enumerate() {
        println!("warning[{index}].code={}", warning.code);
        println!("warning[{index}].tid={:?}", warning.tid);
        println!("warning[{index}].message={}", warning.message);
    }
}
