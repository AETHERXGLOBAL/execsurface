#!/usr/bin/env python3
from hashlib import sha256
from pathlib import Path

SOURCE = Path("crates/execsurface-observe/src/linux_ptrace.rs")
PROVED_SHA256 = "72978ef99fd24b944cb191d5eeae665387ed223271c31e5636818f5ea81def37"
PRODUCTION_SHA256 = "3dc656358a3675a0d78135bb83046bc45c33dca2718c8bd8b68fa090e18eac8b"

text = SOURCE.read_text(encoding="utf-8")
actual_proved = sha256(text.encode("utf-8")).hexdigest()
if actual_proved != PROVED_SHA256:
    raise SystemExit(
        f"refusing cleanup: expected proved source {PROVED_SHA256}, got {actual_proved}"
    )

blocks = [
    '                eprintln!("M9_EXIT_EVENT_RECONCILE_AT_ECHILD reconciled={reconciled:?}");\n',
    '            eprintln!("M9_PREREGISTRATION_STOP_BUFFERED tid={tid}");\n',
    '''                    eprintln!(
                        "M9_GET_SYSCALL_INFO_EXIT_GROUP_DEATH_DEFERRED tid={tid} tgid={}",
                        state.tgid
                    );
''',
    '''            eprintln!(
                "M9_RESTART_ESRCH_DEFERRED tid={tid} tgid={} retired_by_exec={} exit_event_seen={}",
                state.tgid,
                state.retired_by_exec,
                state.exit_event_status.is_some()
            );
''',
    '''                eprintln!(
                    "M9_PREREGISTRATION_STOP_RECONCILED child_tid={child_tid} parent_tid={tid} child_tgid={child_tgid}"
                );
''',
    '''            if former_tid != tid || !retired.is_empty() {
                eprintln!(
                    "M9_EXEC_GROUP_COLLAPSE new_tid={tid} former_tid={former_tid} old_tgid={old_tgid} retired={retired:?}"
                );
            }

''',
]

for index, block in enumerate(blocks, start=1):
    count = text.count(block)
    if count != 1:
        raise SystemExit(
            f"refusing cleanup: expected exactly one diagnostic block {index}, found {count}"
        )
    text = text.replace(block, "", 1)

SOURCE.write_text(text, encoding="utf-8")
actual_production = sha256(text.encode("utf-8")).hexdigest()
if actual_production != PRODUCTION_SHA256:
    raise SystemExit(
        f"production source hash mismatch: expected {PRODUCTION_SHA256}, got {actual_production}"
    )

print(f"M9_PTRACE_PRODUCTION_SOURCE_READY sha256={actual_production}")
