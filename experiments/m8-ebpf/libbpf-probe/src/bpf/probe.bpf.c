// SPDX-License-Identifier: Apache-2.0
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>

#define EVENT_EXEC 1
#define EVENT_FORK 2

struct process_event {
    __u32 kind;
    __u32 pid;
    __u32 related_pid;
    __u32 reserved;
};

/*
 * Syscall-exit trace records use the kernel syscall_trace_exit layout:
 * trace_entry (8 bytes), syscall number (4 bytes + alignment), then return value.
 * The feasibility workflow also records the host tracepoint format so this
 * assumption is explicit evidence rather than an invisible ABI dependency.
 */
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
    __uint(max_entries, 4096);
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

static __always_inline int submit_event(__u32 kind, __u32 pid, __u32 related_pid)
{
    struct process_event *event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event) {
        record_drop();
        return 0;
    }
    event->kind = kind;
    event->pid = pid;
    event->related_pid = related_pid;
    event->reserved = 0;
    bpf_ringbuf_submit(event, 0);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_execve")
int execsurface_m8_exec(void *ctx)
{
    (void)ctx;
    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    return submit_event(EVENT_EXEC, pid, pid);
}

static __always_inline int record_spawn_exit(struct syscall_exit_ctx *ctx)
{
    __s64 child_pid = ctx->ret;
    __u32 parent_pid;

    /* Parent-side successful fork/clone returns the positive child pid. */
    if (child_pid <= 0 || child_pid > 0xffffffffLL)
        return 0;

    parent_pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    return submit_event(EVENT_FORK, parent_pid, (__u32)child_pid);
}

/*
 * Apache-compatible lineage: observe successful process-creation syscall
 * returns instead of dereferencing task_struct. This avoids GPL-restricted
 * kernel-struct access and avoids the hosted-runner sched tracepoint policy
 * boundary while retaining explicit parent -> child process identity.
 */
SEC("tracepoint/syscalls/sys_exit_clone")
int execsurface_m8_clone_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx);
}

SEC("tracepoint/syscalls/sys_exit_clone3")
int execsurface_m8_clone3_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx);
}

SEC("tracepoint/syscalls/sys_exit_fork")
int execsurface_m8_fork_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx);
}

SEC("tracepoint/syscalls/sys_exit_vfork")
int execsurface_m8_vfork_exit(struct syscall_exit_ctx *ctx)
{
    return record_spawn_exit(ctx);
}

char LICENSE[] SEC("license") = "Apache-2.0";
