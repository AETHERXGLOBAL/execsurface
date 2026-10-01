# M11.8 — Performance Evidence & Release Decision Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED**
Final classification: **`M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`**

## Decision summary

The preregistered portable-path benchmark found **no gate-blocking performance regression** in the paired main-vs-M11 run. Public compatibility and authority invariants remained green through M11.7 and the final internal red-team.

The hybrid authority adapter is computationally small in its bounded synthetic benchmark, but M11 does **not** contain a live end-to-end BPF-LSM product observer. Therefore hybrid default/product promotion remains **DEFERRED**.

This classification means only that the isolated **portable M11 hardening branch is eligible for a separate merge/release review**. It does not authorize merging, publishing, changing Marketplace defaults, or claiming production readiness.

## Frozen references

- public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- primary M11.8 benchmark SHA: `8c84fbd6f8b4283816126cd15b359e851894ad69`
- public release: `v0.1.0-alpha.3`
- public stable Action channel: `AETHERXGLOBAL/execsurface@v0.1`

## Team

Fixed roles:

- **Innovative Systems Architect** — searched for the strongest architecture consistent with evidence authority and portable usability.
- **Anti-Deviation / Skeptical Reviewer** — attempted to block promotion on semantic, compatibility, privacy, completeness, or performance grounds.

Dynamic specialists:

- Linux performance methodology
- paired CI statistics
- Rust performance and integration
- BPF-LSM / managed-kernel architecture
- Marketplace / release engineering
- baseline and authority semantics
- independent internal release red-team

## Part A — Portable paired benchmark

Workflow: **M11.8 Performance Release Evidence**
Run: `36451329198`
Conclusion: **success**

Environment:

- Ubuntu 24.04.5
- kernel `6.17.0-1022-azure`
- x86_64
- AMD EPYC 9V74 runner CPU
- rustc `1.98.1`
- cargo `1.98.1`

The threshold was preregistered before the numbers were observed. A nontrivial workload produced a material regression signal only when both:

1. M11 observed median > 1.25 × main observed median; and
2. absolute median increase >= 1,000 microseconds.

Two nontrivial material signals were required for a performance-blocking classification.

Evaluator result:

**`PORTABLE_NO_GATE_BLOCKING_REGRESSION`**

Material regression signal count: **0**

### `/bin/true` — descriptive microbenchmark only

- main direct median: 535 us
- main observed median: 1,998 us
- main observer overhead median: 1,463 us
- main slowdown: ~3.7346x
- M11 direct median: 533 us
- M11 observed median: 2,003 us
- M11 observer overhead median: 1,470 us
- M11 slowdown: ~3.7580x
- M11/main observed-median ratio: ~1.0025x
- delta: +5 us
- material signal: no
- event median: 6 for both

The protocol explicitly forbids using this microbenchmark by itself to promote or reject due to short-command ratio instability.

### Descendant-spawn workload — 20 repetitions

- main direct median: 1,340 us
- main observed median: 6,002 us
- main observer overhead median: 4,662 us
- main slowdown: ~4.4791x
- M11 direct median: 1,343 us
- M11 observed median: 5,950 us
- M11 observer overhead median: 4,607 us
- M11 slowdown: ~4.4304x
- M11/main observed-median ratio: ~0.9913x
- delta: -52 us
- material signal: no
- event median: 19 for both
- both benchmark invocations completed successfully

### Burst-128 file workload — 10 repetitions

- main direct median: 13,541 us
- main observed median: 77,168 us
- main canonicalization median: 1,115 us
- main observer overhead median: 63,627 us
- main slowdown: ~5.6988x
- M11 direct median: 13,635 us
- M11 observed median: 73,825 us
- M11 canonicalization median: 1,123 us
- M11 observer overhead median: 60,190 us
- M11 slowdown: ~5.4144x
- M11/main observed-median ratio: ~0.9567x
- delta: -3,343 us
- material signal: no
- event median: 524 for both
- both benchmark invocations completed successfully

