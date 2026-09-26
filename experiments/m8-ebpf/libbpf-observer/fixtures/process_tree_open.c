#include <errno.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main(void)
{
    int fd = open("/dev/null", O_RDONLY | O_CLOEXEC);
    if (fd < 0)
        return 10;

    pid_t child = fork();
    if (child < 0) {
        close(fd);
        return 11;
    }

    if (child == 0) {
        execl("/bin/sleep", "sleep", "0.20", (char *)0);
        _exit(12);
    }

    /* Keep the successful-open fd live long enough for event-specific /proc resolution. */
    usleep(250000);

    int status = 0;
    while (waitpid(child, &status, 0) < 0) {
        if (errno == EINTR)
            continue;
        close(fd);
        return 13;
    }

    if (close(fd) != 0)
        return 14;

    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0)
        return 15;

    return 0;
}
