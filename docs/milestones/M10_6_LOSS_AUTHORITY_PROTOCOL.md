# M10.6 — LOSS-AUTHORITY-001 Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m106`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Force **real BPF producer/transport loss** in a controlled kernel event stream and determine whether the candidate hybrid evidence architecture can detect the loss explicitly and fail closed rather than treating the resulting partial event stream as complete/PASS-capable evidence.

This gate is about evidence authority under loss. It does not test product performance.

## Fixed roles

- **Innovative Systems Architect** — design observable loss accounting that can become part of proposition authority rather than a best-effort diagnostic.
- **Anti-Deviation / Skeptical Reviewer** — reject synthetic counter injection, inferred loss without a real failed producer reservation, or any classification that allows partial evidence to look clean.

## Dynamic specialists

BPF ring-buffer internals, kernel tracepoints, fault/stress injection, Linux syscall semantics, evidence/completeness semantics, QEMU boot/kernel configuration, CI/reproducibility, and internal red team.

## Frozen mechanism

A minimal research BPF program attaches to `syscalls:sys_enter_getpid` for one explicitly registered target TGID.

For every registered event the BPF program MUST:

1. increment a kernel-side `attempts` counter;
2. attempt `bpf_ringbuf_reserve()` on a deliberately tiny ring buffer (`4096` bytes);
3. if reserve fails, increment a kernel-side `drops` counter;
4. if reserve succeeds, submit exactly one typed event.

The userspace consumer MUST deliberately **not poll the ring buffer while the target generates the burst**. This creates genuine producer reservation failures once the finite ring buffer fills.

No code may directly increment `drops` except the actual `bpf_ringbuf_reserve() == NULL` branch.

## Controlled fixture

- target child `SIGSTOP`s before registration;
- parent registers the child TGID;
- parent resumes the child;
- child performs exactly `N = 200000` raw `syscall(SYS_getpid)` operations;
- parent does not consume ring-buffer events until the child exits;
- after target exit, parent reads kernel `attempts` and `drops`, then drains all accepted ring-buffer records;
- target must exit `0`.

Because the target is not registered before the stop barrier, setup syscalls are outside the measured session.

## Required accounting invariants

For an interpretable forced-loss run:

- `attempts == N`;
- `drops > 0`;
- `accepted_events > 0`;
- `accepted_events < attempts`;
- `accepted_events + drops == attempts`;
- malformed events `== 0`;
- wrong-session events `== 0`;
- target exit `== 0`.

## Frozen authority rule

If `drops > 0`, evidence completeness MUST be false and PASS-capable authority MUST be false regardless of whether every accepted record is otherwise well formed.

The evaluator must emit both:

- `evidence_complete=false`;
- `pass_authority=false`.

## Frozen classifications

- `M10_6_REAL_LOSS_DETECTED_FAIL_CLOSED` — real reserve failures occurred, accounting closes exactly, and authority is explicitly false/incomplete.
- `M10_6_REAL_LOSS_SILENT_AUTHORITY_FAILURE` — real loss occurred but the resulting state is represented as complete or PASS-capable.
- `M10_6_LOSS_ACCOUNTING_MISMATCH` — attempts, accepted events, and drop counts do not reconcile exactly.
- `M10_6_LOSS_NOT_FORCED` — no real reserve failure occurred.
- `M10_6_ENVIRONMENT_BLOCKED` — required BPF/tracepoint capability unavailable.
- `M10_6_INFRA_FAILURE` — build/boot/harness failure before interpretable execution.

## Stop rule

Record the first interpretable run. Do not tune ring size, event size, `N`, or consumer timing after seeing an interpretable result.

## Non-authorizations

Even `M10_6_REAL_LOSS_DETECTED_FAIL_CLOSED` does not authorize public BPF PASS, `learn/check`, baseline interchangeability, backend auto-selection, ptrace replacement, production integration, or changes to `main`, Marketplace, or `v0.1.0-alpha.3`.
