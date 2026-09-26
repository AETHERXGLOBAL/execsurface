// SPDX-License-Identifier: Apache-2.0
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

#define EVENT_EXEC 1
#define EVENT_FORK 2

struct process_event {
    __u32 kind;
    __u32 pid;
    __u32 related_pid;
    __u32 reserved;
};

/*
 * Minimal CO-RE view: only the field required by the feasibility probe is
 * described. preserve_access_index makes the direct field access relocatable
 * against the host kernel BTF instead of freezing task_struct layout.
 */
struct task_struct {
    int pid;
} __attribute__((preserve_access_index));

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

/*
 * BTF tracing provides trusted task_struct pointers. Use direct CO-RE-relocated
 * field loads rather than bpf_probe_read_kernel: the latter is GPL-restricted
 * on this kernel and ExecSurface must not change its Apache-2.0 license merely
 * to make a feasibility probe pass.
 */
SEC("tp_btf/sched_process_fork")
int BPF_PROG(execsurface_m8_fork, struct task_struct *parent, struct task_struct *child)
{
    __s32 parent_pid;
    __s32 child_pid;

    if (!parent || !child)
        return 0;

    parent_pid = __builtin_preserve_access_index(parent->pid);
    child_pid = __builtin_preserve_access_index(child->pid);
    if (parent_pid <= 0 || child_pid <= 0)
        return 0;

    return submit_event(EVENT_FORK, (__u32)parent_pid, (__u32)child_pid);
}

char LICENSE[] SEC("license") = "Apache-2.0";
