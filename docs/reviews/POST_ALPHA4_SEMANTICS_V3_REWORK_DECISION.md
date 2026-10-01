# ExecSurface — Semantics v3 Rework Independent Review Decision

Date: 2026-10-01
Decision: `POST_ALPHA4_SEMANTICS_V3_REWORK_REQUALIFIED_BOUNDED_INTERNAL`
Status: CLOSED — BOUNDED INTERNAL REQUALIFICATION / NO PUBLIC PROMOTION

## Scope

This decision covers the repair of the three material Semantics v3 admission gaps retained from the destructive review of source `5079a990b924d8ccd7ac6414f8a9a2571b54e240`:

1. proposition/subject-unbound proof requirements;
2. invalidating ambiguity coexisting with a required `Complete` claim;
3. vacuous/default proof requirements admitting arbitrary v3 evidence.

The repaired implementation source under independent review was `0200f09557906118dd0e96f8a5a73aa4f5c4a9cc`.
The independent review source was `27f1df750439db9cc4453df7d78cf5b6a38a2ee8`.

## Repair architecture accepted

The bounded repair is accepted because it changes the proof contract rather than adding a test-specific exception:

- `ProofRequirement` carries an explicit `expected_proposition`;
- admission rejects proposition mismatch before guarantee/completeness evaluation;
- a requirement without a bound proposition is fail-closed;
- a bound requirement with no guarantee or completeness obligation is fail-closed;
- `object_identity_conflict` invalidates admission when `ObjectIdentity` is required;
- unknown ambiguity codes fail closed;
- schema v3 remains explicit and side-by-side with public v2 semantics;
- backend/profile metadata does not create semantic authority.

No implicit global state, call-order binding, backend-name inference, threshold relaxation, or public v2 reinterpretation was introduced.

## Retained execution history

### Focused repair gate

Successful run: `36852004709`

- repaired D01-D06 corpus: PASS;
- prior A1 corpus: PASS;
- model/baseline/diff/policy regression set: PASS;
- public alpha.4 and stable `v0.1` anchors unchanged.

Artifact:
- ID: `11155209591`
- name: `post-alpha4-semantics-v3-rework-36852004709-1`
- digest: `sha256:8ab42cadf120cab2a177a919f7e17e9ca77d01d22bf013286cd353c853c9f6de`

Retained predecessor run `36851919778` failed at rustfmt before scientific tests; correction was formatting-only.

### Broad regression replay

Run `36852151298` is retained as a mixed/failing run rather than rewritten:

- repaired Semantics replay: PASS;
- P3 falsifier: PASS;
- M11 fail-closed replay: PASS;
- M12 adversarial replay on Ubuntu 22.04: PASS;
- M12 adversarial replay on Ubuntu 24.04: PASS;
- immutable public boundary: PASS;
- P4 cross-proposition tests reached 12/12 PASS, then an old test initializer failed to compile because the new explicit `expected_proposition` field was not yet supplied;
- P5 A5 reached 12/12 PASS, then P5 A3 failed because the broad replay harness omitted the preregistered pinned SLSA fixture environment.

These were classified as harness/compatibility adaptations, not semantic PASS evidence, and were rechecked separately before this decision.

### P4 compatibility recheck

Successful run: `36852373536`

- cross-proposition falsification: PASS;
- backend-name anti-inflation: PASS;
- live open-object authority: PASS;
- live rename/delete authority: PASS;
- live connect authority: PASS;
- public anchors unchanged.

The only change preceding the recheck was adaptation of P4 test proof requirements to bind the same proposition explicitly; no assertion or authority threshold was weakened.

### P5 pinned-fixture recheck

Successful run: `36852562479`

- pinned SLSA v1 upstream fixture fetched and verified using the frozen upstream commit/blob identity;
- P5 A5 cross-attestation red-team: PASS;
- P5 A3 SLSA provenance binding: PASS;
- P5 A2/A1/A0 composition corpora: PASS;
- immutable public anchors: PASS.

The earlier P5 failure is retained as a missing-fixture harness failure.

### Independent falsification review

Successful run: `36853136002`
Review source: `27f1df750439db9cc4453df7d78cf5b6a38a2ee8`

Independent preregistered F01-F12 corpus:
- **12 passed / 0 failed**.

Reproof in the same run:
- repaired D01-D06: **6 passed / 0 failed**;
- prior A1 corpus: **9 passed / 0 failed**;
- model library: **11 passed / 0 failed**;
- rustfmt: PASS;
- Clippy `-D warnings`: PASS;
- immutable alpha.4/stable-tag boundary: PASS.

Artifact:
- ID: `11156392496`
- name: `semantics-v3-independent-falsification-36853136002-1`
- digest: `sha256:5f0fdeca1a90846fe61ad2ee84e1974139fb8f9161cd9b608aadb8dd97d7f039`
- expiry: `2026-12-30T11:05:07Z`

Two predecessor independent-review runs are retained:
- `36852877964` — rustfmt-only failure before scientific tests;
- `36852982973` — harness misuse of `OpenIntent` detected by Clippy/compile before scientific tests.

Neither predecessor result was relabeled as scientific PASS.

## Independent reviewer judgment

The repaired proof-admission contract resisted both the original destructive attacks and a separate 12-case corpus covering legacy deserialization, actor substitution, operation substitution, execution-chain substitution, future-schema substitution, completeness downgrade, backend/profile laundering, completeness-only valid contracts, scoped object-identity ambiguity, unknown ambiguity, and stronger-evidence proposition substitution.

Within this bounded corpus, no false-admission path survived.

Therefore the earlier material finding remains historically valid for the broken candidate, while the repaired Semantics v3 contract is requalified for continued **internal post-alpha.4 candidate work**.

## What this decision does NOT authorize

This decision does not authorize:

- merge to `main`;
- a new public release;
- retagging `AETHERXGLOBAL/execsurface@v0.1`;
- silent v2→v3 migration;
- automatic baseline migration or mutation;
- arm64 portability claims;
- universal correctness or formal completeness claims;
- external validation, adoption, endorsement, or production-readiness claims;
- P8 closure.

Public alpha.4 remains pinned at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.
P8 remains external-evidence dependent.

## Next promotion state

Semantics v3 A1 may move from `BLOCKED_BY_DESTRUCTIVE_REVIEW` to:

`REQUALIFIED_BOUNDED_INTERNAL — ELIGIBLE FOR NEXT PROMOTION/INTEGRATION GATE`

It is not yet a public product feature.
