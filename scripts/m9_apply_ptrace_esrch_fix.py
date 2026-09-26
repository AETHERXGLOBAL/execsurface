#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_wait_error = '''        if tid < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            return Err(error.into());
        }
'''
new_wait_error = '''        if tid < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
                let mut tracked = tracees.keys().copied().collect::<Vec<_>>();
                tracked.sort_unstable();
                return Err(ObserveError::Protocol(format!(
                    "M9_STALE_TRACEES_AT_ECHILD tracked={tracked:?}"
                )));
            }
            return Err(error.into());
        }
'''

old_terminal = '''        if libc::WIFEXITED(wait_status) {
            if tid == root {
                root_outcome.exit_code = Some(libc::WEXITSTATUS(wait_status));
            }
            tracees.remove(&tid);
            continue;
        }

        if libc::WIFSIGNALED(wait_status) {
            if tid == root {
                root_outcome.signal = Some(libc::WTERMSIG(wait_status));
            }
            tracees.remove(&tid);
            continue;
        }
'''
new_terminal = '''        if record_terminal_wait_status(
            root,
            tid,
            wait_status,
            &mut tracees,
            &mut root_outcome,
        ) {
            continue;
        }
'''

old_event = '''            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_syscall(tid, 0)?;
            continue;
'''
new_event = '''            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
'''

old_syscall = '''            resume_syscall(tid, 0)?;
            continue;
        }

        let suppress_signal = match tracees.get_mut(&tid) {
'''
new_syscall = '''            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }

        let suppress_signal = match tracees.get_mut(&tid) {
'''

old_signal = '''        resume_syscall(tid, if suppress_signal { 0 } else { stop_signal })?;
    }

    collector.observation.outcome = root_outcome;
'''
new_signal = '''        resume_after_observed_stop(
            root,
            tid,
            if suppress_signal { 0 } else { stop_signal },
            &mut tracees,
            &mut root_outcome,
        )?;
    }

    collector.observation.outcome = root_outcome;
'''

old_exec = '''        libc::PTRACE_EVENT_EXEC => {
            apply_exec_fd_semantics(tid, tracees, fd_tables);
            let state = tracees
                .entry(tid)
                .or_insert_with(|| TraceeState::root(fd_tables.root_id()));
            match state.pending_exec.take() {
'''
new_exec = '''        libc::PTRACE_EVENT_EXEC => {
            let former_tid = get_event_message(tid)? as libc::pid_t;
            eprintln!(
                "M9_EXEC_IDENTITY_EVENT new_tid={tid} former_tid={former_tid} tracked_before={:?}",
                tracees.keys().copied().collect::<Vec<_>>()
            );
            apply_exec_fd_semantics(tid, tracees, fd_tables);
            let state = tracees
                .entry(tid)
                .or_insert_with(|| TraceeState::root(fd_tables.root_id()));
            match state.pending_exec.take() {
'''

helper_anchor = '''fn handle_ptrace_event(
'''
helper = '''fn record_terminal_wait_status(
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

fn resume_after_observed_stop(
    root: libc::pid_t,
    tid: libc::pid_t,
    signal: libc::c_int,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    root_outcome: &mut CommandOutcome,
) -> Result<(), ObserveError> {
    match resume_syscall(tid, signal) {
        Ok(()) => Ok(()),
        Err(ObserveError::Os(resume_error))
            if resume_error.raw_os_error() == Some(libc::ESRCH) =>
        {
            let mut terminal_status = 0;
            let waited = unsafe { libc::waitpid(tid, &mut terminal_status, libc::__WALL) };

            if waited == tid
                && record_terminal_wait_status(
                    root,
                    tid,
                    terminal_status,
                    tracees,
                    root_outcome,
                )
            {
                return Ok(());
            }

            if waited < 0 {
                let wait_error = io::Error::last_os_error();
                return Err(ObserveError::Protocol(format!(
                    "PTRACE_SYSCALL returned ESRCH after an observed ptrace-stop for tid {tid}, and exact-TID terminal wait failed: {wait_error}"
                )));
            }

            Err(ObserveError::Protocol(format!(
                "PTRACE_SYSCALL returned ESRCH after an observed ptrace-stop for tid {tid}, but exact-TID wait returned non-terminal status {terminal_status:#x}"
            )))
        }
        Err(error) => Err(error),
    }
}

fn handle_ptrace_event(
'''

replacements = [
    (old_wait_error, new_wait_error, "outer wait ECHILD diagnostic"),
    (old_terminal, new_terminal, "terminal wait handling"),
    (old_event, new_event, "ptrace event restart"),
    (old_syscall, new_syscall, "syscall restart"),
    (old_signal, new_signal, "signal restart"),
    (old_exec, new_exec, "exec identity diagnostic"),
]

for old, new, label in replacements:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly one {label} replacement, found {count}")
    text = text.replace(old, new, 1)

if text.count(helper_anchor) != 1:
    raise SystemExit("expected exactly one helper insertion anchor")
text = text.replace(helper_anchor, helper, 1)

path.write_text(text, encoding="utf-8")
print("M9_PTRACE_ESRCH_CANDIDATE_APPLIED")
