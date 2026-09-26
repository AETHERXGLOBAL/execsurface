#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old = '''    let result = unsafe { libc::ptrace(request, tid, address, data) };
    if result == -1 {
        Err(io::Error::last_os_error().into())
    } else {
        Ok(())
    }
'''
new = '''    let result = unsafe { libc::ptrace(request, tid, address, data) };
    if result == -1 {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            eprintln!("M9_PTRACE_CALL_ESRCH request={request} tid={tid}");
        }
        Err(error.into())
    } else {
        Ok(())
    }
'''

count = text.count(old)
if count != 1:
    raise SystemExit(f"expected exactly one generic ptrace_call body, found {count}")

path.write_text(text.replace(old, new, 1), encoding="utf-8")
print("M9_GENERIC_PTRACE_ESRCH_DIAGNOSTIC_APPLIED")
