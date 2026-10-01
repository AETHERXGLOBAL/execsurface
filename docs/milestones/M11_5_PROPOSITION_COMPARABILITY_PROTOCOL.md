# M11.5 — Proposition-Level Comparability Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — IMPLEMENTATION AUTHORIZED**

## Objective

Make baseline/evidence comparability an explicit proposition-level decision. Backend name, authority label, or the fact that one source is stronger in a bounded fixture must never imply baseline interchangeability.

## Fixed roles

- **Innovative Systems Architect** — define a reusable comparability decision layer that can admit future proofs without weakening current evidence boundaries.
- **Anti-Deviation / Skeptical Reviewer** — reject inferred equivalence, authority-order shortcuts, unsupported-to-unsupported pseudo-equivalence, or any cross-backend PASS path that is not explicitly proved.

## Dynamic specialists

Evidence logic, baseline semantics, canonicalization, policy semantics, Rust type/API review, backward compatibility, and adversarial comparison testing.

## Frozen rules

1. Comparability is evaluated for an exact `EvidenceProposition`.
2. `EquivalentForProposition` is the **only** relation that permits symmetric baseline reuse.
3. `OneWayRefinement` is informative only; it does not permit symmetric baseline reuse.
4. `NotComparable` and `Unknown` fail closed.
5. Identical authority labels from different contracts do not establish equivalence.
6. An unsupported proposition does not become equivalent to another unsupported proposition merely because both lack evidence.
7. Health/completeness and comparability are independent gates: a comparable proposition under incomplete health is still not PASS-eligible.
8. Current ptrace and M11.4 hybrid contracts receive no backend-wide equivalence.

## Same-contract rule

For this gate, proposition equivalence may be accepted only when both observations use the **same complete evidence contract**:

- same evidence-contract version;
- same capability fingerprint;
- same total proposition mapping;
- same support state for the requested proposition.

Any contract difference defaults to `Unknown` unless an explicit registered cross-contract rule exists.

## ptrace ↔ hybrid rule for M11.5

The bounded M10/M11 evidence does not prove symmetric semantic equivalence for any proposition between current ptrace and the M11.4 hybrid adapter.

Therefore every proposition in the frozen 15-proposition universe is explicitly classified `NotComparable` for ptrace ↔ hybrid in M11.5.

This is a scientific rejection of baseline interchangeability, not a rejection of the stronger hybrid evidence itself.

## Required-proposition gate

A symmetric reuse decision takes:

- a finite set of required propositions;
- a comparability relation for each required proposition.

The decision succeeds only if **every required proposition** is explicitly `EquivalentForProposition`.

Missing relation, `Unknown`, `NotComparable`, or `OneWayRefinement` must fail closed and identify the blocking proposition.

## Acceptance tests

M11.5 closes only if all are true:

1. exact same-contract proposition comparison returns `EquivalentForProposition`;
2. a changed fingerprint returns `Unknown` absent an explicit rule;
3. a changed proposition mapping returns `Unknown` absent an explicit rule;
4. all 15 current ptrace ↔ hybrid relations are explicitly `NotComparable`;
5. symmetric reuse accepts only all-equivalent required sets;
6. one `NotComparable`, `Unknown`, `OneWayRefinement`, or missing relation blocks reuse;
7. empty required set does not accidentally grant a meaningful cross-backend comparison claim;
8. frozen baseline-v2 digest test remains green;
9. full-workspace Clippy/tests remain green;
10. no public schema, main, tag, Marketplace, or stable Action mutation.

## Stop rule

Any implementation that derives equivalence from authority class ordering, source naming, unsupported status, or partial proposition overlap blocks M11.6.
