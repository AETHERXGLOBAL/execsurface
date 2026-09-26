# M8.7 — Persistent eBPF Observer Session Architecture

Date: 2026-09-26
Status: **CLOSED / ACCEPTED — BOUNDED PERSISTENT SESSION ARCHITECTURE FEASIBLE**
Tracking: #46
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
Motivation: `docs/milestones/M8_6D_POST_TARGET_LATENCY.md`
Red Team: `docs/milestones/M8_7_RED_TEAM_REVIEW.md`
Next: **M8.8 — controlled public-alpha integration decision**

## Decision

M8.7 establishes a bounded architecture result:

> A single-active-session persistent libbpf observer can keep BPF programs attached across sequential controlled sessions while preserving explicit session attribution, fail-closed loss/state handling, descendant lifecycle drain, deterministic reset checks, and the existing authority boundary.

This is an architecture/feasibility result only.

M8.7 does **not** authorize:

- default or automatic eBPF backend selection;
- eBPF `learn` or `check` authority;
- eBPF PASS authority;
- cross-backend baseline interchangeability;
- full-surface semantic equivalence;
- concurrent/multi-tenant observation;
- a production daemon/service;
- universal Linux compatibility or performance claims.

`ptrace` remains the correctness reference.

## Governance team

### Fixed — Innovation Scientist / Architect

- find the smallest persistent lifecycle that removes redundant attach/detach work;
- keep collection mechanism separate from evidence meaning;
- prefer kernel-side session filtering over accepting ambient host telemetry;
- preserve an upgrade path without prematurely creating a production daemon.

### Fixed — Deviation Prevention / Scientific Integrity

- block benchmark-only timer reductions;
- block stale-event acceptance or PID-only attribution;
- block ambiguous loss/routing state;
- block silent privilege expansion;
- retain counterexamples and killed designs.

### Independent Red Team role

Attacked epoch isolation, PID/TID churn, delayed events, descendants after root exit, kernel-map residue, producer loss, routing failure, event-budget exhaustion, crash/restart, concurrent-session attempts, root registration/release failure, privilege lifetime, and performance claim boundaries.

## Accepted architecture

```text
Persistent privileged collector
        |
        +-- open/load/attach BPF once
        +-- global ring buffer
        +-- task_epoch: TID -> non-zero u64 epoch
        +-- monotonic producer-drop and routing-error counters
        |
        +-- single active session controller
                |
                +-- allocate monotonic epoch
                +-- spawn bootstrap target
                +-- obtain stable root PID/TID
                +-- register root TID -> epoch
                +-- arm userspace tracker
                +-- release bootstrap barrier
                +-- exec workload using same root PID
                +-- propagate epoch at task creation
                +-- accept only matching-epoch events
                +-- drain root + descendants fail-closed
                +-- snapshot health/loss deltas
                +-- require kernel/session state cleanup
        |
        +-- detach only when collector lifetime ends
```

## Session attribution contract

- epoch `0` means no session;
- epochs are non-zero and not reused within one collector lifetime;
- every accepted event carries its epoch;
- mismatched/stale epoch evidence is rejected before active-session mutation;
- task membership is keyed by Linux TID, not collapsed to TGID;
- collector restart creates a new authority lifetime;
- 64-bit epoch exhaustion requires restart rather than reuse.

## Descendant propagation

### Accepted — ordinary `sched_process_fork` tracepoint

The prototype records spawn mechanism at syscall entry, but membership propagation occurs at `tracepoint/sched/sched_process_fork`:

1. read `parent_pid` and `child_pid` from the tracepoint payload;
2. look up parent TID epoch;
3. insert child TID with the same epoch;
4. only then emit the accepted spawn record;
5. task exit emits an epoch-tagged exit event and removes task membership.

The workflow audits the live raw tracepoint layout before using it as evidence. The proved runner layout was:

- `parent_pid`: offset 12, size 4;
- `child_pid`: offset 20, size 4.

### KILLED — syscall-exit membership propagation

Rejected because the child may run and emit observable events before the parent's syscall-return hook executes.

### KILLED — `tp_btf` + direct `task_struct` under Apache-2.0

