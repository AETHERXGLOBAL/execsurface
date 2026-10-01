# ExecSurface — Promotion A1 Semantics v3 Eligibility Decision

Date: 2026-10-01
Tracking: #115
Protocol: `POST_ALPHA4_PROMOTION_A1_SEMANTICS_V3_PROTOCOL.md`
Candidate branch: `integration/post-alpha4-promotion-candidate`

## Decision

**`POST_ALPHA4_PROMOTION_A1_P2_ELIGIBLE_BOUNDED_FOR_CANDIDATE_IMPLEMENTATION`**

Semantics v3 is eligible to proceed to a separately gated, isolated post-alpha.4 candidate implementation **only as an explicit side-by-side version domain**. This decision does not authorize replacing v2, changing public alpha.4, enabling v3 by default, migration, release, tag movement, or merging to `main`.

## Evidence

### Retained failed run

- run: `36840824073`
- source: `bdb931245219288da8d8030075e9fcb2bc574db7`
- result: **FAIL** at `cargo fmt --all -- --check`
- classification: formatting-only failure in `crates/execsurface-model/tests/semantics_v3_promotion.rs`
- Clippy and scientific tests did not execute in this run.

The failure is retained. No metric, threshold, semantic expectation, or attack was changed.

### Minimal correction

- correction source: `2bbb61dcdfaf212b918ac6e3c35ec0e4bbdc6cbf`
- the correction changed only `crates/execsurface-model/tests/semantics_v3_promotion.rs`;
- the diff was rustfmt-only line wrapping/formatting;
- no runtime/public/product file changed.

### Accepted run

- run: `36840921782`
- source: `2bbb61dcdfaf212b918ac6e3c35ec0e4bbdc6cbf`
- result: **SUCCESS**
- artifact: `11151432844`
- artifact digest: `sha256:311fe873402b101e7db441701398af035077b3e4f87012c4cbb6532e9e5376b1`

Accepted checks:

- public alpha.4/stable tag anchors: PASS;
- frozen v2 version-domain checks: PASS;
- explicit v3 version-domain check: PASS;
- preregistered compatibility-boundary checks: PASS;
- A1 research-only change-scope check: PASS;
- rustfmt: PASS;
- model Clippy `-D warnings`: PASS;
- dedicated A1 adversarial tests: **9 passed / 0 failed**;
- baseline unit tests: **7 passed / 0 failed**;
- diff unit tests: **8 passed / 0 failed**;
- existing model unit tests: **7 passed / 0 failed**;
- repeated A1 integration tests under all-target reproof: **9 passed / 0 failed**;
- policy unit tests: **11 passed / 0 failed**.

## What was established

Within the tested bounded scope:

1. a v2-shaped raw observation does not deserialize as a v3 proof-carrying record;
2. a fully shaped proof record labeled schema `2` cannot satisfy the v3 proof admission rule;
3. missing required completeness is non-admissible;
4. `incomplete`, `ambiguous`, and `unsupported` required completeness states are non-admissible;
5. an authoritative-sounding backend name cannot upgrade weak pathname evidence into kernel-object grounding;
6. identical behavior with different proof authority is distinct evidence;
7. tested set-order variation preserves deterministic prototype serialization;
8. all frozen public/v2 version constants remained unchanged;
9. public alpha.4 and stable `v0.1` remained anchored to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.

## Mandatory architecture boundary for the next implementation gate

Any candidate implementation admitted after A1 must preserve:

```text
explicit schema discriminator
  -> v2 parser / verifier / digest rules
  -> v3 parser / verifier / proof rules
  -> unsupported schema error
```

Cross-schema default remains:

- v2 baseline vs v3 candidate -> `INCOMPARABLE_SCHEMA`;
- v3 baseline vs v2 candidate -> `INCOMPARABLE_SCHEMA`.

There is no default PASS-eligible v2 -> v3 projection.

Any future migration must be explicit, non-destructive, preserve original v2 provenance, produce a distinct v3 artifact, and reacquire evidence absent from v2.

## Independent falsifier / anti-drift conclusion

No executed A1 attack exposed a path that justified weakening the frozen S7/S8 constraints. The only failed run was formatting-only and was corrected without semantic change.

This result is **eligibility for candidate implementation**, not evidence that a production v3 parser, baseline, diff, migration path, or public CLI exists or is correct.

## Public state

Unchanged:

- public release: `v0.1.0-alpha.4`;
- public release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`;
- public raw/canonical/baseline semantics: v2;
- P8: externally pending;
- Linux arm64 promotion: not authorized / negative retained.

## Next authorized gate

**A2 — P4 Proposition Authority / Backend Adapter promotion eligibility.**

A2 must consume the A1 side-by-side version boundary and determine which P4 proposition-authority contracts are eligible to become the first isolated v3 candidate implementation. Backend labels remain non-authoritative and unsupported propositions remain non-comparable/non-PASS.
