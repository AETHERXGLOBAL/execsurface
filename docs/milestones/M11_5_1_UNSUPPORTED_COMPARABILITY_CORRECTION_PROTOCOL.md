# M11.5.1 — Unsupported-Proposition Comparability Correction Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — CORRECTION REQUIRED BEFORE M11.6**

## Trigger

Post-closeout adversarial review found an internal contradiction in M11.5.

The frozen M11.5 rule states:

> An unsupported proposition does not become equivalent to another unsupported proposition merely because both lack evidence.

However, the accepted integration-test helper classified every proposition of two identical contracts as `EquivalentForProposition`, including propositions whose support state was `Unsupported`.

The accepted cross-backend ptrace↔hybrid result remains fail-closed (`NotComparable` for all 15 propositions), so no unsafe cross-backend reuse was granted. The defect is in the same-contract proposition rule and must be corrected before M11.6.

## Corrected rule

For two exactly identical complete evidence contracts:

- if the requested proposition is `Supported(authority)` in that contract, relation may be `EquivalentForProposition`;
- if the requested proposition is `Unsupported`, relation is `NotComparable`;
- if contracts differ, relation is `Unknown` absent an explicit registered rule;
- a missing proposition remains fail-closed.

Unsupported evidence is absence of a claim, not an equivalent evidence claim.

## Acceptance tests

1. identical contracts + supported proposition -> `EquivalentForProposition`;
2. identical contracts + unsupported proposition -> `NotComparable`;
3. changed fingerprint -> `Unknown`;
4. changed proposition mapping -> `Unknown`;
5. ptrace↔hybrid remains `NotComparable` for all 15 propositions;
6. symmetric reuse still requires a non-empty set whose every required proposition is explicitly equivalent;
7. full M11 CI remains green, including frozen baseline-v2 digest and full workspace tests.

## Historical preservation

Do not rewrite or delete the original M11.5 result. M11.5.1 records the discovered contradiction and supersedes only the unsupported same-contract equivalence rule.

## Stop rule

M11.6 is blocked until this correction passes the full gate.
