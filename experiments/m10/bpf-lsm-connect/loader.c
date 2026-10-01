#define _GNU_SOURCE
#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <net/if.h>
#include <netinet/in.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <bpf/bpf.h>
#include <bpf/libbpf.h>
#include "bpf_lsm_connect.skel.h"

#define SESSION_ID UINT64_C(0x4d31303500000001)
#define ROLE_SUCCESS 1u
#define ROLE_FAIL 2u
#define KIND_CONNECT_CANDIDATE 1u
#define KIND_CONNECT_EXIT 2u

struct target_role { uint64_t session_id; uint32_t role; uint32_t _pad; };
struct connect_event {
    uint64_t session_id;
    int64_t rc;
    uint32_t addr_be;
    uint32_t tgid;
    uint32_t tid;
    uint32_t role;
    uint16_t port_be;
    uint16_t family;
    uint32_t kind;
};
struct connect_truth {
    uint32_t addr_be;
    uint16_t port_be;
    uint16_t family;
    int connect_rc;
    int connect_errno;
    int peer_ok;
};
struct role_obs {
    unsigned candidates;
    unsigned exits;
    unsigned success_confirms;
    unsigned malformed;
    struct connect_event candidate;
    struct connect_event exit_event;
};
struct observations {
    struct role_obs success_role;
    struct role_obs fail_role;
    unsigned wrong_session;
    unsigned unknown_role;
};

static int on_event(void *ctx, void *data, size_t len)
{
    struct observations *o = ctx;
    const struct connect_event *e = data;
    struct role_obs *r;
    if (len != sizeof(*e)) { o->success_role.malformed++; return 0; }
    if (e->session_id != SESSION_ID) { o->wrong_session++; return 0; }
    if (e->role == ROLE_SUCCESS) r = &o->success_role;
    else if (e->role == ROLE_FAIL) r = &o->fail_role;
    else { o->unknown_role++; return 0; }
    if (e->kind == KIND_CONNECT_CANDIDATE) {
        r->candidates++;
        r->candidate = *e;
    } else if (e->kind == KIND_CONNECT_EXIT) {
        r->exits++;
        r->exit_event = *e;
        if (e->rc == 0) r->success_confirms++;
    } else {
        r->malformed++;
    }
    return 0;
}

static int register_role(struct bpf_lsm_connect_bpf *skel, pid_t pid, uint32_t role)
{
    uint32_t key = (uint32_t)pid;
    struct target_role v = {.session_id = SESSION_ID, .role = role, ._pad = 0};
    return bpf_map_update_elem(bpf_map__fd(skel->maps.targets), &key, &v, BPF_ANY);
}

static int wait_stopped(pid_t pid)
{
    int st = 0;
    if (waitpid(pid, &st, WUNTRACED) != pid) return -1;
    return WIFSTOPPED(st) ? 0 : -1;
}

static int drain_until_exit(struct ring_buffer *rb, pid_t pid, int *status)
{
    for (;;) {
        int p = ring_buffer__poll(rb, 20);
        if (p < 0 && p != -EINTR) return -1;
        pid_t w = waitpid(pid, status, WNOHANG);
        if (w == pid) break;
        if (w < 0) return -1;
    }
    for (int i = 0; i < 10; i++) {
        int p = ring_buffer__poll(rb, 10);
        if (p < 0 && p != -EINTR) return -1;
    }
    return 0;
}

static int ensure_loopback_up(void)
{
    int fd = socket(AF_INET, SOCK_DGRAM, 0);
    struct ifreq ifr;
    if (fd < 0) return -1;
    memset(&ifr, 0, sizeof(ifr));
    strncpy(ifr.ifr_name, "lo", IFNAMSIZ - 1);
    if (ioctl(fd, SIOCGIFFLAGS, &ifr) < 0) { close(fd); return -1; }
    ifr.ifr_flags |= IFF_UP;
    if (ioctl(fd, SIOCSIFFLAGS, &ifr) < 0) { close(fd); return -1; }
    close(fd);
    return 0;
}

