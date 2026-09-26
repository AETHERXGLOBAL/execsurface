# M8.4 — Fail-Closed Loss, Lag, Teardown & Lifecycle Semantics

Date: 2026-09-26
Status: **M8.4a CLOSED / PROVED — M8.4b UNDER EXECUTION**
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

A real multi-threaded target generated a high rate of successful `openat` events against `/dev/null`. The reference CI run produced non-zero kernel-side ring-buffer drops without a synthetic force-loss flag, and the report emitted:

- `dropped_events > 0`;
- `completeness = incomplete_loss`;
- `observation_complete = false`;
- warning `producer_event_loss`.

The same run separately proved userspace event-budget truncation with `--event-limit 1`:

- `completeness = incomplete_limit`;
- `observation_complete = false`;
- warning `event_limit_exceeded`;
- zero producer drops in the truncation control.

Normal repository CI also remained green on the same branch HEAD. Early M8.4a runs that stopped before the experiment because of rustfmt and Rust-1.82 dependency-resolution setup defects are retained as negative/setup evidence; acceptance was based only on the later successful execution of the real failure gates.

## M8.4b — consumer lag and teardown lifecycle

**UNDER EXECUTION — no closure claim until dedicated CI passes.**

### Consumer lag

The experimental collector now accepts `--consumer-lag-ms N` solely as a controlled M8.4 adversarial input. The lag is recorded explicitly in the report and warning stream. Acceptance requires that a lag-induced producer overrun cannot be represented as clean evidence: real producer drops must remain visible and map to `incomplete_loss`.

### Teardown/lifecycle

The previous fixed post-root drain window was rejected as too weak: a descendant can continue after the root process exits and produce later runtime activity.

M8.4b therefore adds lifecycle-aware draining:

1. spawn events add launched tasks to an internal active set;
2. `sched_process_exit` supplies an internal task-exit control marker;
3. after the root exits, polling continues until all known descendants have exited and a quiescence window is observed;
4. a bounded lifecycle timeout prevents indefinite waiting;
5. timeout emits warning `lifecycle_drain_timeout` and `completeness = incomplete_lifecycle`.

The `sched_process_exit` signal is **control-plane health metadata only**. It is not emitted as a user-visible ProcessExit observation and does not upgrade the declared `process_exit` capability, which remains unsupported.

A dedicated teardown fixture forks a descendant that performs an `openat` after its parent/root has already exited. The positive gate requires that this late descendant activity is captured before report closure. A second run intentionally uses a timeout shorter than the descendant delay and must fail closed as `incomplete_lifecycle`.

## M8.4 files

- `experiments/m8-ebpf/libbpf-observer/src/main.rs`
- `experiments/m8-ebpf/libbpf-observer/src/bpf/observer.bpf.c`
- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_event_storm.rs`
- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_teardown_race.rs`
- `.github/workflows/m8-4-loss-semantics.yml`

These remain inside the experimental observer path. They do not change the default ptrace backend, baseline format, policy semantics, or release packaging.

## Remaining M8.4 gates

- **M8.4b:** consumer-lag and teardown/lifecycle CI evidence — OPEN until current workflow passes.
- **M8.4c:** attach/load/decode/lifecycle failure machine-readable evidence.
- **M8.4d:** independent red-team closure and regression preservation.

## Non-negotiable boundaries

- eBPF PASS authority: **NOT AUTHORIZED**.
- cross-backend parity: **OPEN / M8.5**.
- ptrace default/reference: **RETAINED**.
- missing/lost/uncertain events may never be interpreted as clean evidence.
- an internal lifecycle marker does not imply public ProcessExit capability.
