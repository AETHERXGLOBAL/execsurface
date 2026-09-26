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

#define WORKERS 16
#define ARM_ITERATIONS_PER_WORKER 32UL
#define MAX_ITERATIONS_PER_WORKER 2048UL

static _Atomic unsigned long worker_iterations[WORKERS];
static _Atomic unsigned int workers_started = 0;
static _Atomic unsigned int workers_finished_syscalls = 0;

static void *affinity_worker(void *raw_index) {
    const uintptr_t index = (uintptr_t)raw_index;
    cpu_set_t set;
    CPU_ZERO(&set);
    atomic_fetch_add_explicit(&workers_started, 1U, memory_order_release);

    for (unsigned long iteration = 0; iteration < MAX_ITERATIONS_PER_WORKER; ++iteration) {
        long rc = syscall(SYS_sched_getaffinity, 0, sizeof(set), &set);
        if (rc < 0) {
            _exit(91);
        }
        atomic_store_explicit(
            &worker_iterations[index], iteration + 1UL, memory_order_release);
    }

    atomic_fetch_add_explicit(&workers_finished_syscalls, 1U, memory_order_release);

    /*
     * Deliberately generate no more syscalls.  If the leader was only
     * starved by an unbounded stream of ptrace syscall-stops, the event
     * stream is now finite and must drain.  exit_group from the leader still
     * terminates these user-space spinning siblings.
     */
    for (;;) {
        atomic_signal_fence(memory_order_seq_cst);
    }

    return NULL;
}

int main(void) {
    pthread_t workers[WORKERS];

    for (uintptr_t i = 0; i < WORKERS; ++i) {
        int rc = pthread_create(&workers[i], NULL, affinity_worker, (void *)i);
        if (rc != 0) {
            errno = rc;
            perror("pthread_create");
            return 70;
        }
    }

    while (atomic_load_explicit(&workers_started, memory_order_acquire) != WORKERS) {
        atomic_signal_fence(memory_order_seq_cst);
    }

    for (;;) {
        unsigned int armed = 0;
        for (size_t i = 0; i < WORKERS; ++i) {
            if (atomic_load_explicit(&worker_iterations[i], memory_order_acquire) >=
                ARM_ITERATIONS_PER_WORKER) {
                ++armed;
            }
        }
        if (armed == WORKERS) {
            break;
        }
        atomic_signal_fence(memory_order_seq_cst);
    }

    syscall(SYS_exit_group, 0);
    _exit(92);
}
