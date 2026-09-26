#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_init = '''    let mut fd_tables = FdTables::new();
    let mut tracees = HashMap::new();
    tracees.insert(root, TraceeState::root(fd_tables.root_id(), root));

    let mut collector = Collector::new(options.event_limit);
'''
new_init = '''    let mut fd_tables = FdTables::new();
    let mut tracees = HashMap::new();
    tracees.insert(root, TraceeState::root(fd_tables.root_id(), root));
    let mut deferred_newborn_stops: HashMap<libc::pid_t, libc::c_int> = HashMap::new();

    let mut collector = Collector::new(options.event_limit);
'''

old_loop = '''    while !tracees.is_empty() {
        let mut wait_status = 0;
'''
new_loop = '''    while !tracees.is_empty() || !deferred_newborn_stops.is_empty() {
        if tracees.is_empty() && !deferred_newborn_stops.is_empty() {
            let mut deferred = deferred_newborn_stops.keys().copied().collect::<Vec<_>>();
            deferred.sort_unstable();
            return Err(ObserveError::Protocol(format!(
                "M9_DEFERRED_NEWBORNS_WITHOUT_REGISTERING_PARENT tids={deferred:?}"
            )));
        }
        let mut wait_status = 0;
'''

old_echild_head = '''            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
'''
new_echild_head = '''            if error.raw_os_error() == Some(libc::ECHILD) && !deferred_newborn_stops.is_empty() {
                let mut deferred = deferred_newborn_stops.keys().copied().collect::<Vec<_>>();
                deferred.sort_unstable();
                return Err(ObserveError::Protocol(format!(
                    "M9_DEFERRED_NEWBORNS_AT_ECHILD tids={deferred:?}"
                )));
            }
            if error.raw_os_error() == Some(libc::ECHILD) && tracees.is_empty() {
                break;
            }
            if error.raw_os_error() == Some(libc::ECHILD) {
'''

old_terminal_call = '''        if record_terminal_wait_status(
            root,
            tid,
            wait_status,
            &mut tracees,
            &mut root_outcome,
        ) {
            continue;
        }
'''
new_terminal_call = '''        if (libc::WIFEXITED(wait_status) || libc::WIFSIGNALED(wait_status))
            && !tracees.contains_key(&tid)
        {
            let was_deferred = deferred_newborn_stops.remove(&tid).is_some();
            return Err(ObserveError::Protocol(format!(
                "M9_UNTRACKED_TERMINAL_BEFORE_REGISTRATION tid={tid} was_deferred={was_deferred} status={wait_status:#x}"
            )));
        }

        if record_terminal_wait_status(
            root,
            tid,
            wait_status,
            &mut tracees,
            &mut root_outcome,
        ) {
            continue;
        }
'''

old_stop_header = '''        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_int;
        if tracees
'''
new_stop_header = '''        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_int;

        if !tracees.contains_key(&tid) {
            if stop_signal == libc::SIGSTOP && event == 0 {
                if deferred_newborn_stops.insert(tid, wait_status).is_some() {
                    return Err(ObserveError::Protocol(format!(
                        "M9_DUPLICATE_DEFERRED_NEWBORN_STOP tid={tid}"
                    )));
                }
                eprintln!("M9_DEFER_NEWBORN_STOP tid={tid}");
                continue;
            }
            return Err(ObserveError::Protocol(format!(
                "M9_UNTRACKED_NONINITIAL_STOP tid={tid} signal={stop_signal} event={event}"
            )));
        }

        if tracees
'''

old_event_dispatch = '''        if stop_signal == libc::SIGTRAP && event != 0 {
            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;
            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''
new_event_dispatch = '''        if stop_signal == libc::SIGTRAP && event != 0 {
            handle_ptrace_event(tid, event, &mut tracees, &mut fd_tables, &mut collector)?;

            let mut resolved = deferred_newborn_stops
                .keys()
                .filter(|child_tid| tracees.contains_key(child_tid))
                .copied()
                .collect::<Vec<_>>();
            resolved.sort_unstable();
            for child_tid in resolved {
                let deferred_status = deferred_newborn_stops.remove(&child_tid).ok_or_else(|| {
                    ObserveError::Protocol(format!(
                        "M9_DEFERRED_NEWBORN_STATUS_LOST tid={child_tid}"
                    ))
                })?;
                let deferred_signal = libc::WSTOPSIG(deferred_status);
                let deferred_event = ((deferred_status as u32) >> 16) as libc::c_int;
                if !libc::WIFSTOPPED(deferred_status)
                    || deferred_signal != libc::SIGSTOP
                    || deferred_event != 0
                {
                    return Err(ObserveError::Protocol(format!(
                        "M9_DEFERRED_NEWBORN_NOT_INITIAL_STOP tid={child_tid} status={deferred_status:#x} signal={deferred_signal} event={deferred_event}"
                    )));
                }
                let state = tracees.get_mut(&child_tid).ok_or_else(|| {
                    ObserveError::Protocol(format!(
                        "M9_DEFERRED_NEWBORN_NOT_REGISTERED tid={child_tid}"
                    ))
                })?;
                if !state.newborn {
                    return Err(ObserveError::Protocol(format!(
                        "M9_DEFERRED_NEWBORN_ALREADY_CONSUMED tid={child_tid}"
                    )));
                }
                state.newborn = false;
                eprintln!("M9_RESUME_REGISTERED_DEFERRED_NEWBORN tid={child_tid}");
                resume_after_observed_stop(
                    root,
                    child_tid,
                    0,
                    &mut tracees,
                    &mut root_outcome,
                )?;
            }

            resume_after_observed_stop(root, tid, 0, &mut tracees, &mut root_outcome)?;
            continue;
        }
'''

for old, new, label in [
    (old_init, new_init, "deferred newborn map initialization"),
    (old_loop, new_loop, "wait loop condition"),
    (old_echild_head, new_echild_head, "ECHILD deferred-newborn guard"),
    (old_terminal_call, new_terminal_call, "untracked terminal guard"),
    (old_stop_header, new_stop_header, "pre-registration stop deferral"),
    (old_event_dispatch, new_event_dispatch, "registered deferred-newborn resume"),
]:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly one {label} target, found {count}")
    text = text.replace(old, new, 1)

path.write_text(text, encoding="utf-8")
print("M9_PRE_REGISTRATION_STOP_FIX_APPLIED")
