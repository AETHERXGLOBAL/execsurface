# ExecSurface — P5-A2 Verifier Identity Adequacy Addendum

Date: 2026-09-30
Branch: `development/post-alpha4-behavioral-integrity`
Parent gate: `docs/development/P5_A2_SCAI_SVR_BINDING_GATE.md`
Original green candidate source: `f8fdd2438277538d432cecbc3b5337f44f7f07cb`
Status: **PREREGISTERED — EXACTLY 4 ADDENDUM TESTS / ORIGINAL 14 TESTS UNCHANGED**

## Why this addendum exists

The original frozen A2 corpus reached 14/14 PASS on run `36747330206`, but the independent Critical-Milestone / Test-Adequacy review found one claim-to-test mismatch before scientific closure:

- the A2 hypothesis says SCAI and SVR represent the same verifier;
- SCAI v0.3 represents `predicate.producer` as a `ResourceDescriptor` describing the producer;
- SVR v0.2 defines `verifier.id` as the URI identifying the entity that performed verification;
- the original 14-test corpus binds SCAI `producer` to ExecSurface's detailed `verifier_identity`, but it does not directly attack substitution of SVR `verifier.id`.

Therefore the original successful run is retained as a valid **candidate**, not promoted to final A2 closure until this specific adequacy gap is falsified.

## Bounded identity profile

For the ExecSurface A2 research profile, the following cross-attestation relation is preregistered:

`SCAI producer.uri == SVR verifier.id`

and SCAI `producer == conditions.verifier_identity` remains mandatory.

This does not redefine either in-toto predicate. It is an ExecSurface interoperability constraint choosing the producer URI as the common verifier identifier because SVR requires a URI verifier ID. A missing producer URI is therefore insufficient for this bounded cross-attestation verifier-binding claim.

## Fixed roles

- **Innovation Scientist / Systems Architect:** use the smallest existing-standard identity relation; do not create a new predicate or identity format.
- **Anti-Drift / Scientific Integrity Reviewer:** preserve the original 14/14 corpus and all prior failures; do not weaken the same-verifier claim.
- **Independent Falsifier / Red Team:** attack SVR verifier-ID substitution and coordinated SCAI-side rebinding.
- **Independent Critical-Milestone Reviewer:** require this addendum before final A2 closure.

## Exact 4-test addendum corpus

Acceptance requires **4/4 PASS**, with the original A2 corpus still **14/14 PASS**:

1. **same verifier URI succeeds** — the valid fixture has `SCAI producer.uri == SVR verifier.id` and verifies;
2. **SVR verifier-ID substitution fails** — changing only `SVR verifier.id` and recomputing outer digests must fail closed;
3. **coordinated SCAI-side verifier substitution fails against stale SVR identity** — changing both SCAI producer and detailed `verifier_identity`, then rebuilding the SCAI→SVR digest binding, must still fail if SVR verifier ID remains the original identity;
4. **construction rejects missing or mismatched common verifier URI** — the input model must not construct an A2 bundle claiming common verifier identity when producer URI is absent or differs from the SVR verifier ID.

No original A2 test is removed, edited, renumbered, or relaxed by this addendum.

## Acceptance

Final A2 closure additionally requires:

- addendum: 4/4 PASS;
- original A2: 14/14 PASS;
- A1: 12/12 PASS;
- A0: 18/18 PASS;
- critical P4 / Semantics v3 / public M11 regressions remain green;
- immutable alpha.4 / stable tag boundary checks remain green.

Only then may the repository record:

`P5_VERIFICATION_ATTESTATION_BINDING_ESTABLISHED_BOUNDED`

A failure is retained as `TEST_ADEQUACY_COUNTEREXAMPLE` and may justify only the smallest validation-only correction.