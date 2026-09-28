# M10 — Hybrid Observer Team & Execution Governance

Date: 2026-09-28
Branch: `research/m10-hybrid-observer`
Tracking: #88
Stable base: `main@db11761e2d75ebca7d4458dc39094f4aed6a26bb`
Status: **ACTIVE — ISOLATED RESEARCH / NO PRODUCT INTEGRATION AUTHORIZED**

## Mission

Test whether kernel-hook evidence can strengthen selected ExecSurface propositions without weakening the current metadata-only privacy boundary, fail-closed completeness rules, per-command scope, baseline semantics, or public alpha compatibility.

The research branch is intentionally isolated. `main`, Marketplace behavior, `v0.1.0-alpha.3`, current ptrace `learn/check`, baseline authority, and public exit-code semantics remain frozen until M10 reaches an explicit architecture closeout.

## Fixed roles

### Innovative Systems Architect

Permanent for every M10 gate.

Responsibilities:
- search for evidence anchors stronger than userspace syscall-boundary snapshots;
- design additive/hybrid architectures rather than assuming backend replacement;
- separate technical evidence authority from product-default selection;
- investigate BPF LSM, stable tracepoints, kernel object identity, session containment, and lower-perturbation designs;
- propose new abstractions only when they preserve or strengthen evidence semantics.

The role is required to challenge both ptrace and BPF/LSM. Novelty alone is not evidence.

### Anti-Deviation / Skeptical Reviewer

Permanent for every M10 gate.

Responsibilities:
- prevent defense of ptrace because it already exists;
- prevent promotion of BPF/LSM merely because an external reviewer suggested it;
- reject silent changes to PASS meaning, baseline comparability, privacy, or completeness;
- distinguish observation, mediation, enforcement, and evidence generation;
- preserve negative results and counterexamples;
- require proposition-level evidence before authority is widened.

## Dynamic team for M10.1 — PATH-TOCTOU-001

| Role | Purpose |
|---|---|
| Linux ptrace State-Machine Engineer | Audit actual stop/resume and memory-read timing in the current observer. |
| Linux VFS / Path-Resolution Specialist | Define the distinction between userspace pathname intent and kernel-consumed object identity. |
| Concurrency / Memory-Model Engineer | Construct a multithreaded shared-buffer mutation adversary without undefined evaluation criteria. |
| Rust Systems Harness Engineer | Drive the unchanged observer API and preserve machine-readable evidence. |
| C/Linux Fixture Engineer | Build a minimal target whose kernel-opened object can be independently identified. |
| Evidence Semantics Reviewer | Decide exactly what a mismatch proves and what it does not prove. |
| CI / Reproducibility Engineer | Pin commands, retain artifacts, environment identity, and deterministic evaluator logic. |
| Internal Adversarial Red Team | Try to explain away apparent mismatches as harness artifacts before acceptance. |

Dynamic specialists are replaced or removed at each later gate. Only the two fixed roles persist.

## Evidence discipline

Allowed classifications:
- **PROVED** — logical or executable evidence within explicit assumptions.
- **COMPUTATIONAL_EVIDENCE** — repeatable empirical result with preserved artifacts.
- **PARTIAL** — useful but insufficient evidence.
- **OPEN** — not established.
- **KILLED** — hypothesis/design contradicted within the declared scope.

A mismatch is not accepted unless:
1. the observation is marked complete;
2. the target exits normally with an independently decodable ground truth;
3. exactly one relevant pathname-intent event is attributable to the test open;
4. the evaluator preserves the raw observation and run summary or enough derived evidence to reproduce the classification;
5. no runtime/core patch is required to produce the mismatch.

## Branch and merge rules

- All M10 implementation stays on `research/m10-hybrid-observer`.
- No direct runtime change to `main` during research.
- No force push or history rewrite.
- One scientific milestone per commit where practical.
- Failed experiments remain in history with explicit classification.
- A future merge to `main` requires an explicit M10 closeout that separately decides:
  1. proposition-level technical authority;
  2. compatibility/migration semantics;
  3. packaging/privilege viability;
  4. public product default.

## Hard non-authorizations

Until M10 closeout explicitly changes them:
- ptrace remains the public correctness reference;
- BPF/LSM cannot produce public PASS;
- BPF/LSM cannot perform public `learn/check`;
- no automatic backend selection;
- no cross-backend baseline interchangeability;
- no privileged persistent service;
- no new enforcement behavior;
- no claim that LSM/BPF is universally stronger.
