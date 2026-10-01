# ExecSurface — Post-alpha.4 Clean Candidate Assembly Manifest

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Status: **A8-A1 CLOSED PASS — A8-A2 CLOSED PASS — A8-A3 CLOSED PASS — A8-A4 NEXT**

## Closed clean-candidate tranches

### A8-A1 — Semantics v3
Accepted source `1c65327e99aa62a3b42549764bebe6b5b29516e7`; run `36870460513` SUCCESS; decision `POST_ALPHA4_PROMOTION_A8_A1_SEMANTICS_V3_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.
D01-D06 6/6, A1 9/9, full feature model, default workspace, rustfmt and Clippy passed. Pre-scientific failures `36867146485` and `36869713875` retained.

### A8-A2 — Proposition authority
Accepted source `4166ff64b8110e27b47954c1f0875cce81ebf898`; run `36872228726` SUCCESS; decision `POST_ALPHA4_PROMOTION_A8_A2_AUTHORITY_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.
Authority falsifier 16/16, D01-D06, A1, full feature model, default workspace, rustfmt and Clippy passed. Failed run `36871810571` retained; correction `112966e78730aea4e00296d34f2a46510fdb0b47` formatting-only.
No duplicated textual proposition identity; backend names cannot manufacture authority.

### A8-A3 — bounded GCC ephemeral variance
Protocol source `c30e4d886ef44dc11f85436240dc0fcd47e248ce`.
Accepted source `6c35861f66cc238374dc113d9431859bf0340bb9`; run `36874670716` SUCCESS; decision `POST_ALPHA4_PROMOTION_A8_A3_GCC_VARIANCE_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`.

Established:
- exact A3 corpus 12/12 PASS;
- full feature-enabled normalize regression PASS;
- default workspace + explicit normalize-default regression PASS;
- rustfmt and Clippy default/feature PASS;
- A1 and A2 frozen scientific corpora reproof PASS;
- exact product delta and immutable tag anchors PASS;
- `gcc-ephemeral-v4` remains default-off and public normalization profile remains 3;
- only exact GCC producer/role/grammar lifecycle projects;
- repetition/frequency never authorizes;
- raw canonical evidence remains unchanged and projection is a distinct deterministic derived view.

Implementation precision evidence:
- transient SHA-guarded patch harness was added then removed before the A3 gate;
- final `lib.rs` change was independently verified as exactly three added lines: `#[cfg(feature = "gcc-ephemeral-v4")]`, `pub mod gcc_ephemeral_v4;`, and a separator line; no existing normalization line changed.

A1/A2 remain slice-stability regression guards. A3 will also be converted to a slice-stability regression guard before A8-A4.

## Explicit non-admissions through A8-A3
No learned/general variance, frequency authorization, broad temp/cache suppression, CLI/Action default changes, baseline migration, backend equivalence, observer changes, attestation binding, arm64 claim, BPF-LSM default, release/tag/main movement, or P8 claim.

## Next tranche — A8-A4
A8-A4 may integrate only bounded attestation/provenance binding after a real clean-candidate source identity exists. Cryptographic validity, signer identity, standards labels, workflow identity, or provenance metadata must never manufacture semantic authority. Subject/source/baseline/current/verifier/provenance bindings must be explicit and fail closed.

## Release boundary
A8 internal success cannot authorize public alpha.5 while P8 external evidence remains insufficient. Internal evidence cannot be relabeled as external validation.
