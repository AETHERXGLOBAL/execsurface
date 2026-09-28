#define _GNU_SOURCE
#include <errno.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <bpf/bpf.h>
#include <bpf/libbpf.h>
#include "loss_authority.skel.h"

#define SESSION_ID UINT64_C(0x4d31303600000001)
#define N_EVENTS 200000ULL

struct loss_event {
    uint64_t session_id;
    uint64_t sequence;
    uint32_t tgid;
    uint32_t tid;
};

struct obs {
    uint64_t accepted;
    uint64_t malformed;
    uint64_t wrong_session;
    uint32_t target_tgid;
};

static int on_event(void *ctx, void *data, size_t len)
{
    struct obs *o = ctx;
    const struct loss_event *e = data;
    if (len != sizeof(*e)) { o->malformed++; return 0; }
    if (e->session_id != SESSION_ID || e->tgid != o->target_tgid) {
        o->wrong_session++;
        return 0;
    }
    o->accepted++;
    return 0;
}

static int wait_stopped(pid_t pid)
{
    int st = 0;
    if (waitpid(pid, &st, WUNTRACED) != pid) return -1;
    return WIFSTOPPED(st) ? 0 : -1;
}

int main(void)
{
    struct loss_authority_bpf *skel = NULL;
    struct ring_buffer *rb = NULL;
    struct obs obs = {0};
    pid_t child = -1;
    int st = 0, rc = 1;
    uint32_t zero = 0, key = 0;
    uint64_t session = SESSION_ID, attempts = 0, drops = 0;
    int target_exit = -1;

    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    skel = loss_authority_bpf__open_and_load();
    if (!skel) { fprintf(stderr, "stage=open_and_load errno=%d\n", errno); return 42; }
    if (loss_authority_bpf__attach(skel)) { fprintf(stderr, "stage=attach errno=%d\n", errno); rc = 42; goto out; }

    rb = ring_buffer__new(bpf_map__fd(skel->maps.events), on_event, &obs, NULL);
    if (!rb) { fprintf(stderr, "stage=ring_buffer errno=%d\n", errno); rc = 42; goto out; }

    child = fork();
    if (child < 0) { rc = 42; goto out; }
    if (child == 0) {
        raise(SIGSTOP);
        for (uint64_t i = 0; i < N_EVENTS; i++)
            (void)syscall(SYS_getpid);
        _exit(0);
    }

    if (wait_stopped(child)) { rc = 42; goto out; }
    obs.target_tgid = (uint32_t)child;
    key = (uint32_t)child;
    if (bpf_map_update_elem(bpf_map__fd(skel->maps.targets), &key, &session, BPF_ANY)) {
        fprintf(stderr, "stage=register errno=%d\n", errno); rc = 42; goto out;
    }
    if (kill(child, SIGCONT)) { rc = 42; goto out; }

    /* Deliberately do not poll while the producer emits N_EVENTS. */
    if (waitpid(child, &st, 0) != child) { rc = 42; goto out; }
    child = -1;
    target_exit = WIFEXITED(st) ? WEXITSTATUS(st) : -1;

    if (bpf_map_lookup_elem(bpf_map__fd(skel->maps.attempts), &zero, &attempts) ||
        bpf_map_lookup_elem(bpf_map__fd(skel->maps.drops), &zero, &drops)) {
        rc = 42; goto out;
    }

    for (;;) {
        int n = ring_buffer__poll(rb, 0);
        if (n < 0 && n != -EINTR) { rc = 42; goto out; }
        if (n <= 0) break;
    }

    int accounting_ok = attempts == N_EVENTS && drops > 0 && obs.accepted > 0 &&
        obs.accepted < attempts && obs.accepted + drops == attempts &&
        obs.malformed == 0 && obs.wrong_session == 0 && target_exit == 0;
    int evidence_complete = drops == 0;
    int pass_authority = evidence_complete;

    printf("requested=%llu attempts=%llu accepted=%llu drops=%llu target_exit=%d malformed=%llu wrong_session=%llu\n",
        (unsigned long long)N_EVENTS, (unsigned long long)attempts,
        (unsigned long long)obs.accepted, (unsigned long long)drops, target_exit,
        (unsigned long long)obs.malformed, (unsigned long long)obs.wrong_session);
    printf("evidence_complete=%s pass_authority=%s accounting_ok=%s\n",
        evidence_complete ? "true" : "false",
        pass_authority ? "true" : "false",
        accounting_ok ? "true" : "false");

    if (drops > 0 && (evidence_complete || pass_authority)) {
        printf("classification_candidate=M10_6_REAL_LOSS_SILENT_AUTHORITY_FAILURE\n");
        rc = 4;
    } else if (accounting_ok && !evidence_complete && !pass_authority) {
        printf("classification_candidate=M10_6_REAL_LOSS_DETECTED_FAIL_CLOSED\n");
        rc = 0;
    } else if (drops == 0) {
        printf("classification_candidate=M10_6_LOSS_NOT_FORCED\n");
        rc = 5;
    } else {
        printf("classification_candidate=M10_6_LOSS_ACCOUNTING_MISMATCH\n");
        rc = 3;
    }

out:
    if (child > 0) { kill(child, SIGKILL); waitpid(child, NULL, 0); }
    ring_buffer__free(rb);
    loss_authority_bpf__destroy(skel);
    return rc;
}
