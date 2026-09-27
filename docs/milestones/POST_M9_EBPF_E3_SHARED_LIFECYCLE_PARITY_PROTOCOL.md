# Post-M9 — eBPF E3 Shared Lifecycle Proposition Parity Protocol

Date: 2026-09-27
Status: **PREREGISTERED — NOT YET EXECUTED**
Tracking: GitHub issue #85
Baseline HEAD: `d71284de06ac7f50ffcc163ab2c09a01cc408f71`

## Objective

Test only the lifecycle propositions that are genuinely shared between:

- the public native ptrace correctness-reference backend; and
- the unchanged M8.7 persistent eBPF lifecycle architecture.

E3 explicitly does **not** test full runtime-surface equivalence. File, FD, path, network and other ptrace-only propositions remain outside this gate.

Performance timing remains blocked until E3 passes.

## Why a controlled fixture is required

The pinned real workloads used in E2 are concurrent and produce thousands of lifecycle events. Equal aggregate counts are neither required nor sufficient for semantic parity.

E3 therefore uses a deterministic lifecycle fixture with a prospectively frozen process topology so the comparison can be made proposition-by-proposition rather than by raw event count.

## Frozen fixture topology

A runner-local C fixture supports two launch modes:

- `direct` — used by the ptrace reference;
- `barrier <fd>` — used by the unchanged M8.7 persistent observer so root epoch membership is installed before the workload is released.

After launch/release, both modes execute the same workload logic:

1. root creates child `success_child` with `fork()`;
2. `success_child` successfully `execve`s `/bin/true` and exits `0`;
3. root waits for `success_child` and requires clean exit;
4. root creates child `failed_exec_child` with `fork()`;
5. `failed_exec_child` attempts `execve` of the guaranteed-missing path `/__execsurface_post_m9_e3_missing__/exec-probe`;
6. that exec must fail with `ENOENT`; the child then exits `0`;
7. root waits for `failed_exec_child` and requires clean exit;
8. root exits `0`.

No threads, vfork, clone3, file payload assertions or network behavior are part of E3.

## Shared propositions

E3 compares the following topology-only propositions after projecting away backend-specific numeric TIDs and root-launch representation:

### P1 — child creation topology

Both backends must establish exactly two root-created child roles in workload order:

- first child: `success_child`;
- second child: `failed_exec_child`.

The eBPF persistent transcript must preserve parent -> child identity well enough to assign both children to the active root session.

### P2 — successful child exec occurrence

The first spawned child must produce exactly one successful exec occurrence after its spawn.

Path identity is **not** compared in this gate because the M8.7 persistent lifecycle event is occurrence-only.

### P3 — failed exec is not promoted

The second spawned child must produce **zero** successful exec occurrences after its spawn.

A failed `execve` syscall must not be promoted to a process-exec occurrence by either backend.

### P4 — workload outcome

Both workload executions must exit `0` without signal.

### P5 — eBPF lifecycle health

The persistent session used for parity must retain the E2/M8.7 health invariants:

- lifecycle drain complete;
- active remaining `0`;
- stale epoch events `0`;
- integrity errors `0`;
- decode errors `0`;
- producer drops `0`;
- routing errors `0`;
- membership maps empty;
- event limit not hit;
- authority block unchanged.

## Reference evidence

The ptrace side uses the ordinary public:

```text
execsurface observe -- <fixture> direct
```

The projection consumes only:

- `process_spawn` raw events (`event.tid`, `child_tid`);
- `process_exec` raw events (`event.tid`);
- command outcome and health.

Other ptrace events are retained in the raw artifact but ignored for this E3 proposition gate.

## Persistent eBPF evidence

The committed M8.7 BPF program and lifecycle semantics remain unchanged.

Because the normal M8.7 JSON report retains aggregate lifecycle counts but not the accepted event transcript, the CI runner may apply **evidence-only userspace instrumentation** to a temporary build of `persistent-observer/src/main.rs` that records, for accepted current-epoch events only:

- event sequence within the session;
- event kind (`spawn`, `exec`, `exit`);
- event TID;
- TGID;
- spawn child TID (`value`) for spawn events.

The instrumentation may not:

- modify the BPF program;
- change event acceptance/rejection;
- change tracker state transitions;
- change lifecycle timeout, polling, event limits or authority;
- alter the target process;
- add new canonicalization/policy semantics.

The exact runner-only patch must be preserved as evidence.

## Execution shape

Run on one GitHub-hosted Ubuntu 24.04 job:

1. build the public ptrace CLI unchanged;
2. build the deterministic fixture;
3. run ptrace reference **twice** and require reference/reference proposition determinism before comparing eBPF;
4. build the M8.7 persistent observer with transcript-only runner instrumentation;
5. execute one persistent-observer invocation (which naturally contains 2 distinct-epoch sessions);
6. require both persistent sessions independently satisfy P1–P5;
7. compare each eBPF session projection against the deterministic ptrace proposition vector.

No failed run/session may be replaced or retried.

## Acceptance

E3 PASS requires:

- ptrace run 1 complete, warning-free, exit `0`;
- ptrace run 2 complete, warning-free, exit `0`;
- ptrace proposition projections exactly equal to each other;
- ptrace projection satisfies P1–P4;
- persistent invocation exits `0` and has exactly 2 sessions;
- both persistent sessions satisfy P1–P5;
- both persistent proposition projections equal the ptrace proposition vector for P1–P4;
- `full_surface_comparable` remains explicitly `false`;
- `ebpf_pass_authorized` remains explicitly `false`;
- `product_integration_authorized` remains explicitly `false`.

Any mismatch is preserved as a contradiction/failure. No projection rule may be changed after viewing results.

## What E3 PASS would mean

Only:

> For this deterministic lifecycle fixture, the public ptrace reference and the M8.7 persistent eBPF architecture agree on the preregistered shared child-spawn and successful-exec occurrence propositions, while the eBPF session also remains lifecycle-health complete.

It does not establish full backend equivalence or public eBPF authority.

## Next gate if E3 passes

E4 may then preregister same-workload performance requalification on the pinned `just` and `fzf` workloads, with lifecycle/health prerequisites before any timing sample is accepted.
