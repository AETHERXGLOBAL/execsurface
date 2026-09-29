from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text()

old = '''            let clone_flags = if event == libc::PTRACE_EVENT_CLONE {
                match tracees
                    .get(&tid)
                    .and_then(|state| state.pending_syscall.as_ref())
                {
                    Some(PendingSyscall::Clone { flags }) => Some(*flags),
                    _ => {
                        collector.warning(
                            tid,
                            "clone_flags_unavailable",
                            "PTRACE_EVENT_CLONE observed without clone/clone3 flags; fd sharing and thread-group semantics are incomplete",
                        );
                        None
                    }
                }
            } else {
                None
            };

            if event == libc::PTRACE_EVENT_CLONE {
                collector.clone_fd_certification.record_clone(clone_flags);
            }
'''
new = '''            // ptrace event routing is not a syscall-origin identity: clone(2)
            // can surface as PTRACE_EVENT_FORK when its exit signal is SIGCHLD,
            // or as PTRACE_EVENT_VFORK when CLONE_VFORK is present. Preserve the
            // pending clone/clone3 flags for all three event kinds so fd-table
            // sharing is classified from syscall evidence, never from event name.
            let pending_clone_flags = tracees
                .get(&tid)
                .and_then(|state| state.pending_syscall.as_ref())
                .and_then(|pending| match pending {
                    PendingSyscall::Clone { flags } => Some(*flags),
                    _ => None,
                });
            let clone_origin = pending_clone_flags.is_some();
            let clone_flags = if clone_origin {
                pending_clone_flags
            } else if event == libc::PTRACE_EVENT_CLONE {
                collector.warning(
                    tid,
                    "clone_flags_unavailable",
                    "PTRACE_EVENT_CLONE observed without clone/clone3 flags; fd sharing and thread-group semantics are incomplete",
                );
                None
            } else {
                None
            };

            if clone_origin || event == libc::PTRACE_EVENT_CLONE {
                collector.clone_fd_certification.record_clone(clone_flags);
            }
'''
text = replace_once(text, old, new, "strengthen clone-origin routing")

old_c = '''    int flags = 0;
    if (strcmp(argv[1], "shared") == 0) {
        flags |= CLONE_FILES;
    } else if (strcmp(argv[1], "private") != 0) {
        return 3;
    }

    const size_t stack_size = 1u << 20;
'''
new_c = '''    int flags = 0;
    int wait_flags = __WCLONE;
    if (strcmp(argv[1], "shared") == 0) {
        flags = CLONE_FILES;
    } else if (strcmp(argv[1], "shared-sigchld") == 0) {
        flags = CLONE_FILES | SIGCHLD;
        wait_flags = 0;
    } else if (strcmp(argv[1], "shared-vfork") == 0) {
        flags = CLONE_FILES | CLONE_VM | CLONE_VFORK | SIGCHLD;
        wait_flags = 0;
    } else if (strcmp(argv[1], "private") != 0) {
        return 3;
    }

    const size_t stack_size = 1u << 20;
'''
text = replace_once(text, old_c, new_c, "extend C harness modes")
text = replace_once(
    text,
    '''    if (waitpid(child, &status, __WCLONE) < 0) {
''',
    '''    if (waitpid(child, &status, wait_flags) < 0) {
''',
    "mode-specific wait semantics",
)

old_assert = '''        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn normalizes_only_the_tracees_own_proc_identity() {
'''
new_assert = '''        for mode in ["shared-sigchld", "shared-vfork"] {
            let routed = observe(
                &CommandSpec::new(binary.as_os_str()).arg(mode),
                ObserveOptions::default(),
            )
            .unwrap_or_else(|error| panic!("observe {mode} clone routing harness: {error}"));
            assert_eq!(routed.observation.outcome.exit_code, Some(0));
            assert!(
                routed.clone_fd_certification.fully_certified(),
                "clone-origin {mode} transition must retain correlated flags even when ptrace routes the syscall as FORK/VFORK"
            );
            assert!(routed.clone_fd_certification.clone_events >= 1);
            assert!(routed.clone_fd_certification.shared_fd_transitions >= 1);
            assert_eq!(
                routed
                    .observation
                    .warnings
                    .iter()
                    .filter(|warning| warning.code == "clone_flags_unavailable")
                    .count(),
                0
            );
        }

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn normalizes_only_the_tracees_own_proc_identity() {
'''
text = replace_once(text, old_assert, new_assert, "add event-routing assertions")
path.write_text(text)
