# ExecSurface — Post-alpha.4 Promotion A5 P6 Validation-Tooling Protocol

Date: 2026-10-01
Tracking: #115
Parent program: #100
Candidate branch: `integration/post-alpha4-promotion-candidate`
Candidate floor: `3bb0749f08046cbe26de0d9117defc6c93466fa8`
Public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Status: **PREREGISTERED — VALIDATION/TOOLING ONLY / NO RUNTIME FEATURE OR PUBLIC PROMOTION AUTHORIZED**

## Objective

Determine whether the bounded P6 competitive-falsification/comparison infrastructure is eligible to continue as post-alpha.4 internal validation tooling without becoming a ranking engine, marketing verdict, semantic-authority source, release gate substitute, or external-validation substitute.

A5 evaluates the comparison method and evidence-governance contract, not whether ExecSurface is "better" than another product.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — identify the smallest reusable comparison representation that preserves proposition-level facts without collapsing evidence contracts.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks ranking, composite scores, cherry-picking, missing-data laundering, post-result metric changes, and superiority language.
3. **Independent Falsifier / Red Team** — attacks source pinning, proposition scope, incomplete evidence, negative-result retention, ordering, authority inflation, and P8/release claim laundering.
4. **Independent Critical-Milestone Reviewer** — independently verifies exact pins, negative evidence, deterministic output, public anchors, and final wording.

Dynamic specialists:
- experimental design / benchmarking;
- comparative systems evaluation;
- formal semantics / proposition authority;
- provenance / evidence integrity;
- statistics / anti-cherry-picking;
- reproducible CI;
- claims and governance review.

## Frozen P6 boundary

A5 MUST preserve the original P6 closeout:

`P6_FACTUAL_COMPARISON_COMPLETE_BOUNDED`

and its explicit nonclaims:
- no overall winner;
- no product superiority/inferiority conclusion;
- no full backend equivalence;
- no performance ranking;
- no silent inference from missing evidence;
- no public release readiness claim.

The retained P6 negative evidence is mandatory input, including:
- run `36770551463` harness stop;
- ExecSurface S2 `ERROR/INCOMPLETE`;
- Tetragon live `NOT_TESTABLE_YET`;
- cicd-sensor unsigned copied-predicate mutability;
- natural run/DNS/artifact variation;
- `NO_VALID_PERFORMANCE_COMPARISON`.

## Frozen source pins

The validation model must retain these exact historical comparison-source identities:

- Harden-Runner: `e14015d583714f6e62063499dc959a02595150a1`
- cicd-sensor: `1f031a106e23edda1eb496b0bae51fb12e85d62d`
- Tetragon: `666efe6f91e3605ad58683ad226d759d9cf970ca`
- Falco: `e12b1d43e47a2903c07e14479e034d74d523ab9d`
- Tracee: `2f9dc40c20b17c2ba27f6d92b25e62790bd48a62`

A changed/floating comparator identity is `STALE_SOURCE`, not silently current evidence.

## Allowed proposition classifications

Only proposition/scenario-local classes are admitted:

- `OBSERVED_SUPPORTED`
- `OBSERVED_PARTIAL`
- `NOT_OBSERVED`
- `NOT_APPLICABLE`
- `NOT_TESTABLE`
- `AMBIGUOUS`
- `INCOMPLETE`
- `LOST`
- `FIXTURE_OR_INFRA_FAILURE`

These classifications never map to a numeric product score.

## Exactly 12 frozen A5 attacks/invariants

A5-T01. Product/comparator name cannot confer ExecSurface semantic authority.

A5-T02. A comparison record cannot satisfy a Semantics-v3 proof/admission requirement by itself.

A5-T03. Missing, unsupported, incomplete or not-testable evidence remains explicit and cannot be normalized into zero, absence, PASS, or observed support.

A5-T04. No API or derived output may produce a composite winner score, global ranking, tier, top choice, stronger/weaker/better/worse verdict, or product-wide winner.

A5-T05. A partial matrix or omitted proposition cannot yield a global winner or superiority conclusion.

A5-T06. Unit, proposition, measurement-dimension, observer-profile or semantic-contract mismatch must remain incomparable rather than coerced into an ordered comparison.

A5-T07. Comparator source/version/commit pin mismatch must be explicit `STALE_SOURCE`; floating/current-looking labels cannot overwrite the frozen identity.

A5-T08. Retained negative rows/findings cannot be deleted, filtered or suppressed to improve the narrative; closeout requires their presence.

A5-T09. Input row order and product order permutations must produce deterministic factual output with identical retained facts.

A5-T10. Internal P6/A5 evidence cannot claim or imply P8 external validation, adoption, endorsement, independent reproduction, or production readiness.

A5-T11. Proposition/scope overlap cannot be promoted into full semantic equivalence or backend interchangeability.

A5-T12. Successful comparison tooling cannot authorize public release, `main` merge, stable `@v0.1` movement, default Semantics v3, or any product correctness claim.

No attack, expected result, classification, pin, acceptance rule, or boundary may be weakened after execution.

## Cross-layer reproof

A5 cannot close on comparison-tooling tests alone. The gate must also verify:

- immutable `v0.1.0-alpha.4` and stable `v0.1` source identity;
- repaired A1 Semantics-v3 proof-admission corpus;
- A2 proposition-authority / backend-name anti-inflation boundary;
- A3 attestation/provenance authority boundary;
- A4 bounded variance safety boundary;
- P6 historical closeout still contains `NO_VALID_PERFORMANCE_COMPARISON` and the retained negative evidence.

## Kill criteria

Any reproducible path that:
- creates a product-wide score/ranking/verdict;
- converts incomplete/not-testable/missing evidence into favorable evidence;
- grants semantic authority from product identity or comparison outcome;
- suppresses required negative evidence;
- converts overlap into equivalence;
- treats internal evidence as P8 external evidence;
- authorizes release/product correctness from comparison success;

is a **material gap** and blocks A5.

## Allowed outcomes

### `POST_ALPHA4_PROMOTION_A5_P6_VALIDATION_TOOLING_ELIGIBLE_BOUNDED_CANDIDATE`
Allowed only if all 12 frozen attacks pass, historical negative evidence remains retained, cross-layer reproof passes, and the output remains factual/proposition-local with no winner score.

### `POST_ALPHA4_PROMOTION_A5_P6_DEFER`
Use if the tooling remains safe but current reproducibility or usefulness is insufficient.

### `POST_ALPHA4_PROMOTION_A5_P6_BLOCKED_MATERIAL_GAP`
Use if any ranking, authority, missing-data, evidence-retention, equivalence, P8, or release-laundering counterexample survives.

## Non-authorization

A5 does not authorize:
- runtime/public feature promotion;
- merge to `main`;
- public release or tag movement;
- a marketing comparison;
- a best/worst product judgment;
- external validation/adoption/endorsement claims;
- P8 closure;
- arm64 support.

## Anti-drift rule

No composite score, no winner, no partial-pass promotion, and no post-result vocabulary changes. A single material false inference blocks A5.
