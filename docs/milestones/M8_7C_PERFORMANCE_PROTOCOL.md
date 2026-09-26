# M8.7c — Persistent Observer Performance Protocol

Date: 2026-09-26
Status: **MEASURED / ACCEPTED — EXACT-HOST TWO-SESSION EVIDENCE ONLY**
Parent: `M8_7_PERSISTENT_OBSERVER.md`
Tracking: #46

## Objective

Measure whether the M8.7 persistent-attachment architecture materially amortizes the per-invocation libbpf lifecycle cost identified in M8.6, without changing evidence-health, lifecycle-drain, session-isolation, privilege, or authority rules.

The protocol below was committed before interpreting any M8.7c benchmark numbers. The result section was appended only after the frozen protocol completed successfully.

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

## Accepted execution evidence

Evidence commit: `bf0bac374ef0fefef438b6dc70e89b8c2b42963d`

- general CI: **PASS** — run `36264634501`;
- M8.7 Persistent Performance: **PASS** — run `36264634525`;
- raw artifact ID: `10914040205`;
- artifact ZIP SHA-256: `d26ae806bd6dbc0a437b1ab95434d73d99e787c614b93042babc9aa7f063caee`.

Exact measured host:

- Ubuntu 24.04.5 LTS;
- GitHub runner image `20260920.314.1`;
- kernel `6.17.0-1022-azure`;
- x86_64;
- 4 logical CPUs;
- readable kernel BTF;
- all timed modes executed under the same root benchmark process.

Every timed sample passed the applicable evidence-health gates. No timed sample was removed.

### Measured medians

| Mode | Samples | Median ms | MAD ms | p90 ms |
|---|---:|---:|---:|---:|
| direct | 15 | 2.858403 | 0.021842 | 2.948975 |
| ptrace | 15 | 10.020306 | 0.218622 | 11.281625 |
| libbpf per invocation | 15 | 607.278495 | 7.848262 | 625.856694 |
| persistent two-session amortized | 15 paired invocations | 505.770932 | 2.667380 | 513.786955 |
| persistent internal session | 30 sessions | 113.918489 | 0.075004 | 114.174884 |

Measured ratios:

- persistent two-session amortized / per-invocation libbpf = `0.832848`;
- persistent internal session / per-invocation libbpf = `0.187589`;
- persistent two-session amortized / ptrace = `50.474599`;
- per-invocation libbpf / ptrace = `60.604785`.

Therefore, on this exact host and workload:

- the measured two-session amortized persistent path reduced median end-to-end cost by about **16.7%** versus the current per-invocation libbpf path;
- the internal persistent session median was about **81.2% lower** than the per-invocation libbpf median, showing that one-time lifecycle cost is materially separated from per-session work;
- the conservative two-session amortized persistent result remained about **50.5× the ptrace median** and therefore does **not** establish a competitive end-to-end replacement for ptrace;
- the internal session median remained about `113.9 ms`, so removing per-invocation detach alone does not remove all persistent-path latency.

The measurements are consistent with M8.6's conclusion that lifecycle teardown dominates the current per-invocation architecture, while also showing a second remaining per-session latency floor. The current persistent prototype deliberately retains the established lifecycle/quiescence semantics; M8.7c does not authorize shortening those gates merely to improve benchmark numbers.

## M8.7c decision

**ACCEPTED for narrow exact-host measurement only.**

Established:

1. persistent attachment changes the measured cost structure and amortizes part of the per-invocation teardown penalty;
2. the two-session conservative end-to-end result is measurably lower than current per-invocation libbpf on the tested host;
3. one-time detach remains large and is still visible in the two-session amortized result;
4. a separate per-session latency floor remains and requires explanation before any stronger performance claim;
5. ptrace remains substantially faster end-to-end on this tested workload;
6. all authority boundaries remain unchanged.

Not established:

- performance for longer-lived persistent services or larger session counts;
- universal Linux performance;
- production readiness;
- full semantic equivalence;
- eBPF PASS authority;
- product integration authorization.