The shared runner produced a large p95 outlier in the main burst measurements. Therefore M11.8 does **not** claim that M11 is universally faster. The supported statement is only that the preregistered paired run showed no gate-blocking regression.

## Portable artifact

- name: `m11-8-portable-performance-36451329198-1`
- artifact id: `10984116733`
- SHA-256: `4a4b89c00c915cd8a3a1e05789e0d998104a73fa8d753b18c4a87130005f7f44`

## Part B — Hybrid authority adapter benchmark

The current M11 hybrid crate is an authority/capability adapter, not an end-to-end live observer.

Frozen workload:

- 21 batches
- 10,000 iterations per batch
- accepted explicit Linux/x86_64 capability context
- authority contract construction/validation
- representative proposition support lookups
- evidence-health mapping

Measured result:

- median: **213 ns/iteration**
- p95: **217 ns/iteration**
- min: 212 ns
- max: 222 ns

The artifact explicitly records:

- `scope = authority-adapter-only-not-end-to-end-hybrid-observation`
- `product_default_authority = false`
- `end_to_end_hybrid_latency_measured = false`

### Mandatory interpretation

These adapter numbers are **not** BPF-LSM observer latency and cannot authorize hybrid default promotion.

A future hybrid-default decision requires a real managed-kernel end-to-end benchmark including:

- BPF-LSM collection;
- lifecycle/success correlation;
- producer-loss accounting;
- session isolation;
- transport;
- normalization;
- product-level evidence generation.

## Hybrid artifact

- name: `m11-8-hybrid-adapter-36451329198-1`
- artifact id: `10983073482`
- SHA-256: `bcad63bc19c1758e3692d70627146548b2bd8d86ac8d666819d988773613f0be`

## Part C — Compatibility and semantic evidence

M11.7 primary run `36450566559`: success.

A later M11.7 rerun after the hybrid adapter benchmark was added, run `36451185462`, also succeeded.

Latest semantic authority CI covering hybrid adapter code before the performance-workflow-only commit:

- run `36451185374`
- tested SHA `1ce71ff06d7030631aa68e9710f402b33888fd72`
- conclusion: success

The M11.8 workflow commit at `8c84fbd6f8b4283816126cd15b359e851894ad69` changed performance evidence CI, not runtime semantics, and the performance workflow itself completed successfully.

## Part D — Final independent internal red-team

Document: `docs/milestones/M11_FINAL_INDEPENDENT_REDTEAM.md`

Classification:

**`M11_RELEASE_ARGUMENT_SURVIVES_WITH_HYBRID_PROMOTION_BLOCKED`**

The red-team failed to invalidate the bounded portable release-candidate argument, but explicitly blocked stronger claims and hybrid default promotion.

## Release decision

### Portable M11 hardening

**Eligible for a separate merge/release review.**

Reasons:

- authority model is explicit and proposition-level;
- ptrace pathname evidence is not inflated to kernel-object authority;
- shared-FD ambiguity fails closed;
- loss/session ambiguity fails closed;
- baseline-v2 compatibility remains frozen;
- cross-backend baseline reuse remains rejected absent equivalence proof;
- current public Marketplace path remains compatible;
- paired performance gate found no blocking regression;
- final skeptical review did not find a new semantic blocker.

### Hybrid default/backend promotion

**DEFERRED.**

Reasons:

- public runner privilege/LSM constraints remain real;
- M11 hybrid integration is capability/authority-layer work, not a full live product observer;
- no end-to-end hybrid runtime/performance evidence exists yet;
- no baseline interchangeability proof exists;
- M11 intentionally does not authorize public hybrid auto-selection.

## Explicit non-claims

M11.8 does **not** prove:

- production readiness;
- universal Linux compatibility;
- complete observation of all kernel activity;
- universal privacy guarantees;
- universal performance improvement;
- acceptable end-to-end BPF-LSM performance;
- hybrid readiness as the default backend;
- ptrace ↔ hybrid baseline equivalence.

## Final classification

**`M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`**

M11.8 is CLOSED. `main`, the public release, and Marketplace defaults remain unchanged pending a separate integration/release review.
