#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <unistd.h>

#define READ_ATTEMPTS 5000

static int race_fd = -1;
static _Atomic int mutator_started;
static _Atomic int stop_mutator;
static _Atomic uint64_t mutation_cycles;

static void *mutator_main(void *unused) {
    (void)unused;
    const char *names[2] = {"B", "A"};
    unsigned int index = 0;
    atomic_store_explicit(&mutator_started, 1, memory_order_release);
    while (!atomic_load_explicit(&stop_mutator, memory_order_relaxed)) {
        (void)close(race_fd);
        int replacement = open(names[index & 1U], O_RDONLY | O_CLOEXEC);
        index++;
        if (replacement < 0) continue;
        if (replacement != race_fd) {
            if (dup2(replacement, race_fd) < 0) {
                (void)close(replacement);
                continue;
            }
            (void)close(replacement);
        }
        atomic_fetch_add_explicit(&mutation_cycles, 1, memory_order_relaxed);
    }
    return NULL;
}

static int write_truth_log(const unsigned char *truth, size_t count) {
    int fd = open("fd_truth.log", O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd < 0) return -1;
    size_t written = 0;
    while (written < count) {
        ssize_t n = write(fd, truth + written, count - written);
        if (n > 0) {
            written += (size_t)n;
            continue;
        }
        if (n < 0 && errno == EINTR) continue;
        (void)close(fd);
        return -1;
    }
    return close(fd);
}

int main(void) {
    unsigned char truth[READ_ATTEMPTS];
    size_t success_count = 0;
    race_fd = open("A", O_RDONLY | O_CLOEXEC);
    if (race_fd < 0) return 40;

    atomic_init(&mutator_started, 0);
    atomic_init(&stop_mutator, 0);
    atomic_init(&mutation_cycles, 0);

    pthread_t mutator;
    if (pthread_create(&mutator, NULL, mutator_main, NULL) != 0) {
        (void)close(race_fd);
        return 41;
    }
    while (!atomic_load_explicit(&mutator_started, memory_order_acquire)) {}
    while (atomic_load_explicit(&mutation_cycles, memory_order_relaxed) < 100) {}

    for (size_t i = 0; i < READ_ATTEMPTS; i++) {
        unsigned char value = 0;
        ssize_t n = pread(race_fd, &value, 1, 0);
        if (n == 1 && (value == (unsigned char)'A' || value == (unsigned char)'B')) {
            truth[success_count++] = value;
        }
    }

    atomic_store_explicit(&stop_mutator, 1, memory_order_relaxed);
    if (pthread_join(mutator, NULL) != 0) return 42;
    (void)close(race_fd);
    if (write_truth_log(truth, success_count) != 0) return 43;
    if (success_count < 50) return 44;
    return 0;
}
