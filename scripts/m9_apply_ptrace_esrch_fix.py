#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_state = '''#[derive(Debug, Clone)]
struct TraceeState {
    pending_exec: Option<String>,
    pending_syscall: Option<PendingSyscall>,
    newborn: bool,
    fd_table_id: u64,
}

impl TraceeState {
    fn root(fd_table_id: u64) -> Self {
        Self {
            pending_exec: None,
            pending_syscall: None,
            newborn: false,
            fd_table_id,
        }
    }

    fn child(fd_table_id: u64) -> Self {
        Self {
            pending_exec: None,
            pending_syscall: None,
            newborn: true,
            fd_table_id,
        }
    }
}
'''
new_state = '''#[derive(Debug, Clone)]
struct TraceeState {
    pending_exec: Option<String>,
    pending_syscall: Option<PendingSyscall>,
    newborn: bool,
    fd_table_id: u64,
    tgid: libc::pid_t,
}

impl TraceeState {
    fn root(fd_table_id: u64, tgid: libc::pid_t) -> Self {
        Self {
            pending_exec: None,
            pending_syscall: None,
            newborn: false,
            fd_table_id,
            tgid,
        }
    }

    fn child(fd_table_id: u64, tgid: libc::pid_t) -> Self {
        Self {
            pending_exec: None,
            pending_syscall: None,
            newborn: true,
            fd_table_id,
            tgid,
        }
    }
}
'''

old_root = '''    tracees.insert(root, TraceeState::root(fd_tables.root_id()));
'''
new_root = '''    tracees.insert(root, TraceeState::root(fd_tables.root_id(), root));
'''

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
                let mut tracked = tracees
                    .iter()
                    .map(|(tid, state)| (*tid, state.tgid))
                    .collect::<Vec<_>>();
                tracked.sort_unstable();
                return Err(ObserveError::Protocol(format!(
                    "M9_STALE_TRACEES_AT_ECHILD tracked_tid_tgid={tracked:?}"
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

old_event_restart = '''            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_syscall(tid, 0)?;
            continue;
'''
new_event_restart = '''            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
'''

old_syscall_restart = '''            resume_syscall(tid, 0)?;
            continue;
        }

        let suppress_signal = match tracees.get_mut(&tid) {
'''
new_syscall_restart = '''            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }

        let suppress_signal = match tracees.get_mut(&tid) {
'''

old_signal_restart = '''        resume_syscall(tid, if suppress_signal { 0 } else { stop_signal })?;
    }

    collector.observation.outcome = root_outcome;
'''
new_signal_restart = '''        resume_after_observed_stop(
            root,
            tid,
            if suppress_signal { 0 } else { stop_signal },
            &mut tracees,
            &mut root_outcome,
        )?;
    }

    collector.observation.outcome = root_outcome;
'''

old_events = '''        libc::PTRACE_EVENT_FORK | libc::PTRACE_EVENT_VFORK | libc::PTRACE_EVENT_CLONE => {
            let child_tid = get_event_message(tid)? as libc::pid_t;
            let mechanism = match event {
                libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,
                libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,
                _ => SpawnMechanism::Clone,
            };

            let parent_table = tracees
                .get(&tid)
                .map(|state| state.fd_table_id)
                .unwrap_or(fd_tables.root_id());

            let share_files = if event == libc::PTRACE_EVENT_CLONE {
                match tracees
                    .get(&tid)
                    .and_then(|state| state.pending_syscall.as_ref())
                {
                    Some(PendingSyscall::Clone { flags }) => flags & libc::CLONE_FILES as u64 != 0,
                    _ => {
                        collector.warning(
                            tid,
                            "clone_flags_unavailable",
                            "PTRACE_EVENT_CLONE observed without clone/clone3 flags; fd sharing semantics are incomplete",
                        );
                        false
                    }
                }
            } else {
                false
            };

            let child_table = if share_files {
                parent_table
            } else {
                fd_tables.clone_table(parent_table)
            };

            tracees
                .entry(child_tid)
                .or_insert_with(|| TraceeState::child(child_table));

            collector.event(
                tid,
                RawEventKind::ProcessSpawn {
                    child_tid,
                    mechanism,
                },
            );
        }
        libc::PTRACE_EVENT_EXEC => {
            apply_exec_fd_semantics(tid, tracees, fd_tables);
            let state = tracees
                .entry(tid)
                .or_insert_with(|| TraceeState::root(fd_tables.root_id()));
            match state.pending_exec.take() {
                Some(path) => collector.event(tid, RawEventKind::ProcessExec { path }),
                None => collector.warning(
                    tid,
                    "exec_without_path",
                    "PTRACE_EVENT_EXEC was observed without a readable pending exec pathname",
                ),
            }
        }
