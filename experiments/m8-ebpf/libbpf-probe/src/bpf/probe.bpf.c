// SPDX-License-Identifier: Apache-2.0
#include <linux/types.h>
#include <bpf/bpf_helpers.h>

struct exec_event {
    __u32 tgid;
    __u32 tid;
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
        __sync_fetch_and_add(count, 1);
}

SEC("tracepoint/syscalls/sys_enter_execve")
int execsurface_m8_exec(void *ctx)
{
    (void)ctx;
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    struct exec_event *event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event) {
        record_drop();
        return 0;
    }

    event->tgid = pid_tgid >> 32;
    event->tid = (__u32)pid_tgid;
    bpf_ringbuf_submit(event, 0);
    return 0;
}

char LICENSE[] SEC("license") = "Apache-2.0";
