// SPDX-License-Identifier: Apache-2.0
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

static int run_tree(void)
{
    pid_t root = getpid();
    pid_t child = fork();
    if (child < 0) {
        perror("fork");
        return 20;
    }
    if (child == 0) {
        /*
         * M8.7b1 adversary: the descendant is forbidden to exec/exit until
         * it has observed that the original root process is gone and it has
         * been reparented. A root-only lifecycle implementation therefore
         * cannot pass this fixture cleanly.
         */
        for (int i = 0; i < 5000 && getppid() == root; ++i)
            usleep(1000);

        if (getppid() == root)
            _exit(126);

        execl("/bin/true", "true", (char *)NULL);
        _exit(127);
    }

    /* Root exits immediately; the collector must continue draining child. */
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
