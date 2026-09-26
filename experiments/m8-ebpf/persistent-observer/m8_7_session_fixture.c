// SPDX-License-Identifier: Apache-2.0
#include <errno.h>
#include <limits.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

#ifndef M8_7_CHURN_CHILDREN
#define M8_7_CHURN_CHILDREN 64
#endif

static int write_pid_marker(const char *path)
{
    FILE *marker = fopen(path, "w");
    if (marker == NULL) {
        perror("fopen pid marker");
        return 31;
    }

    if (fprintf(marker, "%ld %ld\n", (long)getpid(), (long)getpgrp()) < 0) {
        fclose(marker);
        return 32;
    }
    if (fflush(marker) != 0) {
        fclose(marker);
        return 33;
    }
    if (fsync(fileno(marker)) != 0) {
        fclose(marker);
        return 34;
    }
    if (fclose(marker) != 0)
        return 35;
    return 0;
}

static int run_crash_hold(void)
{
    const char *marker_path = getenv("M8_7_CRASH_MARKER");
    if (marker_path == NULL || marker_path[0] == '\0') {
        fprintf(stderr, "M8_7_CRASH_MARKER is required in crash-hold mode\n");
        return 30;
    }

    int marker_rc = write_pid_marker(marker_path);
    if (marker_rc != 0)
        return marker_rc;

    for (;;)
        pause();
}

static int run_tree(void)
{
    const char *mode = getenv("M8_7_MODE");
    if (mode != NULL && strcmp(mode, "crash-hold") == 0)
        return run_crash_hold();

    pid_t root = getpid();

    for (int child_index = 0; child_index < M8_7_CHURN_CHILDREN; ++child_index) {
        pid_t child = fork();
        if (child < 0) {
            perror("fork");
            return 20;
        }
        if (child == 0) {
            /*
             * M8.7b adversary: every descendant must outlive the original
             * root. The child cannot exec/exit until reparenting proves the
             * root has already gone away.
             */
            for (int i = 0; i < 5000 && getppid() == root; ++i)
                usleep(1000);

            if (getppid() == root)
                _exit(126);

            execl("/bin/true", "true", (char *)NULL);
            _exit(127);
        }
    }

    /*
     * Root exits without waiting. The collector must retain and drain every
     * epoch-propagated descendant before the session can become clean.
     */
    return 0;
}

static int run_barrier(const char *self, const char *fd_text)
{
    char *end = NULL;
    errno = 0;
    long parsed = strtol(fd_text, &end, 10);
    if (errno != 0 || end == fd_text || *end != '\0' || parsed < 0 || parsed > INT_MAX) {
        fprintf(stderr, "invalid barrier fd\n");
        return 23;
    }

    int fd = (int)parsed;
    unsigned char byte = 0;
    ssize_t rc;
    do {
        rc = read(fd, &byte, 1);
    } while (rc < 0 && errno == EINTR);
    close(fd);

    if (rc != 1) {
        fprintf(stderr, "root-registration barrier release failed\n");
        return 24;
    }

    execl(self, self, "tree", (char *)NULL);
    perror("exec self after root-registration barrier");
    return 25;
}

int main(int argc, char **argv)
{
    if (argc == 2 && strcmp(argv[1], "tree") == 0)
        return run_tree();

    if (argc == 3 && strcmp(argv[1], "barrier") == 0)
        return run_barrier(argv[0], argv[2]);

    fprintf(stderr, "usage: %s tree | barrier FD\n", argv[0]);
    return 2;
}
