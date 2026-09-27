#define _GNU_SOURCE
#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <linux/openat2.h>
#include <linux/sched.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/un.h>
#include <sys/wait.h>
#include <unistd.h>

static size_t page_size(void) {
    long p = sysconf(_SC_PAGESIZE);
    if (p <= 0) _exit(90);
    return (size_t)p;
}

static unsigned char *map_pages(size_t count) {
    size_t n = page_size() * count;
    void *p = mmap(NULL, n, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (p == MAP_FAILED) return NULL;
    return (unsigned char *)p;
}

static int do_open_path(const char *path) {
    int fd = open(path, O_RDONLY);
    if (fd >= 0) {
        close(fd);
        return 0;
    }
    return (errno == ENOENT || errno == EFAULT || errno == ENAMETOOLONG) ? 0 : 11;
}

static int string_short(void) {
    return do_open_path("/dev/null");
}

static int string_empty(void) {
    int fd = open("", O_RDONLY);
    if (fd >= 0) {
        close(fd);
        return 12;
    }
    return errno == ENOENT ? 0 : 13;
}

static int string_boundary(int span) {
    size_t ps = page_size();
    unsigned char *m = map_pages(2);
    if (!m) return 14;
    const char *src = "/dev/null";
    size_t n = strlen(src) + 1;
    unsigned char *start = span ? m + ps - 4 : m + ps - n;
    memcpy(start, src, n);
    int rc = do_open_path((char *)start);
    munmap(m, ps * 2);
    return rc;
}

static int string_unmapped_after(void) {
    size_t ps = page_size();
    unsigned char *m = map_pages(2);
    if (!m) return 15;
    memset(m + ps - 8, 'x', 8);
    if (munmap(m + ps, ps) != 0) return 16;
    int rc = do_open_path((char *)(m + ps - 8));
    munmap(m, ps);
    return rc;
}

static int string_max_no_nul(void) {
    size_t ps = page_size();
    unsigned char *m = map_pages(2);
    if (!m) return 17;
    memset(m, 'x', ps * 2);
    int rc = do_open_path((char *)m);
    munmap(m, ps * 2);
    return rc;
}

static int openat2_with_how(struct open_how *how) {
    long fd = syscall(SYS_openat2, AT_FDCWD, "/dev/null", how, sizeof(*how));
    if (fd >= 0) {
        close((int)fd);
        return 0;
    }
    return (errno == ENOSYS || errno == EFAULT || errno == EINVAL) ? 0 : 18;
}

static int fixed_openat2_normal(void) {
    struct open_how how = {0};
    how.flags = O_RDONLY;
    return openat2_with_how(&how);
}

static int fixed_openat2_cross(int inaccessible) {
    size_t ps = page_size();
    unsigned char *m = map_pages(2);
    if (!m) return 19;
    struct open_how local = {0};
    local.flags = O_RDONLY;
    unsigned char *start = m + ps - 8;
    memcpy(start, &local, inaccessible ? 8 : sizeof(local));
    if (inaccessible && munmap(m + ps, ps) != 0) return 20;
    int rc = openat2_with_how((struct open_how *)start);
    munmap(m, ps);
    if (!inaccessible) munmap(m + ps, ps);
    return rc;
}

static int connect_ipv4(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return 21;
    struct sockaddr_in sa = {0};
    sa.sin_family = AF_INET;
    sa.sin_port = htons(9);
    sa.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    (void)connect(fd, (struct sockaddr *)&sa, sizeof(sa));
    close(fd);
    return 0;
}

static int connect_ipv6(void) {
    int fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (fd < 0) return 22;
    struct sockaddr_in6 sa = {0};
    sa.sin6_family = AF_INET6;
    sa.sin6_port = htons(9);
    sa.sin6_addr = in6addr_loopback;
    (void)connect(fd, (struct sockaddr *)&sa, sizeof(sa));
    close(fd);
    return 0;
}

static int connect_unix(void) {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return 23;
    struct sockaddr_un sa = {0};
    sa.sun_family = AF_UNIX;
    snprintf(sa.sun_path, sizeof(sa.sun_path), "/tmp/execsurface-b-no-socket");
    (void)connect(fd, (struct sockaddr *)&sa, sizeof(sa));
    close(fd);
    return 0;
}

static int do_rename(void) {
    const char *a = "/tmp/execsurface-b-rename-a";
    const char *b = "/tmp/execsurface-b-rename-b";
    unlink(a);
    unlink(b);
    int fd = open(a, O_CREAT | O_TRUNC | O_WRONLY, 0600);
    if (fd < 0) return 24;
    close(fd);
    if (rename(a, b) != 0) return 25;
    unlink(b);
    return 0;
}

static int clone3_flags(void) {
#ifdef SYS_clone3
    struct clone_args args;
    memset(&args, 0, sizeof(args));
    args.exit_signal = SIGCHLD;
    long r = syscall(SYS_clone3, &args, sizeof(args));
    if (r == 0) _exit(0);
    if (r < 0) return (errno == ENOSYS || errno == EPERM) ? 0 : 26;
    int st = 0;
    if (waitpid((pid_t)r, &st, 0) != r) return 27;
    return WIFEXITED(st) && WEXITSTATUS(st) == 0 ? 0 : 28;
#else
    return 0;
#endif
}

int main(int argc, char **argv) {
    if (argc != 2) return 64;
    if (!strcmp(argv[1], "string-short")) return string_short();
    if (!strcmp(argv[1], "string-empty")) return string_empty();
    if (!strcmp(argv[1], "string-boundary")) return string_boundary(0);
    if (!strcmp(argv[1], "string-span-pages")) return string_boundary(1);
    if (!strcmp(argv[1], "string-unmapped-after")) return string_unmapped_after();
    if (!strcmp(argv[1], "string-max-no-nul")) return string_max_no_nul();
    if (!strcmp(argv[1], "fixed-openat2-normal")) return fixed_openat2_normal();
    if (!strcmp(argv[1], "fixed-openat2-cross")) return fixed_openat2_cross(0);
    if (!strcmp(argv[1], "fixed-openat2-unmapped")) return fixed_openat2_cross(1);
    if (!strcmp(argv[1], "connect-ipv4")) return connect_ipv4();
    if (!strcmp(argv[1], "connect-ipv6")) return connect_ipv6();
    if (!strcmp(argv[1], "connect-unix")) return connect_unix();
    if (!strcmp(argv[1], "rename")) return do_rename();
    if (!strcmp(argv[1], "clone3")) return clone3_flags();
    return 65;
}
