#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_state_tail = '''    fd_table_id: u64,
    tgid: libc::pid_t,
}
'''
new_state_tail = '''    fd_table_id: u64,
    tgid: libc::pid_t,
    retired_by_exec: bool,
    exit_event_status: Option<libc::c_int>,
}
'''

old_ctor_tail = '''            fd_table_id,
            tgid,
        }
'''
new_ctor_tail = '''            fd_table_id,
            tgid,
            retired_by_exec: false,
            exit_event_status: None,
        }
'''

old_options = '''        | libc::PTRACE_O_TRACECLONE
        | libc::PTRACE_O_TRACEEXEC
        | libc::PTRACE_O_EXITKILL;
'''
new_options = '''        | libc::PTRACE_O_TRACECLONE
        | libc::PTRACE_O_TRACEEXEC
        | libc::PTRACE_O_TRACEEXIT
        | libc::PTRACE_O_EXITKILL;
'''

old_echild = '''            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
                let mut tracked = tracees
                    .iter()
                    .map(|(tid, state)| (*tid, state.tgid))
                    .collect::<Vec<_>>();
                tracked.sort_unstable();
                return Err(ObserveError::Protocol(format!(
                    "M9_STALE_TRACEES_AT_ECHILD tracked_tid_tgid={tracked:?}"
                )));
            }
