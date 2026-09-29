# ExecSurface — P3 V3 Explicit Accepted-Variance Contract Protocol

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **V3 PREREGISTERED — RESEARCH ONLY**

## Objective

Design and executable-test a versioned variance contract that keeps four states semantically distinct:

1. invariant behavior;
2. observed variable candidate behavior;
3. explicitly accepted variable behavior;
4. unseen behavior.

The contract MUST NOT convert recurrence, majority, frequency, union membership, filename similarity, or learning-set observation into authorization.

Public `v0.1.0-alpha.4`, baseline v2, normalization profile v3, `check` behavior and policy remain unchanged.

## Team for V3

Fixed roles:
- Innovation Scientist / Systems Architect;
- Anti-Drift / Scientific Integrity Reviewer;
- Independent Falsifier / Red Team.

Dynamic specialists:
- formal data-contract / deterministic serialization;
- provenance and attestation;
- reproducibility / set-stability analysis;
- canonicalization and baseline compatibility;
- CI/supply-chain poisoning attack modeling;
- privacy / metadata minimization.

## Inputs

V3 consumes only an accepted V1 `VarianceReport` plus an explicit acceptance selection.

The V1 report already binds:
- learning-set digest;
- exact source evidence digests;
- baseline digests;
- comparable tool/command/platform/observer/schema/normalization profile;
- invariant effects;
- union effects;
- per-effect support count and source provenance.

V3 must not infer acceptance from those statistics.

## Contract fields

Research contract schema v1 must contain at minimum:
- contract schema version;
- learning-set digest;
- comparable profile;
- invariant-core digest;
- full variable-candidate records with support count + exact source evidence digests;
- explicitly accepted variable subset, each retaining its support/provenance record;
- no policy verdict.

## Explicit acceptance input

The selection must include:
- selection schema version;
- exact learning-set digest it refers to;
- exact canonical effects selected for acceptance.

Acceptance is by exact canonical effect identity only in V3. Similarity, wildcarding and approximate matching are prohibited.

## Fail-closed validation

Contract construction MUST reject:
- unsupported selection schema;
- learning-set digest mismatch;
- duplicate selected effects;
- selection of an invariant effect as variance;
- selection of an unseen effect;
- selection of an effect not classified `VariableCandidate` by V1.

An empty accepted subset is valid and means that observed variability remains unaccepted.

## Determinism requirement

For the same V1 report and same explicit accepted set, selection input order must not affect serialized contract bytes.

Variable candidates and accepted-variable records must be deterministically ordered.

## Required falsification fixtures

At minimum:
1. effect observed in 1/N learning runs is NOT accepted with empty selection;
2. effect observed in N-1/N runs is NOT accepted with empty selection;
3. exact explicit candidate selection is accepted and carries original source provenance;
4. invariant effect selection is rejected;
5. unseen effect selection is rejected;
6. duplicate selection is rejected;
7. learning-set digest mismatch is rejected;
8. acceptance-order permutation yields byte-identical output;
9. candidate support frequency cannot change acceptance without changing explicit selection.

## Allowed outcomes

- `P3_V3_ACCEPTED_VARIANCE_CONTRACT_PASS_RESEARCH_ONLY`
- `P3_V3_CONTRACT_AUTHORIZATION_LEAK`
- `P3_V3_CONTRACT_INCOMPLETE`

A PASS does not authorize runtime integration. V4 poisoning/falsification remains mandatory before real-workload requalification.
