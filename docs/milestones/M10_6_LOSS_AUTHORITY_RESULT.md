# M10.6 — LOSS-AUTHORITY-001 Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m106`
Protocol: `docs/milestones/M10_6_LOSS_AUTHORITY_PROTOCOL.md`

## Classification

`M10_6_REAL_LOSS_DETECTED_FAIL_CLOSED`

## Accepted execution

Workflow run: `36441126663`  
Job: `108991566916`  
Source SHA: `92efa36c48ed83e2d004bf3505c6bac39cb483e1`  
Artifact: `10979000328`  
Artifact ZIP digest: `sha256:7d1b226e69edda854e7f2f7e3a4af4308b081134f7970cfedb9c6a43b2432e50`

Workflow conclusion: `success`.

## Frozen evidence

Environment:

- guest kernel: `6.8.0-142-generic`;
- kernel BTF readable;
- `syscalls/sys_enter_getpid` tracepoint available;
- probe completed.

Forced-loss fixture:

- requested raw `SYS_getpid` operations: `200000`;
- kernel-side registered producer attempts: `200000`;
- accepted ring-buffer events after delayed drain: `127`;
- real `bpf_ringbuf_reserve()` failures recorded by producer: `199873`;
- target exit: `0`;
- malformed accepted events: `0`;
- wrong-session accepted events: `0`.

Exact accounting:

`127 + 199873 = 200000`

Therefore:

- `attempts == requested`: true;
- `drops > 0`: true;
- `accepted > 0`: true;
- `accepted < attempts`: true;
- `accepted + drops == attempts`: true;
- `accounting_ok=true`.

Frozen authority state:

- `evidence_complete=false`;
- `pass_authority=false`.

The loader and workflow evaluator both emitted:

`M10_6_REAL_LOSS_DETECTED_FAIL_CLOSED`

## Why this is real loss

The `drops` counter is incremented only inside the BPF branch where `bpf_ringbuf_reserve()` returned `NULL`. The consumer intentionally did not poll while the registered target produced the frozen 200,000-event burst into a 4096-byte ring buffer.

No synthetic drop flag or userspace-only loss injection was used.

## Interpretation

Within this controlled fixture, the candidate BPF evidence path can make real producer loss explicit and can prevent a partial event stream from retaining clean/PASS-capable authority.

The result supports the architectural requirement that completeness is a first-class evidence property rather than an inference from absence of observed drift.

## What this proves

For this fixture:

- real ring-buffer producer reservation loss can be forced reproducibly;
- every registered producer attempt was accounted either as one accepted event or one explicit reserve failure;
- recorded loss caused evidence authority to fail closed;
- partial evidence was not represented as complete.

## What this does not prove

This result does **not** establish:

- zero unaccounted loss for every BPF hook or transport topology;
- multi-CPU ordering/completeness under arbitrary production workloads;
- crash/restart durability;
- container/namespace privilege suitability;
- public BPF PASS or `learn/check` authority;
- acceptable production performance;
- baseline interchangeability;
- backend auto-selection;
- ptrace replacement or product integration.

## Decision

M10.6 is **CLOSED / FAIL-CLOSED LOSS ACCOUNTING PROVED FOR THE DECLARED FIXTURE**.

Authorized next gate: **M10.7 — PRIVILEGE-MATRIX-001**.

No change to `main`, `v0.1.0-alpha.3`, Marketplace behavior, public ptrace authority, or baseline semantics is authorized.