The verifier required GPL-compatible BPF licensing for the tested direct `task_struct` access path. M8.7 retained Apache-2.0 and selected the ordinary tracepoint payload instead.

## Root-registration barrier

Accepted bootstrap-exec sequence:

1. parent creates a pipe;
2. Rust `pre_exec` performs only non-blocking setup;
3. the first exec starts the controlled fixture in barrier mode;
4. `Command::spawn()` returns with the stable root PID/TID;
5. parent writes `root_tid -> epoch` and arms the tracker;
6. parent releases the barrier;
7. bootstrap execs itself into workload mode using the same PID;
8. workload then creates descendants.

Launch or release failure cannot become clean evidence.

### KILLED — blocking wait in `pre_exec`

Rejected after it deadlocked with Rust `Command::spawn()` exec synchronization.

## Loss and state-error accounting

M8.4 remains authoritative. Persistent mode adds session-aware routing-state checks.

Kernel-side monotonic counters cover:

- ring-buffer producer drops;
- task-epoch routing/map-state failures.

Userspace snapshots the counters around each session. Any unexpected delta prevents a clean session. Counters are not reset merely to manufacture a clean next session.

Event-budget exhaustion, malformed records, stale epochs, lifecycle timeouts, map residue, decode failures, and routing ambiguity also prevent clean evidence.

## M8.7a — persistent epoch feasibility

**Status: CLOSED / PROVED — bounded feasibility.**

Closing evidence:

- M8.7 workflow `36262244488`: **SUCCESS**;
- general CI `36262244518`: **SUCCESS**;
- live tracepoint ABI audit: **PASS**;
- two sequential sessions under one attachment;
- epochs `[1, 2]` with distinct roots;
- no stale/drop/routing/decode errors;
- complete descendant drain;
- membership and pending-mechanism maps empty after clean sessions;
- no accepted events while no userspace session was active;
- one final detach only;
- authority remained false for eBPF PASS/product integration.

Representative closing-run timing from that feasibility run:

- open `0.107031 ms`;
- load `1.335743 ms`;
- attach `0.949341 ms`;
- session 1 `111.967298 ms`;
- session 2 `112.000849 ms`;
- final detach `706.846054 ms`.

These numbers were feasibility observations, not a performance conclusion.

## M8.7b — adversarial session isolation

**Status: CLOSED / PROVED FOR THE TESTED FAILURE CLASSES.**

The adversarial program established fail-closed behavior for:

- 64 descendants continuing after root exit, with session drain waiting for the propagated descendants;
- concurrent-session attempt while one session is active;
- synthetic stale epoch before session-state mutation;
- malformed wire records;
- unknown event kind;
- target launch failure;
- root barrier release failure;
- real kernel routing-state failure using an isolated constrained task-map build;
- real producer ring-buffer loss using an isolated constrained ring-buffer build;
- event-budget exhaustion;
- collector crash/restart isolation;
- live stale-epoch ring-buffer evidence using an isolated fault build.

Key successful workflows:

| Gate | Workflow run | Result |
|---|---:|---|
| state-machine + 64-descendant post-root drain | `36264064989` | SUCCESS |
| real routing failure | `36264070921` | SUCCESS |
| producer loss | `36264199739` | SUCCESS |
| event budget | `36264339646` | SUCCESS |
| crash / restart | `36263425308` | SUCCESS |
| live stale epoch | `36264999265` | SUCCESS |

The stale-epoch closing HEAD `ce9c390a24aded478bc90a21c0abd8bc32d4080f` also had general CI `36264999252`: **SUCCESS**.

### Fault-build isolation

Fault injection remains compile-time opt-in in the isolated experiment only:

- `M8_7_FAULT_SMALL_TASK_MAP`;
- `M8_7_FAULT_SMALL_RINGBUF`;
- `M8_7_FAULT_STALE_EPOCH`.

Default experimental builds do not define these macros. Default values remain:

- ring buffer: `65536` bytes;
- task epoch map: `4096` entries.

No runtime hidden fault mode is accepted for public productization.

## M8.7c — persistent performance attribution

**Status: CLOSED / MEASURED / ACCEPTED FOR EXACT-HOST EVIDENCE ONLY.**

