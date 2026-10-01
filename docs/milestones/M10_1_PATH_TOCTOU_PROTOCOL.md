# M10.1 — PATH-TOCTOU-001 Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer`
Status: **PREREGISTERED — EXECUTION NOT YET INTERPRETED**

## Question

Can the current ptrace observer's `PathAccessIntent` pathname differ from the object the Linux kernel actually opens when another thread mutates the same userspace pathname buffer while the syscall-entering thread is ptrace-stopped?

## Hypotheses

### H0 — ptrace pathname authority

For every accepted complete test run, the pathname copied by ExecSurface at syscall entry identifies the same A/B test object that the kernel subsequently opens.

### H1 — userspace-pointer TOCTOU exists

At least one accepted complete run can contain:
- a ptrace-observed `FilePathAccess(Open)` naming A while the target proves the opened object was B; or
- a ptrace-observed `FilePathAccess(Open)` naming B while the target proves the opened object was A.

One accepted mismatch is sufficient to **kill universal H0** for this proposition. It does not prove mismatch frequency, exploitability, or that all ptrace evidence is invalid.

## Fixture design

The fixture uses two regular files in an isolated temporary directory:

- `A` contains one byte: `A`
- `B` contains one byte: `B`

A target process creates two threads sharing a two-byte pathname buffer (`'A'`/`'B'` plus NUL):

1. a mutator thread continuously toggles the first pathname byte between `A` and `B` using userspace stores only;
2. an opener thread (the process main thread) waits until mutation is active, then performs exactly one `open()` using the shared buffer;
3. the target reads one byte from the returned fd and exits with a ground-truth code:
   - `11` = kernel-opened object contained `A`;
   - `12` = kernel-opened object contained `B`;
   - other code = fixture/error state.

The mutator deliberately performs no syscall in its hot loop, so it can continue changing shared memory while the opener thread is stopped at ptrace syscall entry.

## Observer under test

The experiment invokes the **unchanged** public library boundary:

`execsurface_observe::observe_command()`

No patch to `crates/execsurface-observe` is permitted in M10.1.

For each run the evaluator extracts:

- exactly one relevant `FilePathAccess { operation: Open }` whose basename is `A` or `B`;
- any relevant `FileDescriptorAccess { operation: Read }` whose basename is `A` or `B`;
- `Observation.complete`;
- warnings;
- target exit code.

`FilePathAccess` is the proposition under attack. `FileDescriptorAccess(Read)` is retained as a secondary diagnostic because it is derived after successful open/fd attribution and may agree with kernel truth even when entry-time pathname intent does not.

## Run count

Default preregistered run count: `500` accepted attempts on one GitHub-hosted Linux x86_64 run.

A later retry may increase N only if this first execution is classified `NO_MISMATCH_OBSERVED_NOT_PROOF`; the first result must remain preserved.

## Acceptance rules

A run is eligible for the primary mismatch count only when:

- `Observation.complete == true`;
- no observer warning exists;
- target exit code is `11` or `12`;
- exactly one relevant A/B `FilePathAccess(Open)` is present.

Runs failing these conditions are counted separately and cannot be used to claim H0 or H1.

## Primary metric

`path_intent_vs_kernel_truth_mismatch_count`

Mapping:
- exit `11` => ground truth A
- exit `12` => ground truth B

If observed A != truth B, or observed B != truth A, increment mismatch count.

## Secondary metric

`fd_read_vs_kernel_truth_mismatch_count`

This metric is diagnostic only for M10.1 and does not decide the gate.

## Frozen classifications

### `PTRACE_PATH_TOCTOU_COUNTEREXAMPLE_OBSERVED`

Requirements:
- eligible runs >= 1;
- primary mismatch count >= 1.

Consequence:
- universal H0 is **KILLED**;
- current `PathAccessIntent` must be documented as observed userspace argument metadata, not universally kernel-authoritative object identity;
- M10 continues toward kernel-hook/object-grounded evidence.

### `NO_MISMATCH_OBSERVED_NOT_PROOF`

Requirements:
- eligible runs >= 1;
- primary mismatch count == 0.

Consequence:
- H0 remains **OPEN**, not proved;
- absence of a sampled race is not evidence that the race is impossible.

### `HARNESS_INCONCLUSIVE`

Used when no eligible run exists or fixture/observer health prevents interpretation.

## Non-claims

M10.1 alone cannot establish:
- that BPF LSM is correct or deployable;
- that ptrace should be removed;
- frequency or security severity of the race in real workloads;
- fd identity race behavior (`FD-SHARE-RACE-001` is separate);
- baseline incompatibility;
- any public product regression.

## Preservation

The workflow must retain a JSON result artifact containing:
- branch/source SHA;
- kernel/OS identity;
- requested and eligible run counts;
- incomplete/warning/error counts;
- A/B truth and observed distributions;
- primary and secondary mismatch counts;
- bounded mismatch examples;
- frozen classification.
