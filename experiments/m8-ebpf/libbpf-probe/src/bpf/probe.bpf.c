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

struct sched_process_fork_ctx {
    __u16 common_type;
    __u8 common_flags;
    __u8 common_preempt_count;
    __s32 common_pid;
    char parent_comm[16];
    __s32 parent_pid;
    char child_comm[16];
    __s32 child_pid;
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

SEC("tracepoint/sched/sched_process_fork")
int execsurface_m8_fork(struct sched_process_fork_ctx *ctx)
{
    if (ctx->parent_pid <= 0 || ctx->child_pid <= 0)
        return 0;
    return submit_event(EVENT_FORK, (__u32)ctx->parent_pid, (__u32)ctx->child_pid);
}

char LICENSE[] SEC("license") = "Apache-2.0";
