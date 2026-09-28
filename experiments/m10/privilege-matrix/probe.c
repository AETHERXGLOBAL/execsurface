#define _GNU_SOURCE
#include <errno.h>
#include <linux/bpf.h>
#include <sched.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

static int try_map_create(int *err_out)
{
    union bpf_attr attr;
    memset(&attr, 0, sizeof(attr));
    attr.map_type = BPF_MAP_TYPE_ARRAY;
    attr.key_size = sizeof(uint32_t);
    attr.value_size = sizeof(uint32_t);
    attr.max_entries = 1;
    errno = 0;
    int fd = (int)syscall(__NR_bpf, BPF_MAP_CREATE, &attr, sizeof(attr));
    int e = errno;
    if (fd >= 0) close(fd);
    *err_out = e;
    return fd >= 0 ? 0 : -1;
}

static void print_result(const char *mode, int ns_rc, int ns_errno)
{
    int bpf_errno = 0;
    int bpf_rc = try_map_create(&bpf_errno);
    printf("mode=%s uid=%u euid=%u gid=%u egid=%u userns_rc=%d userns_errno=%d bpf_rc=%d bpf_errno=%d\n",
           mode, (unsigned)getuid(), (unsigned)geteuid(), (unsigned)getgid(), (unsigned)getegid(),
           ns_rc, ns_errno, bpf_rc, bpf_errno);
}

int main(int argc, char **argv)
{
    const char *mode = argc > 1 ? argv[1] : "root";

    if (strcmp(mode, "root") == 0) {
        print_result("root", 0, 0);
        return 0;
    }

    if (strcmp(mode, "drop") == 0) {
        if (setgid(65534) != 0) {
            printf("mode=drop setup=setgid_failed errno=%d\n", errno);
            return 2;
        }
        if (setuid(65534) != 0) {
            printf("mode=drop setup=setuid_failed errno=%d\n", errno);
            return 2;
        }
        print_result("drop", 0, 0);
        return 0;
    }

    if (strcmp(mode, "userns") == 0) {
        errno = 0;
        int rc = unshare(CLONE_NEWUSER);
        int e = errno;
        print_result("userns", rc, e);
        return 0;
    }

    fprintf(stderr, "usage: %s [root|drop|userns]\n", argv[0]);
    return 2;
}
