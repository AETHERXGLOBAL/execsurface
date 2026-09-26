#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

replacements = [
    (
'''                    m9_h1_7_bounded_restarts += 1;
                    resume_after_observed_stop(
                        root,
                        tid,
                        0,
                        &mut tracees,
                        &mut root_outcome,
                    )?;
                    continue;
''',
'''                    m9_h1_7_bounded_restarts += 1;
                    let tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);
                    eprintln!(
                        "M9_H1_7B_RECOVERY_BEFORE_RESTART tid={tid} tgid={tgid}"
                    );
                    resume_after_observed_stop(
                        root,
                        tid,
                        0,
                        &mut tracees,
                        &mut root_outcome,
                    )?;
                    let restart_esrch = tracees
                        .get(&tid)
                        .map(|state| state.restart_esrch_seen)
                        .unwrap_or(true);
                    eprintln!(
                        "M9_H1_7B_RECOVERY_AFTER_RESTART tid={tid} tgid={tgid} restart_esrch={restart_esrch}"
                    );
                    continue;
''',
        "bounded recovery transition",
    ),
    (
'''    if apply_terminal_outcome(root, tid, wait_status, root_outcome) {
        tracees.remove(&tid);
        return true;
    }
''',
'''    if apply_terminal_outcome(root, tid, wait_status, root_outcome) {
        if let Some(state) = tracees.get(&tid) {
            if state.exit_group_pending {
                eprintln!(
                    "M9_H1_7B_TERMINAL tid={tid} tgid={} exit_event_seen={} restart_esrch={} syscall_info_esrch={} status={wait_status:#x}",
                    state.tgid,
                    state.exit_event_status.is_some(),
                    state.restart_esrch_seen,
                    state.syscall_info_esrch_after_exit_group,
                );
            }
        }
        tracees.remove(&tid);
        return true;
    }
''',
        "terminal transition",
    ),
    (
'''        libc::PTRACE_EVENT_EXIT => {
            let exit_status = get_event_message(tid)? as libc::c_int;
            let state = tracees.get_mut(&tid).ok_or_else(|| {
                ObserveError::Protocol(format!("PTRACE_EVENT_EXIT arrived for untracked tid {tid}"))
            })?;
            state.exit_event_status = Some(exit_status);
        }
''',
'''        libc::PTRACE_EVENT_EXIT => {
            let exit_status = get_event_message(tid)? as libc::c_int;
            let state = tracees.get_mut(&tid).ok_or_else(|| {
                ObserveError::Protocol(format!("PTRACE_EVENT_EXIT arrived for untracked tid {tid}"))
            })?;
            if state.exit_group_pending {
                eprintln!(
                    "M9_H1_7B_EXIT_EVENT tid={tid} tgid={} restart_esrch={} syscall_info_esrch={} status={exit_status:#x}",
                    state.tgid,
                    state.restart_esrch_seen,
                    state.syscall_info_esrch_after_exit_group,
                );
            }
            state.exit_event_status = Some(exit_status);
        }
''',
        "exit event transition",
    ),
    (
'''    if nr == libc::SYS_exit_group {
        let tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);
        for state in tracees.values_mut() {
            if state.tgid == tgid {
                state.exit_group_pending = true;
            }
        }
    }
''',
'''    if nr == libc::SYS_exit_group {
        let tgid = tracees.get(&tid).map(|state| state.tgid).unwrap_or(tid);
        for state in tracees.values_mut() {
            if state.tgid == tgid {
                state.exit_group_pending = true;
            }
        }
        let mut members = tracees
            .iter()
            .filter_map(|(member_tid, state)| (state.tgid == tgid).then_some(*member_tid))
            .collect::<Vec<_>>();
        members.sort_unstable();
        eprintln!(
            "M9_H1_7B_EXIT_GROUP_ARMED caller_tid={tid} tgid={tgid} members={members:?}"
        );
    }
''',
        "exit group arming",
    ),
]

for old, new, label in replacements:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly one {label}, found {count}")
    text = text.replace(old, new, 1)

path.write_text(text, encoding="utf-8")
print("M9_H1_7B_EXIT_GROUP_PROGRESS_INSTRUMENTATION_APPLIED")
