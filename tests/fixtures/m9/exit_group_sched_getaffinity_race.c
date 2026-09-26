#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <unistd.h>

#ifndef SYS_sched_getaffinity
#error "M9-H1.4B requires SYS_sched_getaffinity"
#endif
#ifndef SYS_exit_group
#error "M9-H1.4B requires SYS_exit_group"
#endif

static _Atomic unsigned long worker_iterations = 0;
static _Atomic int worker_started = 0;

static void *affinity_worker(void *unused) {
    (void)unused;
    cpu_set_t set;
    CPU_ZERO(&set);
    atomic_store_explicit(&worker_started, 1, memory_order_release);

    for (;;) {
        long rc = syscall(SYS_sched_getaffinity, 0, sizeof(set), &set);
        if (rc < 0) {
            _exit(91);
        }
        atomic_fetch_add_explicit(&worker_iterations, 1, memory_order_release);
    }

    return NULL;
}

int main(void) {
    pthread_t worker;
    int rc = pthread_create(&worker, NULL, affinity_worker, NULL);
    if (rc != 0) {
        errno = rc;
        perror("pthread_create");
        return 70;
    }

    while (!atomic_load_explicit(&worker_started, memory_order_acquire)) {
        sched_yield();
    }

    /*
     * No sleeps or timing oracle. Wait for proof that the sibling is actively
     * cycling through the exact syscall observed in the external failure, then
     * terminate the whole thread group from the leader.
     */
    while (atomic_load_explicit(&worker_iterations, memory_order_acquire) < 64UL) {
        sched_yield();
    }

    syscall(SYS_exit_group, 0);
    _exit(92);
}
