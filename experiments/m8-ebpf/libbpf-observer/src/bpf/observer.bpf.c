// SPDX-License-Identifier: Apache-2.0
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>

#define EVENT_EXEC 1
#define EVENT_SPAWN 2
#define EVENT_OPEN 3

#define SPAWN_FORK 1
#define SPAWN_VFORK 2
#define SPAWN_CLONE 3

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
    __uint(max_entries, 65536);
} events SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} dropped SEC(".maps");

static __always_inline void record_drop(void)
{
    __u32 key = 0;
    __u64 *count = bpf_map_lookup_elem(&dropped, &key);
    if (count)
        (*count)++;
}

static __always_inline int submit_event(__u32 kind, __u32 pid, __u32 value, __u32 reserved)
{
    struct metadata_event *event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event) {
        record_drop();
        return 0;
    }
    event->kind = kind;
    event->pid = pid;
    event->value = value;
    event->reserved = reserved;
    bpf_ringbuf_submit(event, 0);
    return 0;
}

/* Post-success exec boundary. Path identity is resolved promptly in userspace. */
SEC("tracepoint/sched/sched_process_exec")
int execsurface_m83c_exec(void *ctx)
{
    __u64 pid_tgid;
    (void)ctx;
    pid_tgid = bpf_get_current_pid_tgid();
    return submit_event(EVENT_EXEC, (__u32)(pid_tgid >> 32), (__u32)pid_tgid, 0);
}

static __always_inline int record_spawn_exit(struct syscall_exit_ctx *ctx, __u32 mechanism)
{
    __s64 child_pid = ctx->ret;
    __u32 parent_pid;

    if (child_pid <= 0 || child_pid > 0xffffffffLL)
        return 0;

    parent_pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    return submit_event(EVENT_SPAWN, parent_pid, (__u32)child_pid, mechanism);
}

SEC("tracepoint/syscalls/sys_exit_fork")
int execsurface_m83c_fork_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx, SPAWN_FORK);
}

SEC("tracepoint/syscalls/sys_exit_vfork")
int execsurface_m83c_vfork_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx, SPAWN_VFORK);
}

SEC("tracepoint/syscalls/sys_exit_clone")
int execsurface_m83c_clone_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx, SPAWN_CLONE);
}

SEC("tracepoint/syscalls/sys_exit_clone3")
int execsurface_m83c_clone3_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx, SPAWN_CLONE);
}

SEC("tracepoint/syscalls/sys_exit_openat")
int execsurface_m83c_openat_exit(struct syscall_exit_ctx *ctx)
{
    __s64 fd = ctx->ret;
    __u32 pid;

    if (fd < 0 || fd > 0x7fffffffLL)
        return 0;

    pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    return submit_event(EVENT_OPEN, pid, (__u32)fd, 0);
}

char LICENSE[] SEC("license") = "Apache-2.0";
