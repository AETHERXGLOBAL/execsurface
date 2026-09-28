# M11.1 — Anti-Deviation Review

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`

## Classification

`M11_1_PROTOCOL_DEVIATION_FOUND_BEFORE_CLOSEOUT`

## Fixed reviewers

- **Innovative Systems Architect** — test whether a stronger internal model can express future kernel-backed authority without overstating the current ptrace backend.
- **Anti-Deviation / Skeptical Reviewer** — compare implementation against the preregistered protocol and reject green-CI-only acceptance.

## Finding

The M11.1 implementation reached a green CI run at source SHA `155d84b6002854fae9d8458e8900304f45e4caef`, but the anti-deviation review found a semantic mismatch against the preregistered protocol before closeout.

The preregistered M11.1 protocol freezes this rule:

> successful-operation facts derived from ptrace entry/exit/lifecycle bookkeeping -> at most `DerivedLifecycleModel`.

At the reviewed SHA, `ptrace_contract_v1()` classified these current-ptrace propositions as `KernelSuccessConfirmed`:

- `ProcessSpawnOccurrence`;
- `ProcessExecSuccess`;
- `FileOpenSuccess`.

This is stronger terminology than the frozen M11.1 rule authorizes for the current ptrace contract.

## Decision

The green CI run is **not sufficient to close M11.1**.

The mapping must be corrected to the preregistered authority boundary and an explicit regression test must prevent those ptrace propositions from silently regaining `KernelSuccessConfirmed` authority.

No protocol criterion is weakened. No M10 result is reinterpreted. No public schema, baseline, CLI, Marketplace behavior, tag, release, or `main` behavior is changed.

## Required repair

1. Map `ProcessSpawnOccurrence`, `ProcessExecSuccess`, and `FileOpenSuccess` to `DerivedLifecycleModel` in the current ptrace contract.
2. Add a test that fixes this boundary.
3. Re-run the full M11 authority/compatibility CI gate.
4. Close M11.1 only after the repaired HEAD passes all preregistered acceptance criteria.
