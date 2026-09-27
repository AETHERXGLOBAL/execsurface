# Post-M9 — eBPF E4 Same-Workload Value Requalification Protocol

Date: 2026-09-27
Status: **PREREGISTERED — NOT YET EXECUTED**
Tracking: GitHub issue #85
Baseline evidence:

- E1 historical lifecycle failure attribution complete;
- E2 persistent real-workload health PASS;
- E3 shared lifecycle proposition parity PASS.

## Objective

Measure whether the unchanged M8.7 persistent eBPF architecture provides material end-to-end execution-observation value against the current public ptrace correctness-reference backend on the same pinned real workloads that triggered Post-M9 performance work.

This is a bounded performance requalification gate, not a public-backend promotion gate.

## Immutable authority boundary

Regardless of E4 timing outcome:

- native ptrace remains the public/default correctness-reference backend;
- eBPF remains research-only;
- eBPF `learn` is not authorized;
- eBPF `check` is not authorized;
- eBPF PASS authority is not authorized;
- backend auto-selection is not authorized;
- ptrace/eBPF baseline interchangeability is not authorized;
- `full_surface_comparable` remains false;
- no universal or cross-workload speed claim is authorized.

E4 may establish only a bounded architecture-value result for the exact frozen workloads/runner class.

## Frozen targets

### E4-JUST

Repository/revision:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Command:

```text
cargo +1.90.0 test --all >/dev/null 2>&1
```

### E4-FZF

Repository/revision:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Command:

```text
go test ./... >/dev/null 2>&1
```

## Compared modes

Exactly three modes are timed:

1. **direct** — `/bin/bash -lc <frozen command>`;
2. **ptrace** — public `execsurface observe -- /bin/bash -lc <frozen command>`;
3. **persistent_eBPF_two_session** — unchanged M8.7 persistent observer running two barrier-controlled copies of the same `/bin/bash -lc <frozen command>` within one attached observer lifetime.

The old M8.4d per-invocation libbpf collector is **not** part of E4. Its historical failures remain negative evidence.

## Persistent command adapter

A runner-local barrier adapter may implement only the existing M8.7 fixture contract:

1. wait for the root-registration release byte;
2. close the barrier descriptor;
3. `exec /bin/bash -lc <frozen command>`.

The adapter does not change the workload and is included in persistent end-to-end wall time.

## Privilege and environment

All timed modes run from one root harness so privilege context is not a confounder.

The harness passes only the environment required to execute the already-primed pinned workloads (PATH/toolchain/cache paths and the frozen command). No privilege auto-escalation is added to the public product.

## Priming

Before any timing sample:

- exact pinned revision is verified;
- tracked working tree is clean;
- frozen workload command runs successfully once outside the timing set;
- public ptrace CLI is built once;
- unchanged persistent observer is built once;
- adapter is built once.

Build/setup time is not part of observation runtime timing.

## Sampling

Per target:

- warmups per mode: `3`;
- measured samples per mode: `15`;
- no outlier removal;
- no sample replacement;
- monotonic wall clock (`perf_counter_ns` equivalent);
- one job / one GitHub-hosted Ubuntu 24.04 runner for all compared modes of that target;
- measured mode order rotates across `direct`, `ptrace`, `persistent_eBPF_two_session` so one mode is not systematically first or last.

A failed or unhealthy warmup kills the gate for that target before accepted measurement.

A failed or unhealthy measured sample is preserved and kills the value gate; it is not rerun or replaced.

## Ptrace sample health prerequisite

Every ptrace warmup and measured sample must have:

- process return code `0`;
- valid observation JSON;
- `complete == true`;
- warnings exactly empty;
- target outcome exit `0`, no signal;
- backend remains the public ptrace reference.

## Persistent eBPF sample health prerequisite

Every persistent warmup and measured invocation must:

- return `0`;
- contain exactly two sessions;
- have both target outcomes exit `0`, no signal;
- have both sessions `clean_for_m8_7a == true`;
- lifecycle drain complete;
- active remaining `0`;
- stale epoch events `0`;
- integrity errors `0`;
- decode error delta `0`;
- producer drop delta `0`;
- routing error delta `0`;
- event limit not hit;
- membership map empty;
- pending mechanism map empty;
- final maps empty;
- `unexpected_without_session == 0`;
- `decode_errors_total == 0`;
- authority block unchanged.

No eBPF timing from incomplete evidence is admissible.

## Metrics

For each target calculate:

### Direct

- median wall time;
- MAD;
- min/max;
- p90 nearest-rank.

### Ptrace

Same wall metrics.

### Persistent eBPF primary metric

For each persistent invocation:

```text
amortized_end_to_end_per_session_ms = total_persistent_invocation_wall_ms / 2
```

The primary eBPF summary is the median/MAD/min/max/p90 of those 15 amortized values.

This deliberately includes:

- persistent observer process startup;
- BPF open/load/attach;
- two barrier adapters;
- both target executions;
- lifecycle drains;
- final detach/report overhead;

and amortizes those costs over only **two** sessions. No one-time cost is subtracted.

### Secondary diagnostic metric

Record the 30 internal `session.elapsed_ms` values and summarize them separately. They are diagnostic only and cannot satisfy the acceptance threshold if the primary amortized metric fails.

## Frozen value threshold

For each target independently:

```text
reduction_percent = 100 * (ptrace_median - persistent_amortized_median) / ptrace_median
```

E4 establishes architecture value only if, on **both** pinned targets:

- persistent amortized median < ptrace median; and
- `reduction_percent >= 10.0`.

No averaging across targets is permitted. A large win on one target cannot compensate for a failure on the other.

The `10%` threshold may not be relaxed after observing measurements.

## Classification

### `E4_BOUNDED_VALUE_ESTABLISHED`

Only if both targets pass the frozen threshold and all health gates.

### `E4_VALUE_NOT_ESTABLISHED`

If either target fails the 10% value threshold while health remains valid.

### `E4_BLOCKED_BY_INCOMPLETE_EVIDENCE`

If any required ptrace or eBPF health/completeness prerequisite fails before an admissible result can be formed.

All outcomes and artifacts are retained.

## Interpretation boundary

Even `E4_BOUNDED_VALUE_ESTABLISHED` would mean only:

> On these exact pinned real workloads on the declared GitHub-hosted Ubuntu 24.04 run, the research-only persistent eBPF lifecycle architecture reduced median amortized end-to-end observation wall time by at least 10% relative to the current public ptrace reference, while its declared lifecycle evidence remained complete and healthy.

It would not authorize public integration. A separate productization/authority gate would still be required.
