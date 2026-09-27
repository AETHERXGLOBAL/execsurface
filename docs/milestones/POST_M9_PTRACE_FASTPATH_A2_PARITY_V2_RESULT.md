# Post-M9 — Ptrace Fast-Path A2 Parity v2 Result

Date: 2026-09-27
Status: **BLOCKED — REFERENCE NONDETERMINISM IN CONCURRENT RESTART FIXTURE**
Parent: GitHub issue #84
Workflow run: `36288875048`
Source SHA: `7abef67d917f01cd6c34925a50acdbffd8fbb10a`
Artifact ID: `10921406769`
Artifact digest: `sha256:526a7dc3f09bbdb5f3c1ac26cf2342d444c4847798e1374dac92e386b14e3429`

## Decision

Candidate A2 did **not** fail semantic parity in this run.

The gate stopped before comparing A2 on the `restart` case because the frozen reference observer failed its own reference/reference determinism check for the concurrent restart fixture.

Exact failure:

`reference nondeterminism under frozen raw comparator: restart`

The strict raw comparator is retained unchanged. The failed run is retained unchanged. It is not relabeled as PASS and is not deleted.

## What passed before the block

- frozen helper built successfully;
- Candidate A2 applied only inside the runner;
- strict workspace Clippy passed;
- `execsurface-observe` tests passed;
- full workspace tests passed;
- reference/reference raw parity passed for the deterministic cases reached before `restart`:
  - `irrelevant`;
  - `fileio`;
  - `failed-open`;
  - `signal`.

The workflow then reached `restart` and stopped because the two reference runs differed under the TID-only raw projection.

The activation diagnostic, bidirectional production baseline/diff parity, and all performance-value measurements were therefore skipped by design.

## Interpretation

The v2 `restart` fixture uses a child process to deliver `SIGUSR1` and later write to the pipe while the parent blocks in a restartable read. This creates real parent/child scheduling and event-order freedom. The raw comparator intentionally preserves event order and sequence, so two valid reference executions can differ even before a candidate is involved.

This is a **comparator-fixture counterexample**, not evidence that A2 is semantically equivalent and not evidence that A2 is semantically different.

No product claim follows from it.

## Prospective correction rule

The next run may replace only the concurrent `restart` fixture with a prospectively frozen single-process deterministic restart fixture. The old fixture and v2 failure remain preserved.

The replacement must:

1. use one process only;
2. install `SIGALRM` with `SA_RESTART`;
3. block in `read` on a pipe;
4. have the signal handler perform an async-signal-safe `write` of one byte to the same pipe;
5. require the blocked `read` to complete with that byte after restart;
6. execute a selected file operation afterward to verify that observer phase continues correctly;
7. introduce no candidate-specific normalization;
8. use the exact same frozen raw comparator semantics for every deterministic case.

The old concurrent `restart` mode remains in the helper for provenance and may still be used for non-raw-order stress testing; it is not silently removed.

## Authority boundary

- public runtime on `main`: unchanged;
- public release: `v0.1.0-alpha.3`;
- ptrace correctness authority: unchanged;
- eBPF authority: unchanged;
- Candidate A2 semantic eligibility: **OPEN / BLOCKED**;
- Candidate A2 performance value: **NOT EVALUATED**.
