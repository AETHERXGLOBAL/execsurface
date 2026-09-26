# M9.2 — Independent Adoption Evidence Protocol

Date: 2026-09-27
Status: **OPEN — protocol and intake frozen before accepted M9.2 evidence**
Base main: `93158e044ff993037ce7da7a78e61fbb1b892d00`

## Objective

M9.2 determines whether independent third parties can adopt the existing public ExecSurface product without AETHER X controlling the workload, execution, or desired outcome.

M9.2 does not add observer capabilities and does not change public backend authority. The public correctness reference remains Linux x86_64 `ptrace`.

The historical `M9_EXTERNAL_ADOPTION_PROTOCOL.md` remains useful for L0/L1/L2/L3 evidence-level semantics, but its original phase numbering is superseded by the current M9 sequence. In this program, **M9.2 is the independent-adoption evidence stage**.

## Fixed team

- Adoption Evidence Lead — audits provenance and keeps independent adoption separate from compatibility evidence.
- OSS / Developer Experience Engineer — keeps the public self-service path short and reproducible without executing it for the evaluator.
- CI / Reproducibility Engineer — validates machine-readable records and immutable artifact references.
- Privacy / Provenance Reviewer — rejects secrets, prohibited payloads, unverifiable attribution, and ambiguous consent.
- Runtime Semantics Engineer — checks PASS/REVIEW/BLOCK meaning, comparability, completeness, and drift evidence.
- Innovation Scientist / Adoption Architect — seeks the smallest high-value path to genuine external use.
- Deviation Prevention / Scientific Integrity — prevents relabeling assisted or self-run evidence as adoption.
- Independent Red Team — tries to falsify independence, provenance, completeness, drift visibility, and closeout claims.

## Authority invariants

M9.2 cannot change these:

- `ptrace` is the public correctness reference and default backend;
- eBPF full-surface comparability is false;
- eBPF `learn`, `check`, and PASS authority are not authorized;
- backend auto-selection is not authorized;
- ptrace/eBPF baselines are not interchangeable;
- research-only eBPF timing results cannot count as adoption authority;
- privacy and fail-closed semantics remain unchanged.

## What counts as `INDEPENDENT_USER`

A record counts only when all are true:

1. a third party independently initiates the evaluation or use;
2. the third party executes the workflow on infrastructure they control or independently chose;
3. AETHER X does not execute commands, edit their workload, tune results, choose a result after seeing outcomes, or materially debug the run for them;
4. the evaluated project/workload is not modified by AETHER X;
5. a public or explicitly consented external reference is available;
6. the third party supplies a non-empty attestation confirming independent initiation and execution;
7. the canonical evidence record passes `scripts/m9_validate_evidence.py` and `scripts/m9_validate_independent_adoption.py`;
8. only metadata permitted by the M9 privacy boundary is retained.

Pointing a user to already-public documentation, the published release, or the public intake form is not material assistance. If AETHER X materially helps execute or repair the workflow, classify it as `COLLABORATIVE_EXTERNAL`; it remains useful evidence but does not count toward the positive M9.2 adoption gate.

## Successful adoption record

For an independent record to count toward a positive adoption close, it must additionally show:

- public ExecSurface installation: `PASS`;
- `execsurface doctor`: `PASS` on the supported Linux x86_64 scope;
- baseline creation: `PASS`;
- unchanged rerun/check: `PASS`;
- comparable evidence: `true`;
- complete evidence: `true`;
- no unresolved operational error;
- no suspected false negative in the accepted record.

A failure submitted by an independent user is still first-class M9.2 evidence, but it does not become a successful adoption case. It must remain preserved with its failure classification.

## M9.2 closure states

M9.2 has two different, explicitly non-equivalent closure states.

### Positive close — `PROVED: INDEPENDENT ADOPTION EVIDENCE`

A positive adoption claim requires all of the following:

1. at least **two** countable `INDEPENDENT_USER` records exist;
2. they come from at least **two distinct external project repositories**;
3. they have at least **two distinct attributable external references / initiators**, confirmed during provenance review;
4. both records meet the successful-adoption requirements above;
5. at least one accepted record includes either a natural drift or a preregistered controlled drift where the drift operation succeeds and the product returns `REVIEW` or `BLOCK` while retaining comparability and completeness;
6. any failures, false-positive reports, usability friction, exclusions, and non-countable collaborative cases remain preserved rather than removed from the ledger;
7. Red Team finds no evidence that either countable record was actually AETHER X initiated, AETHER X executed, materially assisted, or cherry-picked from a larger unreported series;
8. CI and current product semantics remain unchanged;
9. closeout states the exact evidence scope and does not generalize to universal Linux adoption or production readiness.

Two repositories alone do not prove two independent initiators. Final closeout requires human provenance review of the external references/attestations in addition to automated validation.

### Negative close — `KILLED: NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE`

A negative close is permitted only after a **predeclared discovery scope** has been executed and preserved. It means the search stage is complete and M9.3 may synthesize the result. It **must not** be described as adoption, validation, or successful M9.2 adoption.

The negative close requires:

1. the discovery scope and search queries/sources are frozen before interpreting results;
2. every discovered candidate is retained as `DISCOVERY_ONLY`, `CANDIDATE_INDEPENDENT`, `KILLED`, or accepted evidence;
3. no M9.1 zero-contact record is relabeled as independent adoption;
4. no AETHER X-authored external PR, guided execution, or materially assisted run is counted as independent;
5. the closeout explicitly states `0 accepted independent adoption records` if none qualify;
6. absence of evidence is not generalized into a claim that nobody uses or wants ExecSurface.

A negative close is a valid research/product result, but it is **not a positive adoption claim**.

## Drift requirement

Controlled drift must keep the same top-level command/wrapper shape used for the learned baseline. A test that changes the root executable and thereby becomes non-comparable does not satisfy the positive drift gate.

The Technical Evaluation Pack's `/bin/bash -lc` baseline/check shape is the reference self-service path. The evaluator may instead use a natural drift from their own project if it is clearly attributable and privacy-safe.

## Performance

Performance measurement is optional for M9.2. If an independent evaluator includes it, the already-frozen M9 3-warmup / 15-measured-sample rules and M6.5 trigger calculation apply unchanged. No reduced sample plan is accepted as equivalent.

## Privacy

Do not collect or request:

- file contents;
- environment-variable values;
- stdin contents;
- network payloads;
- secrets, tokens, credentials, or private keys;
- unrestricted child argv capture.

Public issue submissions must not paste secret-bearing logs. If a submission contains prohibited sensitive content, it cannot be accepted as-is; retain only consented, privacy-safe metadata needed for the claim.

## Failure preservation

Independent failures are valuable evidence. Preserve installation failures, unsupported-host outcomes, observer errors, timeouts, truncation, non-comparability, suspected false negatives, false-positive reports, and user friction with their original context.

No failure may be silently dropped merely because it does not help close M9.2.

## Intake and canonicalization

The public intake surface is `.github/ISSUE_TEMPLATE/m9-independent-adoption.yml` and the self-service procedure is `docs/milestones/M9_2_EXTERNAL_QUICKSTART.md`.

An intake issue is source evidence, not automatically a canonical M9 record. After provenance and privacy review, an accepted submission is represented as an immutable `m9-evidence-v1` JSON record with its public/consented reference and artifact SHA-256 values. The original source reference remains linked.

## Current status

`OPEN`.

Zero-contact M9.1 evidence is closed and merged, but it is not adoption. M9.2 remains open until either the positive adoption gate is satisfied or a separately preregistered discovery scope is exhausted and closed as a negative result without an adoption claim.
