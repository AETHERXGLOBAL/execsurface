# M11 — Authority-Aware Evidence Integration & Portable Semantics Hardening — Closeout

Date: 2026-09-28
Tracking: #89
Research branch: `research/m11-authority-aware-integration`
Status: **CLOSED**
Final classification: **`M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`**

## Executive result

M11 completed all gates M11.0 through M11.8 without modifying the public `main` line, public release, stable Marketplace Action channel, or public default backend.

The portable M11 hardening work is eligible for a **separate integration / merge / next-alpha release review**. That later review is not part of this closeout and is not implicitly authorized by the M11 classification.

The hybrid BPF-LSM architecture remains **explicit, managed/research-only, and non-default**. M11 does not contain a live end-to-end hybrid product observer and therefore does not have the evidence needed to promote hybrid to the public default.

## Fixed review roles

Retained throughout the program:

- **Innovative Systems Architect** — searched for stronger designs and alternatives instead of preserving the existing implementation by default.
- **Anti-Deviation / Skeptical Reviewer** — attempted to reject every gate on semantic weakening, sunk-cost defense, hidden compatibility drift, silent fallback, overclaim, or inadequate evidence.

Dynamic specialists were selected per gate across Linux ptrace/process semantics, BPF/LSM, Rust architecture, evidence models, baseline compatibility, privacy, CI/Marketplace, performance methodology, and release engineering.

## Gate ledger

| Gate | Result | Closeout meaning |
| --- | --- | --- |
| M11.0 — architecture / compatibility freeze | **CLOSED** | Public line and compatibility invariants frozen before integration work. |
| M11.1 — authority-aware evidence model | **CLOSED** | Proposition-level evidence authority model established; stronger claims require stronger evidence. |
| M11.2 — ptrace pathname semantics | **CLOSED** | Syscall-entry pathname evidence cannot satisfy kernel-object identity authority. |
| M11.3 — shared-FD completeness | **CLOSED** | Clone/FD-sharing ambiguity fails closed as `IncompleteAmbiguity`; not PASS-eligible. |
| M11.4 — explicit hybrid capability adapter | **CLOSED** | Managed hybrid prerequisites are explicit; unsupported contexts reject rather than silently fallback. |
| M11.5 — proposition comparability | **CLOSED** | Cross-backend reuse/equivalence rejected unless exact proposition semantics are proven. |
| M11.6 — privacy/loss/adversarial red-team | **CLOSED** | Bounded privacy sentinels passed; loss/session/malformed/unsupported states remain fail-closed. |
| M11.7 — Marketplace/public compatibility | **CLOSED** | Existing public release and stable `@v0.1` paths preserved without hybrid privilege requirements. |
| M11.8 — performance/release decision | **CLOSED** | No gate-blocking portable regression in preregistered paired runs; hybrid default remains deferred. |

## Preserved external-criticism findings

M11 did not erase or redefine the M10 findings that motivated this work.

Still preserved as architectural facts/constraints:

1. ptrace syscall-entry pathname observation is not automatically the kernel-consumed object identity;
2. shared-FD concurrency can defeat stronger completeness assumptions unless ambiguity is represented explicitly;
3. kernel-hook evidence can be stronger for selected propositions;
4. BPF-LSM deployment prerequisites are not generally available on the current public GitHub-hosted path;
5. transport/producer loss must disable PASS authority;
6. candidate events are not equivalent to successful operations.

The response in M11 was to encode these limits into the evidence contract and fail-closed behavior, not to weaken the criticism.

## M11.6 bounded red-team evidence

Primary authority/red-team CI:

- run `36449724228`
- conclusion: **success**

Covered authority model, hybrid adapter, proposition comparability, loss/session fail-closed behavior, ptrace pathname authority, shared-FD ambiguity, privacy sentinel, frozen baseline-v2 compatibility, full-workspace tests, Clippy, formatting, and lockfile integrity.

Classification:

**`M11_6_PRIVACY_LOSS_REDTEAM_PASS_BOUNDED`**

No universal privacy or universal completeness claim follows.

## M11.7 public compatibility evidence

Primary Marketplace regression:

- run `36450566559`
- conclusion: **success**

Later regression after hybrid benchmark code:

- run `36451185462`
- conclusion: **success**

Verified:

- frozen Marketplace surface remained identical to public `main`;
- current public release no-Rust path remained usable;
- `AETHERXGLOBAL/execsurface@v0.1` preserved `pass/0` and controlled `review/10` behavior;
- research-branch portable default required no BPF-LSM/privileged managed-kernel setup;
- frozen baseline-v2 remained compatible;
- hybrid remained explicit and non-default.

