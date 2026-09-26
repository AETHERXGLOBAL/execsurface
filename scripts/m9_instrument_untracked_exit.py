#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old = '''        libc::PTRACE_EVENT_EXIT => {
            let exit_status = get_event_message(tid)? as libc::c_int;
            let state = tracees.get_mut(&tid).ok_or_else(|| {
                ObserveError::Protocol(format!(
                    "PTRACE_EVENT_EXIT arrived for untracked tid {tid}"
                ))
            })?;
            state.exit_event_status = Some(exit_status);
        }
'''
new = '''        libc::PTRACE_EVENT_EXIT => {
            let exit_status = get_event_message(tid)? as libc::c_int;
            if !tracees.contains_key(&tid) {
                let proc_tgid = fs::read_to_string(format!("/proc/{tid}/status"))
                    .ok()
                    .and_then(|status| {
                        status.lines().find_map(|line| {
                            line.strip_prefix("Tgid:")
                                .and_then(|value| value.trim().parse::<libc::pid_t>().ok())
                        })
                    });
                let mut tracked = tracees
                    .iter()
                    .map(|(tracked_tid, state)| {
                        (
                            *tracked_tid,
                            state.tgid,
                            state.retired_by_exec,
                            state.restart_esrch_seen,
                            state.exit_event_status.is_some(),
                        )
                    })
                    .collect::<Vec<_>>();
                tracked.sort_unstable_by_key(|entry| entry.0);
                return Err(ObserveError::Protocol(format!(
                    "M9_UNTRACKED_EXIT_EVENT tid={tid} proc_tgid={proc_tgid:?} exit_status={exit_status:#x} tracked={tracked:?}"
                )));
            }
            let state = tracees.get_mut(&tid).expect("tracee existence checked above");
            state.exit_event_status = Some(exit_status);
        }
'''

count = text.count(old)
if count != 1:
    raise SystemExit(f"expected exactly one untracked EXIT diagnostic target, found {count}")
text = text.replace(old, new, 1)
path.write_text(text, encoding="utf-8")
print("M9_UNTRACKED_EXIT_DIAGNOSTIC_APPLIED")