The protocol was frozen before measurement.

Evidence:

- performance commit: `bf0bac374ef0fefef438b6dc70e89b8c2b42963d`;
- general CI `36264634501`: **SUCCESS**;
- M8.7 Persistent Performance `36264634525`: **SUCCESS**;
- artifact ID `10914040205`;
- artifact ZIP SHA-256 `d26ae806bd6dbc0a437b1ab95434d73d99e787c614b93042babc9aa7f063caee`.

Exact measured host:

- Ubuntu 24.04.5 LTS;
- GitHub runner image `20260920.314.1`;
- kernel `6.17.0-1022-azure`;
- x86_64;
- 4 logical CPUs;
- readable BTF.

Every timed sample passed its evidence-health gates; no timed sample was removed.

| Mode | Samples | Median ms | MAD ms | p90 ms |
|---|---:|---:|---:|---:|
| direct | 15 | 2.858403 | 0.021842 | 2.948975 |
| ptrace | 15 | 10.020306 | 0.218622 | 11.281625 |
| libbpf per invocation | 15 | 607.278495 | 7.848262 | 625.856694 |
| persistent two-session amortized | 15 paired invocations | 505.770932 | 2.667380 | 513.786955 |
| persistent internal session | 30 sessions | 113.918489 | 0.075004 | 114.174884 |

Measured ratios:

- persistent amortized / per-invocation libbpf: `0.832848`;
- persistent internal / per-invocation libbpf: `0.187589`;
- persistent amortized / ptrace: `50.474599`;
- per-invocation libbpf / ptrace: `60.604785`.

Bounded interpretation:

- persistent attachment reduced the measured two-session amortized median by about **16.7%** versus the current per-invocation libbpf path on this host/workload;
- internal persistent session cost was about **81.2% lower** than per-invocation libbpf, confirming that one-time lifecycle cost was separated from session work;
- the conservative persistent end-to-end result still remained about **50.5x ptrace**;
- an additional per-session latency floor of about `113.9 ms` remains.

Therefore M8.7c supports the architectural amortization claim only. It does not support selecting eBPF over ptrace on performance grounds.

## M8.7d — Red Team closure

**Status: CLOSED / ACCEPT.**

See `docs/milestones/M8_7_RED_TEAM_REVIEW.md`.

The Red Team accepts the bounded persistent-session architecture as a valid future research/productization direction because:

- session ownership is explicit;
- stale evidence is rejected;
- descendant tracking survives root exit;
- loss/truncation/routing ambiguity fail closed;
- launch and crash boundaries are explicit;
- fault mechanisms are isolated from default builds;
- Apache-2.0 was preserved rather than weakened for a BPF implementation shortcut;
- performance claims remain narrow and do not override correctness.

The Red Team does **not** accept production deployment or authority promotion.

## Remaining open boundaries after M8.7

These are intentionally not hidden by closure:

- production privilege separation and long-lived service hardening;
- universal kernel/tracepoint compatibility;
- broader semantic surface beyond the proved capability subset;
- concurrent/multi-tenant session isolation;
- the ~113.9 ms measured internal-session latency floor;
- public packaging/update/service lifecycle for a persistent observer;
- any future proof needed for pinned maps/links across collector failure.

## Final authority invariants

At M8.7 closure:

- ptrace correctness reference: **RETAINED**;
- eBPF full-surface comparability: **FALSE**;
- eBPF PASS authority: **NOT AUTHORIZED**;
- eBPF `learn` / `check`: **NOT AUTHORIZED**;
- backend auto-selection: **NOT AUTHORIZED**;
- cross-backend baseline interchangeability: **NOT AUTHORIZED**;
- production daemon/service: **NOT AUTHORIZED**;
- default public install path: **UNCHANGED**.

## Next gate

**M8.8 — controlled public-alpha integration decision.**

M8.8 must begin from merged `main`, not from this unmerged feasibility branch. It must first decide whether any public exposure is justified despite the measured performance gap and remaining privilege/service boundaries. If exposure is pursued, it must be explicit opt-in and must preserve every authority restriction above unless a separate evidence gate proves otherwise.
