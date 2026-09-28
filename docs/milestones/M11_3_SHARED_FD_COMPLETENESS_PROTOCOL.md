# M11.3 — Shared-FD False-Completeness Hardening Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — IMPLEMENTATION AUTHORIZED**

## Objective

Eliminate the specific false-completeness state exposed by M10.2: a ptrace observation must not remain `complete=true` when shared file-descriptor-table concurrency makes fd lifecycle attribution unproven.

This gate does **not** claim to repair shared-FD attribution. It introduces explicit ambiguity detection and fail-closed semantics first.

## Fixed roles

- **Innovative Systems Architect** — find the strongest path that preserves ptrace portability while making ambiguity machine-visible and future-repairable.
- **Anti-Deviation / Skeptical Reviewer** — reject any solution that hides the M10.2 counterexample, labels ambiguity as complete, or weakens PASS eligibility.

## Dynamic specialists

Linux clone/CLONE_FILES semantics, ptrace lifecycle state, concurrent fd tables, evidence completeness, Rust state-machine design, regression testing, and backward-compatibility review.

## Frozen M10.2 evidence

The bounded counterexample observed target-proven relevant reads that were missing or misattributed while the current observer still reported complete evidence. Therefore the universal proposition "current ptrace fd attribution is complete under shared-FD races" is killed.

## M11.3 conservative rule

Until a separately proved attribution repair exists:

> Observation of a `CLONE_FILES` shared descriptor table places ptrace fd-lifecycle authority into an ambiguity state for the session.

The session must become incomplete with warning code:

`shared_fd_table_ambiguity`

This is intentionally conservative. It may reject some sessions whose execution happened not to exercise the race; it must not falsely certify completeness.

## Required implementation

1. add `IncompleteAmbiguity` to the internal observer handoff completeness vocabulary;
2. classify `shared_fd_table_ambiguity` as `IncompleteAmbiguity`;
3. on a proved `CLONE_FILES` clone, record the warning at most once per observation;
4. the warning must set public raw observation `complete=false` using the existing warning mechanism;
5. missing/unreadable clone flags continue to fail closed through the existing warning path;
6. no public schema version or baseline digest migration;
7. preserve the historical M10.2 counterexample test and evidence.

## Acceptance tests

- synthetic ambiguity warning -> `IncompleteAmbiguity` and not PASS-eligible;
- controlled `CLONE_FILES` fixture -> `complete=false` and warning present;
- no `CLONE_FILES` control -> no new ambiguity warning;
- full workspace Clippy/tests remain green;
- frozen baseline-v2 digest remains green;
- no `main`, release, tag, Action channel, or Marketplace mutation.

## Stop rule

Any tested path where proved `CLONE_FILES` sharing is present and the observer can still claim complete evidence blocks M11.4.
