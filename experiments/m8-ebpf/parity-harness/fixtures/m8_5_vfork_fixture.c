#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main(void) {
    int fd = open("/dev/zero", O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
        return 10;
    }

    uint8_t byte = 0;
    if (read(fd, &byte, 1) != 1) {
        close(fd);
        return 11;
    }

    pid_t child = vfork();
    if (child < 0) {
        close(fd);
        return 12;
    }

    if (child == 0) {
        execl("/bin/true", "true", (char *)NULL);
        _exit(127);
    }

    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        close(fd);
        return 13;
    }
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        close(fd);
        return 14;
    }

    /* Keep the successful-open descriptor alive long enough for the
       userspace metadata resolver to observe it without relying on a tiny
       post-open race. */
    usleep(120000);

    if (close(fd) != 0) {
        return 15;
    }
    return 0;
}
