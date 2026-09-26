#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_counter = '''    let mut collector = Collector::new(options.event_limit);
    let mut root_outcome = CommandOutcome::default();

    resume_syscall(root, 0)?;
'''
new_counter = '''    let mut collector = Collector::new(options.event_limit);
    let mut root_outcome = CommandOutcome::default();
    let mut m9_h1_9_bounded_getinfo_esrch = 0usize;

    resume_syscall(root, 0)?;
'''

old_recovery = '''                    let state = tracees.get_mut(&tid).ok_or_else(|| {
                        ObserveError::Protocol(format!(
                            "bounded GET_SYSCALL_INFO ESRCH recovery lost tracked tid {tid}"
                        ))
                    })?;
                    state.syscall_info_esrch_after_exit_group = true;
                    /*
                     * The kernel has already refused a ptrace read while the
                     * same thread group is in an explicitly observed
                     * exit_group teardown. Do not fabricate syscall phase
                     * information and do not restart the dying thread here.
                     * Its subsequent PTRACE_EVENT_EXIT / terminal wait remains
                     * responsible for lifecycle completion.
                     */
                    continue;
'''
new_recovery = '''                    let state = tracees.get_mut(&tid).ok_or_else(|| {
                        ObserveError::Protocol(format!(
                            "bounded GET_SYSCALL_INFO ESRCH recovery lost tracked tid {tid}"
                        ))
                    })?;
                    state.syscall_info_esrch_after_exit_group = true;
                    m9_h1_9_bounded_getinfo_esrch += 1;
                    /*
                     * The kernel has already refused a ptrace read while the
                     * same thread group is in an explicitly observed
                     * exit_group teardown. Do not fabricate syscall phase
                     * information and do not restart the dying thread here.
                     * Its subsequent PTRACE_EVENT_EXIT / terminal wait remains
                     * responsible for lifecycle completion.
                     */
                    continue;
'''

old_finish = '''    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}
'''
new_finish = '''    eprintln!(
        "M9_H1_9_BOUNDED_GETINFO_ESRCH count={m9_h1_9_bounded_getinfo_esrch}"
    );
    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}
'''

replacements = [
    (old_counter, new_counter, "diagnostic counter anchor"),
    (old_recovery, new_recovery, "bounded GET_SYSCALL_INFO ESRCH counter"),
    (old_finish, new_finish, "diagnostic final marker"),
]

for old, new, label in replacements:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly one {label}, found {count}")
    text = text.replace(old, new, 1)

path.write_text(text, encoding="utf-8")
print("M9_H1_9_BOUNDED_RECOVERY_COUNTER_APPLIED")
