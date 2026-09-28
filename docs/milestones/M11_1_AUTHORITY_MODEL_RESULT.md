# M11.1 — Authority-Aware Internal Evidence Model Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — PASS**

## Evidence

Accepted HEAD before closeout note: `155d84b6002854fae9d8458e8900304f45e4caef`.

Accepted GitHub Actions run: `36444648210` — **SUCCESS**.

The dedicated M11 authority gate executed:

- rustfmt check for `execsurface-authority`;
- full-workspace Clippy with warnings denied;
- authority-model tests;
- frozen baseline-v2 digest serialization compatibility test;
- full locked workspace test suite;
- Cargo.lock integrity check.

All passed.

## Implemented invariants

- internal `execsurface-authority` crate with `#![forbid(unsafe_code)]`;
- frozen 15-proposition evidence universe;
- total/non-duplicated ptrace-v2 proposition classification;
- syscall-entry pathname and connect-destination metadata remain `ArgumentObserved`;
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

## Negative evidence retained

The first hardened run failed at rustfmt only. No acceptance criterion was weakened. A rustfmt-only correction was committed and the same gate then passed.

## Decision

**M11.1 CLOSED — PASS.**

Authorized next gate: **M11.2 — ptrace pathname semantics hardening**.
