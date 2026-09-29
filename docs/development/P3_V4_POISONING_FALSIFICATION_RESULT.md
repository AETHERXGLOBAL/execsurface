# ExecSurface — P3 V4 Poisoning / Consolidated Falsification Result

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`

## Formal decision

`P3_V4_FALSIFICATION_PASS_BOUNDED`

This closes the declared V4 falsification suite only. It is not a universal safety, completeness, poisoning-resistance or production-readiness claim.

Public `v0.1.0-alpha.4`, baseline/check semantics, normalization profile v3, policy and stable `@v0.1` remain unchanged.

## Protocol

`docs/development/P3_V4_POISONING_FALSIFICATION_PROTOCOL.md`

Protocol commit:
`1eb4c73927d45aa222ab3967dd73b747280e1609`

## Accepted source and CI

Accepted source:
`1d421b004532fc915fd0493ce91437aa25a8f1ec`

Accepted CI run:
`36559257336`

Accepted job:
`109375874418`

Environment:
GitHub-hosted Ubuntu 24.04.5.

Accepted gates:
- isolated dependency lock generation — PASS;
- rustfmt check — PASS;
- clippy `-D warnings` — PASS;
- consolidated falsifier tests — **10 passed / 0 failed**.

## Attack classifications

All preregistered attacks ended `BLOCKED_AS_DESIGNED` in the declared suite:

- A1 single-run malicious injection: observed as variable candidate, not authorized;
- A2 repeated malicious injection: support increased, acceptance did not;
- A3 duplicate-vote inflation: duplicate evidence digest rejected;
- A4 incomplete-run poisoning: incomplete evidence rejected;
- A5 run-order manipulation: V1 report and V3 contract remained byte-stable;
- A6 profile/observer/normalization mismatch: incomparable learning set rejected;
- A7 GCC grammar mimicry outside bounded producer/role contract: rejected;
- A8 actor substitution against an accepted effect: exact acceptance lookup failed as required;
- A9 unseen lexically similar target: not accepted;
- A10 invariant laundering into accepted variance: rejected.

No `UNCLASSIFIED_AUTHORIZATION_PATH` was observed in the declared suite.

## Preserved failure

Initial V4 run `36559110925` failed at rustfmt before clippy/tests. That failure is retained. Commit `1d421b004532fc915fd0493ce91437aa25a8f1ec` applied only rustfmt-prescribed changes; no attack, assertion, semantic rule or acceptance condition was removed or weakened.

## Interpretation

The combined V1/V2/V3 research stack survived the declared synthetic poisoning/falsification suite:
- recurrence remains descriptive rather than permissive;
- duplicate/incomplete/incomparable evidence fails closed;
- the GCC ephemeral candidate remains producer/role bounded;
- explicit acceptance remains exact and provenance-bound.

This does not establish robustness against all possible workload, observer, kernel, process-lineage or supply-chain attacks.

## Next gate

V5 — real-workload requalification.

V5 must use pinned real workload evidence, preserve negative findings, and measure whether the research stack removes only the targeted proven variance while retaining genuine behavioral differences and unresolved Go cache/module/stdlib variability.

No integration into public alpha.4 is authorized by V4 alone.
