#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static uint64_t monotonic_ns(void) {
    struct timespec ts;
    if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0) {
        return 0;
    }
    return ((uint64_t)ts.tv_sec * 1000000000ULL) + (uint64_t)ts.tv_nsec;
}

static int wait_clean(pid_t child) {
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return 1;
    }
    return !(WIFEXITED(status) && WEXITSTATUS(status) == 0);
}

static int short_exec_workload(void) {
    for (int i = 0; i < 4; ++i) {
        pid_t child = fork();
        if (child < 0) {
            return 1;
        }
        if (child == 0) {
            execl("/bin/true", "true", (char *)NULL);
            _exit(127);
        }
        if (wait_clean(child) != 0) {
            return 1;
        }
    }
    return 0;
}

static int write_markers(const char *path, uint64_t start_ns, uint64_t end_ns) {
    int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd < 0) {
        return 1;
    }
    char buffer[128];
    int length = snprintf(buffer, sizeof(buffer), "%llu %llu\n",
                          (unsigned long long)start_ns,
                          (unsigned long long)end_ns);
    if (length <= 0 || length >= (int)sizeof(buffer)) {
        close(fd);
        return 1;
    }
    ssize_t written = write(fd, buffer, (size_t)length);
    int saved_errno = errno;
    int close_rc = close(fd);
    errno = saved_errno;
    return written == length && close_rc == 0 ? 0 : 1;
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s MARKER_PATH\n", argv[0]);
        return 2;
    }

    uint64_t start_ns = monotonic_ns();
    if (start_ns == 0) {
        return 10;
    }

    if (short_exec_workload() != 0) {
        return 11;
    }

    uint64_t end_ns = monotonic_ns();
    if (end_ns == 0 || end_ns < start_ns) {
        return 12;
    }

    if (write_markers(argv[1], start_ns, end_ns) != 0) {
        return 13;
    }
    return 0;
}
