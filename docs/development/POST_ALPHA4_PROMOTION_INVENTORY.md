# ExecSurface — Post-alpha.4 Promotion Inventory

Date: 2026-10-01
Tracking: #115
Candidate branch: `integration/post-alpha4-promotion-candidate`
A0 rule: **inventory records research state; it does not promote research into the public product.**

## Status vocabulary

`UNASSESSED` | `ELIGIBLE_BOUNDED` | `DEFER` | `BLOCKED` | `NEGATIVE_RETAINED`

No product-affecting row may start A0 as `ELIGIBLE_BOUNDED`.

## Inventory

| Program | Research result | Status | Evidence source / boundary | Promotion note |
|---|---|---|---|---|
| P2 | Semantics v3: proposition-scoped evidence, explicit authority/completeness, proof-carrying observation, bounded compatibility work | UNASSESSED | Parent program #100; accepted bounded research on `development/post-alpha4-behavioral-integrity` | First product-affecting assessment because downstream contracts depend on it. No silent v2->v3 reinterpretation. |
| P3 | Legitimate-variance / ephemeral-identity work | UNASSESSED | Bounded research decision retained on parent branch | Must prove lower false REVIEW with zero new false PASS; frequency never authorizes behavior. |
| P4 | Backend Adapter / Proposition Authority architecture | UNASSESSED | P4 closeout source `320c865d3e81071a8214ad726dca7a587b0fc479` | Promotion requires proposition-by-proposition compatibility; backend name never raises authority. |
| P5 | Runtime attestation/provenance composition using existing standards | UNASSESSED | Final bounded decision `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`; closeout lineage includes `1f4df8f45a115a1638c997107e069c6acd74c51b` | Cryptographic validity remains separate from semantic authority; no custom-standard claim. |
| P6 | Competitive falsification/comparison harness and factual matrix | UNASSESSED | P6 bounded closeout; successful closeout workflow run `36773688443` | Assess as validation/tooling infrastructure separately from runtime feature promotion. No winner score. |
| P7-arm64 | Native arm64 parity path tested and not established portable in bounded experiment | NEGATIVE_RETAINED | Retained P7 A0 failure and bounded arm64 not-portable decision | Must not be promoted as arm64 support. Fresh gate required for any future claim. |
| P7-other | Additional CI/platform research results other than failed arm64 parity | UNASSESSED | P7 bounded closeout; workflow run `36776286847` | Assess individually; platform count cannot weaken evidence contracts. |
| P8 | Current external validation / independent reproduction / criticism | BLOCKED | Issue #114; A5 prereg source `7bd806258ecdca415ef16ffb3689aefa13c0ece2` | Current external evidence minimum not yet satisfied. Internal work cannot manufacture independence. |

## Immutable public anchors

- Public release: `v0.1.0-alpha.4`
- Public release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable Action: `AETHERXGLOBAL/execsurface@v0.1`
- At A0 freeze, both `v0.1.0-alpha.4` and `v0.1` resolve to the public release source above.

## First assessment after A0

A1 will evaluate **P2 Semantics v3 promotion eligibility only**.

A1 must not implement public product changes before it freezes:
- exact v2 -> v3 semantic delta;
- compatibility/migration contract;
- fail-closed invariants;
- rollback/safe-disable boundary;
- adversarial fixtures;
- exact candidate API/schema surface.

If the P2 compatibility boundary cannot be made explicit without reinterpretation of alpha.4, A1 must return `DEFER` or `BLOCKED`, not weaken the gate.
