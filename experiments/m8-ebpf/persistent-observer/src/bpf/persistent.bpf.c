// SPDX-License-Identifier: Apache-2.0
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

#define EVENT_EXEC 1
#define EVENT_SPAWN 2
#define EVENT_EXIT 3

#define SPAWN_UNKNOWN 0
#define SPAWN_FORK 1
#define SPAWN_VFORK 2
#define SPAWN_CLONE 3

#define AX_CSIGNAL 0x000000ffULL
#define AX_SIGCHLD 17ULL
#define AX_CLONE_VFORK 0x00004000ULL

/* Minimal CO-RE shape. tp_btf arguments are trusted kernel pointers, so the
 * prototype uses direct field access rather than GPL-only probe-read helpers. */
struct task_struct {
    int pid;
    int tgid;
} __attribute__((preserve_access_index));

struct metadata_event {
    __u64 epoch;
    __u32 kind;
    __u32 tid;
    __u32 tgid;
    __u32 value;
    __u32 reserved;
    __u32 pad;
};

struct syscall_enter_ctx {
    __u16 common_type;
    __u8 common_flags;
    __u8 common_preempt_count;
    __s32 common_pid;
    __s32 syscall_nr;
    __u32 alignment;
    __u64 args[6];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 65536);
} events SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 4096);
    __type(key, __u32);
    __type(value, __u64);
} task_epoch SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 4096);
    __type(key, __u32);
    __type(value, __u32);
} pending_spawn_mechanism SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} dropped SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} routing_errors SEC(".maps");

static __always_inline void bump_counter(void *map)
{
    __u32 key = 0;
    __u64 *count = bpf_map_lookup_elem(map, &key);
    if (count)
        (*count)++;
}

static __always_inline __u64 lookup_epoch(__u32 tid)
{
    __u64 *epoch = bpf_map_lookup_elem(&task_epoch, &tid);
    return epoch ? *epoch : 0;
}

static __always_inline int submit_event(
    __u64 epoch,
    __u32 kind,
    __u32 tid,
    __u32 tgid,
    __u32 value,
    __u32 reserved)
{
    struct metadata_event *event;

    if (!epoch)
        return 0;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event) {
        bump_counter(&dropped);
        return 0;
    }

    event->epoch = epoch;
    event->kind = kind;
    event->tid = tid;
    event->tgid = tgid;
    event->value = value;
    event->reserved = reserved;
    event->pad = 0;
    bpf_ringbuf_submit(event, 0);
    return 0;
}

static __always_inline __u32 classify_clone_mechanism(__u64 flags, __u64 exit_signal)
{
    if (flags & AX_CLONE_VFORK)
        return SPAWN_VFORK;
    if (exit_signal == AX_SIGCHLD)
        return SPAWN_FORK;
    return SPAWN_CLONE;
}

static __always_inline int remember_spawn_mechanism(__u32 mechanism)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tid = (__u32)pid_tgid;

    if (!lookup_epoch(tid))
        return 0;

    if (bpf_map_update_elem(&pending_spawn_mechanism, &tid, &mechanism, BPF_ANY))
        bump_counter(&routing_errors);
    return 0;
}

static __always_inline int clear_spawn_mechanism(void)
{
    __u32 tid = (__u32)bpf_get_current_pid_tgid();

    if (!lookup_epoch(tid))
        return 0;
    bpf_map_delete_elem(&pending_spawn_mechanism, &tid);
    return 0;
}

SEC("tracepoint/sched/sched_process_exec")
int execsurface_m87_exec(void *ctx)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tid = (__u32)pid_tgid;
    __u32 tgid = (__u32)(pid_tgid >> 32);
    __u64 epoch = lookup_epoch(tid);
    (void)ctx;
    return submit_event(epoch, EVENT_EXEC, tid, tgid, 0, 0);
}

