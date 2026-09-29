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
    """    clone_thread_transitions: u64,\n    ambiguity: bool,\n""",
    """    clone_thread_transitions: u64,\n    clone_origin_fork_events: u64,\n    clone_origin_vfork_events: u64,\n    ambiguity: bool,\n""",
    "certificate event-route counters",
)

text = replace_once(
    text,
    """    fn invalidate(&mut self) {\n        self.ambiguity = true;\n    }\n""",
    """    fn record_clone_creation_event(&mut self, flags: Option<u64>, event: libc::c_int) {\n        self.record_clone(flags);\n        if event == libc::PTRACE_EVENT_FORK {\n            self.clone_origin_fork_events += 1;\n        } else if event == libc::PTRACE_EVENT_VFORK {\n            self.clone_origin_vfork_events += 1;\n        }\n    }\n\n    fn invalidate(&mut self) {\n        self.ambiguity = true;\n    }\n""",
    "syscall-origin event-route recorder",
)

old_event_block = """            let mechanism = match event {\n                libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,\n                libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,\n                _ => SpawnMechanism::Clone,\n            };\n\n            let parent_table = tracees\n                .get(&tid)\n                .map(|state| state.fd_table_id)\n                .unwrap_or(fd_tables.root_id());\n            let parent_tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);\n            let clone_flags = if event == libc::PTRACE_EVENT_CLONE {\n                match tracees\n                    .get(&tid)\n                    .and_then(|state| state.pending_syscall.as_ref())\n                {\n                    Some(PendingSyscall::Clone { flags }) => Some(*flags),\n                    _ => {\n                        collector.warning(\n                            tid,\n                            \"clone_flags_unavailable\",\n                            \"PTRACE_EVENT_CLONE observed without clone/clone3 flags; fd sharing and thread-group semantics are incomplete\",\n                        );\n                        None\n                    }\n                }\n            } else {\n                None\n            };\n\n            if event == libc::PTRACE_EVENT_CLONE {\n                collector.clone_fd_certification.record_clone(clone_flags);\n            }\n"""

new_event_block = """            // Preserve public raw-v2 event-label semantics while deriving the\n            // internal fd-table relation from the causally paired syscall. Linux\n            // may route clone-origin creation as FORK/VFORK depending on clone\n            // flags / exit signal; the ptrace event label is not syscall identity.\n            let mechanism = match event {\n                libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,\n                libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,\n                _ => SpawnMechanism::Clone,\n            };\n            let pending_clone_flags = tracees\n                .get(&tid)\n                .and_then(|state| state.pending_syscall.as_ref())\n                .and_then(|pending| match pending {\n                    PendingSyscall::Clone { flags } => Some(*flags),\n                    _ => None,\n                });\n            let clone_origin = pending_clone_flags.is_some();\n\n            let parent_table = tracees\n                .get(&tid)\n                .map(|state| state.fd_table_id)\n                .unwrap_or(fd_tables.root_id());\n            let parent_tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);\n            let clone_flags = if clone_origin {\n                pending_clone_flags\n            } else if event == libc::PTRACE_EVENT_CLONE {\n                collector.warning(\n                    tid,\n                    \"clone_flags_unavailable\",\n                    \"PTRACE child-creation event requires clone/clone3 origin flags, but no causally paired clone syscall was retained; fd sharing and thread-group semantics are incomplete\",\n                );\n                None\n            } else {\n                None\n            };\n\n            if clone_origin || event == libc::PTRACE_EVENT_CLONE {\n                collector\n                    .clone_fd_certification\n                    .record_clone_creation_event(clone_flags, event);\n            }\n\n            // Raw v2 retains the ptrace event mechanism, not clone flags. If a\n            // clone-origin transition carrying CLONE_FILES is routed as FORK or\n            // VFORK, internal fd tracking can be exact but public v2 cannot carry\n            // the proof. Keep that public surface fail-closed instead of silently\n            // certifying completeness. Known-independent routed clones do not need\n            // this warning and preserve the historical v2 process-spawn behavior.\n            if clone_origin\n                && event != libc::PTRACE_EVENT_CLONE\n                && clone_flags\n                    .map(|flags| flags & libc::CLONE_FILES as u64 != 0)\n                    .unwrap_or(false)\n            {\n                collector.warning(\n                    tid,\n                    \"shared_fd_table_ambiguity\",\n                    \"clone-origin child with CLONE_FILES was routed through a non-CLONE ptrace event; fd tracking is internally correlated, but raw v2 cannot retain the relationship proof\",\n                );\n            }\n"""
text = replace_once(text, old_event_block, new_event_block, "creation-event origin classification")

