# ExecSurface — Post-alpha.4 Clean Candidate Assembly Manifest

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Promotion evidence closeout: `cef8c5b8acb3f3c9e0671c497ad04837709bb024`
A8 protocol source: `5fc3bf3ad9c8074045eae45d57faa2e92340d855`
Status: **A8-A1 CLOSED PASS — A8-A2 IMPLEMENTED / FORMATTING CORRECTION PENDING REPROOF**

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

The A1 workflow is now a slice-stability integration regression gate: accepted Semantics-v3 source/tests remain byte-identical while D01-D06, A1 and default-v2 are rerun on later tranches.

## A8-A2 — Proposition authority

Preregistered protocol source: `7250002f3f7cb83f3f9ca930e8acedb524d57580`.
Implementation source: `34c7060a58270f9bd8ed538640fcf696876e741c`.
Research evidence: P4 closeout `320c865d3e81071a8214ad726dca7a587b0fc479`; promotion A2 `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`.

Admitted delta:
- `crates/execsurface-model/src/authority_v3.rs`;
- feature-gated export in `crates/execsurface-model/src/lib.rs`;
- `crates/execsurface-model/tests/authority_v3_candidate.rs`;
- `.github/workflows/post-alpha4-candidate-a8-a2-authority.yml`;
- this manifest.

Hardening: no duplicated textual `proposition_id`; typed `ProofCarryingObservation.proposition` is the sole proposition identity.

Retained first A8-A2 execution:
- run `36871810571`: provenance/scope/A1-slice/default-boundary PASS; authority-falsification job PASS including 16/16 A8-A2 attacks, D01-D06, A1 9/9, full feature model and Clippy; separate static/default job failed at rustfmt before its default regression, so closeout was skipped and the overall A8-A2 gate did **not** pass.
- correction after this run is formatting-only in `authority_v3.rs` and `authority_v3_candidate.rs`. No test name, assertion, acceptance rule, authority rule, threshold, product semantic behavior or public boundary is changed.

A8-A2 remains pending until one full workflow run passes all jobs including rustfmt/default regression and closeout.

## Explicit non-admissions through A8-A2

Not admitted: CLI/Action default changes; baseline migration; ptrace/Tetragon/backend equivalence; observer adapters; variance; attestation binding; arm64 claim; BPF-LSM default; generic learning/frequency authorization; release/tag/main movement; P8 claims.

## Release boundary

A8 internal success does not authorize public alpha.5 while P8 remains externally blocked. Internal evidence cannot be relabeled as external validation.
