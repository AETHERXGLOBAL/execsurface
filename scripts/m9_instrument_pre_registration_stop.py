#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old = '''        if !libc::WIFSTOPPED(wait_status) {
            continue;
        }

        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_int;
'''
new = '''        if !libc::WIFSTOPPED(wait_status) {
            continue;
        }

        let stop_signal = libc::WSTOPSIG(wait_status);
        let event = ((wait_status as u32) >> 16) as libc::c_int;
        if !tracees.contains_key(&tid) {
            let proc_tgid = fs::read_to_string(format!("/proc/{tid}/status"))
                .ok()
                .and_then(|status| {
                    status.lines().find_map(|line| {
                        line.strip_prefix("Tgid:")
                            .and_then(|value| value.trim().parse::<libc::pid_t>().ok())
                    })
                });
            eprintln!(
                "M9_UNTRACKED_STOP_BEFORE_REGISTRATION tid={tid} proc_tgid={proc_tgid:?} signal={stop_signal} event={event}"
            );
        }
'''

count = text.count(old)
if count != 1:
    raise SystemExit(f"expected exactly one pre-registration stop diagnostic target, found {count}")
text = text.replace(old, new, 1)
path.write_text(text, encoding="utf-8")
print("M9_PRE_REGISTRATION_STOP_DIAGNOSTIC_APPLIED")
