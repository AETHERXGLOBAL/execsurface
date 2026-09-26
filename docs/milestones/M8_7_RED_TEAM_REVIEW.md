# M8.7 — Persistent Observer Red Team Review

Date: 2026-09-26
Status: **ACCEPT — BOUNDED PERSISTENT-SESSION ARCHITECTURE FEASIBLE**
Tracking: #46
Reviewed branch: `milestone/m8-7-persistent-observer`
Reviewed through: `ce9c390a24aded478bc90a21c0abd8bc32d4080f`

## Decision

The repository-level independent Red Team role accepts the M8.7 architecture only for the following bounded claim:

> A single-active-session persistent libbpf observer can keep BPF programs attached across sequential controlled sessions while using a non-zero session epoch, a root-registration barrier, task-creation epoch propagation, fail-closed loss/state accounting, deterministic cleanup checks, and explicit authority boundaries.

This decision is **not** production approval and does **not** authorize public integration, default backend selection, full semantic equivalence, cross-backend baseline interchangeability, or eBPF PASS authority.

The persistent architecture remains an isolated feasibility implementation.

## Evidence reviewed

| Attack / property | Evidence | Result |
|---|---|---|
| Sequential session isolation | M8.7a persistent observer workflow `36262244488` | PASS |
| Live tracepoint ABI audit | M8.7a workflow | PASS |
| Descendants surviving root exit / 64-descendant churn | M8.7 Persistent Observer workflow `36264064989` | PASS |
| Concurrent-session rejection | state-machine adversary suite in `36264064989` | PASS / rejected fail-closed |
| Malformed record / unknown event | state-machine adversary suite in `36264064989` | PASS / rejected fail-closed |
| Target launch failure | state-machine adversary suite in `36264064989` | PASS / explicit error |
| Barrier release failure | state-machine adversary suite in `36264064989` | PASS / explicit error |
| Kernel routing-state failure | M8.7 Routing Failure workflow `36264070921` | PASS / session not clean |
| Producer ring-buffer loss | M8.7 Producer Loss workflow `36264199739` | PASS / session not clean |
| Event-budget truncation | M8.7 Event Budget workflow `36264339646` | PASS / session not clean |
| Collector crash / restart | M8.7 Crash Restart workflow `36263425308` | PASS / old in-flight authority not resumed |
| Real stale epoch delivered through ring buffer | M8.7 Stale Epoch workflow `36264999265` | PASS / second session rejected as clean evidence |
| General product CI at stale-epoch HEAD | CI workflow `36264999252` | PASS |
| Persistent performance protocol | M8.7 Persistent Performance workflow `36264634525` | PASS |
| General CI at performance evidence commit | CI workflow `36264634501` | PASS |

The reviewed fault builds are explicitly opt-in at build time. Default builds do not define the fault macros. The current default experimental values remain a 65,536-byte ring buffer and 4,096-entry task-epoch map; controlled fault builds may shrink those resources or inject a stale epoch only when the corresponding `M8_7_FAULT_*` environment variable is explicitly supplied to the isolated experiment build.

## Red Team findings

### RT-1 — Session epoch boundary

**ACCEPTED for bounded sequential single-session feasibility.**

Every accepted event carries an epoch. Userspace rejects mismatched epochs before mutating active session state. A live fault build intentionally emitted an epoch-1 exec record while epoch 2 was active; the second session recorded stale evidence and failed the clean-session gate.

The epoch is not treated as a substitute for task identity. Membership is still keyed by Linux TID and propagated at task creation.

### RT-2 — Root registration race

**ACCEPTED architecture.**

The controlled bootstrap-exec barrier prevents the target workload from running before the parent obtains the stable root PID/TID, inserts `root_tid -> epoch`, arms the userspace tracker, and explicitly releases the target.

The earlier blocking `pre_exec` barrier remains **KILLED** because it deadlocked with Rust `Command::spawn()` exec synchronization.

### RT-3 — Descendant propagation and root exit

**ACCEPTED for the tested lifecycle subset.**

Membership propagation occurs at ordinary `tracepoint/sched/sched_process_fork`, before the child contributes accepted session events. The 64-descendant adversarial workload required the collector to continue after root exit until the propagated descendants completed, and the session did not close merely because the root process exited.

Syscall-exit-only propagation remains **KILLED** because it permits a child scheduling race before the parent's syscall return.

### RT-4 — Apache-2.0 boundary

**PRESERVED.**

The rejected `tp_btf` / direct `task_struct` implementation required GPL-compatible BPF licensing on the tested verifier path. M8.7 retained Apache-2.0 and instead uses the ordinary `sched_process_fork` tracepoint payload, with live layout auditing before it counts as evidence.

This is preferable to changing project licensing merely to make a prototype pass.

### RT-5 — Loss and resource exhaustion

**FAIL-CLOSED BEHAVIOR PROVED for controlled fault cases.**

Two independent classes were exercised:

