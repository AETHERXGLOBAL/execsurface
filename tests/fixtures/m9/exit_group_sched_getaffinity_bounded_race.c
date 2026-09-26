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
#error "M9-H1.9 requires SYS_sched_getaffinity"
#endif
#ifndef SYS_exit_group
#error "M9-H1.9 requires SYS_exit_group"
#endif

#define WORKERS 24
#define FINAL_BURST 128UL

static _Atomic unsigned int workers_started = 0;
static _Atomic unsigned int arm_final_burst = 0;
static _Atomic unsigned int workers_armed = 0;

static void cpu_relax(void) {
    atomic_signal_fence(memory_order_seq_cst);
}

static void *affinity_worker(void *unused) {
    (void)unused;
    cpu_set_t set;
    CPU_ZERO(&set);

    atomic_fetch_add_explicit(&workers_started, 1U, memory_order_release);
    while (atomic_load_explicit(&arm_final_burst, memory_order_acquire) == 0U) {
        cpu_relax();
    }

    /*
     * Signal before entering the finite syscall burst. The leader waits until
     * every sibling has crossed this point, then immediately enters
     * exit_group. Unlike the earlier infinite-loop fixture, each sibling now
     * contributes only a bounded number of syscall stops. That preserves a
     * strong teardown overlap while preventing an unbounded source of fresh
     * ptrace events from starving an already-pending stop.
     */
    atomic_fetch_add_explicit(&workers_armed, 1U, memory_order_release);
    for (unsigned long i = 0; i < FINAL_BURST; ++i) {
        long rc = syscall(SYS_sched_getaffinity, 0, sizeof(set), &set);
        if (rc < 0) {
            _exit(91);
        }
    }

    /* One final blocking syscall keeps the thread alive without generating
     * an endless stream of ptrace stops if the leader's stop is delayed. */
    for (;;) {
        pause();
    }
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
        cpu_relax();
    }

    atomic_store_explicit(&arm_final_burst, 1U, memory_order_release);
    while (atomic_load_explicit(&workers_armed, memory_order_acquire) != WORKERS) {
        cpu_relax();
    }

    syscall(SYS_exit_group, 0);
    _exit(92);
}
