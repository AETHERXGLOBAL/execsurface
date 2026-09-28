#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include <bpf/bpf.h>
#include <bpf/libbpf.h>

#include "bpf_lsm_file.skel.h"

#define ITERATIONS 100
#define SESSION_ID UINT64_C(0x4d31303300000001)

struct file_event {
    uint64_t session_id;
    uint64_t ktime_ns;
    uint64_t ino;
    uint64_t dev;
    uint32_t tgid;
    uint32_t tid;
    uint32_t f_flags;
    uint32_t mask;
};

struct counts {
    uint64_t dev_a, ino_a;
    uint64_t dev_b, ino_b;
    unsigned a;
    unsigned b;
    unsigned other;
    unsigned wrong_session;
};

static int on_event(void *ctx, void *data, size_t len)
{
    struct counts *c = ctx;
    const struct file_event *e = data;
    if (len != sizeof(*e)) {
        c->other++;
        return 0;
    }
    if (e->session_id != SESSION_ID) {
        c->wrong_session++;
        return 0;
    }
    if (e->dev == c->dev_a && e->ino == c->ino_a)
        c->a++;
    else if (e->dev == c->dev_b && e->ino == c->ino_b)
        c->b++;
    else
        c->other++;
    return 0;
}

static int write_byte_file(const char *path, char value)
{
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY | O_CLOEXEC, 0600);
    if (fd < 0)
        return -1;
    ssize_t n = write(fd, &value, 1);
    int saved = errno;
    close(fd);
    errno = saved;
    return n == 1 ? 0 : -1;
}

static int target_loop(const char *a, const char *b)
{
    raise(SIGSTOP);
    for (int i = 0; i < ITERATIONS; i++) {
        const char *paths[2] = {a, b};
        const char expected[2] = {'A', 'B'};
        for (int j = 0; j < 2; j++) {
            char v = 0;
            int fd = open(paths[j], O_RDONLY | O_CLOEXEC);
            if (fd < 0)
                _exit(20);
            if (read(fd, &v, 1) != 1 || v != expected[j]) {
                close(fd);
                _exit(21);
            }
            close(fd);
        }
    }
    _exit(0);
}

int main(void)
{
    char dir[] = "/tmp/execsurface-m10-3-XXXXXX";
    char path_a[512], path_b[512];
    struct stat st_a, st_b;
    struct bpf_lsm_file_bpf *skel = NULL;
    struct ring_buffer *rb = NULL;
    struct counts counts = {0};
    pid_t child = -1;
    int status = 0, rc = 1;
    uint32_t key0 = 0;
    uint64_t drops = 0;

    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);

    if (!mkdtemp(dir)) {
        perror("mkdtemp");
        return 2;
    }
    snprintf(path_a, sizeof(path_a), "%s/A", dir);
    snprintf(path_b, sizeof(path_b), "%s/B", dir);
    if (write_byte_file(path_a, 'A') || write_byte_file(path_b, 'B') ||
        stat(path_a, &st_a) || stat(path_b, &st_b)) {
        perror("fixture setup");
        goto out;
    }
    if (st_a.st_ino == st_b.st_ino && st_a.st_dev == st_b.st_dev) {
        fprintf(stderr, "fixture files are not distinct\n");
        goto out;
    }

    counts.dev_a = (uint64_t)st_a.st_dev;
    counts.ino_a = (uint64_t)st_a.st_ino;
    counts.dev_b = (uint64_t)st_b.st_dev;
    counts.ino_b = (uint64_t)st_b.st_ino;

    skel = bpf_lsm_file_bpf__open_and_load();
    if (!skel) {
        fprintf(stderr, "classification=BPF_LSM_FILE_HOST_UNAVAILABLE stage=open_and_load errno=%d\n", errno);
        rc = 42;
        goto out;
    }
    if (bpf_lsm_file_bpf__attach(skel)) {
        fprintf(stderr, "classification=BPF_LSM_FILE_HOST_UNAVAILABLE stage=attach errno=%d\n", errno);
        rc = 42;
        goto out;
    }

    rb = ring_buffer__new(bpf_map__fd(skel->maps.events), on_event, &counts, NULL);
    if (!rb) {
        fprintf(stderr, "ring_buffer__new failed\n");
        goto out;
    }

    child = fork();
    if (child < 0) {
        perror("fork");
        goto out;
    }
    if (child == 0)
        target_loop(path_a, path_b);

    if (waitpid(child, &status, WUNTRACED) != child || !WIFSTOPPED(status)) {
        fprintf(stderr, "child did not reach registration barrier\n");
        goto out;
    }

    uint32_t tgid = (uint32_t)child;
    uint64_t session = SESSION_ID;
    if (bpf_map_update_elem(bpf_map__fd(skel->maps.target_tgids), &tgid, &session, BPF_ANY)) {
        perror("bpf_map_update_elem target_tgids");
        goto out;
    }

    if (kill(child, SIGCONT)) {
        perror("SIGCONT");
        goto out;
    }

    for (;;) {
        int p = ring_buffer__poll(rb, 50);
        if (p < 0 && p != -EINTR) {
            fprintf(stderr, "ring_buffer poll error=%d\n", p);
            goto out;
        }
        pid_t w = waitpid(child, &status, WNOHANG);
        if (w == child)
            break;
        if (w < 0) {
            perror("waitpid");
            goto out;
        }
    }
    child = -1;
    for (int i = 0; i < 10; i++)
        ring_buffer__poll(rb, 20);

    bpf_map_delete_elem(bpf_map__fd(skel->maps.target_tgids), &tgid);
    if (bpf_map_lookup_elem(bpf_map__fd(skel->maps.drops), &key0, &drops)) {
        perror("drop counter lookup");
        goto out;
    }

    printf("classification_candidate=%s\n",
           (WIFEXITED(status) && WEXITSTATUS(status) == 0 &&
            counts.a == ITERATIONS && counts.b == ITERATIONS &&
            counts.other == 0 && counts.wrong_session == 0 && drops == 0)
               ? "BPF_LSM_FILE_AUTHORITY_PASS"
               : "BPF_LSM_FILE_AUTHORITY_PARTIAL");
    printf("target_exit=%d\n", WIFEXITED(status) ? WEXITSTATUS(status) : -1);
    printf("session_id=%llu\n", (unsigned long long)SESSION_ID);
    printf("a_dev=%llu a_ino=%llu a_events=%u expected=%d\n",
           (unsigned long long)counts.dev_a, (unsigned long long)counts.ino_a,
           counts.a, ITERATIONS);
    printf("b_dev=%llu b_ino=%llu b_events=%u expected=%d\n",
           (unsigned long long)counts.dev_b, (unsigned long long)counts.ino_b,
           counts.b, ITERATIONS);
    printf("other_session_events=%u wrong_session_events=%u drops=%llu\n",
           counts.other, counts.wrong_session, (unsigned long long)drops);

    rc = (WIFEXITED(status) && WEXITSTATUS(status) == 0 &&
          counts.a == ITERATIONS && counts.b == ITERATIONS &&
          counts.other == 0 && counts.wrong_session == 0 && drops == 0)
             ? 0
             : 3;

out:
    if (child > 0) {
        kill(child, SIGKILL);
        waitpid(child, NULL, 0);
    }
    ring_buffer__free(rb);
    bpf_lsm_file_bpf__destroy(skel);
    unlink(path_a);
    unlink(path_b);
    rmdir(dir);
    return rc;
}
