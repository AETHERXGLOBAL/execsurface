# M8.4 — Fail-Closed Loss, Lag, Teardown & Lifecycle Semantics

Date: 2026-09-26
Status: **M8.4a OPEN — REAL PRODUCER LOSS / TRUNCATION GATE**
Tracking: #40
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
M8.3 closure: `docs/milestones/M8_3C_COLLECTOR.md`, `docs/milestones/M8_3C2_CLI_BRIDGE.md`

## Objective

Deepen the experimental libbpf observation path from “loss is representable” to “controlled loss and lifecycle failure channels are reproducibly detected and fail closed.”

M8.4 does not add eBPF PASS authority and does not perform ptrace/eBPF parity evaluation. Those remain blocked until later gates.

## Fixed governance roles

- **Innovation Scientist / Architect:** seek stronger failure-observability mechanisms and adversarial experiments without weakening semantics.
- **Deviation Prevention / Scientific Integrity:** reject any interpretation of absent events as clean evidence and prevent scope drift into M8.5 parity.

## M8.4a hypothesis

The current real collector already increments a per-CPU `dropped` counter when `bpf_ringbuf_reserve()` fails and maps `dropped_events > 0` to `incomplete_loss`. M8.4a tests that path without adding a synthetic “force loss” flag.

A real multi-threaded target generates a high rate of successful `openat` events against `/dev/null`. If the producer outpaces userspace ring-buffer consumption, ring-buffer reservation failure must be reflected in the existing kernel-side dropped counter. The collector must then emit:

- `dropped_events > 0`;
- `completeness = incomplete_loss`;
- `observation_complete = false`;
- warning `producer_event_loss`.

If the reference runner cannot reproducibly create real pressure, this experiment is **KILLED / NON-REPRODUCIBLE** and M8.4 must add an explicit controlled consumer-lag hook before retrying. The gate must not be marked proved merely because the code path exists.

## Truncation control

The same workflow runs the real collector with `--event-limit 1` against a small event storm. Acceptance requires:

- `completeness = incomplete_limit`;
- `observation_complete = false`;
- warning `event_limit_exceeded`;
- zero producer drops in that small control case.

This distinguishes userspace evidence-budget truncation from kernel producer loss.

## M8.4a files

- `experiments/m8-ebpf/libbpf-observer/src/bin/m8_4_event_storm.rs`
- `.github/workflows/m8-4-loss-semantics.yml`

The event-storm binary is an isolated test fixture inside the experimental observer crate. It does not alter the default ExecSurface binary, ptrace backend, baseline format, policy semantics, or release packaging.

## Required evidence before closure

1. normal repository CI remains green;
2. dedicated M8.4 workflow builds on Rust 1.82;
3. truncation control passes;
4. real producer pressure produces a non-zero kernel drop count;
5. producer loss is classified as `incomplete_loss`;
6. neither report can claim complete/PASS-eligible evidence.

## Next gates after M8.4a

- M8.4b — controlled consumer lag and teardown race semantics;
- M8.4c — attach/load/decode/lifecycle failure machine-readable evidence;
- M8.4d — red-team closure and regression preservation.

## Labels

- M8.4a real ring-buffer loss proof: **OPEN UNTIL CI EVIDENCE**
- event-budget truncation proof: **OPEN UNTIL CI EVIDENCE**
- eBPF PASS authority: **NOT AUTHORIZED**
- cross-backend parity: **OPEN / M8.5**
- ptrace default/reference: **RETAINED**
