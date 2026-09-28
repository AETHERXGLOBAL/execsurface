# M10.2 — FD-SHARE-RACE-001 Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer`
Status: **PREREGISTERED — EXECUTION NOT YET INTERPRETED**

## Team specialization for this gate

Fixed roles remain:
- **Innovative Systems Architect** — search for stronger object-grounded fd evidence.
- **Anti-Deviation / Skeptical Reviewer** — prevent either ptrace or BPF from being promoted without evidence.

Dynamic M10.2 specialists:
- Linux files_struct / fdtable / `CLONE_FILES` specialist;
- ptrace multi-thread event-ordering engineer;
- VFS/file-object lifetime specialist;
- concurrency adversary engineer;
- Rust evidence-harness engineer;
- C/Linux fixture engineer;
- evidence-semantics reviewer;
- CI/reproducibility engineer;
- internal adversarial red team.

## Question

Can the current ptrace observer's `FileDescriptorAccess(Read)` path attribution disagree with the object a target actually read when threads sharing one Linux fd table concurrently close and reuse the same numeric fd?

M10.1 showed that entry-time pathname intent is not universally kernel-authoritative. M10.2 now attacks the stronger post-success fd attribution separately.

## Hypotheses

### H0 — shared-fd attribution integrity

For an accepted complete warning-free run, the ordered sequence of relevant ExecSurface `FileDescriptorAccess(Read)` A/B identities is identical to the ordered sequence of bytes actually returned by the target's successful A/B reads.

### H1 — shared-fd attribution race

Under `CLONE_FILES` close/reuse concurrency, at least one accepted complete run can contain either:

1. a relevant emitted fd-read path whose A/B identity differs from the byte actually returned by that read; or
2. a count divergence between successful A/B reads proven by the target and relevant ExecSurface fd-read events, despite `Observation.complete == true` and no warnings.

One accepted divergence is sufficient to kill universal H0 for this proposition.

## Fixture design

Two files are created in an isolated directory:

- `A` contains byte `A`;
- `B` contains byte `B`.

The target:

1. opens `A` and records the returned numeric fd `N`;
2. creates a pthread mutator, which shares the process fd table (`CLONE_FILES` semantics);
3. the mutator repeatedly:
   - closes `N`;
   - opens alternating `A` and `B`;
   - if needed, duplicates the replacement back onto `N` and closes the temporary fd;
4. concurrently, the main thread repeatedly executes `pread(N, ..., 1, 0)`;
5. every successful one-byte read is appended **in userspace memory** to an ordered ground-truth buffer as `A` or `B`;
6. after the race stops and the mutator joins, the target writes that ground-truth byte sequence to `fd_truth.log`;
7. no mutator read of A/B is permitted, so relevant A/B `FileDescriptorAccess(Read)` events originate only from the measured main-thread reads.

The truth log is written only after concurrency ends and is not itself part of the A/B read-event filter.

## Why this attacks the current design

The current observer pairs each successful positive-byte read with an internal fd-table lookup at ptrace syscall exit. The fd table is updated from observed open/close/dup lifecycle across threads that can share the same Linux `files_struct`.

The adversary attempts to create inter-thread event-order/snapshot pressure around the same numeric fd while preserving an independently observed returned byte for each successful read.

## Frozen workload

One target execution per workflow run.

Default values:
- main-thread read attempts: `5000`;
- mutator alternates A/B continuously until the main thread finishes;
- event budget: unchanged public default.

## Eligibility

Primary interpretation requires:

- `Observation.complete == true`;
- zero observer warnings;
- target exit code `0`;
- truth log contains at least `50` successful A/B reads;
- every truth byte is exactly `A` or `B`;
- relevant emitted fd-read paths are filtered only by basename `A`/`B` and operation `Read`.

If any eligibility condition fails, classify `HARNESS_INCONCLUSIVE` and preserve the failure.

## Primary comparison

Let:

- `T = [t0, t1, ...]` be the target ground-truth successful-read byte sequence;
- `E = [e0, e1, ...]` be the ordered ExecSurface relevant fd-read A/B path sequence.

Primary counters:
- `truth_successful_read_count = |T|`
- `emitted_fd_read_count = |E|`
- `paired_identity_mismatch_count` over paired positions
- `count_delta = |E| - |T|`

## Frozen classifications

### `PTRACE_FD_ATTRIBUTION_COUNTEREXAMPLE_OBSERVED`

Requirements:
- eligibility passes; and
- either `|E| != |T|` or at least one paired A/B identity mismatch exists.

Consequence:
- universal H0 is **KILLED** for shared-fd close/reuse concurrency;
- current fd attribution requires a narrower authority statement and/or stronger kernel-object anchoring.

### `NO_FD_ATTRIBUTION_MISMATCH_OBSERVED_NOT_PROOF`

Requirements:
- eligibility passes;
- `|E| == |T|`;
- paired mismatch count `0`.

Consequence:
- H0 remains **OPEN**, not proved universally;
- this becomes positive bounded evidence for current fd tracking under the exact adversary.

### `HARNESS_INCONCLUSIVE`

Eligibility fails or evidence cannot be compared without ambiguity.

## Non-claims

M10.2 alone cannot establish:
- BPF-LSM correctness;
- universal fd attribution correctness;
- real-workload race frequency;
- that any count/identity divergence is exploitable;
- authorization to change the public backend.

## Preservation

The evidence artifact must retain:
- source SHA and kernel identity;
- complete/warning state;
- target outcome;
- truth and emitted sequence lengths;
- bounded sequence prefixes around mismatches;
- mismatch/count metrics;
- frozen classification.
