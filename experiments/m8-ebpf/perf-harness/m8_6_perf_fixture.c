#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

static int wait_clean(pid_t child) {
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return 1;
    }
    return !(WIFEXITED(status) && WEXITSTATUS(status) == 0);
}

static int fork_exec_true_once(void) {
    pid_t child = fork();
    if (child < 0) {
        return 1;
    }
    if (child == 0) {
        execl("/bin/true", "true", (char *)NULL);
        _exit(127);
    }
    return wait_clean(child);
}

static int workload_short_exec(void) {
    for (int i = 0; i < 4; ++i) {
        if (fork_exec_true_once() != 0) {
            return 1;
        }
    }
    return 0;
}

static int tree_node(int depth) {
    if (depth <= 0) {
        execl("/bin/true", "true", (char *)NULL);
        _exit(127);
    }

    pid_t children[2];
    for (int i = 0; i < 2; ++i) {
        children[i] = fork();
        if (children[i] < 0) {
            return 1;
        }
        if (children[i] == 0) {
            int rc = tree_node(depth - 1);
            _exit(rc == 0 ? 0 : 126);
        }
    }

    int failed = 0;
    for (int i = 0; i < 2; ++i) {
        failed |= wait_clean(children[i]);
    }
    return failed;
}

static int workload_process_tree(void) {
    return tree_node(2);
}

static int open_read_close_loop(int count) {
    unsigned char byte = 0;
    for (int i = 0; i < count; ++i) {
        int fd = open("/dev/zero", O_RDONLY | O_CLOEXEC);
        if (fd < 0) {
            return 1;
        }
        if (read(fd, &byte, 1) != 1) {
            close(fd);
            return 1;
        }
        if (close(fd) != 0) {
            return 1;
        }
    }
    return 0;
}

static int workload_file_open(void) {
    return open_read_close_loop(128);
}

static int workload_high_event_rate(void) {
    return open_read_close_loop(1500);
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s short_exec|process_tree|file_open|high_event_rate\n", argv[0]);
        return 2;
    }

    if (strcmp(argv[1], "short_exec") == 0) {
        return workload_short_exec();
    }
    if (strcmp(argv[1], "process_tree") == 0) {
        return workload_process_tree();
    }
    if (strcmp(argv[1], "file_open") == 0) {
        return workload_file_open();
    }
    if (strcmp(argv[1], "high_event_rate") == 0) {
        return workload_high_event_rate();
    }

    fprintf(stderr, "unknown workload: %s\n", argv[1]);
    return 2;
}
