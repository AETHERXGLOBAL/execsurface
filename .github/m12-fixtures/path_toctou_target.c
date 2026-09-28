#define _GNU_SOURCE
#include <pthread.h>
#include <stdatomic.h>
#include <stddef.h>
#include <stdint.h>
#include <fcntl.h>
#include <unistd.h>

struct shared_path_buffer {
    _Atomic unsigned char first;
    unsigned char nul;
};

_Static_assert(sizeof(_Atomic unsigned char) == 1, "fixture requires one-byte atomic char");
_Static_assert(offsetof(struct shared_path_buffer, nul) == 1, "fixture path bytes must be contiguous");

static struct shared_path_buffer shared_path;
static _Atomic int started;
static _Atomic int stop_mutator;
static _Atomic uint64_t mutations;

static void *mutator_main(void *unused) {
    (void)unused;
    atomic_store_explicit(&started, 1, memory_order_release);
    while (!atomic_load_explicit(&stop_mutator, memory_order_relaxed)) {
        atomic_store_explicit(&shared_path.first, (unsigned char)'A', memory_order_relaxed);
        atomic_fetch_add_explicit(&mutations, 1, memory_order_relaxed);
        atomic_store_explicit(&shared_path.first, (unsigned char)'B', memory_order_relaxed);
        atomic_fetch_add_explicit(&mutations, 1, memory_order_relaxed);
    }
    return NULL;
}

int main(void) {
    atomic_init(&shared_path.first, (unsigned char)'A');
    shared_path.nul = '\0';
    atomic_init(&started, 0);
    atomic_init(&stop_mutator, 0);
    atomic_init(&mutations, 0);

    pthread_t mutator;
    if (pthread_create(&mutator, NULL, mutator_main, NULL) != 0) return 30;
    while (!atomic_load_explicit(&started, memory_order_acquire)) {}
    while (atomic_load_explicit(&mutations, memory_order_relaxed) < 100000) {}

    int fd = open((const char *)&shared_path, O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
        atomic_store_explicit(&stop_mutator, 1, memory_order_relaxed);
        (void)pthread_join(mutator, NULL);
        return 31;
    }

    unsigned char truth = 0;
    ssize_t n = read(fd, &truth, 1);
    (void)close(fd);
    atomic_store_explicit(&stop_mutator, 1, memory_order_relaxed);
    if (pthread_join(mutator, NULL) != 0) return 32;
    if (n != 1) return 33;
    if (truth == (unsigned char)'A') return 11;
    if (truth == (unsigned char)'B') return 12;
    return 34;
}
