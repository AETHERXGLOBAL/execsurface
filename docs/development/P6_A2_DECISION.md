# ExecSurface — P6-A2 Adversarial Decision

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Protocol: `docs/development/P6_A2_ADVERSARIAL_PROTOCOL.md`
Evidence: `docs/development/P6_A2_EVIDENCE_EXTRACTION_001.md`
Machine matrix: `experiments/p6-competitive-falsification/p6-a2-adversarial-matrix.json`

## Decision

**P6_A2_NO_FALSE_PASS_IN_FROZEN_SCOPE**

This is a bounded adversarial result only. It does not claim universal false-PASS absence, overall product superiority, competitor weakness, or release readiness.

## Accepted evidence

- source: `411116ee87bbe45270b1fd8ce84b17abcecd8642`
- run: `36772303119`
- job: `110081460858`
- conclusion: `success`
- artifact: `11124710931`
- artifact digest: `sha256:aa2205eb719baf4a9168ee9694bc6da0cfc5d94fab2230db181ab3fe0a3ba8b8`

Frozen ExecSurface public target:
`v0.1.0-alpha.4` / `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

Frozen workload blob:
`c5986f1acd8bc79a3945e613acc128d5c8a9d168`

## ExecSurface adversarial results

No E1–E5 attack produced native PASS:
- process expansion -> REVIEW;
- file-write expansion -> REVIEW;
- network expansion -> ERROR/INCOMPLETE, fail closed;
- semantic baseline payload tamper -> baseline digest mismatch ERROR;
- stored baseline digest substitution -> baseline digest mismatch ERROR.

The E6 command-contract probe independently showed that a top-level argument-count mismatch is rejected as not comparable (`baseline=2`, `candidate=3`). That result is recorded as the actual public v2 command compatibility behavior, not as a broader workflow/source-identity claim.

Gate counts:
- false PASS: `0`
- infrastructure markers: `0`

## P5 cross-attestation regression guard

The exact accepted P5-A5 test blob `6a64ad84d93a23c35d53e737c90508dc528d9cc1` was re-run unchanged:

**12/12 PASS**

This preserves the accepted baseline/current-surface/evidence/source/workflow substitution, incompleteness, backend-name inflation, semantic duplication and digest-domain defenses.

## cicd-sensor bounded facts

The exact A1 predicate artifact was retrieved and its GitHub artifact digest independently verified.

A2 established only:
1. unchanged replay retains its original GitHub run ID, so an equality-checking consumer can detect expected-run mismatch;
2. copied unsigned predicate JSON can be edited structurally, therefore external integrity/signature verification is required if tamper resistance is desired;
3. the native `passed` field remains a native cicd-sensor result and was not normalized to ExecSurface `PASS`.

No vulnerability label is assigned.

## Explicit non-results

- Harden-Runner offline attestation replay: `NOT_APPLICABLE_TO_OFFLINE_ATTESTATION_REPLAY` under the demonstrated A1 contract.
- Tetragon live A2: `NOT_TESTABLE_YET` until an immutable executable identity is frozen.
- no performance comparison;
- no composite score;
- no public integration/promotion.

## Historical evidence retained

This decision does not replace:
- P5-A5 historical 8/12 counterexample run `36755962689`;
- P6-A1 first-run harness stop `36770551463`;
- P6-A1 real ExecSurface S2 incompleteness evidence.

## Next gate

P6-A3 — reproducibility / privacy / CI-friction matrix.

A3 must repeat accepted same-workload observations independently and report stability separately from setup/privilege/data-flow friction. It must not convert operational friction into a product score.