from pathlib import Path

p = Path("crates/execsurface-observe/src/linux_ptrace.rs")
s = p.read_text()

start_marker = "fn read_c_string(tid: libc::pid_t, address: u64, max_len: usize) -> Result<String, ObserveError> {"
end_marker = "fn peek_word(tid: libc::pid_t, address: u64) -> Result<libc::c_long, ObserveError> {"
assert s.count(start_marker) == 1
assert s.count(end_marker) == 1
start = s.index(start_marker)
end = s.index(end_marker, start)

replacement = r'''fn process_vm_read_exact(
    tid: libc::pid_t,
    address: u64,
    len: usize,
) -> Option<Vec<u8>> {
    if len == 0 {
        return Some(Vec::new());
    }

    let mut bytes = vec![0u8; len];
    let local = libc::iovec {
        iov_base: bytes.as_mut_ptr().cast::<c_void>(),
        iov_len: len,
    };
    let remote = libc::iovec {
        iov_base: address as usize as *mut c_void,
        iov_len: len,
    };

    set_errno(0);
    let read = unsafe { libc::process_vm_readv(tid, &local, 1, &remote, 1, 0) };
    if read == len as isize {
        Some(bytes)
    } else {
        None
    }
}

fn read_c_string(tid: libc::pid_t, address: u64, max_len: usize) -> Result<String, ObserveError> {
    if address == 0 {
        return Err(ObserveError::Protocol("null string pointer".to_owned()));
    }

    let mut bytes = Vec::new();
    let reference_word_len = size_of::<libc::c_long>();

    while bytes.len() < max_len {
        let remaining = max_len - bytes.len();
        let chunk_len = remaining.min(reference_word_len);
        let chunk_address = address + bytes.len() as u64;

        // Candidate B deliberately never requests more remote C-string bytes
        // at one step than the reference PTRACE_PEEKDATA word reader. A short
        // or denied process_vm_readv result is not accepted as evidence and
        // falls back to the exact reference word path.
        let chunk = if let Some(chunk) = process_vm_read_exact(tid, chunk_address, chunk_len) {
            chunk
        } else {
            let word = peek_word(tid, chunk_address)?;
            word.to_ne_bytes()[..chunk_len].to_vec()
        };

        for byte in chunk {
            if byte == 0 {
                return Ok(String::from_utf8_lossy(&bytes).into_owned());
            }
            bytes.push(byte);
            if bytes.len() == max_len {
                break;
            }
        }
    }

    Err(ObserveError::Protocol(format!(
        "string exceeded metadata limit of {max_len} bytes"
    )))
}

fn read_memory(tid: libc::pid_t, address: u64, len: usize) -> Result<Vec<u8>, ObserveError> {
    if address == 0 {
        return Err(ObserveError::Protocol("null memory pointer".to_owned()));
    }

    // Fixed-size metadata is already authorized for exactly `len` bytes.
    // Accept the fast path only on a complete read. Any short/error result
    // is discarded and the current word-by-word PEEKDATA reader is used.
    if let Some(bytes) = process_vm_read_exact(tid, address, len) {
        return Ok(bytes);
    }

    let mut bytes = Vec::with_capacity(len);
    while bytes.len() < len {
        let word = peek_word(tid, address + bytes.len() as u64)?;
        let word_bytes = word.to_ne_bytes();
        let remaining = len - bytes.len();
        bytes.extend_from_slice(&word_bytes[..remaining.min(word_bytes.len())]);
    }
    Ok(bytes)
}

'''

s = s[:start] + replacement + s[end:]
p.write_text(s)
