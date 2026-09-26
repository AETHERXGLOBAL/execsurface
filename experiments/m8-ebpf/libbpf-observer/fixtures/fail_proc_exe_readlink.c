#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <stddef.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

static int is_proc_exe_path(const char *path)
{
    size_t len;
    if (!path || strncmp(path, "/proc/", 6) != 0)
        return 0;
    len = strlen(path);
    return len >= 4 && strcmp(path + len - 4, "/exe") == 0;
}

ssize_t readlink(const char *path, char *buf, size_t bufsiz)
{
    typedef ssize_t (*readlink_fn)(const char *, char *, size_t);
    static readlink_fn real_readlink;

    if (is_proc_exe_path(path)) {
        errno = ENOENT;
        return -1;
    }

    if (!real_readlink)
        real_readlink = (readlink_fn)dlsym(RTLD_NEXT, "readlink");
    if (!real_readlink) {
        errno = ENOSYS;
        return -1;
    }
    return real_readlink(path, buf, bufsiz);
}

ssize_t readlinkat(int dirfd, const char *path, char *buf, size_t bufsiz)
{
    typedef ssize_t (*readlinkat_fn)(int, const char *, char *, size_t);
    static readlinkat_fn real_readlinkat;

    if (is_proc_exe_path(path)) {
        errno = ENOENT;
        return -1;
    }

    if (!real_readlinkat)
        real_readlinkat = (readlinkat_fn)dlsym(RTLD_NEXT, "readlinkat");
    if (!real_readlinkat) {
        errno = ENOSYS;
        return -1;
    }
    return real_readlinkat(dirfd, path, buf, bufsiz);
}
