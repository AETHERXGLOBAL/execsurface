# M9-V1 — Same-Workload eBPF Value Experiment

Date: 2026-09-27
Status: **FROZEN BEFORE MEASUREMENT**
Source baseline: `main@a77f7ed7b2bed381bcdfe95605e4cd57f368685a`
Trigger: M9.1 Batch 2R accepted external workloads crossed the predeclared M6.5 ptrace performance trigger.

## Objective

Test only whether the retained research eBPF paths can provide a meaningful clean end-to-end latency improvement over current ptrace on the **same pinned external workloads and same GitHub runner host class** that reproduced the M6.5 trigger.

This is a value experiment, not a product-integration milestone.

## Authority boundary — unchanged

For every run and report:

- `ptrace_correctness_reference = true`
- `full_surface_comparable = false`
- `ebpf_pass_authorized = false`
- `product_integration_authorized = false`
- eBPF `learn` = not authorized
- eBPF `check` = not authorized
- ptrace/eBPF baseline interchangeability = not authorized
- backend auto-selection = not authorized
- public eBPF exposure = not authorized

No outcome of this experiment changes those flags automatically.

## Team / review roles

- Linux/eBPF Runtime Specialist — preserve kernel routing and lifecycle health semantics.
- Runtime Performance Engineer — exact end-to-end measurement and phase attribution.
- External Workload Validation Lead — pinned provenance and identical workload command.
- **Fixed Innovation Scientist / Systems Architect** — seek the smallest architecture that could create real user value.
- **Fixed Deviation Prevention / Scientific Integrity** — reject benchmark substitutions, threshold changes, and internal-only speed claims.
- **Independent Red Team** — attempt to show that an apparent speedup relies on missing events, weaker health gates, privilege asymmetry, or workload mismatch.

## Frozen workloads

Use the exact pinned projects and CLI commands accepted in M9.1 Batch 2R.

### V1-JUST

- upstream: `casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- setup: `cargo +1.90.0 build --locked --release`
- measured command from copied upstream root:
  - `/bin/bash -lc "./target/release/just --list >/dev/null"`

### V1-RIPGREP

- upstream: `BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- setup: `cargo +1.96.0 build --locked --release`
- measured command:
  - `/bin/bash -lc "./target/release/rg --files . >/dev/null"`

### V1-FD

- upstream: `sharkdp/fd`
- revision: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`
- setup: `cargo +1.90.0 build --locked --release --all-features`
- measured command:
  - `/bin/bash -lc "./target/release/fd --hidden --type f --exclude target . >/dev/null"`

External repositories are copied unchanged to `$RUNNER_TEMP` before build and measurement.

## Host / privilege comparability

- runner class: `ubuntu-24.04` GitHub-hosted runner;
- readable `/sys/kernel/btf/vmlinux` is mandatory for eBPF eligibility;
- all timed modes execute from one root harness so privilege context is identical;
- exact kernel, OS image, CPU count, BTF readability, toolchains, commit SHA, and binary SHA-256 values are recorded.

A workload is `INELIGIBLE`, not a performance win/loss, if eBPF cannot run cleanly on the assigned host.

## Modes

Measure on one runner/job for each workload:

1. `direct` — exact frozen command.
2. `ptrace` — current source `execsurface observe --` around the exact command.
3. `libbpf_per_invocation` — retained M8 libbpf collector around the exact command.
4. `persistent_two_session` — retained M8.7 persistent attachment, two sequential isolated executions of the exact command under one attachment lifetime.

The persistent implementation may receive a **research-only arbitrary-command adapter** solely to replace the old fixed fixture. The adapter must preserve the root-registration barrier by blocking the child before `exec`, registering the root epoch, then releasing it. It must not change kernel filters, lifecycle drain, event limits, loss handling, authority flags, or public product code.

## Sampling

For every mode:

- 3 untimed warmups;
- 15 timed samples;
- deterministic rotating mode order;
- monotonic clock;
- no outlier deletion or sample replacement.

Persistent mode:

- each invocation contains exactly two sequential sessions of the same workload;
- report both two-session amortized end-to-end latency and internal session latency;
- one-time open/load/attach/final-detach costs remain visible and are not silently subtracted.

## Eligibility / health gates

### ptrace

Every sample must:

- return command success;
- emit valid JSON;
- report `complete=true`;
- report target exit code 0 and no signal.

### libbpf per invocation

Every sample must:

- return success;
- avoid all hard incomplete states;
- report `dropped_events=0`;
- report lifecycle drain complete;
- report no collector failure.

### persistent

Every session must retain the M8.7 health gates:

- `clean_for_m8_7a=true`;
- `lifecycle_drain_complete=true`;
- `active_remaining=0`;
- `stale_epoch_events=0`;
- `integrity_errors=0`;
- `decode_error_delta=0`;
- `producer_drop_delta=0`;
- `routing_error_delta=0`;
- `event_limit_hit=false`;
- membership and pending-mechanism maps empty at session end;
- no unexpected event while no session is active;
- final maps empty;
- authority flags remain false as specified above.

The old fixture-specific exact event counts are **not** valid for arbitrary external workloads and therefore are replaced only by lifecycle-consistency gates, not weaker loss/integrity gates.

## Interpretation gates

For each workload separately:

- compare end-to-end medians on the same job/host;
- report direct, ptrace, per-invocation eBPF, persistent amortized, and persistent internal medians;
- report ratios against ptrace;
- retain raw samples and health evidence;
- do not aggregate away a workload that is slower or ineligible.

A candidate eBPF path demonstrates **value evidence** only if it is eligible and its end-to-end median is lower than ptrace on the same workload/job. Internal session latency alone cannot satisfy the value gate.

No minimum improvement percentage is invented after measurement. Magnitude is reported descriptively.

## Outcomes

- `COMPUTATIONAL_EVIDENCE — VALUE SIGNAL`: at least one eligible same-workload eBPF end-to-end mode is faster than ptrace; proceed only to separate M8.8 safety/product gates.
- `KILLED / NO CURRENT VALUE`: eligible eBPF end-to-end modes are not faster on the tested workloads; retain eBPF research-only and prioritize ptrace optimization / product evidence.
- `INELIGIBLE`: host or evidence-health gates prevent valid comparison; do not interpret timings.

## Prohibited shortcuts

- no eBPF `learn`, `check`, PASS, or policy authority;
- no baseline interchangeability;
- no public/backend auto-selection change;
- no removal/shortening of lifecycle drain or quiescence gates to improve numbers;
- no privilege asymmetry between timed modes;
- no test substitution after observing results;
- no threshold weakening;
- no outlier deletion;
- no hidden sample retries;
- no product release or public claim from this experiment alone.