1. a constrained ring-buffer build produced real producer drops; the session could not remain clean;
2. an event-budget exhaustion case explicitly marked truncation and could not remain clean.

A deliberately constrained task-membership map was also used to force real routing-state failure. Routing ambiguity remained explicit and prevented clean evidence.

The Red Team rejects any future design that resets global loss/routing counters solely to manufacture a clean next session.

### RT-6 — Crash / restart

**ACCEPTED for unpinned prototype semantics only.**

The tested prototype does not approve pinned BPF state surviving collector death. Collector failure invalidates in-flight authority; a restarted collector starts a new lifetime and must not resume an old report as clean evidence.

A future production service using pinned maps/links would require a new crash-consistency proof and is outside this acceptance.

### RT-7 — Concurrent sessions

**NOT SUPPORTED, EXPLICITLY REJECTED.**

The first persistent architecture permits one active session only. An attempted second active session is rejected without replacing or mutating the first active session.

No claim of safe concurrent multi-tenant observation is made.

### RT-8 — Fault-injection isolation

**ACCEPTED.**

Fault mechanisms are compile-time opt-ins in the isolated M8.7 experiment. They are absent unless explicit environment switches are supplied to `build.rs`.

The Red Team specifically rejects runtime hidden fault modes in a public binary and rejects allowing test-only ring/map sizing or stale-epoch corruption to become default behavior.

### RT-9 — Performance claim boundary

**ACCEPTED only as exact-host measurement; product-performance claim rejected.**

The frozen M8.7c protocol measured, on one Ubuntu 24.04.5 / kernel 6.17.0-1022-azure GitHub runner:

| Mode | Median ms |
|---|---:|
| direct | 2.858403 |
| ptrace | 10.020306 |
| libbpf per invocation | 607.278495 |
| persistent two-session amortized | 505.770932 |
| persistent internal session | 113.918489 |

On that exact host/workload, the conservative two-session persistent result was about 16.7% lower than per-invocation libbpf, but remained about 50.5x the ptrace median. The internal persistent session remained about 113.9 ms.

Therefore the performance evidence supports only the statement that persistent attachment materially changes the libbpf cost structure. It does **not** support replacing ptrace on performance grounds, does not prove universal Linux speedup, and does not justify automatic backend selection.

The remaining per-session latency floor is an open engineering problem. Timing gates must not be shortened merely to improve benchmark numbers.

### RT-10 — Privilege lifetime

**OPEN for productization.**

Persistent BPF attachment extends the lifetime of privileged observation. M8.7 does not add automatic privilege escalation, a root helper installation, a remote control plane, or a production daemon.

Any later service design requires a separate least-privilege, authentication/authorization, lifecycle, update, and local attack-surface review.

### RT-11 — Privacy

**BOUNDARY RETAINED for the tested prototype.**

The experiment remains metadata-only. M8.7 does not authorize file contents, environment values, stdin contents, network payloads, secrets, or unrestricted argv capture.

A persistent observer must not evolve into generic ambient host telemetry without a separate privacy/security decision.

### RT-12 — Compatibility and semantic scope

**PARTIAL / bounded.**

The tested classic tracepoint layout is audited on the live runner. That does not establish universal kernel compatibility.

M8.7 covers a lifecycle/session-isolation capability subset. It does not establish full parity for all ExecSurface observation classes. M8.5's comparability boundaries remain authoritative.

## Counterexamples and killed designs retained

The following negative evidence is part of the accepted result and must not be erased:

- syscall-exit-only membership propagation: **KILLED** by child scheduling race;
- `tp_btf` + direct `task_struct` under Apache-2.0: **KILLED** by verifier/license boundary;
- blocking barrier in Rust `pre_exec`: **KILLED** by launch deadlock;
- per-invocation libbpf lifecycle as a fast path: **not supported by M8.6/M8.7 performance evidence**;
- producer loss, routing ambiguity, event-budget exhaustion, stale epoch, malformed evidence, launch failure, crash ambiguity, and concurrent-session attempts: **must remain fail-closed**.

## Final M8.7 Red Team decision

**ACCEPT — PERSISTENT SESSION ARCHITECTURE FEASIBLE, WITH STRICT BOUNDARIES.**

M8.7 has enough executable evidence to retain the persistent-session architecture as the correct research direction for any future eBPF productization work.

The following remain **NOT AUTHORIZED**:

- public/default eBPF backend selection;
- eBPF `learn` or `check` authority;
- eBPF PASS authority;
- full-surface comparability;
- ptrace/eBPF baseline interchangeability;
- concurrent or multi-tenant sessions;
- production daemon/service deployment;
- universal Linux compatibility or performance claims.

`ptrace` remains the correctness reference.

Before any public integration decision, PR-level CI, Distribution Probe, Action Smoke, and Registry Packaging checks must remain green. Any future M8.8 work must preserve the above authority boundaries unless a separate evidence gate explicitly changes them.
