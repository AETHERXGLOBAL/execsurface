#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#ifndef SYS_gettid
#error "M9-H1.1 fixture requires Linux SYS_gettid"
#endif

static _Atomic uint64_t checksum = 0;

static void *short_lived_thread(void *arg) {
    uintptr_t token = (uintptr_t)arg;
    pid_t tid = (pid_t)syscall(SYS_gettid);
    atomic_fetch_xor_explicit(
        &checksum,
        ((uint64_t)(uint32_t)tid << 32) ^ (uint64_t)token,
        memory_order_relaxed
    );
    return NULL;
}

static int parse_positive(const char *text, int fallback) {
    if (text == NULL) {
        return fallback;
    }
    char *end = NULL;
    errno = 0;
    long value = strtol(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || value <= 0 || value > 10000) {
        return -1;
    }
    return (int)value;
}

int main(int argc, char **argv) {
    int rounds = parse_positive(argc > 1 ? argv[1] : NULL, 12);
    int threads_per_round = parse_positive(argc > 2 ? argv[2] : NULL, 16);
    int forks_per_round = parse_positive(argc > 3 ? argv[3] : NULL, 4);

    if (rounds < 0 || threads_per_round < 0 || forks_per_round < 0) {
        fprintf(stderr, "usage: %s [rounds] [threads_per_round] [forks_per_round]\n", argv[0]);
        return 64;
    }

    pthread_t *threads = calloc((size_t)threads_per_round, sizeof(*threads));
    if (threads == NULL) {
        perror("calloc");
        return 70;
    }

    for (int round = 0; round < rounds; ++round) {
        for (int i = 0; i < threads_per_round; ++i) {
            uintptr_t token = (uintptr_t)((round + 1) * 100000 + i + 1);
            int rc = pthread_create(&threads[i], NULL, short_lived_thread, (void *)token);
            if (rc != 0) {
                errno = rc;
                perror("pthread_create");
                free(threads);
                return 71;
            }
        }

        for (int i = 0; i < threads_per_round; ++i) {
            int rc = pthread_join(threads[i], NULL);
            if (rc != 0) {
                errno = rc;
                perror("pthread_join");
                free(threads);
                return 72;
            }
        }

        for (int i = 0; i < forks_per_round; ++i) {
            pid_t child = fork();
            if (child < 0) {
                perror("fork");
                free(threads);
                return 73;
            }
            if (child == 0) {
                pid_t tid = (pid_t)syscall(SYS_gettid);
                if (tid <= 0) {
                    _exit(74);
                }
                _exit(0);
            }

            int status = 0;
            if (waitpid(child, &status, 0) != child) {
                perror("waitpid");
                free(threads);
                return 75;
            }
            if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
                fprintf(stderr, "child %ld returned unexpected status %#x\n", (long)child, status);
                free(threads);
                return 76;
            }
        }
    }

    uint64_t final_checksum = atomic_load_explicit(&checksum, memory_order_relaxed);
    printf(
        "M9_H1_1_FIXTURE_PASS rounds=%d threads_per_round=%d forks_per_round=%d checksum=%llu\n",
        rounds,
        threads_per_round,
        forks_per_round,
        (unsigned long long)final_checksum
    );

    free(threads);
    return 0;
}
