# M11.8 — Performance Evidence & Release Decision Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — FINAL GATE AUTHORIZED**

## Objective

Close M11 with an evidence-based release/default-backend decision. Performance evidence is separated into the portable ptrace path and the explicit hybrid authority layer. No result in this gate may be converted into a production-readiness or universal-performance claim.

## Fixed roles

- **Innovative Systems Architect** — search for the strongest release architecture that preserves evidence authority and public usability.
- **Anti-Deviation / Skeptical Reviewer** — attempt to block release/default promotion using semantic regressions, compatibility failures, incomplete measurements, or unsupported performance extrapolation.

## Dynamic specialists

- Linux performance methodology
- paired CI benchmarking / statistics
- Rust performance and allocation review
- BPF-LSM / managed-kernel architecture
- release engineering / Marketplace compatibility
- baseline and evidence-authority semantics
- independent internal release red-team

## Frozen source references

- public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- current public release: `v0.1.0-alpha.3`
- research branch: `research/m11-authority-aware-integration`

M11.8 does not authorize mutation of `main`, public tags, Marketplace publication, or the current release.

## Part A — Portable path paired benchmark

Benchmark `main` and the M11 research branch on the **same GitHub-hosted runner** using separate target directories and identical workload definitions.

Frozen workloads:

1. `/bin/true` — 30 repetitions. Microbenchmark; descriptive only.
2. controlled descendant-spawn fixture — 20 repetitions.
3. controlled file-event burst of 128 files — 10 repetitions.

For every workload record:

- direct median/p95/min/max;
- observed median/p95/min/max;
- median absolute observer overhead;
- median slowdown ratio;
- event-count distribution;
- environment/toolchain evidence.

### Portable regression rule

`/bin/true` cannot by itself block or promote because very short-command ratios are unstable.

For each nontrivial workload (`spawn`, `burst-128`) define a **material regression signal** only when BOTH are true:

- M11 median observed time > 1.25 × main median observed time; and
- the absolute median increase is >= 1,000 microseconds.

Portable release-candidate status is blocked if:

- either nontrivial workload cannot produce complete evidence under its frozen semantics; or
- both nontrivial workloads show a material regression signal; or
- M11 changes the tested CLI/verdict contract.

A single noisy regression signal is recorded but is not enough for a broad performance claim or rejection on a shared CI runner.

## Part B — Explicit hybrid authority-layer benchmark

The current M11 hybrid crate is an **authority/capability adapter only**. It does not load BPF, boot a managed kernel, collect live kernel records, or provide an end-to-end product observer.

Freeze a synthetic adapter benchmark:

- accepted explicit Linux/x86_64 hybrid capability context;
- build/validate the bounded authority contract;
- perform representative proposition lookups;
- map complete/loss health;
- 21 batches × 10,000 iterations;
- record median and p95 nanoseconds per iteration.

This measures **adapter computation only**.

### Mandatory anti-overclaim rule

Adapter timing is **not** end-to-end hybrid observation latency and cannot authorize hybrid product/default promotion.

A hybrid product/default release requires a future end-to-end benchmark that includes the real managed BPF-LSM collector, success-correlation path, loss accounting, session isolation, transport, and normalization on a controlled BPF-LSM-capable environment.

Because M11 intentionally does not integrate that collector into the public/default runtime, absence of such end-to-end performance evidence requires the hybrid product/default decision to remain **DEFERRED**, even if adapter timing is small.

## Part C — Final independent internal red-team

Before the release decision, independently attempt to invalidate all promotion arguments against these facts:

1. M10 PATH-TOCTOU counterexample remains preserved.
2. M10 shared-FD false-completeness counterexample remains preserved and M11 fails closed.
3. pathname argument evidence is not promoted to kernel-object authority.
4. hybrid candidate evidence is not promoted to success evidence.
5. loss/session ambiguity remains non-PASS.
6. baseline-v2 remains frozen and cross-backend reuse is rejected.
7. hybrid remains explicit and no silent privilege/fallback exists.
8. M11.7 public/Marketplace compatibility remains green.
9. no public line mutation occurred.
10. performance conclusions are bounded to the measured runner/workloads.

Any failure blocks a positive portable release-candidate classification.

## Allowed final classifications

Exactly one of:

- `M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`
- `M11_RELEASE_DEFERRED_PERFORMANCE_OR_SEMANTIC_REGRESSION`
- `M11_RELEASE_REJECTED_COMPATIBILITY_OR_AUTHORITY_FAILURE`

The first classification means only that the isolated M11 portable hardening branch is eligible for a separate merge/release review. It does **not** mean production-ready and does not itself authorize merging or publishing.

## Required closeout evidence

- benchmark workflow SHA and run ID;
- raw JSON benchmark artifacts;
- portable paired comparison summary;
- hybrid adapter benchmark summary with explicit scope warning;
- M11.7 compatibility evidence;
- final independent internal red-team result;
- unchanged public `main` SHA;
- final classification and explicit non-claims.
