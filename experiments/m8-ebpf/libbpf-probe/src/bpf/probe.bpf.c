// SPDX-License-Identifier: Apache-2.0
#include <bpf/bpf_helpers.h>

SEC("tracepoint/syscalls/sys_enter_execve")
int execsurface_m8_exec(void *ctx)
{
    (void)ctx;
    return 0;
}

char LICENSE[] SEC("license") = "Apache-2.0";
