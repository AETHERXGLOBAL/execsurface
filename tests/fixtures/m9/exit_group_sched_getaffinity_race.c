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

#define WORKERS 16
#define ITERATIONS_BEFORE_EXIT 64UL

static _Atomic unsigned long worker_iterations = 0;
static _Atomic unsigned int workers_started = 0;

static void *affinity_worker(void *unused) {
    (void)unused;
    cpu_set_t set;
    CPU_ZERO(&set);
    atomic_fetch_add_explicit(&workers_started, 1U, memory_order_release);

    for (;;) {
        long rc = syscall(SYS_sched_getaffinity, 0, sizeof(set), &set);
        if (rc < 0) {
            _exit(91);
        }
        atomic_fetch_add_explicit(&worker_iterations, 1UL, memory_order_release);
    }

    return NULL;
}

int main(void) {
    pthread_t workers[WORKERS];

    for (int i = 0; i < WORKERS; ++i) {
        int rc = pthread_create(&workers[i], NULL, affinity_worker, NULL);
        if (rc != 0) {
            errno = rc;
            perror("pthread_create");
            return 70;
        }
    }

    while (atomic_load_explicit(&workers_started, memory_order_acquire) != WORKERS) {
        sched_yield();
    }

    /*
     * No sleeps and no timing-based success criterion. Every sibling has
     * entered the exact syscall loop observed in the external failure before
     * the leader arms exit_group. Multiple active siblings make the kernel
     * teardown / syscall-stop overlap a controlled stress condition rather
     * than relying on one scheduler slot.
     */
    const unsigned long target = (unsigned long)WORKERS * ITERATIONS_BEFORE_EXIT;
    while (atomic_load_explicit(&worker_iterations, memory_order_acquire) < target) {
        sched_yield();
    }

    syscall(SYS_exit_group, 0);
    _exit(92);
}
