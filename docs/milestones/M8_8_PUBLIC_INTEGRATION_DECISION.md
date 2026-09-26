# M8.8 — Controlled eBPF Public-Alpha Integration Decision

Date: 2026-09-26
Status: **CLOSED / DEFER — KEEP eBPF RESEARCH-ONLY FOR NOW**
Tracking: #49
Input: M8.0–M8.7 evidence

## Decision

Do **not** expose the eBPF backend in the current public-alpha product surface.

Retain the eBPF implementation, persistent-session architecture, workflows, negative evidence, and performance evidence as a governed research asset. Keep the public/default product on the native ptrace correctness backend.

This is a product decision based on current evidence, not a rejection of eBPF as a technology.

## Why DEFER is the stronger product decision

### 1. The original value trigger has not been met by a real workload

M6.5 defined the production fast-path justification before M8 implementation work:

A production eBPF fast path becomes justified when an external workload reproduces either:

1. median ptrace slowdown greater than `2x` on a command whose direct median is at least `100 ms`; or
2. median absolute ptrace observer overhead greater than `500 ms`.

The repository currently has no accepted external-workload evidence meeting either condition.

The M6.5 controlled short synthetic burst had a relative slowdown of `3.722x`, but only `51.569 ms` median absolute ptrace overhead, so it did not cross the predeclared trigger.

### 2. Current exact-host eBPF performance does not create a user benefit

M8.7c used a protocol frozen before measurement and recorded:

| Mode | Median ms |
|---|---:|
| direct | 2.858403 |
| ptrace | 10.020306 |
| libbpf per invocation | 607.278495 |
| persistent two-session amortized | 505.770932 |
| persistent internal session | 113.918489 |

On that exact host/workload:

- persistent attachment materially improved the libbpf cost structure;
- conservative persistent end-to-end cost was about `16.7%` lower than per-invocation libbpf;
- but it still remained about `50.5x` the ptrace median;
- an internal per-session floor of about `113.9 ms` remained.

Therefore there is currently no measured performance basis for asking public-alpha users to choose eBPF over ptrace.

### 3. The eBPF semantic surface is intentionally narrower

M8.5 established only bounded parity for selected propositions. M8.7 established session isolation and failure handling for a controlled capability subset.

Still not authorized:

- full-surface semantic equivalence;
- ptrace/eBPF baseline interchangeability;
- eBPF `learn`;
- eBPF `check`;
- eBPF PASS;
- automatic backend selection.

A public preview that cannot participate in the main learn/check workflow would add a second user-facing path without yet adding a proved product outcome.

### 4. Persistent privilege lifetime is a real product boundary

The tested eBPF route requires privileged BPF load/attach on the reference environment. A persistent observer extends that privileged lifetime.

M8.7 deliberately did not introduce:

- a production daemon;
- an installed root helper;
- automatic elevation;
- a remote control plane;
- a production authentication/authorization model;
- pinned-map/link crash recovery.

Exposing a public preview before those boundaries are designed would create support and security obligations without a demonstrated user-value offset.

### 5. Packaging simplicity is already a product asset

The current public path is intentionally simple:

```bash
cargo install execsurface --locked
```

The M8 experimental libbpf work remains outside that default path. Public exposure must not pull BPF/native build requirements into the normal installation or weaken the existing crates.io/GitHub Release/GitHub Action experience.

There is no current evidence that accepting that added product surface is necessary.

## Candidate review

### A — DEFER public exposure

**SELECTED.**

Benefits:

- preserves the simple, proved ptrace product;
- avoids a public feature with worse measured end-to-end performance on the tested workload;
- avoids premature privilege/service lifecycle commitments;
- preserves all eBPF research as reusable evidence and code;
- keeps future integration evidence-driven rather than roadmap-driven.

### B — Explicit observation-only developer preview

**DEFERRED / NOT SELECTED NOW.**

Technically possible, but current evidence does not establish a concrete user workflow that benefits enough to justify the additional command surface, privilege requirements, support burden, and partial-capability explanation.

A developer preview is not valuable merely because the backend can execute.

### C — Broader public integration

**KILLED under current evidence.**

M8.7 does not provide the semantic, authority, privilege, compatibility, or performance evidence required for broader integration.

## Reopen conditions

Public eBPF integration may be reconsidered only when at least one **value trigger** and all **safety/product gates** below are satisfied.

### Value trigger — at least one required

#### V1 — real ptrace bottleneck

An external real-world workload reproduces the M6.5 trigger:

- median ptrace slowdown > `2x` with direct median >= `100 ms`; or
- median absolute ptrace observer overhead > `500 ms`.

The candidate eBPF path must then demonstrate a meaningful clean end-to-end improvement on the **same workload and host class**, not merely lower internal collector time.

#### V2 — unique required capability

A concrete external user workflow requires an observation capability that the current ptrace product cannot provide reliably or acceptably, and the eBPF route can provide it under explicit completeness/privacy semantics.

This must be a user/product need, not a speculative feature opportunity.

### Safety/product gates — all required

Before any public preview:

1. privilege requirements and failure behavior are explicit and never auto-escalating;
2. production/service lifetime design receives a separate security review if persistent attachment is used;
3. default `cargo install execsurface --locked` remains unaffected;
4. stable GitHub Action behavior remains unaffected unless separately gated;
5. supported kernel/host boundary and diagnostics are explicit;
6. cleanup/crash behavior is defined for the public lifecycle actually shipped;
7. partial/incomplete/non-comparable state is machine-readable and obvious to users;
8. no path exists from preview observation to `learn`, `check`, PASS, auto-selection or baseline interchangeability without a separate evidence gate;
9. privacy remains metadata-only;
10. independent Red Team accepts the final public surface.

## What remains valuable from M8

The M8 research is retained as a strategic technical asset:

- pluggable backend boundary;
- libbpf build/packaging knowledge;
- fail-closed loss semantics;
- controlled cross-backend parity methodology;
- performance attribution tooling;
- persistent-session epoch architecture;
- root-registration barrier;
- task-creation propagation;
- crash/restart and stale-event evidence;
- explicit negative designs and verifier/license findings.

Deferring public exposure does not discard any of this work. It prevents the research asset from weakening the current product before a real user-value trigger exists.

## Authority after M8.8

Unchanged:

- ptrace correctness reference: **RETAINED**;
- default public backend: **ptrace**;
- eBPF public exposure: **DEFERRED**;
- eBPF full-surface comparability: **FALSE**;
- eBPF `learn` / `check`: **NOT AUTHORIZED**;
- eBPF PASS authority: **NOT AUTHORIZED**;
- backend auto-selection: **NOT AUTHORIZED**;
- ptrace/eBPF baseline interchangeability: **NOT AUTHORIZED**;
- production persistent observer service: **NOT AUTHORIZED**.

## Next strategic implication

Do not spend the next product milestone adding more eBPF surface merely to complete an integration narrative.

The highest-value next evidence is **real external workload/adoption evidence**. That evidence can either:

- validate the current ptrace product and drive product adoption work; or
- reveal a real workload where the M6.5 trigger is crossed, providing a justified reason to reopen eBPF optimization/integration.

Until then, eBPF remains a governed research path rather than a public product feature.
