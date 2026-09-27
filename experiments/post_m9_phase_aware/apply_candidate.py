from pathlib import Path

p = Path("crates/execsurface-observe/src/linux_ptrace.rs")
s = p.read_text()

marker = "#[derive(Debug, Clone)]\nstruct TraceeState {"
assert marker in s
enum = """#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyscallPhase {
    Unknown,
    ExpectEntry,
    ExpectExit { skip_info: bool },
}

"""
s = s.replace(marker, enum + marker, 1)

field = "    pending_syscall: Option<PendingSyscall>,\n"
assert s.count(field) == 1
s = s.replace(field, field + "    syscall_phase: SyscallPhase,\n", 1)

init = "            pending_syscall: None,\n            newborn:"
assert s.count(init) == 2
s = s.replace(
    init,
    "            pending_syscall: None,\n            syscall_phase: SyscallPhase::Unknown,\n            newborn:",
)

start = s.index("        if stop_signal == (libc::SIGTRAP | 0x80) {")
end = s.index("\n        let suppress_signal =", start)
new_block = r'''        if stop_signal == (libc::SIGTRAP | 0x80) {
            let can_skip_exit_info = tracees
                .get(&tid)
                .map(|state| {
                    matches!(
                        state.syscall_phase,
                        SyscallPhase::ExpectExit { skip_info: true }
                    )
                })
                .unwrap_or(false);

            if can_skip_exit_info {
                let state = tracees.get_mut(&tid).ok_or_else(|| {
                    ObserveError::Protocol(format!(
                        "phase-aware exit fast path lost tracked tid {tid}"
                    ))
                })?;
                if state.pending_syscall.is_some()
                    || state.pending_exec.is_some()
                    || state.exit_group_pending
                    || state.retired_by_exec
                    || state.exit_event_status.is_some()
                    || state.newborn
                {
                    return Err(ObserveError::Protocol(format!(
                        "phase-aware exit fast path reached unsafe state for tid {tid}"
                    )));
                }
                state.syscall_phase = SyscallPhase::ExpectEntry;
                resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
                continue;
            }

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
                    state.syscall_phase = SyscallPhase::Unknown;
                    continue;
                }
                Err(error) => return Err(error),
            };

            if let Some((nr, args)) = info.entry() {
                if let Some(state) = tracees.get_mut(&tid) {
                    state.syscall_phase = SyscallPhase::Unknown;
                }
                handle_syscall_entry(tid, nr, args, &mut tracees, &mut collector);
                let skip_info = tracees
                    .get(&tid)
                    .map(|state| {
                        state.pending_syscall.is_none()
                            && state.pending_exec.is_none()
                            && !state.exit_group_pending
                            && !state.retired_by_exec
                            && state.exit_event_status.is_none()
                            && !state.newborn
                    })
                    .unwrap_or(false);
                if let Some(state) = tracees.get_mut(&tid) {
                    state.syscall_phase = SyscallPhase::ExpectExit { skip_info };
                }
            } else if let Some(result) = info.exit() {
                handle_syscall_exit(tid, result, &mut tracees, &mut fd_tables, &mut collector);
                if let Some(state) = tracees.get_mut(&tid) {
                    state.syscall_phase = SyscallPhase::ExpectEntry;
                }
            } else if let Some(state) = tracees.get_mut(&tid) {
                state.syscall_phase = SyscallPhase::Unknown;
            }

            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''
s = s[:start] + new_block + s[end:]

restart = "            state.restart_esrch_seen = true;\n            Ok(())"
assert s.count(restart) == 1
s = s.replace(
    restart,
    "            state.restart_esrch_seen = true;\n            state.syscall_phase = SyscallPhase::Unknown;\n            Ok(())",
    1,
)

hstart = s.index("fn handle_ptrace_event(")
matchpos = s.index("    match event {", hstart)
s = (
    s[:matchpos]
    + "    if let Some(state) = tracees.get_mut(&tid) {\n        state.syscall_phase = SyscallPhase::Unknown;\n    }\n\n"
    + s[matchpos:]
)

retired = "                    other_state.pending_syscall = None;\n                    retired.push(*other_tid);"
assert s.count(retired) == 1
s = s.replace(
    retired,
    "                    other_state.pending_syscall = None;\n                    other_state.syscall_phase = SyscallPhase::Unknown;\n                    retired.push(*other_tid);",
    1,
)

execreset = "            state.syscall_info_esrch_after_exit_group = false;\n            tracees.insert(tid, state);"
assert s.count(execreset) == 1
s = s.replace(
    execreset,
    "            state.syscall_info_esrch_after_exit_group = false;\n            state.syscall_phase = SyscallPhase::Unknown;\n            tracees.insert(tid, state);",
    1,
)

p.write_text(s)
