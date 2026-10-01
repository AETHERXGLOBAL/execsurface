# ExecSurface — A8-A4 Attestation / Provenance Binding Protocol

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable public base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Accepted A8-A1 source: `1c65327e99aa62a3b42549764bebe6b5b29516e7`
Accepted A8-A2 source: `4166ff64b8110e27b47954c1f0875cce81ebf898`
Accepted A8-A3 source: `6c35861f66cc238374dc113d9431859bf0340bb9`
P5 promotion evidence source: `0f6cbc68bea380ff25889e762cb8cd5287a9c5a3`
P5 accepted run: `36858724331`
Status: **PREREGISTERED — NO A8-A4 PRODUCT IMPLEMENTATION ACCEPTED YET**

## Question

Can the clean candidate bind verification results to exact subject/source/baseline/current-surface/evidence/verifier/provenance identities using existing attestation standards while keeping cryptographic/provenance validity strictly separate from semantic authority?

## Hypothesis

A small typed binding layer over the already accepted A8-A2 `AuthorityRecord` can make substitution/replay failures explicit without introducing a custom attestation standard and without allowing signatures, signer/workflow names, or `verified=true` provenance metadata to manufacture PASS eligibility.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — minimize the product binding contract and eliminate implicit/duplicated identity sources; reuse existing standards rather than invent a custom predicate.
2. **Anti-Drift / Scientific Integrity Reviewer** — block semantic-authority inflation from cryptography/provenance labels, block hidden defaults, block P8 self-certification, and preserve all prior negative evidence.
3. **Independent Falsifier / Red Team** — attack authority laundering, observer loss, cross-subject replay, predicate substitution, malformed digests, source/baseline/current/verifier substitution, verification-state substitution, duplicate graph roles, and unbound context.
4. **Independent Critical-Milestone Reviewer** — verify clean ancestry, frozen A1/A2/A3 slices, exact product delta, fail-closed behavior, deterministic serialization, and release/P8 boundaries.

Dynamic specialists: supply-chain security/SLSA/in-toto/Sigstore, formal semantics, Rust API/serde, CI/reproducibility.

## Product delta freeze

A8-A4 may add or modify only:
- `crates/execsurface-model/src/attestation_v3.rs` — typed binding contract behind existing `semantics-v3` feature;
- `crates/execsurface-model/src/lib.rs` — feature-gated export only;
- `crates/execsurface-model/tests/attestation_v3_candidate.rs` — frozen adversarial corpus;
- `.github/workflows/post-alpha4-candidate-a8-a4-attestation.yml` — A8-A4 gate;
- `docs/development/POST_ALPHA4_CANDIDATE_ASSEMBLY_MANIFEST.md` — cumulative ledger.

P5 experiment crates/workflows are evidence-only and MUST NOT be copied wholesale into the product candidate.

## Existing-standard boundary

The product may reference existing predicate identifiers only. In particular, SLSA provenance v1 is represented by the exact identifier:

`https://slsa.dev/provenance/v1`

No new ExecSurface custom attestation predicate/schema is claimed or required by A8-A4.

## Candidate contract

The bounded contract may contain:
- `VerificationVerdict`: `Pass`, `Review`, `Block`, `Error`;
- `ObserverHealth`: `Healthy`, `Lost`;
- `DigestBinding`: strict `sha256:<64 hex>` identity;
- `ProvenanceBinding`: predicate type, statement digest, subject digest, verification state;
- `VerificationBindings`: subject/source/baseline/current-surface/evidence/verifier digests plus optional provenance and informational workflow/signer labels;
- `AttestedVerification`: A8-A2 `AuthorityRecord` + observer health + verdict + bindings;
- exact expected-context matching;
- deterministic role manifest with duplicate-role rejection.

## Mandatory semantic rules

- `Pass` requires observer health `Healthy`;
- `Pass` requires `AuthorityRecord::admissible_for()` under the exact proposition-bound `ProofRequirement`;
- `Review`, `Block`, and `Error` never become PASS merely because provenance/signature metadata is valid;
- provenance `verified=true` does not raise semantic authority;
- provenance `verified=false` does not by itself downgrade a semantically admissible observation; provenance verification is reported as a separate dimension;
- if provenance is present, predicate type must equal SLSA provenance v1 exactly;
- provenance subject digest must equal bound subject digest exactly;
- all digest bindings must be strict SHA-256 labels;
- subject/source/baseline/current/evidence/verifier are explicit and mandatory for this bounded candidate;
- expected-context verification must compare every binding exactly;
- duplicate or substituted manifest roles fail closed;
- informational workflow/signer labels are never inputs to semantic PASS eligibility;
- no P8 external-validation state is derivable from this contract.

## Frozen 12-attack corpus

1. verified provenance does not inflate ambiguous authority from REVIEW;
2. verified provenance cannot launder ambiguous authority to PASS;
3. verified provenance cannot launder observer loss to PASS;
4. cross-subject provenance replay is rejected;
5. provenance predicate-type substitution is rejected;
6. malformed provenance statement digest is rejected;
7. source substitution fails exact expected-context binding;
8. baseline substitution fails exact expected-context binding;
9. current-surface substitution fails exact expected-context binding;
10. verifier substitution fails exact expected-context binding;
11. provenance verification state is bound but does not manufacture semantic authority;
12. duplicate or substituted provenance manifest role fails closed.

Additional mandatory product checks:
- malformed mandatory subject/source/baseline/current/evidence/verifier digests fail validation;
- workflow/signer label changes cannot change `semantic_pass_admissible()`;
- serialization/role-manifest ordering is deterministic.

No test, assertion, digest rule, role, or semantic acceptance rule may be weakened after execution.

## Required regressions

The A8-A4 gate must also prove:
- A8-A1 regression guard PASS / D01-D06 6/6 / A1 9/9;
- A8-A2 authority guard PASS / 16/16;
- A8-A3 GCC guard PASS / 12/12;
- default workspace regression PASS;
- rustfmt PASS;
- Clippy `-D warnings` PASS default and relevant feature modes;
- public raw schema remains v2;
- public normalization profile remains 3;
- `semantics-v3` and `gcc-ephemeral-v4` remain default-off;
- public alpha.4 and stable `v0.1` remain immutable.

## Kill criteria

Block A8-A4 if signatures/provenance/workflow/signer metadata can manufacture semantic PASS, ambiguous/lost/unsupported authority can PASS, observer loss can PASS, any identity substitution survives exact-context verification, duplicate graph roles are accepted, malformed digests are accepted, or A1/A2/A3/default behavior regresses.

## Allowed decision

Only one complete successful gate may produce:

`POST_ALPHA4_PROMOTION_A8_A4_ATTESTATION_BINDING_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`

This does not authorize `main`, tag movement, release publication, a new custom attestation standard, arm64 support, backend equivalence, or P8 closure.
