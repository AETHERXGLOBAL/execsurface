# Post-M9 — Ptrace Fast-Path A2 Parity v4 Result

Date: 2026-09-27
Status: **PARTIAL — EXACT RAW PARITY PASSED 12 CASES; CONCURRENT RAW-ORDER GATE BLOCKED BY REFERENCE INTERLEAVING**
Parent: GitHub issue #84
Workflow run: `36289310640`
Source SHA: `c1063c48b08c9062a24ef02168e8b44547df3b2c`
Artifact ID: `10921513558`
Artifact digest: `sha256:311ee8f0f446b533e97b3fc8e381c412c41c5b32d92dd5310e0ec0de0ca5e10d`

## Decision

Candidate A2 remains **experimental and unmerged**, but v4 materially strengthened its semantic evidence.

After correcting the previously defective flattened `process_spawn.child_tid` TID projection, v4 established exact reference/reference determinism and exact reference/A2 equality under the frozen TID-only raw comparator for twelve adversarial cases before reaching the first genuinely concurrent multi-thread ordering counterexample.

The workflow stopped at `fixture-thread-read` because the **reference observer differed from itself in raw event interleaving** across two valid runs. A2 was not compared on that case in v4, and activation/value measurement remained blocked as required.

## Gates passed before the block

- reference binaries/helper build: PASS;
- Candidate A2 applied inside runner only: PASS;
- workspace Clippy with `-D warnings`: PASS;
- `execsurface-observe` tests: PASS;
- full workspace tests: PASS.

## Exact raw parity cases accepted in v4

For each case below:

- reference run 1 == reference run 2 after **only local TID renaming**;
- Candidate A2 == reference under the same projection;
- event count/order/sequence and every non-TID payload remained exact;
- observation `complete = true`;
- warnings = `0`;
- command outcome matched.

Accepted cases:

1. `irrelevant`
2. `fileio`
3. `failed-open`
4. `signal`
5. `restart` — original concurrent restart stress fixture
6. `restart-single`
7. `fork`
8. `vfork`
9. `exec`
10. `failed-exec`
11. `thread-exec`
12. `exit-group`

This includes the previously problematic `restart` and `fork` cases; after the comparator implementation was corrected, both became reference-deterministic and reference/A2-equal under the already-frozen TID-only rule.

## `fixture-thread-read` reference counterexample

The next case launches eight threads that read the same fixed file.

Reference run facts from the v4 artifact:

- run 1: `complete = true`, warnings `0`, exit code `0`;
- run 2: `complete = true`, warnings `0`, exit code `0`;
- event count: `29` in each run;
- event-type counts in each run:
  - `file_descriptor_access`: `15`;
  - `process_spawn`: `8`;
  - `file_path_access`: `5`;
  - `process_exec`: `1`.

A post-run artifact audit found that, after removing only runtime identity/order fields (`sequence`, `tid`, `child_tid`) for diagnostic analysis, the multiset of every remaining event payload was identical across the two reference runs.

The first ordered-payload difference was an interleaving swap: one run emitted a `process_spawn` at the corresponding position while the other emitted a `file_descriptor_access` read. No evidence of missing semantic payload, warning, incomplete observation or target failure was found in this reference/reference counterexample.

This diagnostic multiset audit is **not a new product normalization rule** and is not used to declare candidate parity. Its purpose is only to classify why the strict raw-order comparator is not an applicable equality relation for this concurrent fixture.

## Scientific interpretation

The strict raw comparator remains valid and unchanged for workloads whose reference output is deterministic under that comparator.

For a deliberately concurrent workload, exact raw event order/sequence can vary while the current production canonicalizer is explicitly designed to remove PID/sequence/interleaving variance before baseline/diff semantics are evaluated. The repository already carries regression coverage for this production property (`same_logical_behavior_survives_pid_sequence_root_and_interleaving_variance`).

Therefore it would be scientifically incorrect either to:

- weaken or sort the raw comparator post hoc for every case; or
- reject the production-semantic comparison model merely because raw thread scheduling order is not stable in the reference itself.

The next gate must split the two concerns prospectively.

## Required next gate — v5 split semantic gate

### Lane A — strict raw parity

Retain the corrected TID-only raw comparator unchanged for reference-deterministic cases. Include all twelve accepted v4 cases and independently test `fixture-spawn` if its reference is deterministic.

### Lane B — production semantic parity for concurrent `fixture-thread-read`

Use only the **existing unchanged production canonicalization / baseline / diff machinery** to require:

1. complete, warning-free, matching target outcomes for reference and A2;
2. reference/reference unchanged-workload canonical stability;
3. reference baseline -> A2 check with zero semantic findings;
4. A2 baseline -> reference check with zero semantic findings;
5. no new normalizer rule, sorting rule, policy suppression or candidate-specific projection.

The old raw interleaving failure remains preserved and is not converted to a raw PASS.

## What remains blocked

Until the v5 split semantic gate passes:

- activation/value diagnostics remain non-authoritative;
- external performance benchmark remains blocked;
- Candidate A2 is not eligible for merge;
- no public performance claim is permitted.

## Authority boundary

- public runtime on `main`: unchanged;
- public release: `v0.1.0-alpha.3`;
- ptrace remains public correctness-reference backend;
- eBPF authority remains unchanged;
- Candidate A2: **PARTIAL semantic evidence / unmerged**.
