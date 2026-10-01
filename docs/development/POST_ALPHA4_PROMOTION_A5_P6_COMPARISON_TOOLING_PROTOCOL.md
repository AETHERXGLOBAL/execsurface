# ExecSurface — Promotion A5 P6 Comparison/Falsification Tooling Eligibility Protocol

Date: 2026-10-01
Tracking: #115
Candidate branch: `integration/post-alpha4-promotion-candidate`
Status: **PREREGISTERED — P6 VALIDATION/TOOLING ELIGIBILITY ONLY / NO RUNTIME OR PUBLIC PROMOTION AUTHORIZED**

## Purpose

Evaluate whether the bounded P6 competitive-falsification/comparison machinery may be retained as internal validation infrastructure without allowing comparison facts to become semantic authority, product-correctness evidence, external validation, adoption/endorsement claims, a product ranking, or a composite winner score.

A5 does not reopen P6, rerank products, add competitors, or change the frozen P6 workloads. It attacks the *promotion boundary* around P6 tooling.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — seek the smallest reusable validation layer that improves falsification without contaminating product semantics.
2. **Anti-Drift / Scientific Integrity Reviewer** — freezes this corpus before execution and blocks ranking, marketing inference, post-result rule changes, and deletion of negative evidence.
3. **Independent Falsifier / Red Team** — attacks authority laundering, verdict normalization, unsupported-to-absence conversion, external-validation inflation, ranking, and reproducibility.
4. **Independent Critical-Milestone Reviewer** — independently verifies source identity, frozen P6 blobs, public anchors, negative evidence locators, and final wording.

## Dynamic specialists

- experimental methodology / reproducibility;
- PL / evidence semantics;
- CI/CD runtime observability;
- adversarial benchmarking and metric design;
- attestation/provenance;
- privacy / data-minimization;
- technical-claims review.

## Frozen source boundary

A5 consumes, but must not rewrite, the accepted P6 scientific record:

- `docs/development/P6_COMPETITIVE_FALSIFICATION_PROTOCOL.md`;
- `docs/development/P6_CLOSEOUT_DECISION.md`;
- `experiments/p6-competitive-falsification/p6-a0-manifest.json`;
- `experiments/p6-competitive-falsification/p6-a1-observation-matrix.json`;
- `experiments/p6-competitive-falsification/p6-a2-adversarial-matrix.json`;
- `experiments/p6-competitive-falsification/workload.sh`.

Frozen identities at preregistration:

- A0 manifest blob: `625f5af914f33c9408e28933c031ef183b56f8ae`;
- A1 matrix blob: `6179c4f319be7c7a83d36d20e035abb13f3d792c`;
- A2 matrix blob: `49e67fe84e61b257ee71230206215f1aa7dd87ab`;
- workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`;
- P6 closeout blob: `f87a8309cb7131643bcafe57ef3753d89eada693`;
- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.

Any scientific change to those P6 inputs invalidates this A5 gate and requires a new preregistration.

## Acceptance rule

A5 may be eligible only if all of the following hold in one frozen candidate lineage:

1. P6 factual matrices remain proposition/scenario scoped and retain `aggregate_score_forbidden=true` with no overall winner;
2. foreign/native verdict labels cannot be normalized into ExecSurface PASS without a separately proven semantic mapping;
3. `NOT_TESTABLE`, `NOT_APPLICABLE`, `INCOMPLETE`, and unsupported states cannot be converted into absence, failure, or PASS;
4. comparison-derived facts can never raise semantic authority or replace product correctness evidence;
5. internal P6 results cannot satisfy P8 external-validation requirements;
6. praise, comparator success, repository inclusion, or factual overlap cannot manufacture adoption, endorsement, or production-readiness claims;
7. all preregistered P6 negative evidence remains retained, including the A1 harness stop, alpha.4 S2 incompleteness, Tetragon `NOT_TESTABLE_YET`, unsigned copied-predicate mutability, and the no-performance-comparison decision;
8. exact comparator/source/workload identities remain frozen;
9. normalized factual representation remains deterministic under record ordering;
10. repaired Semantics-v3 and P5 authority/binding reproofs remain green;
11. public alpha.4 and stable `v0.1` remain pinned to the immutable public source;
12. no runtime product crate, public tag, release, or P8 state is modified by A5.

There is no partial-pass promotion and no composite score.

## Exactly 12 frozen A5 attacks/invariants

The A5 gate MUST execute exactly these twelve named tests before any A5 eligibility decision:

- `test_a5_01_aggregate_score_and_overall_winner_remain_forbidden`
- `test_a5_02_foreign_native_verdict_cannot_become_execsurface_pass`
- `test_a5_03_not_testable_and_incomplete_are_not_collapsed`
- `test_a5_04_comparison_fact_cannot_raise_semantic_authority`
- `test_a5_05_comparison_cannot_substitute_for_product_correctness`
- `test_a5_06_internal_p6_cannot_satisfy_p8_external_validation`
- `test_a5_07_praise_overlap_or_listing_cannot_manufacture_adoption_or_endorsement`
- `test_a5_08_negative_evidence_locators_remain_retained`
- `test_a5_09_invalid_performance_ranking_remains_blocked`
- `test_a5_10_frozen_comparator_and_workload_identities_match`
- `test_a5_11_factual_normalization_is_deterministic_under_record_reordering`
- `test_a5_12_competitor_name_or_success_cannot_confer_authority`

Test names, count, assertions, and acceptance rule are frozen before the first A5 workflow execution. Only harness/infrastructure repairs that leave all twelve scientific assertions unchanged are permitted; every failed run remains retained.

## Kill criteria

Any reproducible path below blocks A5:

- a comparison record raises semantic authority;
- a comparator/native success is treated as ExecSurface PASS;
- `NOT_TESTABLE`, unsupported, ambiguous, lost, or incomplete becomes absence/PASS;
- competitor evidence substitutes for ExecSurface correctness;
- internal P6 evidence is counted as P8 external evidence;
- a score, tier, global winner, superiority/inferiority verdict, or performance ranking is produced from the bounded matrix;
- any retained negative P6 evidence disappears or is relabeled as positive;
- comparator identity or product name is treated as authority;
- factual normalization depends on row ordering;
- public v2 semantics/tags are reinterpreted or moved.

## Historical evidence boundary

A5 preserves the original P6 decision `P6_FACTUAL_COMPARISON_COMPLETE_BOUNDED` and all its limits. A5 cannot upgrade P6 into a product feature or external validation. Historical P6 failures and coverage gaps remain evidence, not defects to clean up.

## Allowed decisions

- `POST_ALPHA4_PROMOTION_A5_P6_TOOLING_ELIGIBLE_BOUNDED_INTERNAL`
- `POST_ALPHA4_PROMOTION_A5_P6_TOOLING_BLOCKED_MATERIAL_GAP`
- `POST_ALPHA4_PROMOTION_A5_P6_TOOLING_INCOMPLETE`

No A5 result authorizes `main` merge, public release, tag movement, runtime-feature promotion, product superiority claims, adoption/endorsement claims, production-readiness claims, or P8 closure.