anchor = """    #[test]\n    fn normalizes_only_the_tracees_own_proc_identity() {\n"""
insert = r'''    #[test]
    #[ignore = "C3 live clone-origin event-routing harness; run in the dedicated Linux gate"]
    fn c3_clone_sigchld_fork_event_keeps_clone_fd_authority() {
        use std::process::Command;

        let root = std::env::temp_dir().join(format!(
            "execsurface-c3-clone-sigchld-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create C3 SIGCHLD harness directory");
        let source = root.join("clone_sigchld.c");
        let binary = root.join("clone_sigchld");
        std::fs::write(
            &source,
            r#"#define _GNU_SOURCE
#include <sched.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>

static int child_main(void *unused) {
    (void)unused;
    _exit(0);
}

int main(void) {
    const size_t stack_size = 1u << 20;
    char *stack = malloc(stack_size);
    if (!stack) return 2;
    int flags = CLONE_FILES | SIGCHLD;
    pid_t child = clone(child_main, stack + stack_size, flags, NULL);
    if (child < 0) {
        perror("clone");
        free(stack);
        return 3;
    }
    int status = 0;
    if (waitpid(child, &status, 0) < 0) {
        perror("waitpid");
        free(stack);
        return 4;
    }
    free(stack);
    return WIFEXITED(status) && WEXITSTATUS(status) == 0 ? 0 : 5;
}
"#,
        )
        .expect("write C3 SIGCHLD clone harness");

        let compile = Command::new("cc")
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .status()
            .expect("compile C3 SIGCHLD clone harness");
        assert!(compile.success());

        let observed = observe(
            &CommandSpec::new(binary.as_os_str()),
            ObserveOptions::default(),
        )
        .expect("observe C3 SIGCHLD clone harness");
        assert_eq!(observed.observation.outcome.exit_code, Some(0));
        assert!(observed.clone_fd_certification.fully_certified());
        assert!(observed.clone_fd_certification.shared_fd_transitions >= 1);
        assert!(
            observed.clone_fd_certification.clone_origin_fork_events >= 1,
            "clone(..., CLONE_FILES|SIGCHLD) must expose the FORK-route counter on this Linux gate"
        );
        assert!(
            !observed.observation.complete,
            "raw v2 must remain fail-closed because it cannot retain the routed CLONE_FILES proof"
        );
        assert!(observed
            .observation
            .warnings
            .iter()
            .any(|warning| warning.code == "shared_fd_table_ambiguity"));
        assert!(observed.observation.events.iter().any(|event| matches!(
            &event.kind,
            RawEventKind::ProcessSpawn {
                mechanism: SpawnMechanism::Fork,
                ..
            }
        )));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    #[ignore = "C3 live clone-origin VFORK routing harness; run in the dedicated Linux gate"]
    fn c3_clone_vfork_event_keeps_clone_fd_authority() {
        use std::process::Command;

        let root = std::env::temp_dir().join(format!(
            "execsurface-c3-clone-vfork-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create C3 VFORK harness directory");
        let source = root.join("clone_vfork.c");
        let binary = root.join("clone_vfork");
        std::fs::write(
            &source,
            r#"#define _GNU_SOURCE
#include <sched.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>

static int child_main(void *unused) {
    (void)unused;
    _exit(0);
}

int main(void) {
    const size_t stack_size = 1u << 20;
    char *stack = malloc(stack_size);
    if (!stack) return 2;
    int flags = CLONE_FILES | CLONE_VM | CLONE_VFORK | SIGCHLD;
    pid_t child = clone(child_main, stack + stack_size, flags, NULL);
    if (child < 0) {
        perror("clone");
        free(stack);
        return 3;
    }
    int status = 0;
    if (waitpid(child, &status, 0) < 0) {
        perror("waitpid");
        free(stack);
        return 4;
    }
    free(stack);
    return WIFEXITED(status) && WEXITSTATUS(status) == 0 ? 0 : 5;
}
"#,
        )
        .expect("write C3 VFORK clone harness");

        let compile = Command::new("cc")
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .status()
            .expect("compile C3 VFORK clone harness");
        assert!(compile.success());

        let observed = observe(
            &CommandSpec::new(binary.as_os_str()),
            ObserveOptions::default(),
        )
        .expect("observe C3 VFORK clone harness");
        assert_eq!(observed.observation.outcome.exit_code, Some(0));
        assert!(observed.clone_fd_certification.fully_certified());
        assert!(observed.clone_fd_certification.shared_fd_transitions >= 1);
        assert!(
            observed.clone_fd_certification.clone_origin_vfork_events >= 1,
            "CLONE_VFORK clone origin must expose the VFORK-route counter on this Linux gate"
        );
        assert!(
            !observed.observation.complete,
            "raw v2 must remain fail-closed because it cannot retain the routed CLONE_FILES proof"
        );
        assert!(observed
            .observation
            .warnings
            .iter()
            .any(|warning| warning.code == "shared_fd_table_ambiguity"));
        assert!(observed.observation.events.iter().any(|event| matches!(
            &event.kind,
            RawEventKind::ProcessSpawn {
                mechanism: SpawnMechanism::Vfork,
                ..
            }
        )));

        let _ = std::fs::remove_dir_all(&root);
    }

'''
text = replace_once(text, anchor, insert + anchor, "insert C3 live event-routing harnesses")

path.write_text(text)
