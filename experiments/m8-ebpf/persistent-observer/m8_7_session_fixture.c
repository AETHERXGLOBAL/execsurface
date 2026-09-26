// SPDX-License-Identifier: Apache-2.0
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

static int run_tree(void)
{
    pid_t child = fork();
    if (child < 0) {
        perror("fork");
        return 20;
    }
    if (child == 0) {
        execl("/bin/true", "true", (char *)NULL);
        _exit(127);
    }

    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        perror("waitpid");
        return 21;
    }
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        fprintf(stderr, "child outcome was not clean\n");
        return 22;
    }
    return 0;
}

int main(int argc, char **argv)
{
    if (argc != 2 || strcmp(argv[1], "tree") != 0) {
        fprintf(stderr, "usage: %s tree\n", argv[0]);
        return 2;
    }
    return run_tree();
}