static int bind_loopback(int do_listen, struct sockaddr_in *bound)
{
    int fd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
    socklen_t sl = sizeof(*bound);
    int one = 1;
    if (fd < 0) return -1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    memset(bound, 0, sizeof(*bound));
    bound->sin_family = AF_INET;
    bound->sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    bound->sin_port = htons(0);
    if (bind(fd, (struct sockaddr *)bound, sizeof(*bound)) < 0) { close(fd); return -1; }
    if (getsockname(fd, (struct sockaddr *)bound, &sl) < 0) { close(fd); return -1; }
    if (do_listen && listen(fd, 4) < 0) { close(fd); return -1; }
    return fd;
}

static int run_role(struct bpf_lsm_connect_bpf *skel, struct ring_buffer *rb,
                    const struct sockaddr_in *dst, uint32_t role,
                    struct connect_truth *truth, int *child_exit)
{
    int pfd[2] = {-1, -1};
    int st = 0;
    pid_t child = -1;
    if (pipe(pfd)) return -1;
    child = fork();
    if (child < 0) goto fail;
    if (child == 0) {
        struct connect_truth t;
        struct sockaddr_in peer;
        socklen_t peerlen = sizeof(peer);
        int s;
        close(pfd[0]);
        raise(SIGSTOP);
        memset(&t, 0, sizeof(t));
        t.family = AF_INET;
        t.addr_be = dst->sin_addr.s_addr;
        t.port_be = dst->sin_port;
        s = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
        if (s < 0) _exit(61);
        errno = 0;
        t.connect_rc = connect(s, (const struct sockaddr *)dst, sizeof(*dst));
        t.connect_errno = t.connect_rc == 0 ? 0 : errno;
        if (t.connect_rc == 0 && getpeername(s, (struct sockaddr *)&peer, &peerlen) == 0 &&
            peer.sin_family == AF_INET && peer.sin_addr.s_addr == dst->sin_addr.s_addr &&
            peer.sin_port == dst->sin_port) {
            t.peer_ok = 1;
        }
        if (write(pfd[1], &t, sizeof(t)) != (ssize_t)sizeof(t)) _exit(62);
        close(s);
        if (role == ROLE_SUCCESS)
            _exit(t.connect_rc == 0 && t.peer_ok ? 0 : 63);
        _exit(t.connect_rc < 0 && t.connect_errno == ECONNREFUSED ? 0 : 64);
    }
    close(pfd[1]); pfd[1] = -1;
    if (wait_stopped(child) || register_role(skel, child, role) || kill(child, SIGCONT)) goto fail;
    if (drain_until_exit(rb, child, &st)) goto fail;
    child = -1;
    if (read(pfd[0], truth, sizeof(*truth)) != (ssize_t)sizeof(*truth)) goto fail;
    *child_exit = WIFEXITED(st) ? WEXITSTATUS(st) : -1;
    close(pfd[0]);
    return 0;
fail:
    if (child > 0) { kill(child, SIGKILL); waitpid(child, NULL, 0); }
    if (pfd[0] >= 0) close(pfd[0]);
    if (pfd[1] >= 0) close(pfd[1]);
    return -1;
}