'''
new_echild = '''            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
                let mut unresolved = tracees
                    .iter()
                    .filter_map(|(tid, state)| {
                        state.exit_event_status.is_none().then_some((
                            *tid,
                            state.tgid,
                            state.retired_by_exec,
                        ))
                    })
                    .collect::<Vec<_>>();
                unresolved.sort_unstable_by_key(|entry| entry.0);
                if !unresolved.is_empty() {
                    return Err(ObserveError::Protocol(format!(
                        "M9_STALE_TRACEES_WITHOUT_EXIT_EVIDENCE_AT_ECHILD tracked={unresolved:?}"
                    )));
                }

                let mut reconciled = tracees
                    .iter()
                    .map(|(tid, state)| {
                        (
                            *tid,
                            state.tgid,
                            state.retired_by_exec,
                            state.exit_event_status.unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();
                reconciled.sort_unstable_by_key(|entry| entry.0);
                for (stale_tid, _, _, exit_status) in &reconciled {
                    if !apply_terminal_outcome(root, *stale_tid, *exit_status, &mut root_outcome) {
                        return Err(ObserveError::Protocol(format!(
                            "M9_EXIT_EVENT_STATUS_NONTERMINAL_AT_ECHILD tid={stale_tid} status={exit_status:#x}"
                        )));
                    }
                }
                eprintln!("M9_EXIT_EVENT_RECONCILE_AT_ECHILD reconciled={reconciled:?}");
                tracees.clear();
                break;
            }
'''

old_stop_dispatch = '''        if stop_signal == libc::SIGTRAP && event != 0 {
            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''
new_stop_dispatch = '''        if tracees
            .get(&tid)
            .map(|state| state.retired_by_exec)
            .unwrap_or(false)
            && !(stop_signal == libc::SIGTRAP && event == libc::PTRACE_EVENT_EXIT)
        {
            return Err(ObserveError::Protocol(format!(
                "M9_RETIRED_EXEC_UNEXPECTED_STOP tid={tid} signal={stop_signal} event={event}"
            )));
        }

        if stop_signal == libc::SIGTRAP && event != 0 {
            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''

old_exec_cleanup = '''            let old_tgid = state.tgid;
            let mut retired = tracees
                .iter()
                .filter_map(|(other_tid, other_state)| {
                    (other_state.tgid == old_tgid).then_some(*other_tid)
                })
                .collect::<Vec<_>>();
            retired.sort_unstable();
            tracees.retain(|_, other_state| other_state.tgid != old_tgid);

            state.tgid = tid;
            state.newborn = false;
            tracees.insert(tid, state);
'''
new_exec_cleanup = '''            let old_tgid = state.tgid;
            let mut retired = Vec::new();
            for (other_tid, other_state) in tracees.iter_mut() {
                if other_state.tgid == old_tgid {
                    other_state.retired_by_exec = true;
                    other_state.pending_exec = None;
                    other_state.pending_syscall = None;
                    retired.push(*other_tid);
                }
            }
            retired.sort_unstable();

            state.tgid = tid;
            state.newborn = false;
            state.retired_by_exec = false;
            state.exit_event_status = None;
            tracees.insert(tid, state);
'''

old_terminal_fn = '''fn record_terminal_wait_status(
    root: libc::pid_t,
    tid: libc::pid_t,
    wait_status: libc::c_int,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    root_outcome: &mut CommandOutcome,
) -> bool {
    if libc::WIFEXITED(wait_status) {
        if tid == root {
            root_outcome.exit_code = Some(libc::WEXITSTATUS(wait_status));
        }
        tracees.remove(&tid);
        return true;
    }

    if libc::WIFSIGNALED(wait_status) {
        if tid == root {
            root_outcome.signal = Some(libc::WTERMSIG(wait_status));
        }
        tracees.remove(&tid);
        return true;
    }

    false
}
'''
new_terminal_fn = '''fn apply_terminal_outcome(
    root: libc::pid_t,
    tid: libc::pid_t,
    wait_status: libc::c_int,
    root_outcome: &mut CommandOutcome,
) -> bool {
    if libc::WIFEXITED(wait_status) {
        if tid == root {
            root_outcome.exit_code = Some(libc::WEXITSTATUS(wait_status));
        }
        return true;
    }

    if libc::WIFSIGNALED(wait_status) {
        if tid == root {
            root_outcome.signal = Some(libc::WTERMSIG(wait_status));
        }
        return true;
    }

    false
}

fn record_terminal_wait_status(
    root: libc::pid_t,
    tid: libc::pid_t,
    wait_status: libc::c_int,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    root_outcome: &mut CommandOutcome,
) -> bool {
    if apply_terminal_outcome(root, tid, wait_status, root_outcome) {
        tracees.remove(&tid);
        return true;
    }
    false
}
'''

old_wait_error = '''            if waited < 0 {
                let wait_error = io::Error::last_os_error();
                return Err(ObserveError::Protocol(format!(
                    "PTRACE_SYSCALL returned ESRCH after an observed ptrace-stop for tid {tid}, and exact-TID terminal wait failed: {wait_error}"
                )));
            }
'''
new_wait_error = '''            if waited < 0 {
                let wait_error = io::Error::last_os_error();
                if wait_error.raw_os_error() == Some(libc::ECHILD) {
                    if let Some(exit_status) = tracees
                        .get(&tid)
                        .and_then(|state| state.exit_event_status)
                    {
                        if apply_terminal_outcome(root, tid, exit_status, root_outcome) {
                            tracees.remove(&tid);
                            eprintln!(
                                "M9_ESRCH_RECONCILED_FROM_EXIT_EVENT tid={tid} status={exit_status:#x}"
                            );
                            return Ok(());
                        }
                        return Err(ObserveError::Protocol(format!(
                            "M9_ESRCH_EXIT_EVENT_STATUS_NONTERMINAL tid={tid} status={exit_status:#x}"
                        )));
                    }
                }
                return Err(ObserveError::Protocol(format!(
                    "PTRACE_SYSCALL returned ESRCH after an observed ptrace-stop for tid {tid}, and exact-TID terminal wait failed: {wait_error}"
                )));
            }
'''

old_event_tail = '''        }
        _ => {}
    }
    Ok(())
}
'''
new_event_tail = '''        }
        libc::PTRACE_EVENT_EXIT => {
            let exit_status = get_event_message(tid)? as libc::c_int;
            let state = tracees.get_mut(&tid).ok_or_else(|| {
                ObserveError::Protocol(format!(
                    "PTRACE_EVENT_EXIT arrived for untracked tid {tid}"
                ))
            })?;
            state.exit_event_status = Some(exit_status);
        }
        _ => {}
    }
    Ok(())
}
'''

for old, new, label, expected in [
    (old_state_tail, new_state_tail, "exit-event and exec-retirement state", 1),
    (old_ctor_tail, new_ctor_tail, "lifecycle constructor initialization", 2),
    (old_options, new_options, "PTRACE_O_TRACEEXIT option", 1),
    (old_echild, new_echild, "ECHILD exit-event reconciliation", 1),
    (old_stop_dispatch, new_stop_dispatch, "retired exec stop guard", 1),
    (old_exec_cleanup, new_exec_cleanup, "exec retirement bookkeeping", 1),
    (old_terminal_fn, new_terminal_fn, "terminal outcome factoring", 1),
    (old_wait_error, new_wait_error, "ESRCH exact-wait exit-event reconciliation", 1),
    (old_event_tail, new_event_tail, "PTRACE_EVENT_EXIT capture", 1),
]:
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"expected {expected} {label} replacement(s), found {count}")
    text = text.replace(old, new, expected)

path.write_text(text, encoding="utf-8")
print("M9_PTRACE_EXIT_EVENT_LIFECYCLE_CANDIDATE_APPLIED")
