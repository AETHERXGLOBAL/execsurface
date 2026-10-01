# ExecSurface — Post-alpha.4 Promotion A1 Requalification Protocol

Date: 2026-10-01
Tracking: #115
Status: PREREGISTERED — CANDIDATE-BRANCH REQUALIFICATION / NO PUBLIC PROMOTION

## Target

- candidate branch: `integration/post-alpha4-promotion-candidate`
- repaired ancestry floor: `0200f09557906118dd0e96f8a5a73aa4f5c4a9cc`
- historical broken candidate: `5079a990b924d8ccd7ac6414f8a9a2571b54e240`
- independent repair review: `27f1df750439db9cc4453df7d78cf5b6a38a2ee8`
- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`

The candidate branch was advanced from the historical broken candidate to the repaired source by fast-forward only. Historical counterevidence remains retained.

## Team

Fixed roles:
1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team
4. Independent Critical-Milestone Reviewer

Dynamic specialists:
- PL/formal semantics and schema evolution;
- Rust API/type-system engineering;
- proposition authority and Linux runtime semantics;
- in-toto/SLSA/Sigstore attestation composition;
- compatibility/migration and rollback engineering;
- CI/reproducibility/fault-injection engineering.

## Question

Does the **actual promotion-candidate branch**, after the Semantics v3 repair, preserve the repaired admission contract and all critical predecessor fail-closed guarantees strongly enough to continue to the next internal promotion/integration gate?

## Frozen acceptance rule

Every listed gate must PASS on the candidate branch. There is no composite score and no partial-pass promotion.

### C1 — Candidate and public identity
- branch must be `integration/post-alpha4-promotion-candidate`;
- repaired source `0200f095...` must be an ancestor;
- broken source `5079a990...` remains in history;
- `v0.1.0-alpha.4` and stable `v0.1` must still resolve exactly to `48e0b9a...`;
- v2 raw/canonical/baseline/diff/policy/verdict domains remain v2 and Semantics v3 remains explicit v3.

### C2 — Repaired Semantics v3
- rustfmt clean;
- Clippy `-D warnings` clean;
- D01-D06 repaired destructive corpus PASS;
- original A1 nine-test corpus PASS;
- model/baseline/diff/policy regressions PASS.

### C3 — P3 falsifier
Variance poisoning and false-PASS falsifier corpus must remain PASS. Frequency/similarity never becomes authorization.

### C4 — P4 proposition authority
- cross-proposition falsification PASS;
- backend-name anti-inflation PASS;
- live open-object, rename/delete and connect proposition corpora PASS.

### C5 — P5 attestation/provenance
Using the previously frozen upstream SLSA v1 fixture identity:
- P5 A5 cross-attestation red-team PASS;
- P5 A3 SLSA provenance binding PASS;
- P5 A2/A1/A0 composition PASS.
Signatures, provenance, standards labels or backend names cannot upgrade insufficient semantic authority.

### C6 — M11 fail-closed
Shared-FD ambiguity regression must remain PASS/fail-closed.

### C7 — M12 adversarial replay
On both Ubuntu 22.04 and Ubuntu 24.04:
- bounded PATH-TOCTOU/shared-FD adversarial suite PASS;
- ptrace Linux regression PASS.

## Kill criteria

Any reproducible false admission, authority laundering, proof downgrade, ambiguity/loss bypass, substitution/replay escape, v2 reinterpretation, public-tag movement, or regression in the listed predecessor gates blocks requalification.

Harness failures are retained. Only the smallest correction may be made if it does not change attack meaning, assertions or thresholds.

## Allowed decision

Only if all frozen gates pass:

`POST_ALPHA4_PROMOTION_A1_SEMANTICS_V3_REQUALIFIED_BOUNDED_CANDIDATE`

Otherwise:

`POST_ALPHA4_PROMOTION_A1_REQUALIFICATION_BLOCKED`

## Claim boundary

A successful result authorizes only continuation to the **next internal promotion/integration gate**. It does not authorize:
- merge to `main`;
- public release or stable-tag movement;
- silent v2→v3 migration;
- automatic baseline mutation;
- arm64 portability claims;
- external validation/adoption/endorsement/production-readiness claims;
- P8 closure.
