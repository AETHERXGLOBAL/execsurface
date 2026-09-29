from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text()

text = replace_once(
    text,
    """            let mechanism = if clone_origin {\n                SpawnMechanism::Clone\n            } else {\n                match event {\n                    libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,\n                    libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,\n                    _ => SpawnMechanism::Clone,\n                }\n            };\n""",
    """            // Preserve public raw-v2 ProcessSpawn semantics: mechanism remains\n            // event-label based. Syscall-origin truth is retained separately in the\n            // internal completeness certificate and must not silently reinterpret v2.\n            let mechanism = match event {\n                libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,\n                libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,\n                _ => SpawnMechanism::Clone,\n            };\n""",
    "restore public v2 mechanism compatibility",
)

fork_old = """        assert!(\n            observed.clone_fd_certification.clone_origin_fork_events >= 1,\n            \"Linux should route clone(..., CLONE_FILES|SIGCHLD) through PTRACE_EVENT_FORK under the declared options\"\n        );\n        assert!(observed.observation.events.iter().any(|event| matches!(\n            &event.kind,\n            RawEventKind::ProcessSpawn {\n                mechanism: SpawnMechanism::Clone,\n                ..\n            }\n        )));\n"""
fork_new = """        assert!(\n            observed.clone_fd_certification.clone_origin_fork_events >= 1,\n            \"Linux should route clone(..., CLONE_FILES|SIGCHLD) through PTRACE_EVENT_FORK under the declared options\"\n        );\n        assert!(observed.observation.events.iter().any(|event| matches!(\n            &event.kind,\n            RawEventKind::ProcessSpawn {\n                mechanism: SpawnMechanism::Fork,\n                ..\n            }\n        )));\n"""
text = replace_once(text, fork_old, fork_new, "C3-C raw-v2 compatibility assertion")

vfork_old = """        assert!(\n            observed.clone_fd_certification.clone_origin_vfork_events >= 1,\n            \"Linux should route CLONE_VFORK clone origin through PTRACE_EVENT_VFORK under the declared options\"\n        );\n        assert!(observed.observation.events.iter().any(|event| matches!(\n            &event.kind,\n            RawEventKind::ProcessSpawn {\n                mechanism: SpawnMechanism::Clone,\n                ..\n            }\n        )));\n"""
vfork_new = """        assert!(\n            observed.clone_fd_certification.clone_origin_vfork_events >= 1,\n            \"Linux should route CLONE_VFORK clone origin through PTRACE_EVENT_VFORK under the declared options\"\n        );\n        assert!(observed.observation.events.iter().any(|event| matches!(\n            &event.kind,\n            RawEventKind::ProcessSpawn {\n                mechanism: SpawnMechanism::Vfork,\n                ..\n            }\n        )));\n"""
text = replace_once(text, vfork_old, vfork_new, "C3-D raw-v2 compatibility assertion")

path.write_text(text)
