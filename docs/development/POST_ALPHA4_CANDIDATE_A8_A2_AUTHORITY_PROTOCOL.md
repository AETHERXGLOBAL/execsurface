# ExecSurface — A8-A2 Proposition Authority Candidate Protocol

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable public base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Accepted A8-A1 source: `1c65327e99aa62a3b42549764bebe6b5b29516e7`
P4 bounded research closeout: `320c865d3e81071a8214ad726dca7a587b0fc479`
Promotion A2 evidence source: `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`
Status: **PREREGISTERED — NO A8-A2 PRODUCT IMPLEMENTATION ACCEPTED YET**

## Question

Can the bounded P4 proposition-authority result be expressed as a small, typed product contract on the clean candidate branch without importing research history, without manufacturing authority from backend metadata, and without weakening the repaired Semantics-v3 admission rules?

## Hypothesis

A typed authority record that derives proposition identity exclusively from its `ProofCarryingObservation` can preserve the accepted P4 authority boundaries while removing the research prototype's duplicated textual `proposition_id` source of truth.

This is a candidate hardening change, not a novelty or backend-equivalence claim.

## Fixed review roles

1. **Innovation Scientist / Systems Architect** — minimize the shipped contract and eliminate duplicated proposition identity where that reduces mismatch surface without broadening authority.
2. **Anti-Drift / Scientific Integrity Reviewer** — block research-crate/history import, backend-name authority, unsupported-family promotion, equivalence claims, hidden default changes, and any post-result test weakening.
3. **Independent Falsifier / Red Team** — attack proposition substitution, backend-name laundering, attempt/success confusion, ambiguous/lost/unsupported admission, malformed evidence identity, replay-like derivation omission, and deterministic serialization.
4. **Independent Critical-Milestone Reviewer** — verify exact ancestry, A1 slice stability, admitted paths, retained negatives, workflow evidence, and release/P8 boundaries.

Dynamic specialists: Rust API/semver, formal runtime semantics, Linux proposition authority, reproducibility/CI, and supply-chain evidence identity.

## Product delta freeze

A8-A2 may add or modify only:

- `crates/execsurface-model/src/authority_v3.rs` — typed authority/completeness/evidence-reference contract behind the existing `semantics-v3` feature;
- `crates/execsurface-model/src/lib.rs` — feature-gated export only;
- `crates/execsurface-model/tests/authority_v3_candidate.rs` — frozen A8-A2 adversarial corpus;
- `.github/workflows/post-alpha4-candidate-a8-a2-authority.yml` — A8-A2 gate;
- `docs/development/POST_ALPHA4_CANDIDATE_A8_A2_AUTHORITY_PROTOCOL.md` — this protocol/result ledger;
- `docs/development/POST_ALPHA4_CANDIDATE_ASSEMBLY_MANIFEST.md` — cumulative clean-candidate inventory/status.

A8-A2 MUST NOT copy `experiments/p4-*`, P4 research workflows, ptrace runtime adapters, Tetragon importers, or research documentation into the product candidate.

## Candidate contract

The product contract may contain only:

- `AuthorityState`: `Direct`, `DerivedBounded`, `AttemptOnly`, `Unsupported`, `Ambiguous`, `Lost`;
- `PropositionCompleteness`: `Complete`, `Incomplete`, `NotApplicable`;
- `EvidenceReference`: SHA-256 identity plus optional named derivation;
- `AuthorityRecord`: one Semantics-v3 proof plus authority/completeness/evidence/reason metadata;
- validation/admission and proposition-local comparison helpers.

There is deliberately **no second textual proposition identifier**. The typed `ProofCarryingObservation.proposition` is the sole proposition identity.

## Mandatory fail-closed rules

- evidence digest must be exactly `sha256:` plus 64 hexadecimal characters;
- `Unsupported` requires `NotApplicable`, and `NotApplicable` requires `Unsupported`;
- `Unsupported`, `Ambiguous`, and `Lost` require explicit reason codes;
- `Ambiguous` or `Lost` cannot be `Complete`;
- `AttemptOnly` is valid only for explicit attempt proposition variants;
- `DerivedBounded` requires a non-empty derivation identity;
- backend/profile name never changes authority;
- admission requires a valid record, complete proposition evidence, an admissible authority state, and `ProofCarryingObservation::satisfies()` for the exact requirement;
- different typed propositions are never declared equivalent by comparison;
- incomplete/ambiguous/lost/unsupported evidence never becomes PASS-admissible;
- no backend equivalence, ptrace/Tetragon interchangeability, or global backend score is introduced.

## Frozen attack corpus

A8-A2 must include at least these attacks, with exact assertions frozen before execution:

1. unsupported + complete rejected;
2. non-unsupported + not-applicable rejected;
3. ambiguous + complete rejected;
4. lost + complete rejected;
5. unsupported without reason rejected;
6. ambiguous/lost without reason rejected;
7. attempt-only attached to a non-attempt proposition rejected;
8. derived-bounded without derivation identity rejected;
9. malformed/non-hex SHA-256 evidence identity rejected;
10. backend-name laundering cannot satisfy stronger proof requirements;
11. exact proposition substitution cannot satisfy a bound requirement;
12. unsupported record never becomes admissible;
13. attempt-only can satisfy only its exact attempt proposition requirement;
14. different propositions compare as different, never equivalent;
15. deterministic serialization is stable under reason-code insertion order;
16. Semantics-v3 ambiguity/proof admission still fails closed through the authority wrapper.

No attack may be deleted, renamed to evade a failure, or relaxed after execution.

## Regression requirements

The A8-A2 gate must also reprove:

- accepted A8-A1 Semantics-v3 source/test slice remains byte-identical;
- D01-D06 = 6/6 PASS;
- A1 corpus = 9/9 PASS;
- default-v2 workspace regression PASS;
- rustfmt PASS;
- Clippy `-D warnings` PASS in default and `semantics-v3` modes;
- `semantics-v3` remains default-off;
- `RAW_OBSERVATION_SCHEMA_VERSION` remains 2;
- public `v0.1.0-alpha.4` and stable `v0.1` remain at the immutable alpha.4 source.

## Kill criteria

A8-A2 is blocked if any test shows authority can be raised by a backend/profile name, an unbound/different proposition can satisfy a requirement, attempt evidence satisfies a success proposition, ambiguous/lost/unsupported evidence becomes admissible, evidence identity accepts malformed values, or A1/default-v2 behavior regresses.

## Allowed decision

Only a full successful gate may produce:

`POST_ALPHA4_PROMOTION_A8_A2_AUTHORITY_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`

This decision does not authorize `main`, tag movement, publication, arm64 claims, backend equivalence, or P8 closure.
