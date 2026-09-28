// SPDX-License-Identifier: GPL-2.0
// Research-only M10.3 BPF LSM prototype. Always allows; never enforces.
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

struct file_event {
    __u64 session_id;
    __u64 ktime_ns;
    __u64 ino;
    __u64 dev;
    __u32 tgid;
    __u32 tid;
    __u32 f_flags;
    __u32 mask;
};

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, __u32);
    __type(value, __u64);
} target_tgids SEC(".maps");

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

SEC("lsm/file_open")
int BPF_PROG(m10_file_open, struct file *file, int mask, int ret)
{
    __u64 pid_tgid;
    __u32 tgid, tid;
    __u64 *session;
    struct file_event *ev;
    struct inode *inode;
    struct super_block *sb;

    /* Preserve prior LSM denial and never weaken it. */
    if (ret)
        return ret;

    pid_tgid = bpf_get_current_pid_tgid();
    tgid = pid_tgid >> 32;
    tid = (__u32)pid_tgid;
    session = bpf_map_lookup_elem(&target_tgids, &tgid);
    if (!session)
        return 0;

    ev = bpf_ringbuf_reserve(&events, sizeof(*ev), 0);
    if (!ev) {
        note_drop();
        return 0;
    }

    inode = BPF_CORE_READ(file, f_inode);
    sb = inode ? BPF_CORE_READ(inode, i_sb) : 0;

    ev->session_id = *session;
    ev->ktime_ns = bpf_ktime_get_ns();
    ev->ino = inode ? BPF_CORE_READ(inode, i_ino) : 0;
    ev->dev = sb ? BPF_CORE_READ(sb, s_dev) : 0;
    ev->tgid = tgid;
    ev->tid = tid;
    ev->f_flags = BPF_CORE_READ(file, f_flags);
    ev->mask = (__u32)mask;
    bpf_ringbuf_submit(ev, 0);

    return 0;
}

char LICENSE[] SEC("license") = "GPL";
