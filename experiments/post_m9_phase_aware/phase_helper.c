#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static void alarm_handler(int sig) { (void)sig; }

static int do_fileio(const char *path) {
    int fd = open(path, O_CREAT | O_TRUNC | O_RDWR, 0600);
    if (fd < 0) return 11;
    if (write(fd, "abc", 3) != 3) return 12;
    if (lseek(fd, 0, SEEK_SET) < 0) return 13;
    char b[4] = {0};
    if (read(fd, b, 3) != 3) return 14;
    if (close(fd) != 0) return 15;
    return 0;
}

static void *thread_exec(void *arg) {
    (void)arg;
    execl("/bin/true", "true", (char *)0);
    _exit(91);
}

static void *thread_exit_group(void *arg) {
    (void)arg;
    syscall(SYS_exit_group, 0);
    return 0;
}

int main(int argc, char **argv) {
    if (argc < 2) return 64;
    const char *mode = argv[1];
    const char *path = argc > 2 ? argv[2] : "/tmp/execsurface-phase-file";

    if (!strcmp(mode, "getpid")) {
        unsigned long n = argc > 2 ? strtoul(argv[2], 0, 10) : 0;
        volatile long x = 0;
        for (unsigned long i = 0; i < n; i++) x ^= syscall(SYS_getpid);
        return x == -1 ? 70 : 0;
    }
    if (!strcmp(mode, "fileio")) return do_fileio(path);
    if (!strcmp(mode, "failed-open")) {
        int fd = open("/definitely/not/execsurface-phase-aware", O_RDONLY);
        if (fd >= 0) { close(fd); return 21; }
        return errno == ENOENT ? 0 : 22;
    }
    if (!strcmp(mode, "signal")) {
        struct sigaction sa = {0};
        sa.sa_handler = alarm_handler;
        sigemptyset(&sa.sa_mask);
        if (sigaction(SIGALRM, &sa, 0) != 0) return 31;
        ualarm(1000, 0);
        struct timespec ts = {.tv_sec = 0, .tv_nsec = 100000000}, rem = {0};
        syscall(SYS_nanosleep, &ts, &rem);
        int fd = open("/dev/null", O_RDONLY);
        if (fd < 0) return 32;
        char b = 0;
        (void)read(fd, &b, 1);
        close(fd);
        return 0;
    }
    if (!strcmp(mode, "fork")) {
        int fd = open(path, O_RDONLY);
        if (fd < 0) return 41;
        pid_t p = fork();
        if (p < 0) return 42;
        if (p == 0) { char b = 0; (void)read(fd, &b, 1); _exit(0); }
        int st = 0;
        if (waitpid(p, &st, 0) != p) return 43;
        close(fd);
        return WIFEXITED(st) && WEXITSTATUS(st) == 0 ? 0 : 44;
    }
    if (!strcmp(mode, "vfork")) {
        pid_t p = vfork();
        if (p < 0) return 51;
        if (p == 0) _exit(0);
        int st = 0;
        if (waitpid(p, &st, 0) != p) return 52;
        return WIFEXITED(st) && WEXITSTATUS(st) == 0 ? 0 : 53;
    }
    if (!strcmp(mode, "exec")) {
        execl("/bin/true", "true", (char *)0);
        return 61;
    }
    if (!strcmp(mode, "failed-exec")) {
        execl("/definitely/not/execsurface-phase-aware", "missing", (char *)0);
        if (errno != ENOENT) return 62;
        int fd = open("/dev/null", O_RDONLY);
        if (fd < 0) return 63;
        close(fd);
        return 0;
    }
    if (!strcmp(mode, "thread-exec")) {
        pthread_t t;
        if (pthread_create(&t, 0, thread_exec, 0) != 0) return 71;
        void *r = 0;
        if (pthread_join(t, &r) != 0) return 72;
        return 73;
    }
    if (!strcmp(mode, "exit-group")) {
        pthread_t t;
        if (pthread_create(&t, 0, thread_exit_group, 0) != 0) return 81;
        for (;;) pause();
    }
    return 65;
}
