# M11.1 — Authority-Aware Internal Evidence Model Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — PASS AFTER PROTOCOL REPAIR**

## Classification

`M11_1_AUTHORITY_MODEL_ACCEPTED_AFTER_PROTOCOL_REPAIR`

## Accepted evidence

Accepted implementation SHA: `f15f590da24385e4178addf36d2fbff859a8c98c`.

Accepted GitHub Actions run: `36445981549` — **SUCCESS**.

The dedicated M11 authority gate executed the same frozen acceptance surface:

- rustfmt check for M11-scoped Rust changes;
- full-workspace Clippy with warnings denied;
- authority-model tests;
- frozen baseline-v2 digest serialization compatibility test;
- full locked workspace test suite;
- Cargo.lock integrity check.

All passed after the protocol repair.

## Anti-deviation finding preserved

An earlier green run at SHA `155d84b6002854fae9d8458e8900304f45e4caef` / run `36444648210` is **not** the accepted M11.1 closeout evidence.

The Anti-Deviation / Skeptical Reviewer found that the then-current ptrace authority contract classified `ProcessSpawnOccurrence`, `ProcessExecSuccess`, and `FileOpenSuccess` as `KernelSuccessConfirmed`, while the preregistered M11.1 protocol freezes ptrace entry/exit/lifecycle-derived successful-operation facts at **no stronger than `DerivedLifecycleModel`**.

That deviation is preserved in:

`docs/milestones/M11_1_ANTI_DEVIATION_REVIEW.md`

No criterion was weakened. The implementation was corrected to the frozen protocol and a regression test now prevents those current-ptrace success propositions from silently regaining `KernelSuccessConfirmed` authority.

## Implemented invariants

- internal `execsurface-authority` crate with `#![forbid(unsafe_code)]`;
- frozen 15-proposition evidence universe;
- total/non-duplicated ptrace-v2 proposition classification;
- syscall-entry pathname and connect-destination metadata remain `ArgumentObserved`;
- ptrace spawn/exec-success/file-open-success facts remain `DerivedLifecycleModel`;
- fd read/write/lifecycle attribution remains `DerivedLifecycleModel`;
- ptrace does not claim process-exec kernel-object identity;
- ptrace file-open object identity is not promoted to `KernelObjectSuccessBound`;
- connect attempt is not promoted to connect success;
- `IncompleteAmbiguity` exists and is never PASS-eligible;
- only explicit proposition equivalence permits symmetric baseline reuse;
- malformed/incomplete evidence contracts fail validation.

## Preserved boundaries

No change was made to:

- `main`;
- `v0.1.0-alpha.3`;
- Marketplace behavior;
- default public CLI backend selection;
- public v2 baseline serialization;
- baseline digest format.

## Decision

**M11.1 CLOSED — PASS AFTER PROTOCOL REPAIR.**

M11.2 pathname-semantics hardening remains the authorized next gate. Its acceptance must preserve the M10.1 PATH-TOCTOU counterexample and reject any silent conversion of ptrace syscall-entry metadata into kernel-object proof.
