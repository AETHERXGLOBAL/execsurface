# ExecSurface — Post-alpha.4 Promotion A3: P5 Attestation / Provenance Eligibility Protocol

Date: 2026-10-01
Tracking: #115
Candidate branch: `integration/post-alpha4-promotion-candidate`
Predecessor decision: `POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_REQUALIFIED_BOUNDED_CANDIDATE`
Status: **PREREGISTERED — P5 PROMOTION ELIGIBILITY ONLY / NO PUBLIC INTEGRATION AUTHORIZED**

## Objective
Determine whether the bounded P5 composition of in-toto Statement v1, Runtime Trace v0.1, SCAI v0.3, SVR v0.2, SLSA provenance v1 references and GitHub/Sigstore artifact-attestation identity can be admitted into the post-alpha.4 candidate without allowing cryptographic validity, provenance metadata, verifier identity, or attestation shape to manufacture semantic authority.

A3 evaluates promotion eligibility, not public release readiness.

## Fixed roles
1. **Innovation Scientist / Systems Architect** — seek the smallest standards-composing candidate that preserves semantic authority separation and avoids an unnecessary custom standard.
2. **Anti-Drift / Scientific Integrity Reviewer** — prevent signature/provenance prestige from becoming semantic authority, prevent test weakening, and retain every negative result.
3. **Independent Falsifier / Red Team** — attack subject/source/baseline/current/verifier/provenance bindings, replay, substitution, authority laundering and graph canonicalization.
4. **Independent Critical-Milestone Reviewer** — independently recheck source identity, pinned standards/fixtures, attack coverage, retained failures and final claim wording.

## Dynamic specialist team
- in-toto attestation / Runtime Trace specialist;
- SLSA provenance specialist;
- Sigstore / GitHub artifact-attestation specialist;
- programming-languages / semantic-authority specialist;
- Rust serde / canonicalization specialist;
- software-supply-chain red team;
- reproducibility / GitHub Actions specialist;
- backward-compatibility / release-governance specialist.

## Immutable boundaries
- Public `v0.1.0-alpha.4` remains pinned to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.
- Stable `AETHERXGLOBAL/execsurface@v0.1` remains pinned to the same public source.
- Public raw/canonical/baseline semantics remain v2.
- Repaired Semantics v3 and P4 authority remain candidate/research-only.
- `BACKEND_EQUIVALENCE_NOT_ESTABLISHED` remains true.
- P8 cannot be closed from A3 or any other internal evidence.
- No arm64 support claim is authorized.
- No new custom attestation predicate/schema is justified unless existing standards composition is falsified as insufficient.

## Core separation rule

**Cryptographic validity, signature validity, provenance presence, standards conformance, signer identity, workflow identity, or verifier metadata MUST NOT raise semantic authority.**

Semantic PASS eligibility still requires the independently established semantic contract: healthy observation, direct proposition-scoped authority, complete required evidence, correct subject/context bindings and an explicit admissible policy/verdict path.

A cryptographically valid attestation over weak, ambiguous, incomplete, lost, unsupported, replayed, substituted or wrongly bound evidence remains semantically weak, ambiguous, incomplete, lost, unsupported, replayed, substituted or wrongly bound.

## Frozen A3 promotion-specific adversarial corpus
Exactly **12 A3 promotion attacks/invariants** are frozen before execution:

1. **A3-01 — Provenance prestige non-inflation:** verified provenance attached to ambiguous evidence must remain REVIEW/non-PASS.
2. **A3-02 — Ambiguous-to-PASS laundering:** valid-looking verified provenance must not allow ambiguous authority to build a PASS bundle.
3. **A3-03 — Loss-to-PASS laundering:** valid-looking verified provenance must not allow observer loss/incomplete evidence to build a PASS bundle.
4. **A3-04 — Cross-subject provenance replay:** provenance bound to another subject must be rejected.
5. **A3-05 — Predicate-type substitution:** non-SLSA or substituted provenance predicate type must be rejected where the SLSA v1 reference is required.
6. **A3-06 — Provenance statement-digest corruption:** malformed provenance statement digest must be rejected.
7. **A3-07 — Source substitution:** an otherwise valid graph must fail against a different expected source identity.
8. **A3-08 — Baseline substitution:** an otherwise valid graph must fail against a different expected baseline digest.
9. **A3-09 — Current-surface substitution:** an otherwise valid graph must fail against a different expected current-surface digest.
10. **A3-10 — Verifier-identity substitution:** an otherwise valid graph must fail against a different expected verifier identity/id.
11. **A3-11 — Provenance verification-state binding:** `verified=false` and `verified=true` are distinct provenance contexts; one must not satisfy the other while semantic verdict remains independently represented.
12. **A3-12 — Graph-role replay/duplication:** duplicate or substituted provenance graph roles/digests must fail closed.

The corpus is immutable for the first A3 execution. A harness/format/build failure may receive only the smallest harness-only correction; no assertion, threshold or semantic expectation may change after results are observed.

## Mandatory historical replays
A3 must additionally reprove, unchanged:
- P5-A5 12/12 cross-attestation attacks including baseline/current/source substitution and duplicate semantic SVR property;
- P5-A3 pinned SLSA provenance-binding corpus;
- P5-A2 SCAI/SVR and verifier-identity corpora;
- P5-A1 Runtime Trace mapping;
- P5-A0 standards-composition corpus;
- repaired Semantics-v3 D01–D06 and A1 promotion corpus;
- P4 A2 authority sensitivity and cross-proposition/backend-name non-inflation boundaries;
- M11 shared-FD fail-closed regression;
- immutable public alpha.4/stable-tag anchors.

## Kill criteria
A3 is BLOCKED if any reproducible path allows any of the following:
- signature/provenance/verifier metadata raises semantic authority;
- ambiguous/incomplete/lost evidence becomes PASS because it is attested or signed;
- a different subject/source/baseline/current surface/verifier satisfies the expected graph context;
- provenance predicate/digest/verification state is not bound where claimed;
- replay or duplicate graph roles create a newly accepted identity;
- repaired Semantics-v3 or P4 authority guarantees regress;
- public v2 semantics or immutable alpha.4 anchors change;
- historical negative evidence is deleted, relabeled or bypassed.

## Allowed decisions
- `POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_ELIGIBLE_BOUNDED_CANDIDATE`
- `POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_BLOCKED_REWORK_REQUIRED`
- `POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_DEFER_INCOMPLETE`

No composite score and no partial-pass promotion.

## Success boundary
A positive A3 result means only that the tested P5 standards-composition path is eligible for the next internal candidate-integration gate under these bounded contracts. It does **not** authorize merge to `main`, public release, stable tag movement, endorsement/adoption claims, production-readiness claims, P8 closure, or any claim that cryptographic validity implies semantic truth.
