# M8.4 — Fail-Closed Loss, Lag, Teardown & Lifecycle Semantics

Date: 2026-09-26
Status: **M8.4a–M8.4b CLOSED / PROVED — M8.4c UNDER EXECUTION**
Tracking: #40
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
M8.3 closure: `docs/milestones/M8_3C_COLLECTOR.md`, `docs/milestones/M8_3C2_CLI_BRIDGE.md`

## Objective

Deepen the experimental libbpf observation path from “loss is representable” to “controlled loss and lifecycle failure channels are reproducibly detected and fail closed.”

M8.4 does not add eBPF PASS authority and does not perform ptrace/eBPF parity evaluation. Those remain blocked until later gates.

## Fixed governance roles

- **Innovation Scientist / Architect:** seek stronger failure-observability mechanisms and adversarial experiments without weakening semantics.
- **Deviation Prevention / Scientific Integrity:** reject any interpretation of absent events as clean evidence and prevent scope drift into M8.5 parity.

## M8.4a — real producer loss and truncation

**CLOSED / PROVED on the declared GitHub-hosted Ubuntu 24.04 reference environment.**

The collector increments a per-CPU `dropped` counter when `bpf_ringbuf_reserve()` fails and maps `dropped_events > 0` to `incomplete_loss`.

A real multi-threaded target generated a high rate of successful `openat` events against `/dev/null`. The reference CI produced non-zero kernel-side ring-buffer drops without a synthetic force-loss flag and emitted `incomplete_loss`, `observation_complete=false`, and warning `producer_event_loss`.

The same gate separately proved userspace event-budget truncation with `--event-limit 1`: `incomplete_limit`, `observation_complete=false`, `event_limit_exceeded`, and zero producer drops in the control case.

Early M8.4a runs that stopped before the experiment because of rustfmt and Rust-1.82 dependency-resolution setup defects remain part of the negative/setup record. Closure is based only on the later successful real-failure run.

## M8.4b — consumer lag and teardown lifecycle

**CLOSED / PROVED on the declared reference environment.**

### Controlled consumer lag

`--consumer-lag-ms 150` was applied before ring-buffer polling while a multi-threaded target generated `openat` traffic. The run produced real producer drops and was classified `incomplete_loss`; warning codes included both `consumer_lag_injected` and `producer_event_loss`. A lagging consumer therefore cannot silently become clean evidence.

### Post-root descendant drain

The previous fixed eight-poll post-root window was rejected as insufficient. M8.4b replaced it with lifecycle-aware draining:

1. spawn events add launched tasks to an internal active set;
2. `sched_process_exit` supplies an internal task/TID exit control marker;
3. after the root exits, polling continues until known descendants have exited and a quiescence window is observed;
4. a bounded timeout prevents indefinite waiting;
5. timeout emits `lifecycle_drain_timeout` and `incomplete_lifecycle`.

The positive teardown fixture proved that a descendant performing `openat` after the root parent exited was still captured before report closure. The adversarial short-timeout run proved the same situation becomes `incomplete_lifecycle` rather than a false clean completion.

The internal exit marker is **control-plane health metadata only**. It is not emitted as a user-visible ProcessExit observation and does not upgrade the declared `process_exit` capability, which remains unsupported.

All M8.4b substantive gates passed. One intermediate aggregate-harness run failed only because the final Python command omitted stdin-script marker `-`; every semantic gate had already passed. The harness syntax was corrected without changing thresholds or semantics, and the complete workflow then passed. Normal repository CI remained green.

## M8.4c — machine-readable collector failure states

**UNDER EXECUTION — no closure claim until CI passes.**

Current target:

- pre-target BPF open/load/attach failure must write a machine-readable report and must not run the target;
- malformed/decode failure must remain explicit and produce incomplete evidence rather than disappearing into an abrupt poll error;
- lifecycle failure remains machine-readable as established in M8.4b;
- the experimental CLI bridge must accept authorized incomplete states while continuing to reject complete/PASS-eligible claims;
- no ptrace fallback is permitted.

Proposed completeness additions are narrowly scoped to the experimental path: `incomplete_collector`, `incomplete_decode`, and the already-proved `incomplete_lifecycle`.

## M8.4 files

- `experiments/m8-ebpf/libbpf-observer/src/main.rs`
- `experiments/m8-ebpf/libbpf-observer/src/bpf/observer.bpf.c`
- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_event_storm.rs`
- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_teardown_race.rs`
- `.github/workflows/m8-4-loss-semantics.yml`

These remain inside the experimental observer path except for narrowly bounded validation logic in the observation-only CLI bridge. They do not change the default ptrace backend, baseline format, policy semantics, or release packaging.

## Remaining M8.4 gates

- **M8.4c:** attach/load/decode/lifecycle failure machine-readable evidence — UNDER EXECUTION.
- **M8.4d:** independent red-team closure and regression preservation.

## Non-negotiable boundaries

- eBPF PASS authority: **NOT AUTHORIZED**.
- cross-backend parity: **OPEN / M8.5**.
- ptrace default/reference: **RETAINED**.
- missing/lost/uncertain events may never be interpreted as clean evidence.
- an internal lifecycle marker does not imply public ProcessExit capability.
