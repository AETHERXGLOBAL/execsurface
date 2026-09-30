# ExecSurface — P5 Runtime Attestation & Provenance Closeout Decision

Date: 2026-09-30
Parent program: #100
Protocol: `docs/development/P5_CLOSEOUT_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED**

Retained A5 result:

**P5_A5_CROSS_ATTESTATION_GRAPH_PASS_BOUNDED**

Scientific wording boundary:

> No demonstrated need for a new custom predicate/schema in the bounded P5 experiment.

This is not a universal representation claim and does not state that custom domain data is absent. ExecSurface-specific SCAI attribute/condition data remains domain-specific while using existing standard predicate structures.

## Independent closeout evidence

Successful closeout workflow:

- source SHA: `c3caa857581d6370008fe30779bd5fd084b28751`
- workflow: `.github/workflows/p5-closeout.yml`
- run: `36760093433`
- conclusion: `success`
- evidence artifact ID: `11118009105`
- artifact name: `p5-closeout-36760093433-1`
- artifact digest: `sha256:94312e440d6873bb3577ada92e3e0f13b52d103ccf63db962c8d367a8c52a815`
- artifact size: `38377` bytes
- artifact expiry at capture: `2026-10-30T18:39:18Z`

The closeout run independently verified:

1. predecessor ancestry from P4 through P5-A4 and both A5 evidence sources;
2. immutable public `v0.1.0-alpha.4` and `v0.1` source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
3. no public `crates/` changes introduced by P5 research after P4 closeout;
4. exact successful A5 run and artifact metadata/digest;
5. exact retained historical A5 counterexample run and artifact metadata/digest;
6. internal evidence-file checksums for both A5 artifacts;
7. the frozen 12/12 successful A5 result and candidate marker;
8. the retained 8/12 historical counterexample result;
9. mandatory ExecSurface attestation payload representation;
10. existing Runtime Trace, SCAI, SVR, and SLSA predicate types in the research model;
11. independent live re-verification of the accepted P5-A4 GitHub/Sigstore attestation.

## Retained negative evidence

The first scientifically executable P5-A5 run remains part of the scientific record:

- source SHA: `a7753bee6cc9f145adc7823dd82837fa40cfca01`
- run: `36755962689`
- frozen result: **8 passed / 4 failed**
- artifact ID: `11116268894`
- artifact digest: `sha256:97aa4db53e27355d8b2d62e86d6f1ba5c2ee6d7d6e10607b965e4730fa878314`

The four surviving attacks were baseline substitution, current-surface substitution, source substitution, and duplicate semantic SVR property. They were classified as implementation-level cross-attestation binding/canonicalization gaps, not evidence of a standards representation gap. The exact 12-test corpus and acceptance threshold were not weakened.

## Corrected A5 evidence

Successful corrected A5 source/run:

- source SHA: `d6d0d45b686ed8c9332ea6b0924537a345049bfb`
- run: `36758531046`
- result: **12/12 PASS**
- artifact ID: `11117568130`
- artifact digest: `sha256:d1ba8f88a42b0b3f23880445f6063735cef99b306861241ed2e04cdeee57d343`

The same run also re-proved accepted P5-A4/A3/A2/A1/A0, critical P4 authority gates, Semantics v3, and public M11.

## What P5 establishes, bounded

For the tested research scope, ExecSurface can represent and bind its runtime-verification result using a composition of:

- in-toto Statement v1;
- Runtime Trace v0.1;
- SCAI v0.3;
- SVR v0.2;
- optional exact SLSA provenance v1 reference;
- GitHub/Sigstore artifact-attestation identity and verification.

The composition preserves explicit subject, source/artifact/workflow context, baseline/current-surface digests, observer/capability/completeness, policy, verdict, evidence, provenance, and verifier identity under the tested graph verifier.

Cryptographic/signature validity remains separate from semantic authority. Signatures, provenance, backend names, and standards labels do not upgrade an otherwise insufficient behavioral-verification result.

## Non-claims / promotion boundary

This decision does NOT authorize:

- public integration or release promotion;
- changes to `main`, alpha.4 semantics, stable `@v0.1`, public v2 meaning, or the default observer;
- a universal claim that the standards composition represents every runtime-verification system;
- a claim that generic consumers automatically understand ExecSurface proposition authority;
- a SLSA level claim;
- deletion or reinterpretation of negative evidence.

P5 is therefore **scientifically closed, bounded, research-only**. The next parent-program phase is **P6 — Competitive Falsification**, which must compare overlapping factual propositions rather than produce a marketing winner score.