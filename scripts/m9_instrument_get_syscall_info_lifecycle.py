#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_init = '''    let mut collector = Collector::new(options.event_limit);
    let mut root_outcome = CommandOutcome::default();
'''
new_init = '''    let mut collector = Collector::new(options.event_limit);
    let mut root_outcome = CommandOutcome::default();
    let mut get_syscall_info_esrch_diagnostics = Vec::new();
'''

old_syscall_stop = '''        if stop_signal == (libc::SIGTRAP | 0x80) {
            let info = syscall_info(tid)?;
            if let Some((nr, args)) = info.entry() {
'''
new_syscall_stop = '''        if stop_signal == (libc::SIGTRAP | 0x80) {
            let info = match syscall_info(tid) {
                Ok(info) => info,
                Err(ObserveError::Protocol(message))
                    if message.starts_with("M9_PTRACE_ESRCH op=GET_SYSCALL_INFO") =>
                {
                    let state = tracees.get(&tid).ok_or_else(|| {
                        ObserveError::Protocol(format!(
                            "M9_GET_SYSCALL_INFO_ESRCH_UNTRACKED tid={tid}"
                        ))
                    })?;
                    let tgid = state.tgid;
                    let pending_syscall = state.pending_syscall.is_some();
                    let retired_by_exec = state.retired_by_exec;
                    let restart_esrch_seen = state.restart_esrch_seen;
                    let exit_event_seen = state.exit_event_status.is_some();

                    let proc_status = fs::read_to_string(format!("/proc/{tid}/status")).ok();
                    let proc_state = proc_status.as_deref().and_then(|status| {
                        status.lines().find_map(|line| {
                            line.strip_prefix("State:").map(str::trim).map(str::to_owned)
                        })
                    });
                    let proc_tgid = proc_status.as_deref().and_then(|status| {
                        status.lines().find_map(|line| {
                            line.strip_prefix("Tgid:")
                                .and_then(|value| value.trim().parse::<libc::pid_t>().ok())
                        })
                    });
                    let mut task_tids = fs::read_dir(format!("/proc/{tgid}/task"))
                        .ok()
                        .into_iter()
                        .flatten()
                        .filter_map(Result::ok)
                        .filter_map(|entry| entry.file_name().to_string_lossy().parse::<libc::pid_t>().ok())
                        .collect::<Vec<_>>();
                    task_tids.sort_unstable();
                    let task_present = task_tids.binary_search(&tid).is_ok();
                    let task_count = task_tids.len();

                    eprintln!(
                        "M9_GET_SYSCALL_INFO_ESRCH_CONTEXT tid={tid} tgid={tgid} pending_syscall={pending_syscall} retired_by_exec={retired_by_exec} restart_esrch_seen={restart_esrch_seen} exit_event_seen={exit_event_seen} proc_present={} proc_state={proc_state:?} proc_tgid={proc_tgid:?} task_present={task_present} task_count={task_count}",
                        proc_status.is_some(),
                    );
                    get_syscall_info_esrch_diagnostics.push((
                        tid,
                        tgid,
                        pending_syscall,
                        retired_by_exec,
                        restart_esrch_seen,
                        exit_event_seen,
                        proc_status.is_some(),
                        task_present,
                        task_count,
                    ));
                    // Diagnostic-only: do not resume a tracee that has already refused a
                    // ptrace inspection. Continue collecting later lifecycle notifications,
                    // but force the overall observation to ERROR before returning it.
                    continue;
                }
                Err(error) => return Err(error),
            };
            if let Some((nr, args)) = info.entry() {
'''

old_finish = '''    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}
'''
new_finish = '''    if !get_syscall_info_esrch_diagnostics.is_empty() {
        return Err(ObserveError::Protocol(format!(
            "M9_GET_SYSCALL_INFO_ESRCH_LIFECYCLE_DIAGNOSTIC observations={get_syscall_info_esrch_diagnostics:?}"
        )));
    }

    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}
'''

old_exit_capture = '''            state.exit_event_status = Some(exit_status);
        }
'''
new_exit_capture = '''            state.exit_event_status = Some(exit_status);
            eprintln!(
                "M9_EXIT_EVENT_OBSERVED tid={tid} tgid={} retired_by_exec={} restart_esrch_seen={} exit_status={exit_status:#x}",
                state.tgid,
                state.retired_by_exec,
                state.restart_esrch_seen,
            );
        }
'''

for old, new, label, expected in [
    (old_init, new_init, "diagnostic accumulator", 1),
    (old_syscall_stop, new_syscall_stop, "GET_SYSCALL_INFO ESRCH context capture", 1),
    (old_finish, new_finish, "diagnostic fail-closed finish", 1),
    (old_exit_capture, new_exit_capture, "exit event marker", 1),
]:
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"expected {expected} {label} replacement(s), found {count}")
    text = text.replace(old, new, expected)

path.write_text(text, encoding="utf-8")
print("M9_GET_SYSCALL_INFO_LIFECYCLE_DIAGNOSTIC_APPLIED")
