# M11.5 — Proposition-Level Comparability Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — PASS / CROSS-BACKEND REUSE REJECTED**

## Classification

`M11_5_PROPOSITION_COMPARABILITY_FAIL_CLOSED`

## Accepted evidence

Accepted source SHA: `a32e3152bf67a37f30f5fa72ea73cc5af1eeda33`.

Accepted GitHub Actions run: `36449083081` — **SUCCESS**.

The accepted gate passed full-workspace Clippy, authority tests, explicit hybrid tests, ptrace path-authority tests, shared-FD ambiguity tests, the frozen baseline-v2 digest test, the complete locked workspace test suite, and lockfile integrity.

## Proven decision rules

- exact identical evidence contracts are equivalent per proposition;
- fingerprint drift is `Unknown` unless an explicit rule exists;
- proposition-mapping drift is `Unknown` unless an explicit rule exists;
- symmetric baseline reuse requires every required proposition to be `EquivalentForProposition`;
- `OneWayRefinement`, `NotComparable`, `Unknown`, or a missing relation blocks symmetric reuse;
- an empty required set does not grant reuse by vacuous truth.

## ptrace ↔ hybrid result

For the complete frozen 15-proposition M11 universe, current ptrace ↔ M11.4 hybrid comparability is explicitly:

`NotComparable`

for every proposition.

Therefore no current ptrace baseline may be reused as a hybrid baseline, and no hybrid baseline may be reused as a ptrace baseline, on the basis of current evidence.

This is not a claim that the evidence sources have equal strength. It is a rejection of unproved semantic interchangeability.

## Preserved boundaries

No public schema, baseline-v2 format, digest, `main`, tag, Marketplace behavior, stable Action channel, or default backend selection changed.

## Decision

**M11.5 CLOSED — PASS / CROSS-BACKEND BASELINE REUSE REJECTED.**

Authorized next gate: **M11.6 — privacy / loss / adversarial red-team**.
