#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_state = '''    retired_by_exec: bool,
    restart_esrch_seen: bool,
    exit_event_status: Option<libc::c_int>,
}
'''
new_state = '''    retired_by_exec: bool,
    restart_esrch_seen: bool,
    exit_event_status: Option<libc::c_int>,
    exit_group_pending: bool,
    syscall_info_esrch_after_exit_group: bool,
}
'''

old_ctor = '''            retired_by_exec: false,
            restart_esrch_seen: false,
            exit_event_status: None,
        }
'''
new_ctor = '''            retired_by_exec: false,
            restart_esrch_seen: false,
            exit_event_status: None,
            exit_group_pending: false,
            syscall_info_esrch_after_exit_group: false,
        }
'''

old_syscall_stop = '''        if stop_signal == (libc::SIGTRAP | 0x80) {
            let info = syscall_info(tid)?;
            if let Some((nr, args)) = info.entry() {
                handle_syscall_entry(tid, nr, args, &mut tracees, &mut collector);
            } else if let Some(result) = info.exit() {
                handle_syscall_exit(tid, result, &mut tracees, &mut fd_tables, &mut collector);
            }
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''
new_syscall_stop = '''        if stop_signal == (libc::SIGTRAP | 0x80) {
            let info = match syscall_info(tid) {
                Ok(info) => info,
                Err(ObserveError::Os(error)) if error.raw_os_error() == Some(libc::ESRCH) => {
                    let recoverable = tracees
                        .get(&tid)
                        .map(|state| {
                            state.exit_group_pending
                                && state.pending_syscall.is_none()
                                && !state.retired_by_exec
                                && state.exit_event_status.is_none()
                        })
                        .unwrap_or(false);
                    if !recoverable {
                        return Err(ObserveError::Protocol(format!(
                            "PTRACE_GET_SYSCALL_INFO returned ESRCH for tid {tid} without bounded exit-group lifecycle evidence"
                        )));
                    }
                    let state = tracees.get_mut(&tid).ok_or_else(|| {
                        ObserveError::Protocol(format!(
                            "bounded GET_SYSCALL_INFO ESRCH recovery lost tracked tid {tid}"
                        ))
                    })?;
                    state.syscall_info_esrch_after_exit_group = true;
                    eprintln!(
                        "M9_GET_SYSCALL_INFO_EXIT_GROUP_DEATH_DEFERRED tid={tid} tgid={}",
                        state.tgid
                    );
                    /*
                     * The kernel has already refused a ptrace read while the
                     * same thread group is in an explicitly observed
                     * exit_group teardown. Do not fabricate syscall phase
                     * information and do not restart the dying thread here.
                     * Its subsequent PTRACE_EVENT_EXIT / terminal wait remains
                     * responsible for lifecycle completion.
                     */
                    continue;
                }
                Err(error) => return Err(error),
            };
            if let Some((nr, args)) = info.entry() {
                handle_syscall_entry(tid, nr, args, &mut tracees, &mut collector);
            } else if let Some(result) = info.exit() {
                handle_syscall_exit(tid, result, &mut tracees, &mut fd_tables, &mut collector);
            }
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''

old_entry_prefix = '''    let nr = nr as libc::c_long;

    if nr == libc::SYS_execve {
'''
new_entry_prefix = '''    let nr = nr as libc::c_long;

    if nr == libc::SYS_exit_group {
        let tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);
        for state in tracees.values_mut() {
            if state.tgid == tgid {
                state.exit_group_pending = true;
            }
        }
    }

    if nr == libc::SYS_execve {
'''

old_exec_reset = '''            state.retired_by_exec = false;
            state.restart_esrch_seen = false;
            state.exit_event_status = None;
            tracees.insert(tid, state);
'''
new_exec_reset = '''            state.retired_by_exec = false;
            state.restart_esrch_seen = false;
            state.exit_event_status = None;
            state.exit_group_pending = false;
            state.syscall_info_esrch_after_exit_group = false;
            tracees.insert(tid, state);
'''

old_echild_tuple = '''                            state.retired_by_exec,
                            state.restart_esrch_seen,
                        ))
'''
new_echild_tuple = '''                            state.retired_by_exec,
                            state.restart_esrch_seen,
                            state.exit_group_pending,
                            state.syscall_info_esrch_after_exit_group,
                        ))
'''

old_echild_reconciled_tuple = '''                            state.retired_by_exec,
                            state.restart_esrch_seen,
                            state.exit_event_status.unwrap(),
                        )
'''
new_echild_reconciled_tuple = '''                            state.retired_by_exec,
                            state.restart_esrch_seen,
                            state.exit_group_pending,
                            state.syscall_info_esrch_after_exit_group,
                            state.exit_event_status.unwrap(),
                        )
'''

old_echild_loop = '''                for (stale_tid, _, _, _, exit_status) in &reconciled {
'''
new_echild_loop = '''                for (stale_tid, _, _, _, _, _, exit_status) in &reconciled {
'''

replacements = [
    (old_state, new_state, "exit-group lifecycle state", 1),
    (old_ctor, new_ctor, "exit-group constructor state", 2),
    (old_syscall_stop, new_syscall_stop, "bounded GET_SYSCALL_INFO ESRCH recovery", 1),
    (old_entry_prefix, new_entry_prefix, "exit_group propagation", 1),
    (old_exec_reset, new_exec_reset, "exec reset", 1),
    (old_echild_tuple, new_echild_tuple, "ECHILD unresolved diagnostics", 1),
    (old_echild_reconciled_tuple, new_echild_reconciled_tuple, "ECHILD reconciled diagnostics", 1),
    (old_echild_loop, new_echild_loop, "ECHILD reconciled tuple loop", 1),
]

for old, new, label, expected in replacements:
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"expected {expected} {label} replacement(s), found {count}")
    text = text.replace(old, new, expected)

path.write_text(text, encoding="utf-8")
print("M9_GET_SYSCALL_INFO_EXIT_GROUP_FIX_CANDIDATE_APPLIED")
