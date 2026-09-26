#![no_std]
#![no_main]

#[allow(
    clippy::all,
    dead_code,
    improper_ctypes_definitions,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unnecessary_transmutes,
    unsafe_op_in_unsafe_fn,
)]
#[rustfmt::skip]
mod vmlinux;

use aya_ebpf::{
    helpers::bpf_get_current_pid_tgid,
    macros::{btf_tracepoint, map, tracepoint},
    maps::{PerCpuArray, RingBuf},
    programs::{BtfTracePointContext, TracePointContext},
};
use vmlinux::task_struct;

const EVENT_EXEC: u32 = 1;
const EVENT_FORK: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessEvent {
    pub kind: u32,
    pub pid: u32,
    pub related_pid: u32,
    pub reserved: u32,
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
    emit(ProcessEvent {
        kind: EVENT_EXEC,
        pid: (pid_tgid >> 32) as u32,
        related_pid: pid_tgid as u32,
        reserved: 0,
    });
    0
}

#[btf_tracepoint(function = "sched_process_fork")]
pub fn execsurface_m8_fork(ctx: BtfTracePointContext) -> u32 {
    let parent: *const task_struct = ctx.arg(0);
    let child: *const task_struct = ctx.arg(1);
    if parent.is_null() || child.is_null() {
        return 0;
    }

    // task_struct is generated from kernel BTF with aya-tool. No field offsets are
    // hard-coded in ExecSurface source.
    let parent_pid = unsafe { (*parent).pid };
    let child_pid = unsafe { (*child).pid };
    if parent_pid > 0 && child_pid > 0 {
        emit(ProcessEvent {
            kind: EVENT_FORK,
            pid: parent_pid as u32,
            related_pid: child_pid as u32,
            reserved: 0,
        });
    }
    0
}

#[inline(always)]
fn emit(event: ProcessEvent) {
    match EVENTS.reserve::<ProcessEvent>(0) {
        Some(mut slot) => {
            slot.write(event);
            slot.submit(0);
        }
        None => record_drop(),
    }
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
