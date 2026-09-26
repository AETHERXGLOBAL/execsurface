#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_counter_anchor = '''    let mut collector = Collector::new(options.event_limit);
    let mut root_outcome = CommandOutcome::default();

    resume_syscall(root, 0)?;
'''
new_counter_anchor = '''    let mut collector = Collector::new(options.event_limit);
    let mut root_outcome = CommandOutcome::default();
    let mut m9_h1_7_bounded_restarts = 0usize;

    resume_syscall(root, 0)?;
'''

old_recovery = '''                    let state = tracees.get_mut(&tid).ok_or_else(|| {
                        ObserveError::Protocol(format!(
                            "bounded GET_SYSCALL_INFO ESRCH recovery lost tracked tid {tid}"
                        ))
                    })?;
                    state.syscall_info_esrch_after_exit_group = true;
                    eprintln!(
                        "M9_GET_SYSCALL_INFO_EXIT_GROUP_DEATH_DEFERRED tid={tid} tgid={}",
                        state.tgid
                    );
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
new_recovery = '''                    {
                        let state = tracees.get_mut(&tid).ok_or_else(|| {
                            ObserveError::Protocol(format!(
                                "bounded GET_SYSCALL_INFO ESRCH recovery lost tracked tid {tid}"
                            ))
                        })?;
                        state.syscall_info_esrch_after_exit_group = true;
                    }
                    /*
                     * waitpid has already reported a syscall-stop. Informational
                     * PTRACE_GET_SYSCALL_INFO can race with exit_group teardown
                     * and return ESRCH, but leaving that reported ptrace-stop
                     * unrestarted can deadlock lifecycle completion. Perform
                     * exactly one normal restart here. The existing restart
                     * path records ESRCH without retrying, and later EXIT or
                     * terminal evidence remains mandatory for reconciliation.
                     */
                    m9_h1_7_bounded_restarts += 1;
                    resume_after_observed_stop(
                        root,
                        tid,
                        0,
                        &mut tracees,
                        &mut root_outcome,
                    )?;
                    continue;
'''

old_finish = '''    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}
'''
new_finish = '''    eprintln!(
        "M9_H1_7_BOUNDED_GET_SYSCALL_INFO_ESRCH_RESTARTS count={m9_h1_7_bounded_restarts}"
    );
    collector.observation.outcome = root_outcome;
    Ok(collector.observation)
}
'''

replacements = [
    (old_counter_anchor, new_counter_anchor, "bounded restart counter", 1),
    (old_recovery, new_recovery, "single bounded restart", 1),
    (old_finish, new_finish, "post-observation counter emission", 1),
]

for old, new, label, expected in replacements:
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"expected {expected} {label} replacement(s), found {count}")
    text = text.replace(old, new, expected)

path.write_text(text, encoding="utf-8")
print("M9_H1_7_SINGLE_RESTART_CANDIDATE_APPLIED")
