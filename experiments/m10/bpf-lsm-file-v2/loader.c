#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include <bpf/bpf.h>
#include <bpf/libbpf.h>
#include "bpf_lsm_file_v2.skel.h"

#define SESSION_ID UINT64_C(0x4d31303344000001)

struct file_event { uint64_t session_id, dev, ino; uint32_t tgid, tid; };
struct truth { uint64_t dev, ino; int ok; };
struct observed { struct file_event first; unsigned total, wrong_session, malformed; };

static int on_event(void *ctx, void *data, size_t len)
{
    struct observed *o = ctx;
    const struct file_event *e = data;
    if (len != sizeof(*e)) { o->malformed++; return 0; }
    if (e->session_id != SESSION_ID) { o->wrong_session++; return 0; }
    if (o->total == 0) o->first = *e;
    o->total++;
    return 0;
}

static int make_file(const char *path)
{
    int fd = open(path, O_CREAT|O_TRUNC|O_WRONLY|O_CLOEXEC, 0600);
    if (fd < 0) return -1;
    char x='X'; int ok=write(fd,&x,1)==1 ? 0 : -1; int e=errno;
    close(fd); errno=e; return ok;
}

int main(void)
{
    char dir[]="/tmp/execsurface-m10-3d-XXXXXX", path[512];
    int pfd[2]={-1,-1}, status=0, rc=1;
    pid_t child=-1;
    struct bpf_lsm_file_v2_bpf *skel=NULL;
    struct ring_buffer *rb=NULL;
    struct truth gt={0}; struct observed obs={0};
    uint32_t zero=0, tgid=0; uint64_t drops=0, session=SESSION_ID;

    if (!mkdtemp(dir)) return 2;
    snprintf(path,sizeof(path),"%s/target",dir);
    if (make_file(path) || pipe(pfd)) goto out;

    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    skel=bpf_lsm_file_v2_bpf__open_and_load();
    if (!skel) { fprintf(stderr,"stage=open_and_load errno=%d\n",errno); rc=42; goto out; }
    if (bpf_lsm_file_v2_bpf__attach(skel)) { fprintf(stderr,"stage=attach errno=%d\n",errno); rc=42; goto out; }
    rb=ring_buffer__new(bpf_map__fd(skel->maps.events),on_event,&obs,NULL);
    if (!rb) goto out;

    child=fork();
    if (child<0) goto out;
    if (child==0) {
        close(pfd[0]);
        raise(SIGSTOP);
        struct truth t={0}; struct stat st;
        int fd=open(path,O_RDONLY|O_CLOEXEC);
        if (fd>=0 && fstat(fd,&st)==0) { t.dev=(uint64_t)st.st_dev; t.ino=(uint64_t)st.st_ino; t.ok=1; }
        if (write(pfd[1],&t,sizeof(t))!=(ssize_t)sizeof(t)) _exit(31);
        if (fd>=0) close(fd);
        _exit(t.ok?0:32);
    }
    close(pfd[1]); pfd[1]=-1;

    if (waitpid(child,&status,WUNTRACED)!=child || !WIFSTOPPED(status)) goto out;
    tgid=(uint32_t)child;
    if (bpf_map_update_elem(bpf_map__fd(skel->maps.target_tgids),&tgid,&session,BPF_ANY)) goto out;
    if (kill(child,SIGCONT)) goto out;

    for (;;) {
        int p=ring_buffer__poll(rb,20);
        if (p<0 && p!=-EINTR) goto out;
        pid_t w=waitpid(child,&status,WNOHANG);
        if (w==child) break;
        if (w<0) goto out;
    }
    child=-1;
    for (int i=0;i<10;i++) ring_buffer__poll(rb,10);

    if (read(pfd[0],&gt,sizeof(gt))!=(ssize_t)sizeof(gt)) goto out;
    if (bpf_map_lookup_elem(bpf_map__fd(skel->maps.drops),&zero,&drops)) goto out;

    int target_exit=WIFEXITED(status)?WEXITSTATUS(status):-1;
    int identity_match=obs.total==1 && gt.ok && obs.first.dev==gt.dev && obs.first.ino==gt.ino;
    printf("ground_dev=%llu ground_ino=%llu ground_ok=%d\n",(unsigned long long)gt.dev,(unsigned long long)gt.ino,gt.ok);
    printf("event_dev=%llu event_ino=%llu event_tgid=%u events_total=%u malformed=%u wrong_session=%u drops=%llu\n",
        (unsigned long long)obs.first.dev,(unsigned long long)obs.first.ino,obs.first.tgid,
        obs.total,obs.malformed,obs.wrong_session,(unsigned long long)drops);
    printf("target_exit=%d identity_match=%d\n",target_exit,identity_match);

    if (target_exit==0 && gt.ok && obs.total==1 && obs.malformed==0 && obs.wrong_session==0 && drops==0) {
        if (identity_match) { printf("classification_candidate=M10_3_OBJECT_IDENTITY_MATCH\n"); rc=0; }
        else { printf("classification_candidate=M10_3_OBJECT_IDENTITY_MISMATCH\n"); rc=4; }
    } else {
        printf("classification_candidate=M10_3_EVIDENCE_INCOMPLETE\n"); rc=3;
    }
out:
    if (child>0) { kill(child,SIGKILL); waitpid(child,NULL,0); }
    if (pfd[0]>=0) close(pfd[0]);
    if (pfd[1]>=0) close(pfd[1]);
    ring_buffer__free(rb); bpf_lsm_file_v2_bpf__destroy(skel);
    unlink(path); rmdir(dir); return rc;
}
