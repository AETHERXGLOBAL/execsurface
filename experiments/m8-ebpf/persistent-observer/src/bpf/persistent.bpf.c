// SPDX-License-Identifier: Apache-2.0
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>

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

#ifndef M8_7_RINGBUF_MAX_ENTRIES
#define M8_7_RINGBUF_MAX_ENTRIES 65536
#endif

#ifndef M8_7_TASK_EPOCH_MAX_ENTRIES
#define M8_7_TASK_EPOCH_MAX_ENTRIES 4096
#endif

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

/*
 * sched_process_fork uses two __string fields. In the raw tracepoint record
 * those are represented as 32-bit __data_loc values. CI audits the live
 * tracepoint format before this layout counts as M8.7 evidence.
 */
struct sched_process_fork_ctx {
    __u16 common_type;
    __u8 common_flags;
    __u8 common_preempt_count;
    __s32 common_pid;
    __u32 parent_comm_loc;
    __s32 parent_pid;
    __u32 child_comm_loc;
    __s32 child_pid;
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    /* M8.7b5 may shrink this only in an isolated fault-evidence build. */
    __uint(max_entries, M8_7_RINGBUF_MAX_ENTRIES);
} events SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    /*
     * Production-feasibility builds use 4096. M8.7b5 may compile this
     * experiment with a deliberately tiny map to force a real kernel-side
     * propagation failure and prove routing_errors closes the session.
     */
    __uint(max_entries, M8_7_TASK_EPOCH_MAX_ENTRIES);
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
    __u64 emitted_epoch = epoch;

    if (!epoch)
        return 0;

#ifdef M8_7_FAULT_STALE_EPOCH
    /*
     * M8.7d fault-evidence build only: corrupt epoch-2 exec records so a
     * real ring-buffer event from the active session arrives carrying the
     * previous epoch. Default builds never define this macro.
     */
    if (epoch == 2 && kind == EVENT_EXEC)
        emitted_epoch = 1;
#endif

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event) {
        bump_counter(&dropped);
        return 0;
    }

    event->epoch = emitted_epoch;
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

/*
 * Task creation remains the propagation boundary. sched_process_fork is
 * emitted from the parent's fork path before the new task is made runnable,
 * so membership is installed before the child can contribute accepted
 * session events. The ordinary tracepoint payload avoids direct task_struct
 * access and therefore preserves the Apache-2.0 BPF license boundary.
 */
SEC("tracepoint/sched/sched_process_fork")
int execsurface_m87_task_created(struct sched_process_fork_ctx *ctx)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 current_tid = (__u32)pid_tgid;
    __u32 parent_tgid = (__u32)(pid_tgid >> 32);
    __u32 parent_tid;
    __u32 child_tid;
    __u64 epoch;
    __u32 mechanism = SPAWN_UNKNOWN;
    __u32 *stored;

    if (ctx->parent_pid <= 0 || ctx->child_pid <= 0)
        return 0;

    parent_tid = (__u32)ctx->parent_pid;
    child_tid = (__u32)ctx->child_pid;

    if (parent_tid != current_tid) {
        bump_counter(&routing_errors);
        return 0;
    }

    epoch = lookup_epoch(parent_tid);
    if (!epoch)
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

/* Cleanup pending classification when process creation fails or after the fork
 * tracepoint already consumed it. Deleting a missing key is harmless. */
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
