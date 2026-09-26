# M8.7c — Persistent Observer Performance Protocol

Date: 2026-09-26
Status: **FROZEN BEFORE MEASUREMENT**
Parent: `M8_7_PERSISTENT_OBSERVER.md`
Tracking: #46

## Objective

Measure whether the M8.7 persistent-attachment architecture materially amortizes the per-invocation libbpf lifecycle cost identified in M8.6, without changing evidence-health, lifecycle-drain, session-isolation, privilege, or authority rules.

This protocol is committed before interpreting any M8.7c benchmark numbers.

## Claim boundary

M8.7c may establish only exact-host measured performance for the tested prototype and workload.

It must not establish or imply:

- universal Linux speedup;
- production readiness;
- full semantic equivalence with ptrace;
- eBPF PASS authority;
- that eBPF is intrinsically faster than ptrace;
- that a two-session feasibility process is the final service architecture.

`ptrace` remains the correctness reference. `full_surface_comparable=false`, `ebpf_pass_authorized=false`, and `product_integration_authorized=false` remain mandatory.

## Benchmark subject

Use a dedicated neutral native benchmark fixture, separate from the adversarial 64-descendant lifecycle fixture.

The benchmark workload shall:

1. execute a fixed small process tree;
2. wait for all descendants before root exit;
3. avoid deliberate post-root reparenting delay;
4. perform no network I/O;
5. use no external service;
6. return deterministic success only after all children have completed.

The persistent path retains its root-registration barrier. That extra bootstrap exec is part of the measured prototype cost and is not subtracted.

## Modes

All timed modes run under the same root privilege context on the same runner:

1. `direct` — benchmark fixture only;
2. `ptrace` — current ExecSurface ptrace observation path;
3. `libbpf_per_invocation` — current experimental per-invocation libbpf collector;
4. `persistent_two_session` — M8.7 persistent observer running two sequential isolated sessions under one attachment lifetime.

The persistent result is reported in two forms:

- **internal session latency**: the machine-readable `elapsed_ms` reported for each clean session, which excludes one-time open/load/attach/final-detach;
- **two-session amortized end-to-end latency**: measured wall time of the persistent observer invocation divided by exactly two sessions. This intentionally charges half of the one-time final detach to each session and makes no infinite-session extrapolation.

No result may extrapolate beyond the measured two-session amortization.

## Sampling

For each non-persistent single-session mode:

- 3 untimed warmups;
- 15 timed samples.

For persistent mode:

- 3 untimed complete two-session invocations;
- 15 timed complete two-session invocations;
- 30 internal session-latency samples are therefore available, but paired-session structure must remain visible in raw evidence.

Timed mode order rotates deterministically to reduce ordering bias. No outlier is deleted.

Clock: monotonic `time.perf_counter_ns` for external wall measurements; internal session timings remain the Rust monotonic `Instant` measurements emitted by the prototype.

Derived statistics:

- median;
- minimum;
- maximum;
- median absolute deviation (MAD);
- nearest-rank p90.

## Eligibility / health gates

A timed sample is eligible only if the relevant execution succeeds and all applicable evidence-health gates are clean.

For `ptrace`:

- command succeeds;
- observation reports complete evidence.

For `libbpf_per_invocation`:

- command succeeds;
- no hard incomplete loss/limit/decode/collector/lifecycle state;
- `dropped_events=0`;
- lifecycle drain complete.

For `persistent_two_session` every reported session must satisfy:

- `clean_for_m8_7a=true`;
- `lifecycle_drain_complete=true`;
- `active_remaining=0`;
- `stale_epoch_events=0`;
- `integrity_errors=0`;
- `decode_error_delta=0`;
- `producer_drop_delta=0`;
- `routing_error_delta=0`;
- `event_limit_hit=false`;
- membership and pending-mechanism maps empty;
- no unexpected event while no session is active;
- final maps empty;
- authority flags remain false for eBPF PASS and product integration.

If any timed sample fails its health gate, the benchmark is **INELIGIBLE**, not a performance win/loss.

## Interpretation gates

M8.7c may conclude that persistent attachment materially changes measured per-session cost only if:

1. all samples are eligible;
2. raw JSON evidence is retained;
3. the exact runner/kernel/BTF context is recorded;
4. the per-invocation and persistent measurements are produced in the same workflow on the same host;
5. the two-session amortized result is compared directly with the current per-invocation libbpf result;
6. internal session latency is reported separately from amortized end-to-end latency;
7. detach cost is never silently excluded from the amortized end-to-end result.

The result may be faster, slower, or inconclusive. No threshold will be changed after seeing measurements.
