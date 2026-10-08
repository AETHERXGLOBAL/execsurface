# ExecSurface v1 — Internal Qualification Governance Amendment

Date: 2026-10-08  
Parent program: P9 / issue #129  
Current source baseline: `main@84bdf7b03b3a67d5765178abd93b7741ef649aff`

Decision:

**EXTERNAL_VALIDATION_NOT_REQUIRED_FOR_V1_RELEASE_DECISION — INTERNAL_QUALIFICATION_REQUIRED_AT_MAXIMUM_RIGOR**

## Scope of this amendment

This is an explicit owner/governance decision to remove external execution/use and external real-workload evidence as mandatory release blockers for the v1.0 decision.

It does **not** convert missing external evidence into PASS.

It does **not** claim independent external validation.

It does **not** erase, rewrite, or downgrade the value of existing external evidence.

It changes the release-governance rule from:

> v1 cannot release without the frozen P9.1/P9.5 external-evidence minimum

to:

> v1 may proceed to an internally qualified stable release if every internal semantic, adversarial, compatibility, productization, release-control, artifact, rollback, environment and exact-RC gate passes under the strengthened internal review protocol below.

Any public statement about v1 must remain accurate:

> **internally qualified under the documented AETHER X gate system; external independent validation is not claimed.**

## Research classification

ExecSurface remains a runtime behavioral-verification / execution-semantics / semantic-evidence-correctness project.

This amendment does not reclassify the work as cybersecurity research.

## Scientific-integrity rule

Internal review cannot become “external” merely by assigning a different team name.

Therefore:

- internal reviewers are described as **organizationally separated internal reviewers**;
- external independence is never claimed;
- missing external evidence remains a known evidence limitation;
- external evidence can still be accepted later and may strengthen future releases, but is not a release prerequisite under this amended governance.

## Strengthened internal qualification board

The v1 release program uses separated roles with explicit non-overlap where practical.

### Team A — Runtime Semantics & Correctness

Primary duties:

- execution semantics;
- FD state-machine correctness;
- object/effect identity;
- baseline and normalization semantics;
- policy and verdict correctness;
- fail-closed evidence behavior.

This team proposes fixes but does not make the final release decision.

### Team B — Destruction / Falsification

Primary duties:

- preregister counterexamples before fixes;
- attack each stable claim;
- construct cross-gate and lifecycle failures;
- challenge aliasing, lineage, incompleteness, compatibility and materialization assumptions;
- preserve every valid RED state.

Rules:

- no acceptance-threshold weakening;
- no deleting negative evidence;
- no back-editing preregistered attack sets after seeing results;
- whenever a GREEN state is reached, add at least one new qualitatively different attack class before final closeout unless the board records why no further bounded class is justified.

### Team C — Compatibility / Upgrade / Rollback

Primary duties:

- Alpha.6 -> v1 upgrade;
- Alpha.5 historical incompatibility boundary;
- policy schema 1/2/3 compatibility;
- stable CLI/Action surface;
- rollback to the last qualified release;
- SemVer and deprecation discipline.

### Team D — Release Engineering / Artifact Provenance

Primary duties:

- immutable tag/source identity;
- stable GitHub Release classification;
- checksum/attestation;
- `@v1` promotion;
- crates.io publication ordering;
- clean zero-contact artifact consumption;
- no bypass of release authority.

### Team E — Product / New-User Qualification

Primary duties:

- landing -> install -> doctor -> learn -> check -> Action;
- target-command correctness separation;
- custody setup;
- packaging and current-doc consistency;
- supported-environment truthfulness.

### Team F — Independent Internal Critical Review Board

This board must not author the candidate fix being reviewed.

Primary duties:

- read exact-head evidence;
- review RED history and rejected alternatives;
- verify no test weakening;
- verify no maturity inflation;
- issue only one of:
  - `INTERNAL_RELEASE_AUTHORIZED`
  - `REWORK_REQUIRED`
  - `REMAIN_PRE_V1`

The board must explicitly state:

- what was proven;
- what remains unproven;
- which claims are bounded;
- that external independent validation is not claimed.

## Blind / separated destruction rule

For the final v1 RC round:

1. the candidate SHA is frozen;
2. Team B receives the frozen claims and product surface, not a list of recent implementation fixes as its attack plan;
3. Team B submits its attack corpus before any RC correction;
4. any valid counterexample invalidates the RC SHA;
5. corrected RC receives a new SHA and the full required gate chain reruns;
6. the Critical Review Board sees both the original RED and the corrected evidence.

