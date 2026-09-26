# M8 — Pluggable Observation Backends & eBPF Evidence Architecture

Date: 2026-09-26
Status: **CLOSED — M8.0–M8.8 COMPLETE / PUBLIC eBPF EXPOSURE DEFERRED**
Tracking: #34, #37, #40, #42, #44, #46, #49

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
- M8.8 product decision: `docs/milestones/M8_8_PUBLIC_INTEGRATION_DECISION.md`

## Objective and final result

M8 evaluated an eBPF observation path without weakening the semantics, privacy boundary, fail-closed behavior, installation path, or evidence quality established by the native Linux ptrace reference backend.

M8 was additive, not a rewrite.

> A different collection mechanism may change collection cost, but it may not silently change what ExecSurface means by observed, complete, comparable, or PASS.

The research program succeeded in building and adversarially evaluating the eBPF path. The product decision is to **defer public eBPF exposure** until a real external workload or capability requirement provides a justified user-value trigger.

## Authority at M8 closure

- native Linux ptrace remains the correctness reference and default public backend;
- existing canonicalization, baseline, diff, policy, verdict, report and public installation semantics remain authoritative;
- eBPF is not evidence-equivalent merely because it emits similar event names;
- incomplete, lost, stale, truncated, ambiguous or unsupported evidence cannot silently produce PASS;
- ptrace-learned and eBPF-learned evidence are not automatically interchangeable;
- eBPF `learn` / `check` authority is not authorized;
- eBPF PASS authority is not authorized;
- automatic backend selection is not authorized;
- production persistent eBPF service/daemon deployment is not authorized;
- public eBPF exposure is deferred under M8.8.

M8.5 proved only bounded semantic parity for explicitly controlled propositions. It did not prove full-surface equivalence.

## Team model

### Fixed — Innovation Scientist / Architect

- separate collection mechanism from evidence meaning;
- seek performance improvements only when evidence quality is preserved;
- keep future expansion possible without converting ExecSurface into generic telemetry, EDR, antivirus or sandboxing.

### Fixed — Deviation Prevention / Scientific Integrity

- preserve the product objective: accepted execution surface → later execution → deterministic drift evidence;
- reject unsupported completeness, performance, portability, security or novelty claims;
- retain negative evidence and killed designs;
- block any change that makes backend uncertainty indistinguishable from no drift.

Dynamic specialists were selected per gate across Linux/eBPF, Rust systems, semantics, BTF/kernel compatibility, performance, security/privacy, CI/release and independent review.

## Stable backend boundary

