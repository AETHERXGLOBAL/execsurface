#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::bpf_get_current_pid_tgid,
    macros::{map, tracepoint},
    maps::{PerCpuArray, RingBuf},
    programs::TracePointContext,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExecEvent {
    pub tgid: u32,
    pub tid: u32,
}

// Intentionally tiny for the M8.2 pressure test. A production size is a later decision.
#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(4096, 0);

// Producer-side accounting is separate from the lossy transport. A full ring buffer
// therefore cannot silently erase the fact that evidence became incomplete.
#[map]
static DROPPED: PerCpuArray<u64> = PerCpuArray::with_max_entries(1, 0);

#[tracepoint]
pub fn execsurface_m8_exec(_ctx: TracePointContext) -> u32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let event = ExecEvent {
        tgid: (pid_tgid >> 32) as u32,
        tid: pid_tgid as u32,
    };

    match EVENTS.reserve::<ExecEvent>(0) {
        Some(mut slot) => {
            slot.write(event);
            slot.submit(0);
        }
        None => record_drop(),
    }

    0
}

#[inline(always)]
fn record_drop() {
    if let Some(ptr) = DROPPED.get_ptr_mut(0) {
        // This is a per-CPU cell, so the update does not race with another CPU's cell.
        unsafe {
            *ptr = (*ptr).saturating_add(1);
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 11] = *b"Apache-2.0\0";
