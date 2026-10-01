// SPDX-License-Identifier: GPL-2.0
// M10.4 research-only hybrid exec evidence probe. Never enforces.
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

#define KIND_LSM_CANDIDATE 1
#define KIND_EXEC_SUCCESS 2

struct target_role {
    __u64 session_id;
    __u32 role;
    __u32 _pad;
};

struct exec_event {
    __u64 session_id;
    __u64 dev;
    __u64 ino;
    __u32 tgid;
    __u32 tid;
    __u32 role;
    __u32 kind;
};

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 32);
    __type(key, __u32);
    __type(value, struct target_role);
} targets SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);
} events SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} drops SEC(".maps");

static __always_inline void note_drop(void)
{
    __u32 key = 0;
    __u64 *v = bpf_map_lookup_elem(&drops, &key);
    if (v)
        __sync_fetch_and_add(v, 1);
}

static __always_inline void emit_for_file(const struct target_role *target,
                                           __u32 tgid, __u32 tid,
                                           __u32 kind, const struct file *file)
{
    struct exec_event *ev;
    struct inode *inode;
    struct super_block *sb;

    if (!file)
        return;

    ev = bpf_ringbuf_reserve(&events, sizeof(*ev), 0);
    if (!ev) {
        note_drop();
        return;
    }

    inode = BPF_CORE_READ(file, f_inode);
    sb = inode ? BPF_CORE_READ(inode, i_sb) : 0;
    ev->session_id = target->session_id;
    ev->dev = sb ? BPF_CORE_READ(sb, s_dev) : 0;
    ev->ino = inode ? BPF_CORE_READ(inode, i_ino) : 0;
    ev->tgid = tgid;
    ev->tid = tid;
    ev->role = target->role;
    ev->kind = kind;
    bpf_ringbuf_submit(ev, 0);
}

SEC("lsm/bprm_check_security")
int BPF_PROG(m10_exec_candidate, struct linux_binprm *bprm, int ret)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = pid_tgid >> 32;
    __u32 tid = (__u32)pid_tgid;
    struct target_role *target;
    struct file *file;

    if (ret)
        return ret;

    target = bpf_map_lookup_elem(&targets, &tgid);
    if (!target)
        return 0;

    file = BPF_CORE_READ(bprm, file);
    emit_for_file(target, tgid, tid, KIND_LSM_CANDIDATE, file);
    return 0;
}

SEC("tp_btf/sched_process_exec")
int BPF_PROG(m10_exec_success, struct task_struct *p, pid_t old_pid,
             struct linux_binprm *bprm)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = pid_tgid >> 32;
    __u32 tid = (__u32)pid_tgid;
    struct target_role *target;
    struct file *file;

    (void)p;
    (void)old_pid;
    target = bpf_map_lookup_elem(&targets, &tgid);
    if (!target)
        return 0;

    file = BPF_CORE_READ(bprm, file);
    emit_for_file(target, tgid, tid, KIND_EXEC_SUCCESS, file);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
