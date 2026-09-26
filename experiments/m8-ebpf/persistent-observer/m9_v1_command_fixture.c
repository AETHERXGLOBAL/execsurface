#define _GNU_SOURCE
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static int parse_fd(const char *text) {
    char *end = NULL;
    errno = 0;
    long value = strtol(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || value < 0 || value > 0x7fffffffL) {
        return -1;
    }
    return (int)value;
}

int main(int argc, char **argv) {
    if (argc != 3 || strcmp(argv[1], "barrier") != 0) {
        fprintf(stderr, "usage: %s barrier FD\n", argv[0]);
        return 64;
    }

    int fd = parse_fd(argv[2]);
    if (fd < 0) {
        fprintf(stderr, "invalid barrier fd\n");
        return 64;
    }

    unsigned char byte = 0;
    ssize_t n;
    do {
        n = read(fd, &byte, 1);
    } while (n < 0 && errno == EINTR);
    close(fd);
    if (n != 1) {
        fprintf(stderr, "barrier release failed\n");
        return 70;
    }

    const char *workdir = getenv("M9_V1_WORKDIR");
    const char *script = getenv("M9_V1_SCRIPT");
    if (workdir == NULL || *workdir == '\0' || script == NULL || *script == '\0') {
        fprintf(stderr, "M9_V1_WORKDIR and M9_V1_SCRIPT are required\n");
        return 64;
    }
    if (chdir(workdir) != 0) {
        fprintf(stderr, "chdir(%s): %s\n", workdir, strerror(errno));
        return 72;
    }

    execl("/bin/bash", "bash", "-lc", script, (char *)NULL);
    fprintf(stderr, "exec /bin/bash: %s\n", strerror(errno));
    return 127;
}
