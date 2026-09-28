# M12.5 — Performance / Resource Regression Protocol

Date: 2026-09-28
Tracking: #90
Branch: `integration/m12-portable-clean`
Status: PREREGISTERED — NUMBERS NOT YET OBSERVED

## Fixed roles

- **Innovative Systems Architect** — seek a stronger low-cost release path and measurement design; do not assume the candidate is efficient because it is semantically safer.
- **Anti-Deviation / Skeptical Reviewer** — block threshold changes after observing results, benchmark cherry-picking, CI-noise overclaim, hidden resource growth, or converting incomplete evidence into PASS.

Dynamic specialists: Linux/ptrace performance, Rust runtime/build engineering, paired-CI statistics, resource measurement, and release engineering.

## Frozen references

- public main: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- portable candidate runtime/result boundary: `2d6b32c30c6062d7aea035d4a59a70ba834647f4`

The workflow itself may be committed after this protocol. It must build and compare the exact two refs above; later documentation/workflow commits are not allowed to move the tested runtime candidate silently.

## Environments

Run the complete paired protocol independently on:

- `ubuntu-22.04`
- `ubuntu-24.04`

Record kernel, CPU, rustc, cargo, and exact checked-out SHAs.

## Workloads

Use the existing frozen `execsurface-bench` implementation from each ref.

1. `/bin/true` — 30 repetitions; descriptive microbenchmark only.
2. descendant spawn fixture — 20 repetitions; nontrivial.
3. burst-128 file fixture — 10 repetitions; nontrivial.

Each main/candidate invocation must complete with rc=0. A failed/non-JSON benchmark makes that environment `PERFORMANCE_EVIDENCE_INCOMPLETE`; it cannot count as a pass.

## Performance material-regression rule

Preserve the M11.8 preregistered rule without relaxation:

A **nontrivial** workload is a material performance-regression signal only when both:

1. candidate observed median > **1.25 ×** main observed median; and
2. candidate observed median - main observed median >= **1,000 microseconds**.

Two nontrivial material signals in one environment are required for `PERFORMANCE_BLOCK`.

`/bin/true` cannot independently block or authorize release because short-command ratios are unstable.

## Resource sentinels

Resource evidence is supplemental engineering evidence, not a universal resource guarantee.

### Peak RSS sentinel

For each benchmark process, collect GNU `/usr/bin/time -v` maximum resident set size.
A workload is a material RSS signal only when both:

1. candidate max RSS > **1.20 ×** main max RSS; and
2. candidate max RSS - main max RSS >= **16,384 KiB**.

Because hosted-runner RSS is noisy and GNU time does not measure whole-system memory, RSS blocks only if the same named workload produces a material RSS signal on **both** Ubuntu environments.

### CLI release-binary size sentinel

Build the `execsurface` release CLI from both refs. A material binary-size signal requires both:

1. candidate binary > **1.10 ×** main binary size; and
2. candidate - main >= **524,288 bytes**.

Binary-size materiality blocks because it is deterministic enough to reproduce and is not runner-load dependent.

## Event-shape sanity

For each successful paired benchmark, record median event counts. Any unexplained event-count change in the candidate requires review; performance improvement obtained by silently observing less is not acceptable evidence.

The known fail-closed shared-FD change is not exercised as a performance workload in this gate because an intentionally incomplete workload is not a valid timing comparison for PASS-eligible observation.

## Classification

Per-environment evaluator:

- `PERFORMANCE_EVIDENCE_INCOMPLETE` if a required benchmark cannot be compared;
- `PERFORMANCE_BLOCK` if >=2 nontrivial material performance signals occur;
- otherwise `NO_PERFORMANCE_BLOCKING_REGRESSION`.

Overall M12.5:

- `M12_5_INCOMPLETE` if either environment is incomplete;
- `M12_5_PERFORMANCE_RESOURCE_REJECT` if either environment performance-blocks, if the same RSS workload materially regresses on both environments, or if binary-size materiality is observed;
- otherwise `M12_5_NO_GATE_BLOCKING_PERFORMANCE_RESOURCE_REGRESSION`.

## Non-claims

A passing result does not prove:

- that the candidate is universally faster;
- stable p95/tail latency on shared GitHub runners;
- whole-system memory equivalence;
- lower production cost;
- production readiness;
- hybrid/BPF-LSM end-to-end performance.

The supported conclusion is only whether this frozen paired gate found a preregistered blocking regression.