# ExecSurface — Post-alpha.4 Clean Candidate Assembly Manifest

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Promotion evidence closeout: `cef8c5b8acb3f3c9e0671c497ad04837709bb024`
A8 protocol source: `5fc3bf3ad9c8074045eae45d57faa2e92340d855`
Status: **A8-A0 CLEAN BASE VERIFIED — A8-A1 ADMITTED FILES FROZEN**

## Clean-base rule

This branch was created directly from immutable public alpha.4. The post-alpha.4 promotion/research branch is evidence-only and MUST NOT enter this branch through merge, fast-forward, cherry-pick of broad history, or subtree import.

## A8-A1 admitted product/test files

Only the following paths may differ from alpha.4 during the A8-A1 Semantics-v3 tranche:

1. `crates/execsurface-model/Cargo.toml`
   - add a default-off `semantics-v3` feature only;
   - default feature set remains empty.

2. `crates/execsurface-model/src/lib.rs`
   - expose `semantics_v3` only behind `#[cfg(feature = "semantics-v3")]`;
   - preserve `RAW_OBSERVATION_SCHEMA_VERSION = 2` and all alpha.4 v2 types/defaults unchanged.

3. `crates/execsurface-model/src/semantics_v3.rs`
   - bounded repaired Semantics-v3 proof model from the accepted A1/A7 evidence lineage;
   - unavailable in default build.

4. `crates/execsurface-model/tests/semantics_v3_rework.rs`
   - exact D01-D06 destructive repair corpus, feature-gated by workflow invocation.

5. `crates/execsurface-model/tests/semantics_v3_promotion.rs`
   - exact A1 nine-test compatibility/authority corpus, feature-gated by workflow invocation.

6. `.github/workflows/post-alpha4-candidate-a8-a1-semantics-v3.yml`
   - clean-branch provenance, file-scope, default-off, v2 regression and v3 adversarial reproof gate.

7. `docs/development/POST_ALPHA4_CANDIDATE_ASSEMBLY_MANIFEST.md`
   - this frozen manifest.

## Explicit non-admissions in A8-A1

A8-A1 does not admit changes to:
- CLI behavior;
- GitHub Action behavior;
- baseline schema or baseline files;
- canonical v2 semantics;
- diff/policy/verdict defaults;
- observers or ptrace runtime;
- variance handling;
- attestation/provenance runtime integration;
- GitLab/self-hosted support;
- arm64 support;
- public release metadata, tags, `main`, or `@v0.1`.

## Rollback / safe-disable

The `semantics-v3` feature is default-off. Building without it must produce the alpha.4 v2 model behavior and public API surface except for the inert Cargo feature declaration itself. Removing/disabling the feature requires no baseline migration and mutates no user data.

## A8-A1 acceptance

A8-A1 passes only if one workflow execution proves:
- this branch descends directly from alpha.4 without promotion-branch merge ancestry;
- only the seven admitted paths differ from alpha.4;
- default features are empty and Semantics v3 is not compiled/exposed by default;
- alpha.4 v2 model/baseline/diff/policy regressions pass without `semantics-v3`;
- feature-enabled D01-D06 = 6/6 PASS;
- feature-enabled A1 = 9/9 PASS;
- model tests and Clippy/rustfmt pass both relevant modes;
- public `v0.1.0-alpha.4` and stable `v0.1` still resolve to the immutable source;
- no release, tag movement, `main` merge, or P8 closure occurs.

No partial pass is allowed.
