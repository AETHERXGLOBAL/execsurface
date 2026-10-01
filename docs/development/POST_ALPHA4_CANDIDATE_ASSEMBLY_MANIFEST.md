# ExecSurface — Post-alpha.4 Clean Candidate Assembly Manifest

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Status: **A8-A1 CLOSED PASS — A8-A2 CLOSED PASS — A8-A3 IMPLEMENTATION STAGED**

## Closed clean-candidate tranches

### A8-A1 — Semantics v3
Accepted source `1c65327e99aa62a3b42549764bebe6b5b29516e7`; run `36870460513` SUCCESS; decision `POST_ALPHA4_PROMOTION_A8_A1_SEMANTICS_V3_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.
D01-D06 6/6, A1 9/9, full feature model, default workspace, rustfmt and Clippy passed. Pre-scientific failures `36867146485` and `36869713875` retained.

### A8-A2 — Proposition authority
Accepted source `4166ff64b8110e27b47954c1f0875cce81ebf898`; run `36872228726` SUCCESS; decision `POST_ALPHA4_PROMOTION_A8_A2_AUTHORITY_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.
Authority falsifier 16/16, D01-D06, A1, full feature model, default workspace, rustfmt and Clippy passed. Failed run `36871810571` retained; correction `112966e78730aea4e00296d34f2a46510fdb0b47` formatting-only.
No duplicated textual proposition identity; backend names cannot manufacture authority.

A1 and A2 are now slice-stability regression guards on every later tranche.

## A8-A3 — bounded GCC ephemeral variance

Protocol source: `c30e4d886ef44dc11f85436240dc0fcd47e248ce`.
Evidence source: P3 promotion source `96ef1e2f6f316460cc88af4f89403455a910b2b4`, run `36861859874`.

Staged product delta:
- default-off `gcc-ephemeral-v4` feature in `execsurface-normalize`;
- bounded derived projection module `gcc_ephemeral_v4.rs`;
- exact 12-test adversarial candidate corpus;
- feature-gated module export to be applied by a SHA-guarded transient patch harness before the A3 gate;
- no public/default normalization behavior or version changes.

Frozen rule: only `/usr/bin/gcc` family `gcc`, execution-chain tail match, exact `$TMP/cc[0-9A-Za-z]{6}.s` grammar, create+write Open plus Delete, no conflict/rename. Frequency never authorizes; raw evidence is retained unchanged; output is a derived deterministic view.

A8-A3 is not closed until A1/A2 regressions, the exact 12-test corpus, default normalize/workspace regression, rustfmt, Clippy, feature/default-boundary and tag-anchor gates all pass in one accepted run.

## Explicit non-admissions through A8-A3
No learned/general variance, frequency authorization, broad temp/cache suppression, CLI/Action default changes, baseline migration, backend equivalence, observer changes, attestation binding, arm64 claim, BPF-LSM default, release/tag/main movement, or P8 claim.

## Release boundary
A8 internal success cannot authorize public alpha.5 while P8 external evidence remains insufficient.
