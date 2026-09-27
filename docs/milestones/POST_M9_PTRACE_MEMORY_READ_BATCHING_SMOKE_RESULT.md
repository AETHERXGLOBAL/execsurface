# Post-M9 — Candidate B Smoke Result

Date: 2026-09-27
Status: **CLOSED — SMOKE PASS ONLY**
Parent: GitHub issue #84
Protocol: `docs/milestones/POST_M9_PTRACE_MEMORY_READ_BATCHING_PROTOCOL.md`

## Preserved first attempt

Run `36298711665` at source `abb251f94412346b1a19e395617cc303f081b323` failed at the formatting gate only.

- candidate patch application: PASS
- privacy-shape static gate: PASS
- `cargo fmt --check`: FAIL
- Clippy/tests: not reached
- preserved artifact: `10924183040`
- artifact digest: `sha256:bfbf871c3dcad4ed619b1165eca8ab4f65bb73469c53287d0a7b00af7d35abd2`

The formatting defect was a rustfmt-only function-signature shape. No semantic rule, candidate mechanism, privacy boundary, threshold, or test criterion was changed.

## Corrected smoke

Run `36298792193` at source `c15098760afe53f4eed4aa8bcd783bd3319992c2` completed successfully.

Passed:

- deterministic Candidate B patch application inside the runner only;
- privacy-shape static gate;
- `cargo fmt --all -- --check`;
- strict workspace Clippy with `-D warnings`;
- existing `execsurface-observe` tests;
- existing workspace tests;
- isolated evidence recording/upload.

Classification:

`CANDIDATE_B_SMOKE_PASS_ONLY`

This is **not** adversarial semantic acceptance, activation evidence, performance evidence, merge authorization, or a public performance claim.

Public native ptrace source remains unchanged by Candidate B. The next authorized step is the preregistered adversarial semantic-parity plus activation gate.