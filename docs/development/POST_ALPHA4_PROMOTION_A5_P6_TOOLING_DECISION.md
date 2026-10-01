# ExecSurface — Promotion A5 P6 Validation-Tooling Decision

Date: 2026-10-01
Tracking: #115
Protocol: `POST_ALPHA4_PROMOTION_A5_P6_TOOLING_PROTOCOL.md`
Candidate branch: `integration/post-alpha4-promotion-candidate`
Accepted candidate source: `5e1ed9ae3640bb6ffa3f33cc2be735041399022f`

## Decision

**`POST_ALPHA4_PROMOTION_A5_P6_VALIDATION_TOOLING_ELIGIBLE_BOUNDED_CANDIDATE`**

The bounded P6 competitive-falsification/comparison infrastructure is eligible to continue as internal post-alpha.4 validation tooling only.

It is not a runtime feature, ranking engine, release gate, semantic-authority source, marketing comparison, or substitute for P8 external validation.

## Accepted execution evidence

Final workflow run:
- run: `36864447147` — **SUCCESS**;
- source: `5e1ed9ae3640bb6ffa3f33cc2be735041399022f`;
- closeout artifact: `11163263388`;
- closeout digest: `sha256:547f48bb3d43805003afc24bffff727b3b0a38083ba81ccf36219dcc9c5011c8`;
- frozen A5 attack artifact: `11163336806`;
- attack digest: `sha256:ef5c9b6b1a43c4fe0f69ce6632f328dc1f509bf8cede52d07ef8e76cd2b7d0cd`.

Accepted checks:
- exact preregistered A5 corpus: **12/12 PASS**;
- evaluator static Python gate: PASS;
- explicit no-ranking/no-release-positive output check: PASS;
- original P6 non-ranking boundaries retained: PASS;
- retained P6 historical harness failure `36770551463` independently rechecked as failure: PASS;
- A1 repaired Semantics-v3 reproof: PASS;
- A2 proposition-authority / cross-proposition reproof: PASS;
- A3 attestation-authority reproof: PASS;
- A4 bounded GCC variance reproof: PASS;
- P3 poisoning falsifier replay: PASS;
- immutable alpha.4/stable `v0.1` anchors: PASS;
- closeout: PASS.

## What A5 established

Within the tested bounded scope:

1. product/comparator identity cannot manufacture ExecSurface semantic authority;
2. a comparison record cannot become Semantics-v3 proof admission by itself;
3. `INCOMPLETE`, `NOT_TESTABLE`, missing and unsupported states remain explicit and are not normalized into PASS/absence/zero;
4. score/rank/tier/winner inputs are rejected rather than aggregated;
5. a partial matrix fails closed for a requested complete comparison scope;
6. proposition, dimension, semantic-contract and source-identity mismatches remain incomparable;
7. source-pin mismatch is explicit `STALE_SOURCE`;
8. mandatory negative evidence cannot be suppressed from closeout;
9. factual matrix output is deterministic under tested row/product ordering permutations;
10. internal evidence cannot claim P8 external validation, adoption, endorsement, independent reproduction or production readiness;
11. overlapping comparison axes do not establish backend equivalence;
12. successful comparison tooling cannot authorize release, `main` merge, stable-tag movement or default Semantics v3.

## Retained failed execution

The first A5 workflow run remains retained:
- run `36864270080` at source `f543f6c62c9939caa321cc5eccc2e8a4c4470b00`;
- A5 12-test scientific corpus itself passed;
- the run failed in a governance grep because the original P6 sentence used Markdown emphasis around `does not`, while the harness searched an unformatted literal string;
- correction `5e1ed9ae3640bb6ffa3f33cc2be735041399022f` changed only that grep to a Markdown-safe invariant substring;
- no A5 test, expectation, frozen source pin, classification, negative-evidence requirement, comparison rule, semantic boundary or threshold changed.

This failure remains historical evidence and is not relabeled as success.

## Retained P6 negative evidence

A5 explicitly requires and preserves:
- first P6-A1 harness stop `36770551463`;
- ExecSurface alpha.4 S2 `ERROR/INCOMPLETE`;
- Tetragon live `NOT_TESTABLE_YET`;
- cicd-sensor unsigned copied-predicate mutability observation;
- natural DNS/run/artifact variation across P6-A3;
- `NO_VALID_PERFORMANCE_COMPARISON` and the prohibition on product speed ranking.

## Frozen promotion boundary

A5 eligibility is limited to internal validation/falsification tooling.

The tooling may:
- retain pinned proposition-level facts;
- expose explicit source staleness/incomparability;
- preserve negative evidence;
- support reproducible falsification and factual comparison.

The tooling may not:
- score or rank products;
- state a best/worst or overall winner;
- infer superiority/inferiority;
- convert comparator identity or favorable results into semantic authority;
- convert missing/incomplete/not-testable evidence into favorable evidence;
- infer backend equivalence from overlap;
- claim external validation/adoption/endorsement;
- authorize a public release or product correctness.

## Public state

Unchanged:
- public release: `v0.1.0-alpha.4`;
- public source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`;
- public semantics remain v2;
- P8 remains externally pending;
- P7 arm64 negative result remains retained and non-promoted.

## Next gate

Proceed to a separately preregistered assessment of **P7-other** platform/CI research. The retained P7 arm64 negative result is excluded from positive promotion and remains `NEGATIVE_RETAINED` unless a fresh independent gate produces new evidence.