int main(void)
{
    struct bpf_lsm_connect_bpf *skel = NULL;
    struct ring_buffer *rb = NULL;
    struct observations obs = {0};
    struct connect_truth s_truth = {0}, f_truth = {0};
    struct sockaddr_in success_dst, fail_dst;
    int listener = -1, reserved = -1, accepted = -1;
    int success_exit = -1, fail_exit = -1, accept_ok = 0, rc = 1;
    uint32_t zero = 0;
    uint64_t drops = 0;

    if (ensure_loopback_up()) { perror("loopback"); return 42; }
    listener = bind_loopback(1, &success_dst);
    reserved = bind_loopback(0, &fail_dst);
    if (listener < 0 || reserved < 0 || success_dst.sin_port == fail_dst.sin_port) {
        fprintf(stderr, "stage=fixture errno=%d\n", errno); rc = 42; goto out;
    }

    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    skel = bpf_lsm_connect_bpf__open_and_load();
    if (!skel) { fprintf(stderr, "stage=open_and_load errno=%d\n", errno); rc = 42; goto out; }
    if (bpf_lsm_connect_bpf__attach(skel)) { fprintf(stderr, "stage=attach errno=%d\n", errno); rc = 42; goto out; }
    rb = ring_buffer__new(bpf_map__fd(skel->maps.events), on_event, &obs, NULL);
    if (!rb) goto out;

    if (run_role(skel, rb, &success_dst, ROLE_SUCCESS, &s_truth, &success_exit)) goto out;
    if (s_truth.connect_rc == 0) {
        accepted = accept(listener, NULL, NULL);
        if (accepted >= 0) { accept_ok = 1; close(accepted); accepted = -1; }
    }
    if (run_role(skel, rb, &fail_dst, ROLE_FAIL, &f_truth, &fail_exit)) goto out;
    if (bpf_map_lookup_elem(bpf_map__fd(skel->maps.drops), &zero, &drops)) goto out;

    int s_dest_match = obs.success_role.candidates == 1 &&
        obs.success_role.candidate.family == AF_INET &&
        obs.success_role.candidate.addr_be == s_truth.addr_be &&
        obs.success_role.candidate.port_be == s_truth.port_be;
    int health = obs.success_role.malformed == 0 && obs.fail_role.malformed == 0 &&
        obs.wrong_session == 0 && obs.unknown_role == 0 && drops == 0;
    int fail_false_promotion = obs.fail_role.success_confirms > 0 ||
        (obs.fail_role.exits == 1 && obs.fail_role.exit_event.rc == 0);

    printf("success_truth_family=%u success_truth_addr=%u success_truth_port=%u success_connect_rc=%d success_peer_ok=%d accept_ok=%d\n",
        s_truth.family, ntohl(s_truth.addr_be), ntohs(s_truth.port_be), s_truth.connect_rc, s_truth.peer_ok, accept_ok);
    printf("success_candidates=%u success_exits=%u success_confirms=%u candidate_family=%u candidate_addr=%u candidate_port=%u exit_rc=%lld\n",
        obs.success_role.candidates, obs.success_role.exits, obs.success_role.success_confirms,
        obs.success_role.candidate.family, ntohl(obs.success_role.candidate.addr_be), ntohs(obs.success_role.candidate.port_be),
        (long long)obs.success_role.exit_event.rc);
    printf("fail_truth_port=%u fail_connect_rc=%d fail_errno=%d fail_candidates=%u fail_exits=%u fail_success_confirms=%u fail_exit_rc=%lld\n",
        ntohs(f_truth.port_be), f_truth.connect_rc, f_truth.connect_errno,
        obs.fail_role.candidates, obs.fail_role.exits, obs.fail_role.success_confirms,
        (long long)obs.fail_role.exit_event.rc);
    printf("success_exit=%d fail_exit=%d malformed=%u wrong_session=%u unknown_role=%u drops=%llu\n",
        success_exit, fail_exit, obs.success_role.malformed + obs.fail_role.malformed,
        obs.wrong_session, obs.unknown_role, (unsigned long long)drops);

    if (fail_exit == 0 && f_truth.connect_rc < 0 && f_truth.connect_errno == ECONNREFUSED && fail_false_promotion) {
        printf("classification_candidate=M10_5_FAILED_CONNECT_FALSE_PROMOTION\n"); rc = 4;
    } else if (success_exit == 0 && s_truth.connect_rc == 0 && s_truth.peer_ok && accept_ok && health &&
               obs.success_role.exits == 1 && obs.success_role.exit_event.rc == 0 &&
               obs.success_role.success_confirms == 1 && s_dest_match &&
               fail_exit == 0 && f_truth.connect_rc < 0 && f_truth.connect_errno == ECONNREFUSED &&
               obs.fail_role.exits == 1 && obs.fail_role.exit_event.rc < 0 && obs.fail_role.success_confirms == 0) {
        printf("classification_candidate=M10_5_HYBRID_CONNECT_MATCH\n"); rc = 0;
    } else if (success_exit == 0 && obs.success_role.exits == 1 && obs.success_role.exit_event.rc == 0 &&
               s_truth.connect_rc == 0 && s_truth.peer_ok && health && !s_dest_match) {
        printf("classification_candidate=M10_5_CONNECT_DESTINATION_MISMATCH\n"); rc = 5;
    } else {
        printf("classification_candidate=M10_5_EVIDENCE_INCOMPLETE\n"); rc = 3;
    }

out:
    if (accepted >= 0) close(accepted);
    ring_buffer__free(rb);
    bpf_lsm_connect_bpf__destroy(skel);
    if (reserved >= 0) close(reserved);
    if (listener >= 0) close(listener);
    return rc;
}