SEC("tracepoint/sched/sched_process_exit")
int execsurface_m87_exit(void *ctx)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tid = (__u32)pid_tgid;
    __u32 tgid = (__u32)(pid_tgid >> 32);
    __u64 epoch = lookup_epoch(tid);
    int rc;
    (void)ctx;

    if (!epoch)
        return 0;

    submit_event(epoch, EVENT_EXIT, tid, tgid, 0, 0);
    rc = bpf_map_delete_elem(&task_epoch, &tid);
    if (rc)
        bump_counter(&routing_errors);
    bpf_map_delete_elem(&pending_spawn_mechanism, &tid);
    return 0;
}

/* Task creation is the propagation boundary. Using syscall-exit to install the
 * child epoch would be racy because the new task may run before the parent's
 * syscall-return tracepoint. The tp_btf hook executes at sched_process_fork and
 * installs membership before the child can contribute accepted session events. */
SEC("tp_btf/sched_process_fork")
int BPF_PROG(
    execsurface_m87_task_created,
    struct task_struct *parent,
    struct task_struct *child)
{
    __u32 parent_tid = (__u32)parent->pid;
    __u32 parent_tgid = (__u32)parent->tgid;
    __u32 child_tid = (__u32)child->pid;
    __u64 epoch = lookup_epoch(parent_tid);
    __u32 mechanism = SPAWN_UNKNOWN;
    __u32 *stored;

    if (!epoch || !child_tid)
        return 0;

    stored = bpf_map_lookup_elem(&pending_spawn_mechanism, &parent_tid);
    if (stored)
        mechanism = *stored;

    if (bpf_map_update_elem(&task_epoch, &child_tid, &epoch, BPF_ANY)) {
        bump_counter(&routing_errors);
        return 0;
    }

    bpf_map_delete_elem(&pending_spawn_mechanism, &parent_tid);
    return submit_event(
        epoch,
        EVENT_SPAWN,
        parent_tid,
        parent_tgid,
        child_tid,
        mechanism);
}

SEC("tracepoint/syscalls/sys_enter_fork")
int execsurface_m87_fork_enter(void *ctx)
{
    (void)ctx;
    return remember_spawn_mechanism(SPAWN_FORK);
}

SEC("tracepoint/syscalls/sys_enter_vfork")
int execsurface_m87_vfork_enter(void *ctx)
{
    (void)ctx;
    return remember_spawn_mechanism(SPAWN_VFORK);
}

SEC("tracepoint/syscalls/sys_enter_clone")
int execsurface_m87_clone_enter(struct syscall_enter_ctx *ctx)
{
    __u64 flags = ctx->args[0];
    return remember_spawn_mechanism(
        classify_clone_mechanism(flags, flags & AX_CSIGNAL));
}

/* clone3's clone_args live in user memory. The Apache-2.0 prototype does not
 * use the GPL-restricted helper rejected in M8.2/M8.5, so mechanism remains
 * explicitly unknown instead of being invented. */
SEC("tracepoint/syscalls/sys_enter_clone3")
int execsurface_m87_clone3_enter(void *ctx)
{
    (void)ctx;
    return remember_spawn_mechanism(SPAWN_UNKNOWN);
}

/* Cleanup pending classification when process creation fails or after the BTF
 * fork hook already consumed it. Deleting a missing key is harmless. */
SEC("tracepoint/syscalls/sys_exit_fork")
int execsurface_m87_fork_exit(void *ctx)
{
    (void)ctx;
    return clear_spawn_mechanism();
}

SEC("tracepoint/syscalls/sys_exit_vfork")
int execsurface_m87_vfork_exit(void *ctx)
{
    (void)ctx;
    return clear_spawn_mechanism();
}

SEC("tracepoint/syscalls/sys_exit_clone")
int execsurface_m87_clone_exit(void *ctx)
{
    (void)ctx;
    return clear_spawn_mechanism();
}

SEC("tracepoint/syscalls/sys_exit_clone3")
int execsurface_m87_clone3_exit(void *ctx)
{
    (void)ctx;
    return clear_spawn_mechanism();
}

char LICENSE[] SEC("license") = "Apache-2.0";