Classification:

**`M11_7_PUBLIC_COMPATIBILITY_PASS_BOUNDED`**

## M11.8 primary performance evidence

Primary preregistered paired run:

- run `36451329198`
- benchmark SHA `8c84fbd6f8b4283816126cd15b359e851894ad69`
- conclusion: **success**
- evaluator: **`PORTABLE_NO_GATE_BLOCKING_REGRESSION`**
- material regression signals: **0**

The measured nontrivial workloads did not cross the preregistered blocking threshold. The shared runner exhibited noise/outliers, so the result is intentionally limited to **no gate-blocking regression in this paired run** rather than a claim that M11 is universally faster.

Primary artifacts:

- `m11-8-portable-performance-36451329198-1`
  - id `10984116733`
  - SHA-256 `4a4b89c00c915cd8a3a1e05789e0d998104a73fa8d753b18c4a87130005f7f44`
- `m11-8-hybrid-adapter-36451329198-1`
  - id `10983073482`
  - SHA-256 `bcad63bc19c1758e3692d70627146548b2bd8d86ac8d666819d988773613f0be`

The primary hybrid adapter benchmark was adapter-only and explicitly declared that it did not measure end-to-end hybrid latency.

## M11.8 replication on final result HEAD

After the M11.8 result was committed, the same frozen workflow ran again on final-result HEAD:

- run `36452012715`
- tested SHA `41311b44ec74a3d7283709ee04d1605e572d5e70`
- conclusion: **success**
- portable evaluator: **`PORTABLE_NO_GATE_BLOCKING_REGRESSION`**
- material regression signals: **0**

Replication nontrivial comparisons:

- descendant-spawn observed median: main `5791 us`, M11 `5791 us`, ratio `1.0`, delta `0 us`, material signal `false`;
- burst-128 observed median: main `88691 us`, M11 `85025 us`, ratio `0.958665...`, delta `-3666 us`, material signal `false`.

The replication's adapter-only hybrid benchmark measured median `164 ns/iteration`, p95 `169 ns/iteration`, but again explicitly recorded:

- `product_default_authority = false`;
- `end_to_end_hybrid_latency_measured = false`.

Replication artifacts included:

- portable artifact id `10983588232`, SHA-256 `872e1097ac3d7bf508efa9f4ce5692d1fa3b191781c372272ec47cede4f4c3ac`;
- hybrid adapter artifact id `10983853103`, SHA-256 `43cb5ebf50b1bc6ff5303e3cebc2d9c62c78df62a382b003c9933aaf7b8e43bc`.

The replication reinforces the bounded no-regression conclusion but does not turn CI-runner measurements into a performance guarantee.

## Final independent internal red-team

Document:

`docs/milestones/M11_FINAL_INDEPENDENT_REDTEAM.md`

Classification:

**`M11_RELEASE_ARGUMENT_SURVIVES_WITH_HYBRID_PROMOTION_BLOCKED`**

The skeptical review could not invalidate the bounded portable integration/release-review argument. It did invalidate any attempt to infer hybrid-default readiness, universal completeness, production readiness, or end-to-end hybrid performance readiness.

## Public-state integrity

At final pre-close verification:

- public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- M11 result HEAD before this closeout commit: `41311b44ec74a3d7283709ee04d1605e572d5e70`
- public release remains `v0.1.0-alpha.3`
- stable Marketplace Action remains `AETHERXGLOBAL/execsurface@v0.1`
- public default backend remains unchanged.

M11 closeout itself is documentation-only and does not authorize a public-line mutation.

## Final decision

### Portable authority-aware hardening

**Advance to a separate integration / merge / next-alpha release review.**

This is a review eligibility decision, not an automatic merge/publish decision.

### Hybrid BPF-LSM default/product path

**DEFER.**

Before hybrid can be reconsidered for default/product status, evidence must include an actual live managed BPF-LSM end-to-end observer implementation and benchmark covering collection, success correlation, loss accounting, session isolation, transport, normalization, privilege/deployment boundary, and product evidence generation.

## Explicit non-claims

This M11 closeout does not prove or claim:

- production readiness;
- universal Linux compatibility;
- complete observation of all kernel activity;
- universal privacy guarantees;
- universal performance improvement;
- acceptable end-to-end BPF-LSM performance;
- hybrid readiness as a public default;
- ptrace ↔ hybrid baseline equivalence.

## Final classification

**`M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`**

M11 is complete. The next authorized program boundary is a **separate integration / merge / release review** for the portable M11 hardening only. Hybrid promotion remains outside that boundary until new evidence closes the missing end-to-end managed-kernel gap.
