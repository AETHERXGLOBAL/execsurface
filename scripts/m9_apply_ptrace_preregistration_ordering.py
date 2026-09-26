#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_state_init = '''    let mut tracees = HashMap::new();
    tracees.insert(root, TraceeState::root(fd_tables.root_id(), root));

    let mut collector = Collector::new(options.event_limit);
'''
new_state_init = '''    let mut tracees = HashMap::new();
    tracees.insert(root, TraceeState::root(fd_tables.root_id(), root));
    let mut preregistration_stops: HashMap<libc::pid_t, libc::c_int> = HashMap::new();

    let mut collector = Collector::new(options.event_limit);
'''

old_loop = '''    while !tracees.is_empty() {
'''
new_loop = '''    while !tracees.is_empty() || !preregistration_stops.is_empty() {
'''

old_echild_prefix = '''            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
'''
new_echild_prefix = '''            if error.raw_os_error() == Some(libc::ECHILD)
                && tracees.is_empty()
                && preregistration_stops.is_empty()
            {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) && !preregistration_stops.is_empty() {
                let mut pending = preregistration_stops.keys().copied().collect::<Vec<_>>();
                pending.sort_unstable();
                return Err(ObserveError::Protocol(format!(
                    "M9_PREREGISTRATION_STOPS_AT_ECHILD pending_tids={pending:?}"
                )));
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
'''

old_terminal_dispatch = '''        if record_terminal_wait_status(root, tid, wait_status, &mut tracees, &mut root_outcome) {
            continue;
        }

        if !libc::WIFSTOPPED(wait_status) {
'''
new_terminal_dispatch = '''        if (libc::WIFEXITED(wait_status) || libc::WIFSIGNALED(wait_status))
            && !tracees.contains_key(&tid)
        {
            return Err(ObserveError::Protocol(format!(
                "M9_UNTRACKED_TERMINAL_WAIT tid={tid} status={wait_status:#x}"
            )));
        }

        if record_terminal_wait_status(root, tid, wait_status, &mut tracees, &mut root_outcome) {
            continue;
        }

        if !libc::WIFSTOPPED(wait_status) {
'''

old_stop_dispatch = '''        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_int;

        if tracees
'''
new_stop_dispatch = '''        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_int;

        if !tracees.contains_key(&tid) {
            if stop_signal != libc::SIGSTOP || event != 0 {
                return Err(ObserveError::Protocol(format!(
                    "M9_UNEXPECTED_STOP_BEFORE_REGISTRATION tid={tid} signal={stop_signal} event={event} status={wait_status:#x}"
                )));
            }
            if preregistration_stops.insert(tid, wait_status).is_some() {
                return Err(ObserveError::Protocol(format!(
                    "M9_DUPLICATE_PREREGISTRATION_STOP tid={tid}"
                )));
            }
            eprintln!("M9_PREREGISTRATION_STOP_BUFFERED tid={tid}");
            continue;
        }

        if tracees
'''

old_event_call = '''            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
'''
new_event_call = '''            handle_ptrace_event(
                tid,
                event,
                &mut tracees,
                &mut preregistration_stops,
                &mut fd_tables,
                &mut collector,
            )?;
'''

old_event_signature = '''fn handle_ptrace_event(
    tid: libc::pid_t,
    event: libc::c_int,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    fd_tables: &mut FdTables,
    collector: &mut Collector,
) -> Result<(), ObserveError> {
'''
new_event_signature = '''fn handle_ptrace_event(
    tid: libc::pid_t,
    event: libc::c_int,
    tracees: &mut HashMap<libc::pid_t, TraceeState>,
    preregistration_stops: &mut HashMap<libc::pid_t, libc::c_int>,
    fd_tables: &mut FdTables,
    collector: &mut Collector,
) -> Result<(), ObserveError> {
'''

old_child_registration = '''            tracees
                .entry(child_tid)
                .or_insert_with(|| TraceeState::child(child_table, child_tgid));

            collector.event(
'''
new_child_registration = '''            tracees
                .entry(child_tid)
                .or_insert_with(|| TraceeState::child(child_table, child_tgid));

            if let Some(buffered_status) = preregistration_stops.remove(&child_tid) {
                let buffered_signal = libc::WSTOPSIG(buffered_status);
                let buffered_event = ((buffered_status as u32) >> 16) as libc::c_int;
                if !libc::WIFSTOPPED(buffered_status)
                    || buffered_signal != libc::SIGSTOP
                    || buffered_event != 0
                {
                    return Err(ObserveError::Protocol(format!(
                        "M9_INVALID_BUFFERED_PREREGISTRATION_STOP tid={child_tid} signal={buffered_signal} event={buffered_event} status={buffered_status:#x}"
                    )));
                }
                let child_state = tracees.get_mut(&child_tid).ok_or_else(|| {
                    ObserveError::Protocol(format!(
                        "M9_REGISTERED_CHILD_STATE_MISSING tid={child_tid}"
                    ))
                })?;
                child_state.newborn = false;
                eprintln!(
                    "M9_PREREGISTRATION_STOP_RECONCILED child_tid={child_tid} parent_tid={tid} child_tgid={child_tgid}"
                );
                resume_syscall(child_tid, 0)?;
            }

            collector.event(
'''

replacements = [
    (old_state_init, new_state_init, "pre-registration stop map initialization", 1),
    (old_loop, new_loop, "wait loop pending-stop lifetime", 1),
    (old_echild_prefix, new_echild_prefix, "ECHILD pending-stop gate", 1),
    (old_terminal_dispatch, new_terminal_dispatch, "untracked terminal fail-closed gate", 1),
    (old_stop_dispatch, new_stop_dispatch, "pre-registration stop buffering", 1),
    (old_event_call, new_event_call, "ptrace event preregistration map wiring", 1),
    (old_event_signature, new_event_signature, "ptrace event preregistration parameter", 1),
    (old_child_registration, new_child_registration, "child registration reconciliation", 1),
]

for old, new, label, expected in replacements:
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"expected {expected} {label} replacement(s), found {count}")
    text = text.replace(old, new, expected)

path.write_text(text, encoding="utf-8")
print("M9_PTRACE_PREREGISTRATION_ORDERING_CANDIDATE_APPLIED")