This is organizational separation, not external independence.

## Mandatory v1 internal gate chain

External P9.1/P9.5 are no longer release blockers.

The minimum mandatory internal chain becomes:

### IQ0 — Source and contract freeze

- exact v1 stable contract;
- exact source SHA;
- no unreviewed scope expansion.

### IQ1 — Semantic kernel / Stage-2 replay

- CI;
- Stage-2 Final Internal Gate;
- adversarial regression;
- ptrace lifecycle;
- policy/custody/identity/materialization regressions.

### IQ2 — Destruction Round A

Attack stable semantics, including:

- incomplete evidence;
- FD replacement/recovery;
- object/path aliasing;
- rename lineage;
- output-parent rebinding;
- hardlink ambiguity;
- policy schema boundaries;
- target-outcome separation.

### IQ3 — Compatibility / upgrade / rollback

- Alpha.6 profile-4 -> exact v1 RC;
- Alpha.5 profile-3 explicit incompatibility;
- policy schema 1/2/3 behavior;
- Action input/output contract;
- no baseline/policy mutation;
- rollback.

### IQ4 — Environment / installation qualification

At minimum, according to the frozen contract:

- exact prebuilt v1 artifact on the qualified prebuilt environment;
- exact local-build/registry route on each declared environment;
- MSRV;
- ptrace capability/doctor behavior;
- unsupported environments fail or are documented honestly.

### IQ5 — Performance characterization

- representative declared workload classes;
- fixed protocol;
- retained negative/no-fit results;
- no universal low-overhead claim.

### IQ6 — Release-control-plane destruction

- stable `v1.0.0` classification;
- immutable tag/source binding;
- `@v1` promotion only after artifact/consumer proof;
- stable vs prerelease metadata;
- rollback/revocation rehearsal;
- Alpha-line regression.

### IQ7 — Productization / zero-contact consumers

- release binary;
- checksum;
- attestation;
- doctor;
- learn/check;
- controlled REVIEW;
- stable Action PASS/REVIEW/BLOCK/ERROR;
- custody;
- exact registry install;
- docs/current-status consistency.

### IQ8 — Destruction Round B

A second attack corpus must be created after IQ1-IQ7 are GREEN.

It must be qualitatively different from Round A and must include at least:

- release-ordering failures;
- stale/mismatched immutable tag;
- stable-channel mispointing;
- rollback edge cases;
- environment-floor mismatch;
- contract/documentation divergence;
- one novel cross-layer semantic attack selected by Team B.

### IQ9 — Independent Internal Critical Review

The Critical Review Board reviews the complete evidence bundle and issues the final internal decision.

No merge/release action occurs solely because automated tests are GREEN.

## Required evidence labeling

Future v1 documentation must distinguish:

- **[EXECUTABLE INTERNAL EVIDENCE]** — CI/tests/workflows on exact source;
- **[INTERNAL REVIEW]** — organizationally separated human/agent review;
- **[HISTORICAL EXTERNAL EVIDENCE]** — genuine prior external records;
- **[NOT EXTERNALLY VALIDATED]** — any stable release claim under this governance amendment.

## Changes to P9 interpretation

### P9.1

Old role: mandatory independent external evidence.

New state:

**WAIVED_AS_RELEASE_BLOCKER_BY_GOVERNANCE**

The underlying evidence status remains incomplete.

### P9.5

Old role: mandatory external real-workload adoption evidence.

New state:

**WAIVED_AS_RELEASE_BLOCKER_BY_GOVERNANCE**

The underlying evidence status remains incomplete.

### P9.6

May proceed once all remaining internal blockers close and the strengthened IQ0-IQ9 chain is satisfied.

### P9.7

The final release decision must use the strengthened internal critical-review board and must state that external validation is not claimed.

## Anti-marketing rule

The absence of an external blocker does not authorize phrases such as:

- independently validated;
- externally certified;
- proven in production;
- industry validated;
- universally production-ready.

Allowed bounded phrasing after a successful v1 release is:

> ExecSurface v1.0 is internally qualified under AETHER X GLOBAL's published compatibility, adversarial, artifact, rollback and productization gates for the documented support boundary. Independent external validation is not claimed.

## Final governance decision

**EXTERNAL_VALIDATION_NOT_REQUIRED_FOR_V1_RELEASE_DECISION — INTERNAL_QUALIFICATION_REQUIRED_AT_MAXIMUM_RIGOR**

This amendment is explicit, versioned and auditable. It is not a claim that internal review equals external review.
