// SPDX-License-Identifier: Apache-2.0
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>

int main(int argc, char **argv)
{
    struct stat st;
    int fd;

    if (argc != 2) {
        fprintf(stderr, "usage: %s <path>\n", argv[0]);
        return 64;
    }

    /* Loader registers this TGID in the BPF session map before the tested open. */
    if (raise(SIGSTOP) != 0) {
        perror("raise");
        return 65;
    }

    fd = open(argv[1], O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
        perror("open");
        return 66;
    }

    if (fstat(fd, &st) != 0) {
        perror("fstat");
        close(fd);
        return 67;
    }

    printf("{\"dev\":%llu,\"ino\":%llu}\n",
           (unsigned long long)st.st_dev,
           (unsigned long long)st.st_ino);
    fflush(stdout);
    close(fd);
    return 0;
}
