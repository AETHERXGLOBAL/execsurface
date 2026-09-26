#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

removals = [
    '                eprintln!("M9_EXIT_EVENT_RECONCILE_AT_ECHILD reconciled={reconciled:?}");\n',
    '            eprintln!("M9_PREREGISTRATION_STOP_BUFFERED tid={tid}");\n',
    '''                    eprintln!(\n                        "M9_GET_SYSCALL_INFO_EXIT_GROUP_DEATH_DEFERRED tid={tid} tgid={}",\n                        state.tgid\n                    );\n''',
    '''            eprintln!(\n                "M9_RESTART_ESRCH_DEFERRED tid={tid} tgid={} retired_by_exec={} exit_event_seen={}",\n                state.tgid,\n                state.retired_by_exec,\n                state.exit_event_status.is_some()\n            );\n''',
    '''                eprintln!(\n                    "M9_PREREGISTRATION_STOP_RECONCILED child_tid={child_tid} parent_tid={tid} child_tgid={child_tgid}"\n                );\n''',
    '''                eprintln!(\n                    "M9_EXEC_GROUP_COLLAPSE new_tid={tid} former_tid={former_tid} old_tgid={old_tgid} retired={retired:?}"\n                );\n''',
]

for index, block in enumerate(removals, start=1):
    count = text.count(block)
    if count != 1:
        raise SystemExit(f"expected exactly one diagnostic output block {index}, found {count}")
    text = text.replace(block, "", 1)

if "eprintln!" in text:
    raise SystemExit("unexpected eprintln! remains after bounded M9 diagnostic cleanup")

path.write_text(text, encoding="utf-8")
print("M9_PTRACE_DIAGNOSTIC_OUTPUT_STRIPPED")