'''
new_events = '''        libc::PTRACE_EVENT_FORK | libc::PTRACE_EVENT_VFORK | libc::PTRACE_EVENT_CLONE => {
            let child_tid = get_event_message(tid)? as libc::pid_t;
            let mechanism = match event {
                libc::PTRACE_EVENT_FORK => SpawnMechanism::Fork,
                libc::PTRACE_EVENT_VFORK => SpawnMechanism::Vfork,
                _ => SpawnMechanism::Clone,
            };

            let parent_table = tracees
                .get(&tid)
                .map(|state| state.fd_table_id)
                .unwrap_or(fd_tables.root_id());
            let parent_tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);
            let clone_flags = if event == libc::PTRACE_EVENT_CLONE {
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

            let share_files = clone_flags
                .map(|flags| flags & libc::CLONE_FILES as u64 != 0)
                .unwrap_or(false);
            let child_tgid = if clone_flags
                .map(|flags| flags & libc::CLONE_THREAD as u64 != 0)
                .unwrap_or(false)
            {
                parent_tgid
            } else {
                child_tid
            };

            let child_table = if share_files {
                parent_table
            } else {
                fd_tables.clone_table(parent_table)
            };

            tracees
                .entry(child_tid)
                .or_insert_with(|| TraceeState::child(child_table, child_tgid));

            collector.event(
                tid,
                RawEventKind::ProcessSpawn {
                    child_tid,
                    mechanism,
                },
            );
        }
        libc::PTRACE_EVENT_EXEC => {
            let former_tid = get_event_message(tid)? as libc::pid_t;
            let mut state = if former_tid == tid {
                tracees.remove(&tid).ok_or_else(|| {
                    ObserveError::Protocol(format!(
                        "PTRACE_EVENT_EXEC arrived for untracked tid {tid}"
                    ))
                })?
            } else {
                let execing = tracees.remove(&former_tid).ok_or_else(|| {
                    ObserveError::Protocol(format!(
                        "PTRACE_EVENT_EXEC remapped tid {former_tid} -> {tid}, but former tid was not tracked"
                    ))
                })?;
                if let Some(displaced_leader) = tracees.remove(&tid) {
                    if displaced_leader.tgid != execing.tgid {
                        return Err(ObserveError::Protocol(format!(
                            "PTRACE_EVENT_EXEC identity conflict former_tid={former_tid} new_tid={tid} former_tgid={} displaced_tgid={}",
                            execing.tgid, displaced_leader.tgid
                        )));
                    }
                }
                execing
            };

            let old_tgid = state.tgid;
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

            if former_tid != tid || !retired.is_empty() {
                eprintln!(
                    "M9_EXEC_GROUP_COLLAPSE new_tid={tid} former_tid={former_tid} old_tgid={old_tgid} retired={retired:?}"
                );
            }

            apply_exec_fd_semantics(tid, tracees, fd_tables);
            let state = tracees.get_mut(&tid).ok_or_else(|| {
                ObserveError::Protocol(format!(
                    "PTRACE_EVENT_EXEC lost reconciled state for tid {tid}"
                ))
            })?;
            match state.pending_exec.take() {
                Some(path) => collector.event(tid, RawEventKind::ProcessExec { path }),
                None => collector.warning(
                    tid,
                    "exec_without_path",
                    "PTRACE_EVENT_EXEC was observed without a readable pending exec pathname",
                ),
            }
        }
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
    (old_state, new_state, "tracee thread-group identity"),
    (old_root, new_root, "root tgid initialization"),
    (old_wait_error, new_wait_error, "outer wait ECHILD diagnostic"),
    (old_terminal, new_terminal, "terminal wait handling"),
    (old_event_restart, new_event_restart, "ptrace event restart"),
    (old_syscall_restart, new_syscall_restart, "syscall restart"),
    (old_signal_restart, new_signal_restart, "signal restart"),
    (old_events, new_events, "fork/clone/exec lifecycle reconciliation"),
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
print("M9_PTRACE_ESRCH_THREAD_GROUP_CANDIDATE_APPLIED")
