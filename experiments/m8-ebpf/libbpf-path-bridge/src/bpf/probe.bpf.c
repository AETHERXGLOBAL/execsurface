// SPDX-License-Identifier: Apache-2.0
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>

#define EVENT_EXEC 1
#define EVENT_FILE_OPEN 3

struct metadata_event {
    __u32 kind;
    __u32 pid;
    __u32 value;
    __u32 reserved;
};

struct syscall_exit_ctx {
    __u16 common_type;
    __u8 common_flags;
    __u8 common_preempt_count;
    __s32 common_pid;
    __s32 syscall_nr;
    __u32 alignment;
    __s64 ret;
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 16384);
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_execve")
int execsurface_m83_exec(void *ctx)
{
    (void)ctx;
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    struct metadata_event *event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;
    event->kind = EVENT_EXEC;
    event->pid = (__u32)(pid_tgid >> 32);
    event->value = (__u32)pid_tgid;
    event->reserved = 0;
    bpf_ringbuf_submit(event, 0);
    return 0;
}

SEC("tracepoint/syscalls/sys_exit_openat")
int execsurface_m83_openat_exit(struct syscall_exit_ctx *ctx)
{
    __s64 fd = ctx->ret;
    __u64 pid_tgid;
    struct metadata_event *event;

    if (fd < 0 || fd > 0xffffffffLL)
        return 0;

    pid_tgid = bpf_get_current_pid_tgid();
    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;
    event->kind = EVENT_FILE_OPEN;
    event->pid = (__u32)(pid_tgid >> 32);
    event->value = (__u32)fd;
    event->reserved = 0;
    bpf_ringbuf_submit(event, 0);
    return 0;
}

char LICENSE[] SEC("license") = "Apache-2.0";
