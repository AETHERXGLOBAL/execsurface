# ExecSurface — Post-alpha.4 Clean Candidate Assembly Manifest

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Promotion evidence closeout: `cef8c5b8acb3f3c9e0671c497ad04837709bb024`
A8 protocol source: `5fc3bf3ad9c8074045eae45d57faa2e92340d855`
Status: **A8-A1 CLOSED PASS — A8-A2 CLOSED PASS — A8-A3 NEXT**

## Clean-base invariant

This branch descends directly from immutable public alpha.4. Promotion/research history is evidence only and is never merged wholesale into the candidate.

Public anchors remain immutable during A8:
- `v0.1.0-alpha.4` -> `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable `v0.1` -> same source.

## A8-A1 — Semantics v3

Accepted source: `1c65327e99aa62a3b42549764bebe6b5b29516e7`
Accepted workflow run: `36870460513` — SUCCESS.
Decision: `POST_ALPHA4_PROMOTION_A8_A1_SEMANTICS_V3_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.

Established: clean ancestry/tag anchors; default-off Semantics v3; public raw schema v2; D01-D06 6/6; A1 9/9; feature model/default workspace/rustfmt/Clippy PASS.

Retained A8-A1 pre-scientific failures:
- `36867146485`: missing pinned rustfmt/clippy components; scientific corpus did not execute.
- `36869713875`: formatting-only + lockfile dev-dependency entry; scientific corpus did not execute.
No assertion, threshold, proof rule, semantic admission rule, default behavior, or public tag was weakened.

The A1 workflow is a slice-stability integration regression gate on later tranches.

## A8-A2 — Proposition authority

Preregistered protocol source: `7250002f3f7cb83f3f9ca930e8acedb524d57580`.
Implementation source: `34c7060a58270f9bd8ed538640fcf696876e741c`.
Formatting-only correction source: `112966e78730aea4e00296d34f2a46510fdb0b47`.
Accepted source: `4166ff64b8110e27b47954c1f0875cce81ebf898`.
Accepted workflow run: `36872228726` — SUCCESS.
Decision: `POST_ALPHA4_PROMOTION_A8_A2_AUTHORITY_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.
Research evidence: P4 closeout `320c865d3e81071a8214ad726dca7a587b0fc479`; promotion A2 `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`.

Established:
- exact authority attack corpus 16/16 PASS;
- D01-D06 6/6 PASS and A1 9/9 PASS;
- full feature-enabled model PASS;
- rustfmt and Clippy default/v3 PASS;
- default alpha.4-compatible regression PASS;
- backend names cannot manufacture authority;
- unsupported/ambiguous/lost remain fail-closed;
- no duplicated textual proposition identity exists in the product contract.

Retained negative A8-A2 execution:
- `36871810571`: scientific authority job PASS, but separate static job failed rustfmt before default regression; overall gate did not pass and closeout was skipped.
- correction `112966e78730aea4e00296d34f2a46510fdb0b47` was formatting-only; no test/assertion/authority rule/threshold changed.

The A2 workflow is now an authority-slice integration regression gate on later tranches.

## Explicit non-admissions through A8-A2

Not admitted: CLI/Action default changes; baseline migration; ptrace/Tetragon/backend equivalence; observer adapters; variance; attestation binding; arm64 claim; BPF-LSM default; generic learning/frequency authorization; release/tag/main movement; P8 claims.

## Release boundary

A8 internal success does not authorize public alpha.5 while P8 remains externally blocked. Internal evidence cannot be relabeled as external validation.
