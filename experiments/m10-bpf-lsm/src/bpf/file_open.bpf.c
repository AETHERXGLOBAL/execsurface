// SPDX-License-Identifier: GPL-2.0-only
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

struct file_open_event {
    __u64 session_id;
    __u32 tgid;
    __u32 tid;
    __u64 inode;
    __u32 dev;
    __u32 flags;
};

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 128);
    __type(key, __u32);
    __type(value, __u64);
} sessions SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 65536);
} events SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} dropped SEC(".maps");

static __always_inline void record_drop(void)
{
    __u32 key = 0;
    __u64 *count = bpf_map_lookup_elem(&dropped, &key);
    if (count)
        __sync_fetch_and_add(count, 1);
}

SEC("lsm/file_open")
int BPF_PROG(execsurface_m10_file_open, struct file *file, int ret)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = (__u32)(pid_tgid >> 32);
    __u32 tid = (__u32)pid_tgid;
    __u64 *session_id;
    struct file_open_event *event;
    struct inode *inode;
    struct super_block *sb;

    /* Audit-only: preserve an earlier LSM denial and never introduce one. */
    if (ret != 0)
        return ret;

    session_id = bpf_map_lookup_elem(&sessions, &tgid);
    if (!session_id)
        return ret;

    inode = BPF_CORE_READ(file, f_inode);
    if (!inode)
        return ret;
    sb = BPF_CORE_READ(inode, i_sb);
    if (!sb)
        return ret;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event) {
        record_drop();
        return ret;
    }

    event->session_id = *session_id;
    event->tgid = tgid;
    event->tid = tid;
    event->inode = BPF_CORE_READ(inode, i_ino);
    event->dev = BPF_CORE_READ(sb, s_dev);
    event->flags = BPF_CORE_READ(file, f_flags);
    bpf_ringbuf_submit(event, 0);

    return ret;
}

char LICENSE[] SEC("license") = "GPL";
