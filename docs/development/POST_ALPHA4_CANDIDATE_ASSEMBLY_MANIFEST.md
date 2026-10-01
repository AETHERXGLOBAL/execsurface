# ExecSurface — Post-alpha.4 Clean Candidate Assembly Manifest

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Promotion evidence closeout: `cef8c5b8acb3f3c9e0671c497ad04837709bb024`
A8 protocol source: `5fc3bf3ad9c8074045eae45d57faa2e92340d855`
Status: **A8-A1 CLOSED PASS — A8-A2 IMPLEMENTED / GATE PENDING**

## Clean-base invariant

This branch descends directly from immutable public alpha.4. Promotion/research history is evidence only and is never merged wholesale into the candidate.

Public anchors remain immutable during A8:
- `v0.1.0-alpha.4` -> `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable `v0.1` -> same source.

## A8-A1 — Semantics v3

Accepted source: `1c65327e99aa62a3b42549764bebe6b5b29516e7`
Accepted workflow run: `36870460513` — SUCCESS.
Decision: `POST_ALPHA4_PROMOTION_A8_A1_SEMANTICS_V3_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.

Established:
- clean alpha.4 ancestry and tag anchors;
- default-off `semantics-v3` feature;
- public raw schema remains v2;
- exact D01-D06 = 6/6 PASS;
- original A1 corpus = 9/9 PASS;
- feature-enabled model PASS;
- default alpha.4-compatible workspace regression PASS;
- rustfmt and Clippy `-D warnings` PASS.

Retained pre-scientific negative evidence:
- `36867146485`: pinned Rust minimal profile omitted rustfmt/clippy; scientific corpus did not execute.
- `36869713875`: rustfmt-only differences plus stale lockfile dev-dependency entry; scientific corpus did not execute.
- corrections were harness/format/lockfile-only and changed no assertion, threshold, proof rule, semantic admission rule, default behavior, or public tag.

After closure, the A1 workflow was converted to a slice-stability integration regression gate. It keeps the accepted A1 semantic/test files byte-identical while rerunning D01-D06, A1 and default-v2 regressions on every later tranche.

## A8-A2 — Proposition authority

Preregistered protocol source: `7250002f3f7cb83f3f9ca930e8acedb524d57580`.
Research evidence sources: P4 closeout `320c865d3e81071a8214ad726dca7a587b0fc479`; promotion A2 `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`.

Admitted implementation delta:
- `crates/execsurface-model/src/authority_v3.rs`;
- feature-gated export in `crates/execsurface-model/src/lib.rs`;
- `crates/execsurface-model/tests/authority_v3_candidate.rs`;
- `.github/workflows/post-alpha4-candidate-a8-a2-authority.yml`;
- this cumulative manifest.

Product hardening decision: do not carry the P4 research prototype's duplicated textual `proposition_id`. Typed `ProofCarryingObservation.proposition` is the sole proposition identity, reducing mismatch/laundering surface without broadening authority.

A8-A2 remains pending until its exact 16-attack corpus, A1 regressions, default-v2 regression, rustfmt, Clippy, ancestry/scope and immutable-tag gates all PASS in one accepted workflow run.

## Explicit non-admissions through A8-A2

Not admitted:
- CLI/GitHub Action default behavior changes;
- baseline schema migration;
- ptrace/Tetragon/backend equivalence;
- observer/runtime adapter changes;
- variance handling;
- attestation runtime binding;
- arm64 support claim;
- BPF-LSM default backend;
- generic learning/frequency authorization;
- release/tag/main movement;
- P8 external-validation claim.

## Release boundary

A8 internal success does not authorize public alpha.5 while P8 remains externally blocked. Internal evidence cannot be relabeled as external validation.
