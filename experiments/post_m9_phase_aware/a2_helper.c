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

static void noop_handler(int sig) { (void)sig; }

static int do_fileio(const char *path) {
    int fd = open(path, O_CREAT | O_TRUNC | O_RDWR, 0600);
    if (fd < 0) return 11;
    if (write(fd, "abc", 3) != 3) return 12;
    if (lseek(fd, 0, SEEK_SET) < 0) return 13;
    char b[4] = {0};
    if (read(fd, b, 3) != 3) return 14;
    int dupfd = dup(fd);
    if (dupfd < 0) return 15;
    if (close(dupfd) != 0) return 16;
    if (close(fd) != 0) return 17;
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
    const char *path = argc > 2 ? argv[2] : "/tmp/execsurface-a2-fixed";

    if (!strcmp(mode, "getpid")) {
        unsigned long n = argc > 2 ? strtoul(argv[2], 0, 10) : 0;
        volatile long x = 0;
        for (unsigned long i = 0; i < n; i++) x ^= syscall(SYS_getpid);
        return x == -1 ? 70 : 0;
    }

    if (!strcmp(mode, "fileio")) return do_fileio(path);

    if (!strcmp(mode, "failed-open")) {
        int fd = open("/definitely/not/execsurface-a2", O_RDONLY);
        if (fd >= 0) {
            close(fd);
            return 21;
        }
        return errno == ENOENT ? 0 : 22;
    }

    if (!strcmp(mode, "signal")) {
        struct sigaction sa = {0};
        sa.sa_handler = noop_handler;
        sigemptyset(&sa.sa_mask);
        if (sigaction(SIGALRM, &sa, 0) != 0) return 31;
        ualarm(1000, 0);
        struct timespec ts = {.tv_sec = 0, .tv_nsec = 100000000};
        struct timespec rem = {0};
        (void)syscall(SYS_nanosleep, &ts, &rem);
        int fd = open("/dev/null", O_RDONLY);
        if (fd < 0) return 32;
        char b = 0;
        ssize_t got = read(fd, &b, 1);
        if (got < 0) {
            close(fd);
            return 33;
        }
        if (close(fd) != 0) return 34;
        return 0;
    }

    if (!strcmp(mode, "restart")) {
        int p[2];
        if (pipe(p) != 0) return 35;
        struct sigaction sa = {0};
        sa.sa_handler = noop_handler;
        sa.sa_flags = SA_RESTART;
        sigemptyset(&sa.sa_mask);
        if (sigaction(SIGUSR1, &sa, 0) != 0) return 36;
        pid_t parent = getpid();
        pid_t c = fork();
        if (c < 0) return 37;
        if (c == 0) {
            close(p[0]);
            usleep(10000);
            kill(parent, SIGUSR1);
            usleep(10000);
            char x = 'x';
            ssize_t wrote = write(p[1], &x, 1);
            if (wrote != 1) _exit(39);
            _exit(0);
        }
        close(p[1]);
        char x = 0;
        ssize_t got = read(p[0], &x, 1);
        close(p[0]);
        int st = 0;
        if (waitpid(c, &st, 0) != c) return 40;
        if (got != 1 || x != 'x') return 41;
        return WIFEXITED(st) && WEXITSTATUS(st) == 0 ? 0 : 42;
    }

    if (!strcmp(mode, "fork")) {
        int fd = open(path, O_RDONLY);
        if (fd < 0) return 43;
        pid_t c = fork();
        if (c < 0) return 44;
        if (c == 0) {
            char b = 0;
            ssize_t got = read(fd, &b, 1);
            _exit(got < 0 ? 45 : 0);
        }
        int st = 0;
        if (waitpid(c, &st, 0) != c) return 46;
        if (close(fd) != 0) return 47;
        return WIFEXITED(st) && WEXITSTATUS(st) == 0 ? 0 : 48;
    }

    if (!strcmp(mode, "vfork")) {
        pid_t c = vfork();
        if (c < 0) return 49;
        if (c == 0) _exit(0);
        int st = 0;
        if (waitpid(c, &st, 0) != c) return 50;
        return WIFEXITED(st) && WEXITSTATUS(st) == 0 ? 0 : 51;
    }

    if (!strcmp(mode, "exec")) {
        execl("/bin/true", "true", (char *)0);
        return 52;
    }

    if (!strcmp(mode, "failed-exec")) {
        execl("/definitely/not/execsurface-a2", "missing", (char *)0);
        if (errno != ENOENT) return 53;
        int fd = open("/dev/null", O_RDONLY);
        if (fd < 0) return 54;
        if (close(fd) != 0) return 55;
        return 0;
    }

    if (!strcmp(mode, "thread-exec")) {
        pthread_t t;
        if (pthread_create(&t, 0, thread_exec, 0) != 0) return 56;
        void *r = 0;
        if (pthread_join(t, &r) != 0) return 57;
        return 58;
    }

    if (!strcmp(mode, "exit-group")) {
        pthread_t t;
        if (pthread_create(&t, 0, thread_exit_group, 0) != 0) return 59;
        for (;;) pause();
    }

    return 65;
}
