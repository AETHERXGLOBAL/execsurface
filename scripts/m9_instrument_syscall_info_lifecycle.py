#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_state_tail = '''    retired_by_exec: bool,
    restart_esrch_seen: bool,
    exit_event_status: Option<libc::c_int>,
}
'''
new_state_tail = '''    retired_by_exec: bool,
    restart_esrch_seen: bool,
    exit_event_status: Option<libc::c_int>,
    diagnostic_last_syscall_nr: Option<u64>,
    diagnostic_exit_group_pending: bool,
}
'''

old_ctor_tail = '''            retired_by_exec: false,
            restart_esrch_seen: false,
            exit_event_status: None,
        }
'''
new_ctor_tail = '''            retired_by_exec: false,
            restart_esrch_seen: false,
            exit_event_status: None,
            diagnostic_last_syscall_nr: None,
            diagnostic_exit_group_pending: false,
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
                    let state = tracees.get(&tid);
                    let tgid = state.map(|state| state.tgid);
                    let mut group_members = tgid
                        .map(|tgid| {
                            tracees
                                .iter()
                                .filter_map(|(member_tid, member)| {
                                    (member.tgid == tgid).then_some((
                                        *member_tid,
                                        member.diagnostic_last_syscall_nr,
                                        member.diagnostic_exit_group_pending,
                                        member.pending_syscall.is_some(),
                                        member.exit_event_status.is_some(),
                                        member.retired_by_exec,
                                        member.restart_esrch_seen,
                                    ))
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    group_members.sort_unstable_by_key(|entry| entry.0);
                    let proc_status_exists = fs::metadata(format!("/proc/{tid}/status")).is_ok();
                    return Err(ObserveError::Protocol(format!(
                        "M9_GET_SYSCALL_INFO_ESRCH_STATE tid={tid} tgid={tgid:?} last_syscall_nr={:?} exit_group_pending={} selected_pending={} exit_event_seen={} retired_by_exec={} restart_esrch_seen={} proc_status_exists={proc_status_exists} group_members={group_members:?}",
                        state.and_then(|state| state.diagnostic_last_syscall_nr),
                        state.map(|state| state.diagnostic_exit_group_pending).unwrap_or(false),
                        state.map(|state| state.pending_syscall.is_some()).unwrap_or(false),
                        state.map(|state| state.exit_event_status.is_some()).unwrap_or(false),
                        state.map(|state| state.retired_by_exec).unwrap_or(false),
                        state.map(|state| state.restart_esrch_seen).unwrap_or(false),
                    )));
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

old_nr_cast = '''    let nr = nr as libc::c_long;

    if nr == libc::SYS_execve {
'''
new_nr_cast = '''    if let Some(state) = tracees.get_mut(&tid) {
        state.diagnostic_last_syscall_nr = Some(nr);
    }

    let nr = nr as libc::c_long;

    if nr == libc::SYS_exit_group {
        let tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);
        let mut members = Vec::new();
        for (member_tid, member) in tracees.iter_mut() {
            if member.tgid == tgid {
                member.diagnostic_exit_group_pending = true;
                members.push(*member_tid);
            }
        }
        members.sort_unstable();
        eprintln!(
            "M9_EXIT_GROUP_ARMED caller_tid={tid} tgid={tgid} exit_code={} members={members:?}",
            args[0] as i32
        );
    }

    if nr == libc::SYS_execve {
'''

old_exit_start = '''fn handle_syscall_exit(
    tid: libc::pid_t,
    result: i64,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    fd_tables: &mut FdTables,
    collector: &mut Collector,
) {
    let Some(pending) = tracees
'''
new_exit_start = '''fn handle_syscall_exit(
    tid: libc::pid_t,
    result: i64,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    fd_tables: &mut FdTables,
    collector: &mut Collector,
) {
    if let Some(state) = tracees.get_mut(&tid) {
        state.diagnostic_last_syscall_nr = None;
    }
    let Some(pending) = tracees
'''

old_exec_reset = '''            state.retired_by_exec = false;
            state.restart_esrch_seen = false;
            state.exit_event_status = None;
            tracees.insert(tid, state);
'''
new_exec_reset = '''            state.retired_by_exec = false;
            state.restart_esrch_seen = false;
            state.exit_event_status = None;
            state.diagnostic_last_syscall_nr = None;
            state.diagnostic_exit_group_pending = false;
            tracees.insert(tid, state);
'''

for old, new, label, expected in [
    (old_state_tail, new_state_tail, "diagnostic lifecycle state", 1),
    (old_ctor_tail, new_ctor_tail, "diagnostic constructor state", 2),
    (old_syscall_stop, new_syscall_stop, "GET_SYSCALL_INFO lifecycle capture", 1),
    (old_nr_cast, new_nr_cast, "syscall entry and exit_group tracking", 1),
    (old_exit_start, new_exit_start, "successful syscall exit phase clear", 1),
    (old_exec_reset, new_exec_reset, "exec diagnostic state reset", 1),
]:
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"expected {expected} {label} replacement(s), found {count}")
    text = text.replace(old, new, expected)

path.write_text(text, encoding="utf-8")
print("M9_SYSCALL_INFO_LIFECYCLE_DIAGNOSTIC_APPLIED")
