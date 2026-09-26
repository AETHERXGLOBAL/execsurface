# M8 — Pluggable Observation Backends & eBPF Evidence Architecture

Date: 2026-09-26
Status: **OPEN — M8.0–M8.7 CLOSED / M8.8 CONTROLLED PUBLIC-ALPHA INTEGRATION NEXT**
Tracking: #34, #37, #40, #42, #44, #46

Evidence index:

- M8.2 decision: `docs/milestones/M8_2_STACK_DECISION.md`
- M8.2 evidence: `docs/milestones/M8_2_EVIDENCE.md`
- M8.4 evidence: `docs/milestones/M8_4_LOSS_SEMANTICS.md`
- M8.5 evidence: `docs/milestones/M8_5_SEMANTIC_PARITY.md`
- M8.5 Red Team: `docs/milestones/M8_5_RED_TEAM_REVIEW.md`
- M8.6 evidence: `docs/milestones/M8_6_PERFORMANCE_COMPATIBILITY.md`
- M8.6 root-cause evidence: `docs/milestones/M8_6D_POST_TARGET_LATENCY.md`
- M8.6 Red Team: `docs/milestones/M8_6_RED_TEAM_REVIEW.md`
- M8.7 persistent architecture: `docs/milestones/M8_7_PERSISTENT_OBSERVER.md`
- M8.7 performance: `docs/milestones/M8_7C_PERFORMANCE_PROTOCOL.md`
- M8.7 Red Team: `docs/milestones/M8_7_RED_TEAM_REVIEW.md`

## Objective

Add an eBPF observation path without weakening the semantics, privacy boundary, fail-closed behavior, or evidence quality already established by the native Linux ptrace reference backend.

M8 is additive. It is not a rewrite.

> A faster collection mechanism may change collection cost, but it may not silently change what ExecSurface means by observed, complete, comparable, or PASS.

## Authority that remains in force

M6.5 remains authoritative except where a later M8 gate explicitly proved a narrower statement:

- native Linux ptrace remains the correctness reference backend;
- existing canonicalization, baseline, diff, policy, verdict, and report semantics remain authoritative;
- an eBPF backend is not evidence-equivalent merely because it emits similar event names;
- incomplete, lost, stale, truncated, ambiguous, or unsupported evidence cannot silently produce PASS;
- ptrace-learned and eBPF-learned evidence are not automatically interchangeable;
- eBPF `learn` / `check` authority is not authorized;
- eBPF PASS authority is not authorized;
- automatic backend selection is not authorized.

M8.5 proved bounded semantic parity only for explicitly controlled propositions. It did not prove full-surface equivalence.

## Team model

### Fixed — Innovation Scientist / Architect

- seek the strongest backend architecture that remains semantically explicit;
- separate collection mechanism from evidence meaning;
- pursue performance improvements only when evidence quality is preserved;
- keep future expansion possible without turning ExecSurface into EDR, antivirus, sandboxing, or generic telemetry.

### Fixed — Deviation Prevention / Scientific Integrity

- preserve the product objective: accepted execution surface → later execution → deterministic drift evidence;
- reject unsupported completeness, performance, portability, security, or novelty claims;
- require negative evidence and killed designs to remain recorded;
- block any change that makes backend uncertainty indistinguishable from no drift.

### Dynamic specialists

Linux/eBPF, Rust systems, observation semantics, BTF/kernel compatibility, performance measurement, security/privacy, CI/release, and independent cross-backend review are selected per gate.

## Stable architecture boundary

```text
Command / target
      |
Observation Session
      |
Backend selection
   /        \
ptrace      eBPF
 REF      candidate
   \        /
Raw typed backend evidence + health
      |
Completeness / comparability gate
      |
Canonicalization
      |
Baseline / Diff
      |
Policy / Verdict
      |
Report
```

The backend boundary ends before canonical semantic interpretation. A backend supplies typed raw observations plus explicit capability and collection-health metadata. It does not decide policy or verdict.

