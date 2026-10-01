# ExecSurface — Semantics v3 Rework Independent Falsification Protocol

Date: 2026-10-01
Status: PREREGISTERED — INDEPENDENT REVIEW / NO PUBLIC PROMOTION

## Review target

- repaired source under review: `0200f09557906118dd0e96f8a5a73aa4f5c4a9cc`
- review branch: `review/post-alpha4-semantics-v3-rework-falsification`
- original broken candidate remains retained: `5079a990b924d8ccd7ac6414f8a9a2571b54e240`
- public alpha.4 source remains immutable: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action remains `AETHERXGLOBAL/execsurface@v0.1`

This review is deliberately separate from the repair branch. It must not modify the production/public alpha.4 path, the stable tag, or P8 evidence state.

## Fixed independent roles

- Independent Falsifier / Red Team — owns attack design and failure retention.
- Independent Critical-Milestone Reviewer — decides whether the repair is requalified.
- Innovation Scientist / Systems Architect — may propose stronger attacks but may not weaken acceptance criteria.
- Anti-Drift / Scientific Integrity Reviewer — rejects test relaxation, hidden state, backend-name authority, and reinterpretation of failures.

## Question

Did the rework fix the underlying proof-admission contract generally, or merely overfit the previously known D01-D03 examples?

## Frozen falsification corpus

The following attacks are preregistered before execution:

- F01 — deserialized requirement missing `expected_proposition` must fail closed.
- F02 — same file target with a substituted actor must not satisfy the original proposition contract.
- F03 — same actor/target with a substituted operation must not satisfy the original proposition contract.
- F04 — same actor/target with a substituted execution chain must not satisfy the original proposition contract.
- F05 — schema version substitution above v3 must fail closed.
- F06 — required completeness downgraded to `NotRequired`, `Incomplete`, `Ambiguous`, or `Unsupported` must fail closed.
- F07 — backend/profile-name mutation must not rescue a proposition mismatch.
- F08 — bound requirement with only real completeness obligations remains a valid non-vacuous contract when those obligations are met.
- F09 — known `object_identity_conflict` must block when `ObjectIdentity` is required.
- F10 — the same object-identity ambiguity must not automatically poison an unrelated requirement that does not depend on object identity.
- F11 — an unknown ambiguity code must fail closed rather than silently inherit authority.
- F12 — additional evidence guarantees may strengthen a matching record but cannot make a mismatched proposition admissible.

## Acceptance rule

Requalification requires **12/12 PASS**, plus:

- Rust formatting clean;
- Clippy `-D warnings` clean;
- original repaired D01-D06 corpus still PASS;
- original A1 corpus still PASS;
- public tags unchanged at alpha.4 source;
- no mutation of P8 external-evidence state.

Any reproducible false admission is material counterevidence and blocks requalification. Harness/formatting failures are retained separately and may be corrected only without changing attack meaning or assertions.

## Claim boundary

A successful review can establish only a bounded internal statement: the repaired Semantics v3 proof-admission contract resisted the preregistered known and independent falsification corpus. It does not establish universal correctness, external validation, adoption, production readiness, public release authorization, or P8 closure.
