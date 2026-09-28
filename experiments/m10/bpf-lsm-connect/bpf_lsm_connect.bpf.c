// SPDX-License-Identifier: GPL-2.0
// M10.5 research-only hybrid connect evidence probe. Never enforces.
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

#define AF_INET_VALUE 2
#define KIND_CONNECT_CANDIDATE 1
#define KIND_CONNECT_EXIT 2

struct target_role {
    __u64 session_id;
    __u32 role;
    __u32 _pad;
};

struct connect_event {
    __u64 session_id;
    __s64 rc;
    __u32 addr_be;
    __u32 tgid;
    __u32 tid;
    __u32 role;
    __u16 port_be;
    __u16 family;
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

static __always_inline struct connect_event *reserve_event(const struct target_role *target,
                                                            __u32 tgid, __u32 tid,
                                                            __u32 kind)
{
    struct connect_event *ev = bpf_ringbuf_reserve(&events, sizeof(*ev), 0);
    if (!ev) {
        note_drop();
        return 0;
    }
    __builtin_memset(ev, 0, sizeof(*ev));
    ev->session_id = target->session_id;
    ev->tgid = tgid;
    ev->tid = tid;
    ev->role = target->role;
    ev->kind = kind;
    return ev;
}

SEC("lsm/socket_connect")
int BPF_PROG(m10_socket_connect, struct socket *sock, struct sockaddr *address,
             int addrlen, int ret)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = pid_tgid >> 32;
    __u32 tid = (__u32)pid_tgid;
    struct target_role *target;
    struct connect_event *ev;
    struct sockaddr_in sin = {};

    (void)sock;
    if (ret)
        return ret;
    target = bpf_map_lookup_elem(&targets, &tgid);
    if (!target)
        return 0;
    if (addrlen < (int)sizeof(sin))
        return 0;
    if (bpf_probe_read_kernel(&sin, sizeof(sin), address) < 0)
        return 0;
    if (sin.sin_family != AF_INET_VALUE)
        return 0;

    ev = reserve_event(target, tgid, tid, KIND_CONNECT_CANDIDATE);
    if (!ev)
        return 0;
    ev->family = sin.sin_family;
    ev->addr_be = sin.sin_addr.s_addr;
    ev->port_be = sin.sin_port;
    bpf_ringbuf_submit(ev, 0);
    return 0;
}

SEC("tp/syscalls/sys_exit_connect")
int m10_connect_exit(struct trace_event_raw_sys_exit *ctx)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = pid_tgid >> 32;
    __u32 tid = (__u32)pid_tgid;
    struct target_role *target = bpf_map_lookup_elem(&targets, &tgid);
    struct connect_event *ev;

    if (!target)
        return 0;
    ev = reserve_event(target, tgid, tid, KIND_CONNECT_EXIT);
    if (!ev)
        return 0;
    ev->rc = ctx->ret;
    bpf_ringbuf_submit(ev, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
