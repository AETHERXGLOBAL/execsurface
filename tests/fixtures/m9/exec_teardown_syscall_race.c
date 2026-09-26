#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

#ifndef SYS_gettid
#error "M9-H1.4 fixture requires Linux SYS_gettid"
#endif

static _Atomic int ready_workers = 0;
static _Atomic int release_exec = 0;
static _Atomic uint64_t activity = 0;

struct worker_args {
    int worker_count;
};

static void *syscall_worker(void *opaque) {
    struct worker_args *args = (struct worker_args *)opaque;
    atomic_fetch_add_explicit(&ready_workers, 1, memory_order_release);

    while (!atomic_load_explicit(&release_exec, memory_order_acquire)) {
        long tid = syscall(SYS_gettid);
        atomic_fetch_xor_explicit(
            &activity,
            ((uint64_t)(uint32_t)tid << 32) ^ (uint64_t)(uintptr_t)pthread_self(),
            memory_order_relaxed
        );
    }

    /* Keep generating syscall-stops while the sibling begins exec teardown. */
    for (;;) {
        long tid = syscall(SYS_gettid);
        atomic_fetch_add_explicit(&activity, (uint64_t)(uint32_t)tid + 1u, memory_order_relaxed);
        sched_yield();
    }

    return NULL;
}

static void *exec_worker(void *opaque) {
    struct worker_args *args = (struct worker_args *)opaque;
    while (atomic_load_explicit(&ready_workers, memory_order_acquire) < args->worker_count) {
        sched_yield();
    }

    /* Ensure workers are actively issuing traced syscalls before group teardown. */
    uint64_t before = atomic_load_explicit(&activity, memory_order_relaxed);
    while (atomic_load_explicit(&activity, memory_order_relaxed) == before) {
        sched_yield();
    }

    atomic_store_explicit(&release_exec, 1, memory_order_release);

    char *const argv[] = {(char *)"true", NULL};
    char *const envp[] = {NULL};
    execve("/bin/true", argv, envp);
    perror("execve");
    _exit(111);
}

static int parse_workers(const char *text) {
    if (text == NULL) {
        return 48;
    }
    char *end = NULL;
    errno = 0;
    long value = strtol(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || value < 2 || value > 256) {
        return -1;
    }
    return (int)value;
}

int main(int argc, char **argv) {
    int worker_count = parse_workers(argc > 1 ? argv[1] : NULL);
    if (worker_count < 0) {
        fprintf(stderr, "usage: %s [worker_count:2..256]\n", argv[0]);
        return 64;
    }

    pthread_t *workers = calloc((size_t)worker_count, sizeof(*workers));
    if (workers == NULL) {
        perror("calloc");
        return 70;
    }

    struct worker_args args = {.worker_count = worker_count};
    for (int i = 0; i < worker_count; ++i) {
        int rc = pthread_create(&workers[i], NULL, syscall_worker, &args);
        if (rc != 0) {
            errno = rc;
            perror("pthread_create worker");
            return 71;
        }
    }

    pthread_t exec_thread;
    int rc = pthread_create(&exec_thread, NULL, exec_worker, &args);
    if (rc != 0) {
        errno = rc;
        perror("pthread_create exec");
        return 72;
    }

    /* Successful execution never returns here: exec_worker replaces the process. */
    rc = pthread_join(exec_thread, NULL);
    if (rc != 0) {
        errno = rc;
        perror("pthread_join exec");
        return 73;
    }

    fprintf(stderr, "M9_H1_4_UNEXPECTED_EXEC_RETURN activity=%llu\n",
            (unsigned long long)atomic_load_explicit(&activity, memory_order_relaxed));
    free(workers);
    return 74;
}
