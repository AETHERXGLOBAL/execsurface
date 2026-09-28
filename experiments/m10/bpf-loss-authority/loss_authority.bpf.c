// SPDX-License-Identifier: GPL-2.0
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

struct loss_event {
    __u64 session_id;
    __u64 sequence;
    __u32 tgid;
    __u32 tid;
};

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 8);
    __type(key, __u32);
    __type(value, __u64);
} targets SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 4096);
} events SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} attempts SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} drops SEC(".maps");

SEC("tp/syscalls/sys_enter_getpid")
int m10_loss_getpid(void *ctx)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = pid_tgid >> 32;
    __u32 tid = (__u32)pid_tgid;
    __u64 *session = bpf_map_lookup_elem(&targets, &tgid);
    __u32 zero = 0;
    __u64 *attemptp;
    __u64 *dropp;
    __u64 seq;
    struct loss_event *e;

    (void)ctx;
    if (!session)
        return 0;

    attemptp = bpf_map_lookup_elem(&attempts, &zero);
    if (!attemptp)
        return 0;

    seq = __sync_fetch_and_add(attemptp, 1) + 1;
    e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e) {
        dropp = bpf_map_lookup_elem(&drops, &zero);
        if (dropp)
            __sync_fetch_and_add(dropp, 1);
        return 0;
    }

    e->session_id = *session;
    e->sequence = seq;
    e->tgid = tgid;
    e->tid = tid;
    bpf_ringbuf_submit(e, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
