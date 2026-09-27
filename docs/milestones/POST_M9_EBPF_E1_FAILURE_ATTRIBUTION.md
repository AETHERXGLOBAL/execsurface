# Post-M9 — eBPF E1 Failure Attribution

Date: 2026-09-27
Status: **E1 OPEN — STATIC ATTRIBUTION COMPLETE / LIVE DIAGNOSTIC NEXT**
Tracking: GitHub issue #85
Source-of-truth HEAD at attribution start: `2eceb9409271f8464120ab26956e20194e8c6f8d`

## Authority boundary

Unchanged throughout E1:

- native ptrace is the public/default correctness-reference backend;
- eBPF remains research-only;
- eBPF `learn`, `check`, PASS authority, backend auto-selection and ptrace/eBPF baseline interchangeability remain unauthorized;
- no public performance claim is authorized;
- no incomplete state may be relabeled clean;
- no collector semantic change is authorized until E1 attributes the blocking lifecycle failure.

## Question

M9.1 crossed the real-workload trigger for renewed eBPF value research but the frozen value screen stopped before measured samples because the experimental libbpf collector sometimes returned `incomplete_lifecycle`.

E1 separates three questions that must not be conflated:

1. Is `incomplete_capability` itself blocking the M9.1 value screen?
2. Why does `lifecycle_drain_complete` become false on the pinned real workloads?
3. Does the M9.1 value path exercise the lifecycle architecture that M8.7 later proved, or an older collector path?

## Finding E1-F1 — `incomplete_capability` is not the blocking failure

**PROVED from the frozen harness and retained M9.1 artifact.**

`scripts/m9_1_ebpf_value_screen.py` defines the hard-incomplete set as:

- `incomplete_loss`;
- `incomplete_limit`;
- `incomplete_decode`;
- `incomplete_collector`;
- `incomplete_lifecycle`.

`incomplete_capability` is intentionally absent from that hard-failure set. A libbpf sample is clean only when:

- the collector exits zero;
- completeness is not hard-incomplete;
- dropped events are zero;
- `lifecycle_drain_complete == true`;
- collector failure is absent.

The preserved fzf artifact proves this path was exercised: libbpf warmup 1 reported `completeness=incomplete_capability`, `lifecycle_drain_complete=true`, `dropped_events=0`, and the harness recorded `clean=true`.

Therefore broadening eBPF capabilities is **not required to unblock E1 value measurement**. Capability expansion remains a separate semantic/product problem.

## Finding E1-F2 — the actual blocker is lifecycle drain timeout

**PROVED from retained M9.1 artifacts and current collector source.**

### `casey/just`

Pinned workload:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

First libbpf warmup:

- collector return code: `0`;
- dropped events: `0`;
- event count: `52813`;
- `completeness=incomplete_lifecycle`;
- `lifecycle_drain_complete=false`;
- wall time: `6911.697939 ms`;
- harness status: `PARTIAL_NO_VALUE_CLAIM`.

### `junegunn/fzf`

Pinned workload:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Warmup 1:

- event count `4289`;
- `incomplete_capability`;
- lifecycle drain complete;
- clean `true`;
- wall time `1072.189955 ms`.

Warmup 2:

- event count `4293`;
- `incomplete_lifecycle`;
- lifecycle drain incomplete;
- dropped events `0`;
- wall time `3005.291339 ms`;
- harness stopped before measured samples.

The current `libbpf-observer` default lifecycle timeout is exactly `2000 ms`. Its post-root drain returns `incomplete_lifecycle` when the internal active-descendant set does not reach zero before that deadline.

The approximate extra two seconds in the failing runs is therefore consistent with the explicit fail-closed timeout path, not with producer loss or target failure.

## Finding E1-F3 — M9.1 value screen used the older M8.4d per-invocation collector

**PROVED.**

`.github/workflows/m9-1-ebpf-value-screen.yml` builds and invokes:

`experiments/m8-ebpf/libbpf-observer`

The collector identifies itself as `m8.4d-experimental-v1` and uses userspace membership tracking around globally attached BPF programs.

It does **not** use:

`experiments/m8-ebpf/persistent-observer`

and therefore does not exercise the M8.7 session-epoch/root-registration architecture during the M9.1 value screen.

## Finding E1-F4 — M8.7 already rejected the old child-membership propagation boundary

**PROVED design divergence.**

The current M8.4d BPF collector emits child membership from successful `sys_exit_fork`, `sys_exit_vfork`, `sys_exit_clone`, and `sys_exit_clone3`. Userspace then learns the child and drains any events that arrived while the child identity was still unknown.

M8.7 explicitly killed syscall-exit membership propagation because the child can be runnable and emit evidence before the parent's syscall-return hook. The accepted persistent architecture instead propagates the session epoch at `tracepoint/sched/sched_process_fork`, before the child contributes accepted session events.

M8.7 also adds:

- kernel-side `task_epoch` membership;
- non-zero session epochs;
- root-registration barrier before workload release;
- explicit routing-error accounting;
- kernel map cleanup checks;
- fail-closed stale-epoch rejection.

This proves that the M9.1 value screen was measuring a lifecycle path that the repository had already superseded for persistent-session lifecycle correctness.

It does **not**, by itself, prove that the old syscall-exit race caused each M9.1 timeout.

## Remaining E1 uncertainty

**OPEN:** on each observed `incomplete_lifecycle` run, did the old collector time out because:

A. one or more descendants were genuinely still alive after the root command exited; or

B. the userspace active set contained stale task identities whose kernel tasks were already gone; or

C. a mixture of both.

This distinction matters. Increasing the timeout would be invalid if membership is stale, while changing membership logic would be unnecessary if the workload intentionally leaves real descendants alive beyond the declared observation boundary.

## Live diagnostic protocol — preregistered before execution

The next E1 run may add **diagnostic-only instrumentation inside the CI runner build**. The committed public collector source and semantics must remain unchanged.

Frozen targets and commands:

1. `casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
   - `cargo +1.90.0 test --all >/dev/null 2>&1`
2. `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`
   - `go test ./... >/dev/null 2>&1`

For three libbpf per-invocation repetitions per target, preserve:

- default `2000 ms` lifecycle timeout;
- default event limit;
- no consumer lag injection;
- no loss/decode/poll fault injection;
- same target command and revision;
- collector exit/report unchanged for classification.

If and only if the normal lifecycle timeout branch is reached, the runner-only diagnostic build may emit metadata-only diagnostics for each task identity still present in the internal active set:

- TID;
- whether `/proc/<tid>` still exists;
- `Pid`, `Tgid`, `PPid`, and process state from `/proc/<tid>/status` when available;
- total known/active/pending counts.

It must not capture command lines, environment values, file contents, stdin, network payloads, secrets, or unrestricted argv.

### E1 attribution rules

- active-set TID absent from `/proc` at timeout => **stale membership evidence**;
- active-set TID still present in `/proc` at timeout => **live descendant evidence** for that identity at that instant;
- both classes present => mixed failure;
- no lifecycle timeout => retain as a clean control; do not force failure;
- no timeout threshold change after observing results.

No runtime code fix and no performance value measurement is authorized by this diagnostic run.

## Next decision

After the diagnostic evidence:

- if stale membership is established, E2 should port the already-proved M8.7 propagation/session discipline into a bounded value-requalification collector rather than extending the killed syscall-exit membership design;
- if only genuinely live descendants are established, E2 must first define the correct session ownership/drain boundary without merely increasing timeouts to manufacture clean evidence;
- if mixed, both boundaries must be addressed fail-closed before E3 parity/health.
