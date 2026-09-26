# M8.4 — Fail-Closed Loss, Lag, Teardown & Lifecycle Semantics

Date: 2026-09-26
Status: **CLOSED / PROVED — M8.5 NEXT**
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

**CLOSED / PROVED.**

The collector now preserves explicit failure evidence instead of collapsing important failure channels into stderr-only errors:

- pre-target BPF open/load/attach/ring-buffer setup failure writes a report with `completeness=incomplete_collector`, `observation_complete=false`, `root_pid=null`, `collector_failure.target_started=false`, and no target execution;
- malformed/decode failure is recorded as `incomplete_decode` with explicit warning evidence rather than terminating polling before a report is written;
- lifecycle timeout remains `incomplete_lifecycle` as established in M8.4b;
- all failure reports remain ineligible for PASS.

A controlled unprivileged run proved that BPF setup failure occurs before the target is launched and still produces machine-readable evidence. A controlled decode-failure run proved that malformed-event state is surfaced without being mistaken for clean evidence.

The current M8.3 CLI bridge intentionally remains an observation-only experimental integration surface. Product-level surfacing of the expanded M8.4 incomplete-state vocabulary is reserved for **M8.7 public-alpha integration**, where CLI/schema/documentation/release compatibility are evaluated together. This does not weaken M8.4 acceptance: Issue #40 requires the collector failure classes themselves to be explicit, machine-readable, bounded, and fail-closed.

## M8.4d — independent red-team closure

**CLOSED / PROVED.**

The independent red-team review found one material lifecycle gap before merge: a `ring.poll()` failure after target launch could previously return abruptly before writing a machine-readable report and could leave the launched workload running.

The design was hardened before closure:

1. the experimental target is launched in its own process group;
2. post-start collector failure writes `incomplete_collector` with `collector_failure.target_started=true`;
3. the target process group is terminated and the root is reaped before returning failure;
4. the report records whether termination was completed;
5. actual runtime poll and post-root poll failures route through the same fail-closed path;
6. drop-counter read failure after target execution also routes through machine-readable collector failure;
7. a controlled post-start failure test launches a background descendant and proves neither the root completion sentinel nor descendant leak sentinel can appear after containment.

The final M8.4 adversarial workflow passed all gates together:

- pre-target collector failure safety;
- post-start process-group containment;
- event-budget truncation;
- real kernel-side ring-buffer loss;
- controlled consumer lag;
- post-root descendant drain;
- lifecycle timeout;
- decode failure;
- aggregate no-PASS authority preservation.

Normal repository CI on the same implementation also passed Format, Clippy, Tests, and Lockfile integrity.

## M8.4 files

- `experiments/m8-ebpf/libbpf-observer/src/main.rs`
- `experiments/m8-ebpf/libbpf-observer/src/bpf/observer.bpf.c`
- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_event_storm.rs`
- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_teardown_race.rs`
- `.github/workflows/m8-4-loss-semantics.yml`

These remain inside the experimental observer path. They do not change the default ptrace backend, baseline format, policy semantics, or release packaging.

## Closure decision

**M8.4 is CLOSED / PROVED.**

What M8.4 establishes:

- known producer loss cannot silently become clean evidence;
- userspace truncation cannot silently become clean evidence;
- consumer lag cannot silently hide producer loss;
- post-root descendant activity is drained or explicitly timed out;
- pre-target and post-start collector failures are machine-readable and bounded;
- decode failure is explicit;
- controlled failure never grants PASS authority.

What M8.4 does **not** establish:

- ptrace/eBPF semantic parity;
- cross-backend evidence equivalence;
- production or public-alpha eBPF readiness;
- performance superiority;
- broad kernel/platform compatibility.

Those remain M8.5–M8.7 work.

## Non-negotiable boundaries

- eBPF PASS authority: **NOT AUTHORIZED**.
- cross-backend parity: **OPEN / M8.5 NEXT**.
- ptrace default/reference: **RETAINED**.
- missing/lost/uncertain events may never be interpreted as clean evidence.
- an internal lifecycle marker does not imply public ProcessExit capability.