```text
Command / target
      |
Observation Session
      |
Backend selection
   /        \
ptrace      eBPF
 REF      research
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

A backend supplies typed raw observations plus explicit capability and collection-health metadata. It does not decide policy or verdict.

## Completeness and fail-closed model

The eBPF work retained explicit non-complete states covering loss, limits, unsupported capability, lifecycle, decode and collector failure.

M8.4 proved controlled fail-closed handling for producer loss, truncation, consumer lag, lifecycle timeout, decode/setup failures, post-start collector failure and containment. M8.7 extended that discipline across persistent sessions with routing-state ambiguity, crash/restart, stale epochs, launch/barrier failure and per-session accounting.

A condition that can lose selected evidence without detection remains a **KILLED** design for PASS-capable use.

## Capability and comparability model

Backend capability remains explicit and versioned. An eBPF backend may support a strict subset; unsupported capability must be explicit.

Cross-backend states remain:

- same-backend comparable;
- explicitly cross-backend parity-proven for a bounded proposition;
- cross-backend non-comparable.

M8.5 proved bounded class-local parity for controlled process-lineage, exec-occurrence and focused positive successful-open propositions. That does not authorize full baseline interchangeability.

## Privacy boundary

The metadata-only boundary is retained. eBPF does not authorize persistence of file contents, environment values, stdin contents, network payloads, secrets or unrestricted argv values.

Persistent observation must not become generic ambient host telemetry.

## Implementation choice

M8.2 compared Aya and libbpf/libbpf-rs with executable feasibility evidence. libbpf-rs/libbpf was selected for the experimental path while Aya remained a preserved viable alternative.

Stable kernel tracepoints are preferred when they establish the needed semantics. Dynamic/internal kernel hooks require separate evidence.

## Persistent attachment result

M8.6 showed that per-invocation libbpf teardown dominated measured lifecycle cost on the tested host. M8.7 therefore evaluated persistent attachment rather than weakening timing or cleanup semantics.

M8.7 proved, for the bounded controlled prototype:

- one open/load/attach lifetime across sequential sessions;
- one active session at a time;
- non-zero monotonic session epoch;
- kernel `TID → epoch` membership;
- root-registration bootstrap barrier;
- descendant epoch propagation at `sched_process_fork` task creation;
- epoch-tagged accepted events;
- per-session drop/routing/decode/limit accounting;
- descendant drain after root exit;
- deterministic state cleanup;
- crash/restart invalidation;
- explicit rejection of stale/cross-session events;
- one final detach at collector shutdown.

### Negative designs retained

- **KILLED:** syscall-exit-only membership propagation due child scheduling race.
- **KILLED under Apache-2.0:** direct `task_struct` access through the attempted `tp_btf` path because the tested verifier required GPL-compatible program licensing.
- **KILLED:** blocking root barrier inside Rust `pre_exec` because it deadlocked with `Command::spawn()` exec synchronization.
- per-invocation libbpf as a low-overhead fast path is **not supported by the measured-host evidence**.

## M8.7 adversarial closure

Executable fail-closed evidence covered:

- sequential isolated epochs under one attachment;
- descendants surviving root exit and 64-descendant churn;
- concurrent-session rejection;
- malformed/unknown event state;
- launch and barrier failure;
- real kernel routing-state failure;
- real producer ring-buffer loss;
- live event-budget truncation;
- collector crash/restart;
- live stale epoch delivered through kernel → ring buffer → userspace.

Independent Red Team decision:

**ACCEPT — BOUNDED PERSISTENT-SESSION ARCHITECTURE FEASIBLE.**

That is architecture acceptance, not public-product approval.

## M8.7 exact-host performance result

The performance protocol was frozen before measurement. On the tested Ubuntu 24.04.5 / kernel 6.17.0-1022-azure GitHub runner, medians were:

| Mode | Median |
|---|---:|
| direct | 2.858403 ms |
| ptrace | 10.020306 ms |
| libbpf per invocation | 607.278495 ms |
| persistent two-session amortized | 505.770932 ms |
| persistent internal session | 113.918489 ms |

The persistent two-session result was about 16.7% lower than per-invocation libbpf on that exact host/workload, but remained about 50.5x the ptrace median. A roughly 113.9 ms internal session latency floor remained.

Therefore persistent attachment improved the libbpf architecture, but current evidence does not establish a performance reason to expose or select eBPF over ptrace.

## Privilege boundary

Ptrace remains the ordinary launched-command reference. eBPF may require capabilities or privilege depending on host policy.

ExecSurface must not silently elevate privilege, weaken security controls, hide why eBPF is unavailable, install an unreviewed privileged helper/service, or turn the feasibility collector into a remote control plane.

Production privilege separation remains a separate future gate.

## M8.8 — public integration decision

**CLOSED / DEFER — KEEP eBPF RESEARCH-ONLY FOR NOW.**

M6.5 declared before M8 that production eBPF becomes justified only when an external workload reproduces either:

1. ptrace median slowdown > `2x` with direct median >= `100 ms`; or
2. ptrace median absolute observer overhead > `500 ms`.

No accepted external-workload evidence currently meets that trigger.

Additionally:

- current exact-host eBPF end-to-end performance is worse than ptrace;
- semantic coverage remains narrower;
- `learn`, `check`, PASS and baseline interchangeability remain unauthorized;
- persistent privilege/service lifecycle remains an open product boundary;
- the default `cargo install execsurface --locked` path is already simple and should not absorb BPF/native dependency burden without a proved user benefit.

Therefore a public developer preview is deferred rather than shipped solely because the backend is technically feasible.

### Reopen rule

Reconsider public eBPF integration only when either:

- a real external workload crosses the predeclared M6.5 ptrace cost trigger and eBPF proves a meaningful clean end-to-end improvement on the same workload; or
- a concrete external user workflow requires a capability that eBPF can uniquely provide under explicit completeness/privacy semantics.

All privilege, packaging, diagnostics, lifecycle, privacy and authority gates must still pass before exposure.

## Final gate sequence

1. **M8.0 — CLOSED / ACCEPTED:** architecture contract.
2. **M8.1 — CLOSED / ACCEPTED:** non-breaking backend abstraction; ptrace retained.
3. **M8.2 — CLOSED / ACCEPTED:** feasibility and libbpf-rs/libbpf selection.
4. **M8.3 — CLOSED / ACCEPTED:** isolated metadata-only observer and observation-only bridge.
5. **M8.4 — CLOSED / PROVED:** fail-closed loss/lifecycle gate.
6. **M8.5 — CLOSED / ACCEPTED:** bounded semantic parity and Red Team closure.
7. **M8.6 — CLOSED / ACCEPTED:** performance/compatibility evidence and teardown attribution.
8. **M8.7 — CLOSED / ACCEPTED:** bounded persistent-session feasibility, adversarial evidence, performance measurement and Red Team closure.
9. **M8.8 — CLOSED / DEFER:** public eBPF exposure deferred pending real user-value evidence.

## Final status

- M8 research program: **CLOSED / ACCEPTED**
- public eBPF integration: **DEFERRED**
- eBPF research asset: **RETAINED**
- ptrace correctness reference/default public backend: **RETAINED**
- eBPF full-surface comparability: **FALSE**
- eBPF PASS authority: **NOT AUTHORIZED**
- automatic cross-backend baseline interchangeability: **NOT AUTHORIZED**
- automatic backend selection: **NOT AUTHORIZED**

The next strategic evidence should come from independent adoption and real external workloads, not from adding more speculative eBPF surface.