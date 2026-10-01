# ExecSurface — Post-alpha.4 Destructive Falsification Review Protocol

Date: 2026-10-01
Tracking: #116
Promotion gate: #115
External validation gate: #114

Status: **PREREGISTERED — INTERNAL INDEPENDENT-STYLE REVIEW — NOT EXTERNAL EVIDENCE**

## 1. Frozen source under attack

- promotion candidate source: `5079a990b924d8ccd7ac6414f8a9a2571b54e240`
- review branch: `review/post-alpha4-destructive-falsification`
- immutable public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`

No test result from this review may be counted as P8 external validation.

## 2. Review team

Fixed roles:
1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team
4. Independent Critical-Milestone Reviewer

Dynamic specialists: PL/formal semantics, Linux ptrace/shared-FD, filesystem object identity/TOCTOU, network syscall semantics, Rust API/concurrency, in-toto/SLSA/Sigstore, reproducibility/CI, privacy, release/migration/rollback.

## 3. First-wave hypotheses frozen before execution

### D01 — proposition binding

**Hypothesis to kill:** a generic proof requirement can be satisfied by a record for a different proposition/subject merely because guarantee and completeness sets match.

Design basis: Semantics v3 states that a proposition kind declares its minimum guarantees/completeness dependencies and that a baseline binds accepted behavior together with its minimum proof contract.

**Attack:** construct two distinct pathname propositions with identical guarantees/completeness, define a requirement intended for the first, and attempt to satisfy it with the second.

**Required safe result:** the wrong proposition must be non-admissible.

### D02 — invalidating ambiguity consistency

**Hypothesis to kill:** a record can claim a required completeness dimension as `complete` while also carrying an invalidating ambiguity annotation and still satisfy the proof requirement.

Design basis: Semantics v3 requires every guarantee constraint and no invalidating ambiguity/completeness state.

**Attack:** mark `object_identity = complete`, inject an explicit `object_identity_conflict` ambiguity code, and require object-identity completeness.

**Required safe result:** the contradictory record must be non-admissible.

### D03 — underspecified requirement bypass

**Hypothesis to kill:** an empty/default `ProofRequirement` can make an arbitrary v3 record admissible even though proposition kinds are supposed to declare mandatory proof requirements.

**Attack:** evaluate a normal record against `ProofRequirement::default()`.

**Required safe result:** an empty requirement cannot constitute sufficient proof for semantic acceptance/PASS eligibility.

## 4. Replay corpus

Regardless of D01–D03 outcome, execute independent existing gates where available:

- existing post-alpha.4 Semantics-v3 promotion attacks;
- P4 cross-proposition authority attacks;
- P5 A5 cross-attestation substitution/replay attacks;
- M11 shared-FD ambiguity regression;
- P3 poisoning/falsification where executable without changing the frozen corpus;
- public alpha.4 / v2 compatibility and immutable-boundary checks where already encoded.

Failures in replay are retained exactly. No failed test is removed to obtain a clean close.

## 5. Material-gap rule

A single reproducible false-admission path in D01–D03 is sufficient to classify the affected Semantics-v3 promotion item as **BLOCKED / REWORK REQUIRED** until the smallest correction is implemented and the exact frozen test is rerun unchanged.

No threshold adjustment, test deletion, weakened assertion, broad exception, or post-hoc redefinition is permitted.

Harness/format-only defects may receive only the smallest correction while retaining the failed run.

## 6. Broader frozen attack families

After first-wave semantic attacks, continue through:

1. v2↔v3 migration/proof downgrade;
2. shared-FD / fd reuse / clone relation;
3. PATH-TOCTOU/object authority;
4. rename/delete success/replay/ABI;
5. connect success/EINPROGRESS/failure/malformed sockaddr/FD reuse;
6. P3 variance poisoning/frequency authorization/collision;
7. P4 external-trace/backend-name/session-completeness laundering;
8. P5 source/baseline/current/verifier substitution and duplicate semantic properties;
9. privacy/loss/truncation/fail-closed regressions;
10. P7 negative arm64 evidence remaining non-promoted;
11. deterministic serialization and evidence retention.

## 7. Final classifications

Only one:

- `POST_ALPHA4_DESTRUCTIVE_REVIEW_SURVIVES_BOUNDED_INTERNAL`
- `POST_ALPHA4_DESTRUCTIVE_REVIEW_MATERIAL_GAP_FOUND`
- `POST_ALPHA4_DESTRUCTIVE_REVIEW_INCOMPLETE`

No result implies external validation, adoption, endorsement, production readiness, or P8 closure.