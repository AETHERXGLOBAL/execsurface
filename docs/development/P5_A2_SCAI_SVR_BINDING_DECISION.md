# ExecSurface — P5-A2 SCAI / SVR Binding Decision

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Decision: **P5_VERIFICATION_ATTESTATION_BINDING_ESTABLISHED_BOUNDED**
Scope: research/development only; no public integration or release authorization.

## Decision

The bounded P5-A2 composition is accepted for the tested research profile.

The accepted contract is:

- SCAI v0.3 carries the detailed ExecSurface behavioral-verification assertion;
- SVR v0.2 carries the concise verification summary over the same subject;
- `SCAI producer == conditions.verifier_identity`;
- the common verifier identity is explicitly bound as `SCAI producer.uri == SVR verifier.id`;
- missing or substituted common verifier identity fails closed;
- runtime evidence is content-bound through the SCAI evidence descriptor;
- SVR policy identity remains bound to the exact policy descriptor;
- attestation presence, standard name, backend name, provenance presence or signature plumbing cannot raise semantic authority, completeness or verdict.

This is not a claim that SCAI/SVR by themselves prove runtime truth or that cryptographic verification implies an ExecSurface semantic PASS.

## Accepted source and CI evidence

Accepted source SHA:

`63b9a9fcde16d9dbb66aa484e73a1e2338afd6cb`

Full P5-A2 gate:

- workflow run: `36750284527`
- result: PASS
- artifact: `11113484839`
- artifact name: `p5-a2-36750284527-1`
- artifact SHA-256: `45f786ed36ca399ee35d595a5a38997a517edef088ff13764c8831b2da180984`
- artifact size: `9367` bytes

The accepted gate passed all of the following in the same run:

1. frozen alpha.4 / stable `v0.1` boundaries;
2. pinned in-toto SCAI/SVR standards snapshot;
3. lock, rustfmt and clippy static gates;
4. original P5-A2 frozen corpus: **14/14 PASS**;
5. verifier-identity adequacy addendum: **4/4 PASS**;
6. P5-A1 reproof: **12/12 PASS**;
7. P5-A0 reproof: **18/18 PASS**;
8. critical P4 authority reproofs, including bounded live B1/B2/B3 gates;
9. Semantics v3 reproof;
10. public M11 regression reproof;
11. candidate decision emitted only after the full gate passed.

## Retained failure history

Failures are retained and are part of the decision record.

### Identity-regression discovery

Runs `36748367396` and `36748367402` exposed that the newly enforced verifier-identity invariant invalidated the older A0/A1 fixtures before their intended adversarial logic ran. The observed reason was `verifier_identity_mismatch`.

This was treated as a forward-compatibility fixture defect under the stronger preregistered A2 identity contract, not as permission to weaken that contract.

Correction was deliberately minimal:

- no original A2 test was edited;
- no addendum test was edited;
- no threshold, expected error, authority rule or verdict rule was relaxed;
- only the A0/A1 fixture verifier URI was normalized to the already-declared `verifier_id`;
- the historical accepted A0/A1 source SHAs and evidence remain unchanged.

### Static formatting failure

Run `36749844334` failed only because the rewritten A0/A1 test files lacked the rustfmt-required terminal newline. It is retained as a pre-scientific/static failure. The correction was formatting-only.

## Independent adequacy correction

The original 14-test A2 corpus did not directly attack substitution of `SVR verifier.id`. The independent critical-milestone review therefore preregistered a separate four-test adequacy addendum before acceptance.

The CI gate now executes that addendum explicitly and blocks acceptance unless all four tests pass. The original 14-test corpus remains unchanged.

## Boundaries preserved

This decision does **not**:

- change public `v0.1.0-alpha.4` or stable `v0.1`;
- change public/raw baseline-v2 meaning;
- change the default observer;
- grant authority based on backend, standard, signer, GitHub, Sigstore or SLSA branding;
- manufacture SLSA provenance;
- authorize a new proprietary in-toto predicate;
- authorize release/promotion.

## Next gate

Proceed to **P5-A3 — SLSA provenance binding** under the frozen P5 protocol.

A3 must use a real/pinned provenance statement or fixture with exact digest and predicate-type binding and must retain negative controls for wrong digest, wrong subject, wrong workflow/source, unverified provenance, and provenance absence. No SLSA level may be inferred merely from provenance presence.
