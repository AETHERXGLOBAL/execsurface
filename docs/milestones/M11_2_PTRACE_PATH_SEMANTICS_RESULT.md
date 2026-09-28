# M11.2 — ptrace Pathname Semantics Hardening Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — PASS**

## Accepted evidence

Accepted implementation HEAD before closeout note: `fdce9c1d6d7511be4276e59427d16d1e61f88e32`.

Accepted GitHub Actions run: `36445850578` — **SUCCESS**.

The accepted gate ran:

- formatting checks for the M11 authority crate and new M11 observer bridge test;
- full-workspace Clippy with warnings denied;
- authority-model tests;
- ptrace authority bridge tests;
- frozen baseline-v2 digest compatibility test;
- full locked workspace tests;
- Cargo.lock integrity.

All passed.

## Closed invariant

The legacy ptrace descriptor may retain its existing public capability labels for compatibility, but those labels no longer imply stronger M11 evidence authority.

The internal M11 contract now enforces:

- ptrace exec-attempt pathname -> `ArgumentObserved`;
- ptrace file path access intent -> `ArgumentObserved`;
- ptrace connect-attempt destination -> `ArgumentObserved`;
- ptrace process-exec object identity -> `Unsupported`;
- ptrace file-open object identity is not `KernelObjectSuccessBound`;
- exact kernel-object-bound authority requests against weaker ptrace evidence fail explicitly.

This preserves the M10.1 PATH-TOCTOU counterexample instead of hiding it behind a legacy capability name.

## Negative evidence retained

An earlier M11.2 run failed only because a temporary workspace-wide rustfmt check encountered historical M10 research files that predated the M11 formatting gate. Those files were not rewritten. The formatting gate was scoped to M11-owned changes while Clippy and the complete workspace test suite remained global and strict.

## Compatibility boundary

No change was made to:

- `main`;
- `v0.1.0-alpha.3`;
- Marketplace behavior;
- public raw Observation schema;
- baseline-v2 serialization or digest;
- default ptrace backend selection.

## Decision

**M11.2 CLOSED — PASS.**

Authorized next gate: **M11.3 — shared-FD false-completeness hardening**.
