#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <bpf/bpf.h>
#include <bpf/libbpf.h>
#include "bpf_lsm_exec.skel.h"

#define SESSION_ID UINT64_C(0x4d31303400000001)
#define ROLE_SUCCESS 1u
#define ROLE_FAIL 2u
#define KIND_LSM_CANDIDATE 1u
#define KIND_EXEC_SUCCESS 2u

struct target_role { uint64_t session_id; uint32_t role; uint32_t _pad; };
struct exec_event { uint64_t session_id, dev, ino; uint32_t tgid, tid, role, kind; };
struct truth { uint64_t dev, ino; int ok; };
struct role_obs { unsigned candidates, successes, malformed; struct exec_event last_candidate, success; };
struct observations { struct role_obs success_role, fail_role; unsigned wrong_session, unknown_role; };

static int on_event(void *ctx, void *data, size_t len)
{
    struct observations *o=ctx; const struct exec_event *e=data; struct role_obs *r;
    if (len!=sizeof(*e)) { o->success_role.malformed++; return 0; }
    if (e->session_id!=SESSION_ID) { o->wrong_session++; return 0; }
    if (e->role==ROLE_SUCCESS) r=&o->success_role;
    else if (e->role==ROLE_FAIL) r=&o->fail_role;
    else { o->unknown_role++; return 0; }
    if (e->kind==KIND_LSM_CANDIDATE) { r->candidates++; r->last_candidate=*e; }
    else if (e->kind==KIND_EXEC_SUCCESS) { r->successes++; r->success=*e; }
    else r->malformed++;
    return 0;
}

static int register_role(struct bpf_lsm_exec_bpf *skel,pid_t pid,uint32_t role)
{
    uint32_t key=(uint32_t)pid; struct target_role v={.session_id=SESSION_ID,.role=role,._pad=0};
    return bpf_map_update_elem(bpf_map__fd(skel->maps.targets),&key,&v,BPF_ANY);
}

static int wait_stopped(pid_t pid)
{
    int st=0; if (waitpid(pid,&st,WUNTRACED)!=pid) return -1; return WIFSTOPPED(st)?0:-1;
}

static int drain_until_exit(struct ring_buffer *rb,pid_t pid,int *status)
{
    for (;;) {
        int p=ring_buffer__poll(rb,20); if (p<0 && p!=-EINTR) return -1;
        pid_t w=waitpid(pid,status,WNOHANG); if (w==pid) break; if (w<0) return -1;
    }
    for (int i=0;i<10;i++) { int p=ring_buffer__poll(rb,10); if (p<0 && p!=-EINTR) return -1; }
    return 0;
}

static void close_pair(int pfd[2])
{
    if (pfd[0]>=0) close(pfd[0]);
    if (pfd[1]>=0) close(pfd[1]);
}

static int run_success(struct bpf_lsm_exec_bpf *skel,struct ring_buffer *rb,struct truth *gt,int *child_exit)
{
    int pfd[2]={-1,-1},st=0; pid_t child=-1;
    if (pipe(pfd)) return -1;
    child=fork(); if (child<0) { close_pair(pfd); return -1; }
    if (child==0) {
        close(pfd[0]); raise(SIGSTOP);
        struct truth t={0}; struct stat sb; int fd=open("/bin/busybox",O_PATH|O_CLOEXEC);
        if (fd>=0 && fstat(fd,&sb)==0) { t.dev=(uint64_t)sb.st_dev; t.ino=(uint64_t)sb.st_ino; t.ok=1; }
        if (write(pfd[1],&t,sizeof(t))!=(ssize_t)sizeof(t)) _exit(41);
        if (!t.ok) _exit(42);
        char *const argv[]={(char *)"true",NULL}; char *const envp[]={NULL};
        syscall(SYS_execveat,fd,"",argv,envp,AT_EMPTY_PATH); _exit(43);
    }
    close(pfd[1]); pfd[1]=-1;
    if (wait_stopped(child)||register_role(skel,child,ROLE_SUCCESS)||kill(child,SIGCONT)) goto fail;
    if (drain_until_exit(rb,child,&st)) goto fail; child=-1;
    if (read(pfd[0],gt,sizeof(*gt))!=(ssize_t)sizeof(*gt)) goto fail;
    *child_exit=WIFEXITED(st)?WEXITSTATUS(st):-1; close(pfd[0]); return 0;
fail:
    if (child>0) { kill(child,SIGKILL); waitpid(child,NULL,0); }
    close_pair(pfd); return -1;
}

