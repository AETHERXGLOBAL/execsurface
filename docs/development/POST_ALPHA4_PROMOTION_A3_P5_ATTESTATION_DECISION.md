# ExecSurface — Post-alpha.4 Promotion A3 P5 Attestation / Provenance Decision

Date: 2026-10-01
Tracking: #115
Candidate branch: `integration/post-alpha4-promotion-candidate`
Protocol: `POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_PROTOCOL.md`

## Decision

**`POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_ELIGIBLE_BOUNDED_CANDIDATE`**

Within the frozen A3 scope, the existing P5 composition using in-toto Statement v1, Runtime Trace v0.1, SCAI v0.3, SVR v0.2, SLSA provenance v1 references and GitHub/Sigstore artifact-attestation identity remains eligible for continued post-alpha.4 candidate integration.

This result does **not** establish that cryptographic validity implies semantic authority. The opposite boundary remains mandatory: signatures, provenance, standards labels, signer identity, workflow identity and verifier metadata do not raise semantic authority.

## Frozen source and evidence

Accepted candidate source:
`0f6cbc68bea380ff25889e762cb8cd5287a9c5a3`

Accepted workflow run:
`36858724331` — **SUCCESS**

Closeout artifact:
- ID: `11160521675`
- name: `post-alpha4-promotion-a3-p5-attestation-36858724331-1`
- digest: `sha256:d56abd784e9c6ee5f23e36417177756c118781f2a0c8be15c8b5220bf78414ca`
- expiry: `2026-12-30T11:58:26Z`

Promotion-specific attack artifact:
- ID: `11160171982`
- name: `post-alpha4-a3-attacks-36858724331-1`
- digest: `sha256:13c5dc170767fbc6e37b02ae0df6e9f3c72da5e078cc0df1c95e07ded10adb5e`

Sigstore reverification artifact:
- ID: `11160531607`
- name: `post-alpha4-a3-sigstore-36858724331-1`
- digest: `sha256:a40bae0e52c1ea63b66c824cf11b7feb72d84f1d6671fa1abc73388a98fe6f14`

## Retained negative execution evidence

Initial A3 run:
`36858589653` at source `f39ec43916c1971f2ea728ec05a19f1b2aaa4f28` — **FAIL** before the scientific A3 corpus because `cargo fmt --check` detected formatting differences in the newly added A3 test file.

The failure is retained. The only correction was the exact rustfmt-only formatting change recorded at:
`0f6cbc68bea380ff25889e762cb8cd5287a9c5a3`

No assertion, attack, threshold, model rule, product crate, protocol criterion or semantic expectation changed.

## Promotion-specific falsification result

The frozen A3 corpus passed **12/12**:
1. verified provenance did not inflate ambiguous REVIEW evidence;
2. verified provenance could not launder ambiguous authority into PASS;
3. verified provenance could not launder observer loss into PASS;
4. cross-subject provenance replay was rejected;
5. provenance predicate-type substitution was rejected;
6. malformed provenance statement digest was rejected;
7. source substitution failed expected-context binding;
8. baseline substitution failed expected-context binding;
9. current-surface substitution failed expected-context binding;
10. verifier-identity substitution failed expected-context binding;
11. provenance verification state remained explicitly bound while semantic authority remained independent;
12. duplicate/substituted provenance graph roles failed closed.

## Historical P5 reproof

The accepted A3 run also re-proved:
- P5-A5 cross-attestation red-team: **12/12 PASS**;
- P5-A3 pinned SLSA provenance-binding corpus: **PASS**;
- P5-A2 SCAI/SVR binding: **PASS**;
- P5-A2 verifier-identity addendum: **PASS**;
- P5-A1 Runtime Trace mapping: **PASS**;
- P5-A0 standards composition: **PASS**;
- accepted GitHub/Sigstore attestation independently reverified: **PASS**.

The historical P5-A5 negative run `36755962689` remains explicitly retained as a failure and was checked by the A3 gate.

## Upstream semantic-authority reproof

The accepted A3 run additionally re-proved:
- repaired Semantics-v3 promotion/rework corpus: **PASS**;
- P4 promotion authority sensitivity: **PASS**;
- P4 cross-proposition falsification: **PASS**;
- M11 shared-FD fail-closed regression: **PASS**;
- immutable public alpha.4 and stable `v0.1` anchors: **PASS**.

## Scientific conclusion

No executed A3 attack demonstrated that a valid-looking signature, provenance record, standards label or verifier identity can manufacture semantic authority inside the tested P5 candidate composition.

The bounded P5 research conclusion therefore remains intact:

**`P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`**

and there remains **no demonstrated need for a new custom predicate/schema in the bounded tested scope**.

This is not a claim that all future attestation integrations are safe. Any new predicate, signer model, verifier path, external provenance source or migration path requires its own binding and falsification gate.

## Public-state boundary

Unchanged:
- public release: `v0.1.0-alpha.4`;
- public release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`;
- public raw/canonical/baseline semantics: v2;
- no `main` merge authorized;
- no public P5/v3 promotion authorized;
- no arm64 claim authorized;
- P8 remains externally pending and cannot be self-certified from this result.

## Next promotion boundary

A3 authorizes only the next internal promotion assessment. Remaining research items must still be assessed independently; no research PASS converts automatically into a public feature.