## Completeness and fail-closed model

The eBPF work retains explicit non-complete states such as:

- complete for a declared capability subset;
- incomplete loss;
- incomplete limit;
- incomplete capability;
- incomplete lifecycle;
- incomplete decode;
- collector/error failure.

M8.4 proved controlled fail-closed handling for real producer loss, truncation, consumer lag, lifecycle timeout, decode/setup failures, post-start collector failure, and containment. M8.7 extended that discipline across persistent sessions, including routing-state ambiguity, crash/restart, stale epochs, launch/barrier failure, and per-session accounting.

A condition that can lose selected evidence without detection is a **KILLED** design for PASS-capable use.

## Capability and comparability model

Backend capability remains explicit and versioned. Relevant classes include process lifecycle, pathname intent, successful-open identity, fd-attributed effects, fd lifecycle/inheritance, rename/delete, network connect destination, relative-path semantics, causal executable chains, and loss visibility.

An eBPF backend may support a strict subset. Unsupported capability must be explicit.

Cross-backend states remain conceptually:

- same-backend comparable;
- explicitly cross-backend parity-proven for a bounded proposition;
- cross-backend non-comparable.

M8.5 proved bounded class-local parity for controlled process-lineage, exec-occurrence, and focused positive successful-open propositions. That does not authorize full baseline interchangeability.

## Privacy boundary

The metadata-only boundary is retained. The eBPF path does not authorize persistence of:

- file contents;
- environment values;
- stdin contents;
- network payloads;
- secret material;
- unrestricted argv values.

Persistent observation must not become generic ambient host telemetry.

## Implementation choice

M8.2 compared Aya and libbpf/libbpf-rs using executable feasibility evidence. libbpf-rs/libbpf was selected for the experimental path because it aligned well with Linux BPF/CO-RE while remaining buildable with the project's Rust 1.82 product line under the recorded dependency-resolution procedure. Aya remains preserved as a viable alternative, not declared technically invalid.

Stable kernel tracepoints are preferred where they establish the needed semantics. Dynamic/internal kernel hooks require separate justification and compatibility evidence.

## Persistent attachment result

M8.6 showed that per-invocation libbpf teardown dominated measured lifecycle cost on the tested host. M8.7 therefore tested a persistent attachment architecture rather than shortening evidence timers.

M8.7 selected and proved, for a bounded controlled prototype:

- one open/load/attach lifetime across sequential sessions;
- single active session;
- non-zero monotonic session epoch;
- kernel `TID → epoch` membership;
- root-registration bootstrap barrier;
- descendant epoch propagation at `sched_process_fork` task creation;
- epoch-tagged accepted events;
- per-session drop/routing/decode/limit accounting;
- descendant drain after root exit;
- deterministic map/session cleanup;
- crash/restart invalidation;
- explicit rejection of stale/cross-session events;
- one final detach at collector shutdown.

### Negative designs retained

- **KILLED:** syscall-exit-only membership propagation, due child scheduling race.
- **KILLED under Apache-2.0:** direct `task_struct` access through the attempted `tp_btf` path, because the tested verifier required GPL-compatible program licensing.
- **KILLED:** blocking root barrier inside Rust `pre_exec`, because it deadlocked with `Command::spawn()` exec synchronization.
- per-invocation libbpf as a low-overhead fast path is **not supported by the measured-host evidence**.

## M8.7 adversarial closure

M8.7 closed with executable fail-closed evidence for:

- two sequential isolated epochs under one attachment;
- 64 descendants surviving root exit;
- concurrent-session rejection;
- malformed and unknown event state;
- launch and barrier failure;
- real kernel routing-state failure;
- real producer ring-buffer loss;
- live event-budget truncation;
- collector crash and restart;
- a real stale epoch delivered through kernel → ring buffer → userspace.

For the BPF code head used by the final regression review, all seven triggered M8.7/general workflows were successful: CI, Persistent Observer, Routing Failure, Crash Restart, Producer Loss, Event Budget, and Persistent Performance.