static int run_failure(struct bpf_lsm_exec_bpf *skel,struct ring_buffer *rb,int *child_exit,int *expected_errno)
{
    int pfd[2]={-1,-1},st=0; pid_t child=-1;
    if (pipe(pfd)) return -1;
    child=fork(); if (child<0) { close_pair(pfd); return -1; }
    if (child==0) {
        close(pfd[0]); raise(SIGSTOP);
        char *const argv[]={(char *)"missing",NULL}; char *const envp[]={NULL};
        execve("/definitely-not-present-m10-4",argv,envp);
        int e=errno; if (write(pfd[1],&e,sizeof(e))!=(ssize_t)sizeof(e)) _exit(51); _exit(e==ENOENT?0:52);
    }
    close(pfd[1]); pfd[1]=-1;
    if (wait_stopped(child)||register_role(skel,child,ROLE_FAIL)||kill(child,SIGCONT)) goto fail;
    if (drain_until_exit(rb,child,&st)) goto fail; child=-1;
    if (read(pfd[0],expected_errno,sizeof(*expected_errno))!=(ssize_t)sizeof(*expected_errno)) goto fail;
    *child_exit=WIFEXITED(st)?WEXITSTATUS(st):-1; close(pfd[0]); return 0;
fail:
    if (child>0) { kill(child,SIGKILL); waitpid(child,NULL,0); }
    close_pair(pfd); return -1;
}

int main(void)
{
    struct bpf_lsm_exec_bpf *skel=NULL; struct ring_buffer *rb=NULL; struct observations obs={0}; struct truth gt={0};
    int success_exit=-1,fail_exit=-1,fail_errno=0,rc=1; uint32_t zero=0; uint64_t drops=0;
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    skel=bpf_lsm_exec_bpf__open_and_load(); if (!skel) { fprintf(stderr,"stage=open_and_load errno=%d\n",errno); return 42; }
    if (bpf_lsm_exec_bpf__attach(skel)) { fprintf(stderr,"stage=attach errno=%d\n",errno); rc=42; goto out; }
    rb=ring_buffer__new(bpf_map__fd(skel->maps.events),on_event,&obs,NULL); if (!rb) goto out;
    if (run_success(skel,rb,&gt,&success_exit)) goto out;
    if (run_failure(skel,rb,&fail_exit,&fail_errno)) goto out;
    if (bpf_map_lookup_elem(bpf_map__fd(skel->maps.drops),&zero,&drops)) goto out;

    int cand_match=obs.success_role.candidates>0 && obs.success_role.last_candidate.dev==gt.dev && obs.success_role.last_candidate.ino==gt.ino;
    int success_match=obs.success_role.successes==1 && obs.success_role.success.dev==gt.dev && obs.success_role.success.ino==gt.ino;
    int health=obs.success_role.malformed==0 && obs.fail_role.malformed==0 && obs.wrong_session==0 && obs.unknown_role==0 && drops==0;
    printf("ground_dev=%llu ground_ino=%llu ground_ok=%d\n",(unsigned long long)gt.dev,(unsigned long long)gt.ino,gt.ok);
    printf("success_candidates=%u success_confirms=%u candidate_dev=%llu candidate_ino=%llu success_dev=%llu success_ino=%llu\n",obs.success_role.candidates,obs.success_role.successes,(unsigned long long)obs.success_role.last_candidate.dev,(unsigned long long)obs.success_role.last_candidate.ino,(unsigned long long)obs.success_role.success.dev,(unsigned long long)obs.success_role.success.ino);
    printf("fail_candidates=%u fail_confirms=%u fail_errno=%d\n",obs.fail_role.candidates,obs.fail_role.successes,fail_errno);
    printf("success_exit=%d fail_exit=%d malformed=%u wrong_session=%u unknown_role=%u drops=%llu\n",success_exit,fail_exit,obs.success_role.malformed+obs.fail_role.malformed,obs.wrong_session,obs.unknown_role,(unsigned long long)drops);

    if (fail_exit==0 && fail_errno==ENOENT && obs.fail_role.successes>0) { printf("classification_candidate=M10_4_FAILED_EXEC_FALSE_PROMOTION\n"); rc=4; }
    else if (success_exit==0 && fail_exit==0 && gt.ok && health && obs.success_role.successes==1 && obs.fail_role.successes==0 && cand_match && success_match) { printf("classification_candidate=M10_4_HYBRID_EXEC_MATCH\n"); rc=0; }
    else if (success_exit==0 && gt.ok && health && obs.success_role.successes==1 && !success_match) { printf("classification_candidate=M10_4_EXEC_OBJECT_MISMATCH\n"); rc=5; }
    else { printf("classification_candidate=M10_4_EVIDENCE_INCOMPLETE\n"); rc=3; }
out:
    ring_buffer__free(rb); bpf_lsm_exec_bpf__destroy(skel); return rc;
}
