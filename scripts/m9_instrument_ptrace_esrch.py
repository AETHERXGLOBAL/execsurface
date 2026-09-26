#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text(encoding="utf-8")

old_syscall_info = '''fn syscall_info(tid: libc::pid_t) -> Result<PtraceSyscallInfo, ObserveError> {
    let mut info = MaybeUninit::<PtraceSyscallInfo>::zeroed();
    let result = unsafe {
        libc::ptrace(
            PTRACE_GET_SYSCALL_INFO_REQUEST,
            tid,
            size_of::<PtraceSyscallInfo>() as *mut c_void,
            info.as_mut_ptr() as *mut c_void,
        )
    };
    if result == -1 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(unsafe { info.assume_init() })
}
'''
new_syscall_info = '''fn syscall_info(tid: libc::pid_t) -> Result<PtraceSyscallInfo, ObserveError> {
    let mut info = MaybeUninit::<PtraceSyscallInfo>::zeroed();
    let result = unsafe {
        libc::ptrace(
            PTRACE_GET_SYSCALL_INFO_REQUEST,
            tid,
            size_of::<PtraceSyscallInfo>() as *mut c_void,
            info.as_mut_ptr() as *mut c_void,
        )
    };
    if result == -1 {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            return Err(ObserveError::Protocol(format!(
                "M9_PTRACE_ESRCH op=GET_SYSCALL_INFO tid={tid}"
            )));
        }
        return Err(error.into());
    }
    Ok(unsafe { info.assume_init() })
}
'''

old_get_event = '''fn get_event_message(tid: libc::pid_t) -> Result<u64, ObserveError> {
    let mut value: libc::c_ulong = 0;
    ptrace_call(
        libc::PTRACE_GETEVENTMSG,
        tid,
        ptr::null_mut(),
        (&mut value as *mut libc::c_ulong).cast::<c_void>(),
    )?;
    Ok(value as u64)
}
'''
new_get_event = '''fn get_event_message(tid: libc::pid_t) -> Result<u64, ObserveError> {
    let mut value: libc::c_ulong = 0;
    match ptrace_call(
        libc::PTRACE_GETEVENTMSG,
        tid,
        ptr::null_mut(),
        (&mut value as *mut libc::c_ulong).cast::<c_void>(),
    ) {
        Ok(()) => Ok(value as u64),
        Err(ObserveError::Os(error)) if error.raw_os_error() == Some(libc::ESRCH) => {
            Err(ObserveError::Protocol(format!(
                "M9_PTRACE_ESRCH op=GETEVENTMSG tid={tid}"
            )))
        }
        Err(error) => Err(error),
    }
}
'''

old_peek = '''fn peek_word(tid: libc::pid_t, address: u64) -> Result<libc::c_long, ObserveError> {
    set_errno(0);
    let result = unsafe {
        libc::ptrace(
            libc::PTRACE_PEEKDATA,
            tid,
            address as usize as *mut c_void,
            ptr::null_mut::<c_void>(),
        )
    };
    let error = io::Error::last_os_error();
    if result == -1 && error.raw_os_error() != Some(0) {
        return Err(error.into());
    }
    Ok(result)
}
'''
new_peek = '''fn peek_word(tid: libc::pid_t, address: u64) -> Result<libc::c_long, ObserveError> {
    set_errno(0);
    let result = unsafe {
        libc::ptrace(
            libc::PTRACE_PEEKDATA,
            tid,
            address as usize as *mut c_void,
            ptr::null_mut::<c_void>(),
        )
    };
    let error = io::Error::last_os_error();
    if result == -1 && error.raw_os_error() != Some(0) {
        if error.raw_os_error() == Some(libc::ESRCH) {
            return Err(ObserveError::Protocol(format!(
                "M9_PTRACE_ESRCH op=PEEKDATA tid={tid} address={address:#x}"
            )));
        }
        return Err(error.into());
    }
    Ok(result)
}
'''

for old, new, label in [
    (old_syscall_info, new_syscall_info, "GET_SYSCALL_INFO"),
    (old_get_event, new_get_event, "GETEVENTMSG"),
    (old_peek, new_peek, "PEEKDATA"),
]:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly one {label} instrumentation target, found {count}")
    text = text.replace(old, new, 1)

path.write_text(text, encoding="utf-8")
print("M9_PTRACE_ESRCH_OPERATION_INSTRUMENTATION_APPLIED")
