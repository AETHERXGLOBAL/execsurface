# Post-M9 — Ptrace Fast-Path A2 External Value Protocol

Date: 2026-09-27
Status: **PREREGISTERED BEFORE VALUE EXECUTION**
Parent: GitHub issue #84
Candidate: A2
Semantic prerequisite: `docs/milestones/POST_M9_PTRACE_FASTPATH_A2_PARITY_V5_RESULT.md`

## Objective

Determine whether Candidate A2 produces a reproducible, practically meaningful reduction in native ptrace observer wall time on the same two pinned external runtime workloads that established the Post-M9 cost signal, without changing their observed production semantics.

This is an exact-host/workload engineering value experiment. It is not a universal Linux performance benchmark and does not itself authorize release.

## Frozen implementations

### Reference

Build the current `main` source **before** applying Candidate A2 and copy the resulting release `execsurface` binary to an immutable runner-local path.

### Candidate A2

Apply the already-reviewed deterministic patcher:

`experiments/post_m9_phase_aware/apply_candidate_a2.py`

Then build a second release `execsurface` binary. The target projects are built once and shared by both observer binaries.

No other runtime source change is allowed between reference and candidate.

## Frozen external targets

### PTRACE-A2-VALUE-RG-001

Repository:

`BurntSushi/ripgrep`

Revision:

`3fce3b5bb0236da2df6d99672afb8a719642eca7`

Build:

`cargo +1.96.0 build --locked --release`

Runtime command:

`cd '$WORK_ROOT' && ./target/release/rg --no-heading --line-number 'ripgrep' README.md >/dev/null`

### PTRACE-A2-VALUE-FZF-001

Repository:

`junegunn/fzf`

Revision:

`b1be3a8be1b833ce5b92fbbac11637643d60a046`

Checkout requires full history/tags, matching the accepted M9/Post-M9 harness.

Build:

`make`

Runtime command:

`cd '$WORK_ROOT' && printf 'alpha\nbeta\ngamma\n' | ./target/fzf-linux_amd64 --filter=beta --select-1 --exit-0 >/dev/null`

## Host and toolchain

- runner: `ubuntu-24.04` GitHub-hosted runner;
- reference and A2 are measured in the same job for each target;
- ExecSurface toolchain: Rust `1.90.0`;
- ripgrep toolchain: Rust `1.96.0`;
- timeout per observed run: `120 s`;
- no CPU affinity or governor claim unless explicitly measured and recorded.

Record kernel, architecture, CPU model, toolchain versions and exact source SHAs.

## Semantic prerequisite inside each value job

Before timing is accepted:

1. direct target command exits 0;
2. reference observation is complete, warning-free and exits 0;
3. A2 observation is complete, warning-free and exits 0;
4. reference baseline -> reference unchanged check yields zero diff findings;
5. reference baseline -> A2 unchanged check yields zero diff findings;
6. A2 baseline -> reference unchanged check yields zero diff findings.

Use the unchanged public `learn` / `check --diff-only --json` implementation. No policy suppression or new normalization is allowed.

If this prerequisite fails, timing value for that target is **not accepted**.

## Warmup and measured samples

Per target:

- reference warmups: `3`;
- A2 warmups: `3`;
- reference measured samples: `15`;
- A2 measured samples: `15`.

### Frozen order

Warmups:

1. reference -> A2
2. A2 -> reference
3. reference -> A2

Measured pairs:

- odd pair number: reference -> A2;
- even pair number: A2 -> reference.

No clean sample is removed. Timeouts/failures remain recorded and invalidate the positive value gate rather than being silently excluded.

## Timed operation

Primary timing is end-to-end observer wall time for:

`execsurface observe -- /bin/bash -lc '<frozen runtime command>'`

Each sample JSON must independently satisfy:

- process return code 0;
- `complete = true`;
- warnings = `[]`;
- target exit code `0`;
- target signal `null`.

Timing includes the current normal observer process overhead but excludes project build/install time and excludes any `strace` diagnostic run.

## Statistics

For each mode/target retain all raw wall-time samples and report:

- median;
- median absolute deviation (MAD);
- candidate absolute median reduction;
- candidate percentage median reduction:

`100 * (reference_median - candidate_median) / reference_median`.

No outlier removal.

## Predeclared value acceptance gate

Candidate A2 establishes engineering value only if **both** pinned targets independently satisfy all of:

1. semantic prerequisite PASS;
2. all 3 + 15 A2 observations complete and warning-free;
3. all 3 + 15 reference observations complete and warning-free;
4. A2 median ptrace observer wall time < reference median;
5. A2 median ptrace observer wall-time reduction >= **10%**;
6. no sample was removed or replaced after seeing timing results.

If one target is below 10%, classify:

`SEMANTICALLY_VALID / VALUE_NOT_ESTABLISHED`

unless a stronger negative condition applies.

Do not lower the threshold after results are known.

## Secondary diagnostics

After primary timings are complete, an optional non-timed `strace` diagnostic may count `PTRACE_GET_SYSCALL_INFO` requests on reference and A2 for attribution support. It may not replace the primary wall-time gate and must be labeled diagnostic/perturbed.

## Prohibited shortcuts

- no different target build for reference and A2;
- no target source modification;
- no sample exclusion based on speed;
- no threshold change;
- no baseline/policy/normalizer change;
- no concurrent parallel execution of reference and A2 samples;
- no eBPF comparison/substitution;
- no merging A2 before this experiment passes.

## After a PASS

A PASS makes A2 eligible for a separate merge/integration gate. It does not automatically:

- modify `main` runtime source;
- publish a new release;
- justify universal performance wording;
- change eBPF authority.

Any public wording must remain bounded to exact measured workloads/host class until broader evidence exists.
