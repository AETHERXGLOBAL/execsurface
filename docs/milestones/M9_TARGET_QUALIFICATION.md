# M9.1 — Zero-Contact External Target Qualification

Date: 2026-09-26
Status: **FROZEN COHORT / POST-M9.0 EXECUTION ACTIVE**
Tracking: #51
Protocol merge: `1decfc8672b89e05b88d20ee061fdf32033cd722`

## Purpose

Freeze the first M9.1 external workload cohort without selecting targets based on favorable ExecSurface results.

The cohort identities and revisions below were originally committed on the preserved superseded branch before the first `casey/just` execution. That historical ordering is retained as evidence that the target was not selected after observing its result. Accepted M9.1 collection, however, begins only after the M9.0 protocol merge above.

All cases below are `ZERO_CONTACT_EXTERNAL_REPRO`. They are compatibility/performance evidence and **not independent adoption**.

## Cohort

### T1 — `casey/just`

- repository: `casey/just`
- pinned revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- upstream workload: `cargo +1.90.0 test --all`
- classification: `ZERO_CONTACT_EXTERNAL_REPRO`
- first post-freeze role: reproduce the preserved pre-freeze ptrace observer counterexample under the canonical M9.0 protocol before considering any code fix.

### T2 — `BurntSushi/ripgrep`

- repository: `BurntSushi/ripgrep`
- pinned revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- upstream workload: `cargo test --all`
- classification: `ZERO_CONTACT_EXTERNAL_REPRO`
- execution order: after T1 evidence is understood well enough to distinguish target-specific failure from a product-general observer failure.

### T3 — `pytest-dev/pytest`

- repository: `pytest-dev/pytest`
- pinned revision: `8721173580390a9d297e5af06cac3f0b6841f425`
- focused workload: `pytest testing/test_config.py`
- classification: `ZERO_CONTACT_EXTERNAL_REPRO`
- execution order: after T1/T2 harness health is established.

### C0 — `sharkdp/fd`

Prior M7 compatibility target only. Retained as calibration context and **not counted as new M9.1 expansion evidence** unless a new, explicitly identified post-freeze experiment is run.

## Selection integrity

Historical pre-result qualification commit on the preserved superseded branch:

- `5e22bab...` — M9.1 target qualification

The first `casey/just` execution workflow was added later at:

- `d52fd331dd7664651295a94c996cb1c09cc83646`

No target is removed because it failed. No replacement target may be introduced merely to improve success rate. Additions require a documented reason independent of observed ExecSurface outcome.

## Execution rules

For each accepted post-freeze workload:

1. pin the exact upstream revision;
2. use the unmodified upstream workload;
3. install the public ExecSurface release through the declared public path;
4. reproduce the upstream workload directly first;
5. preserve every operational failure;
6. if performance is attempted, follow the canonical 3-warmup/15-measured alternating protocol exactly;
7. produce a canonical `m9-evidence-v1` record;
8. retain raw artifacts and SHA-256 digests;
9. keep privacy metadata-only;
10. never relabel zero-contact evidence as adoption.

## Acceptance boundary

Target qualification proves only that the cohort, pins, commands, classification, and ordering are frozen. It proves no compatibility, performance, correctness, adoption, or eBPF claim.