Independent Red Team decision:

**ACCEPT — BOUNDED PERSISTENT-SESSION ARCHITECTURE FEASIBLE.**

This is architecture acceptance, not product approval.

## M8.7 exact-host performance result

The performance protocol was frozen before measurement. On the tested Ubuntu 24.04.5 / kernel 6.17.0-1022-azure GitHub runner, medians were:

| Mode | Median |
|---|---:|
| direct | 2.858403 ms |
| ptrace | 10.020306 ms |
| libbpf per invocation | 607.278495 ms |
| persistent two-session amortized | 505.770932 ms |
| persistent internal session | 113.918489 ms |

The conservative two-session persistent result was about 16.7% lower than per-invocation libbpf on that exact host/workload, but remained about 50.5× the ptrace median. The internal session latency is not claimed to be pure eBPF overhead; conservative lifecycle/quiescence rules contribute to it.

Therefore M8.7 establishes that persistent attachment changes the cost structure in the intended direction. It does **not** establish that eBPF is faster than ptrace, universal Linux performance, or production readiness.

## Privilege boundary

Ptrace remains the ordinary rootless-friendly reference for launched commands. eBPF may require capabilities or privilege depending on host policy.

ExecSurface must not:

- silently elevate privilege;
- weaken kernel/security controls automatically;
- hide why eBPF is unavailable;
- install an unreviewed privileged helper/service;
- convert a feasibility collector into a remote control plane.

A persistent privileged service has a longer-lived attack surface. Production privilege separation remains an explicit future productization gate.

## Gate sequence

1. **M8.0 — CLOSED / ACCEPTED:** architecture contract.
2. **M8.1 — CLOSED / ACCEPTED:** non-breaking observer abstraction; ptrace retained.
3. **M8.2 — CLOSED / ACCEPTED:** eBPF feasibility and libbpf-rs/libbpf selection.
4. **M8.3 — CLOSED / ACCEPTED:** isolated metadata-only libbpf observer and observation-only CLI bridge.
5. **M8.4 — CLOSED / PROVED:** fail-closed loss/lifecycle gate.
6. **M8.5 — CLOSED / ACCEPTED:** bounded semantic parity and independent Red Team closure.
7. **M8.6 — CLOSED / ACCEPTED:** exact-host performance/compatibility evidence; per-invocation detach identified as dominant measured lifecycle cost.
8. **M8.7 — CLOSED / ACCEPTED:** bounded persistent-session architecture feasibility, adversarial isolation/failure evidence, exact-host performance measurement, and independent Red Team closure.
9. **M8.8 — NEXT:** controlled public-alpha integration decision.

## M8.8 boundary

M8.8 may evaluate an explicit opt-in public-alpha surface, but it must not assume production daemon readiness merely because M8.7 proved architecture feasibility.

Before any public integration decision it must address, at minimum:

- explicit backend selection and no silent fallback;
- incomplete-state CLI/schema surfacing;
- privilege and service lifecycle boundaries;
- packaging/install isolation so the stable ptrace path is not degraded;
- compatibility diagnostics;
- technical-evaluation documentation;
- release/rollback evidence;
- continued ptrace reference authority;
- continued prohibition on eBPF PASS unless a separate gate proves and authorizes it.

## Current status

- M8 objective: **OPEN — M8.8 CONTROLLED PUBLIC-ALPHA INTEGRATION NEXT**
- M8.0–M8.7: **CLOSED** under their recorded bounded claims
- M8.7 persistent architecture: **ACCEPTED — FEASIBLE, NOT PRODUCTION APPROVED**
- M8.8 public-alpha integration decision: **NEXT / NOT STARTED**
- eBPF full-surface comparability: **FALSE**
- eBPF PASS authority: **NOT AUTHORIZED**
- automatic cross-backend baseline interchangeability: **NOT AUTHORIZED**
- automatic backend selection: **NOT AUTHORIZED**
- ptrace correctness reference: **RETAINED**
