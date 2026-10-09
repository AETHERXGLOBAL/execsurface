# ExecSurface Production Readiness

Program: `P9 — Production Readiness & v1.0 Qualification`
Tracking issue: `#129`
Program start HEAD: `88284764c70dddf137e89a34b48519328e530ad9`
Current public release at program start: `v0.1.0-alpha.5`

Current gap-analysis refresh: 2026-10-08  
Current public release at closeout: `v1.0.0`  
Current gap analysis: `docs/development/V1_PRODUCTION_READINESS_GAP_ANALYSIS.md`  
Current stable contract: `docs/COMPATIBILITY.md`  
Stable support/deprecation policy: `docs/SUPPORT_POLICY.md`

The original gate criteria below remain retained as the frozen decision framework. The 2026-10-08 closeout records Stage-2 remediation, Alpha.6 productization, v1 stable-contract freeze, RC1 qualification, release-control hardening, stable publication and registry proof without rewriting earlier evidence.

Governance amendment: `docs/development/V1_INTERNAL_QUALIFICATION_GOVERNANCE.md`

The owner explicitly removed P9.1 and P9.5 external evidence as mandatory release blockers. They remain **WAIVED_AS_RELEASE_BLOCKER_BY_GOVERNANCE**, not PASS. Missing external evidence remains a disclosed limitation. Stable v1.0.0 was released only after the strengthened internal IQ0-IQ9 qualification chain, release-control proof and public artifact/registry verification. Independent external validation is not claimed.

Final P9 decision: **`RELEASE_V1_0`**  
Stable release workflow: `37771109825` — SUCCESS  
Registry publication workflow: `37771295572` — SUCCESS  
Release source: `e70169b959f2163715090c371335fa6c5591e3e4`

This document freezes the evidence contract for deciding whether ExecSurface may move from public alpha to a production-qualified stable `v1.0` release.

It is a gate document, not a marketing maturity claim.

`INTERNAL GREEN CI != PRODUCTION READINESS`

`PUBLIC RELEASE != INDEPENDENT ADOPTION`

`FEATURE COUNT != MATURITY`

## Production objective

A `v1.0` decision requires evidence that independent users can install, understand, integrate and operate ExecSurface within the declared support boundary, while preserving its fail-closed semantics, compatibility contract, release integrity and explicit limitations.

Production qualification does not require feature breadth. It requires bounded claims that survive independent use and adversarial review.

## Fixed decision roles

1. **Production Readiness Lead / Systems Architect** — owns the end-to-end gate chain and removes nonessential scope.
2. **Innovation Scientist** — searches for high-leverage improvements that reduce adoption friction or strengthen evidence quality without uncontrolled scope expansion.
3. **Anti-Drift / Scientific Integrity Reviewer** — blocks maturity inflation, gate skipping, vanity features and reinterpretation of negative evidence.
4. **Independent Falsifier / Red Team** — attacks false PASS, observer incompleteness, upgrade assumptions, packaging, provenance and operational failure modes.
5. **Independent Critical-Milestone Reviewer** — independently verifies source identity, evidence provenance, closeout wording and release boundaries.

Dynamic specialists are added only when a gate requires them: Rust/SemVer, Linux runtime/ptrace, developer experience, CI/CD, software supply chain, packaging, performance, external adoption and supportability.

## Gate matrix

| Gate | Requirement | Evidence source | Start state | Closeout rule |
|---|---|---|---|---|
| P9.0 | Freeze the production qualification contract | This document + issue #129 | PASS / FROZEN | Criteria are versioned, bounded and linked from current roadmap/docs |
| P9.1 | Independent external evidence | P8 issue #114 + public intake #118 | WAIVED AS RELEASE BLOCKER / EVIDENCE STILL INCOMPLETE | Historical/current external evidence is retained honestly; v1 release eligibility no longer depends on closing this row |
| P9.2 | Public-consumer reliability | Operational CI, public artifact, crates.io and Action consumption | PASS / CONTINUOUS | Supported consumer paths repeatedly pass without semantic relaxation |
| P9.3 | Compatibility & stability contract | Versioning/schema/CLI/migration evidence | PASS_BOUNDED / CLOSED | Stable compatibility promises and deprecation/migration rules were tested on the exact v1 RC |
| P9.4 | Repository & supply-chain controls | GitHub rules/protection, workflow permissions, release/provenance evidence | PASS_BOUNDED / CLOSED | Release-critical changes are gated by the approved control plane and immutable-v1 tag/channel mechanics were verified |
| P9.5 | Real-workload adoption evidence | External workloads not created solely for demonstration | WAIVED AS RELEASE BLOCKER / EVIDENCE STILL INCOMPLETE | External workload evidence remains valuable but is no longer mandatory for the v1 release decision |
| P9.6 | v1.0 RC qualification | Frozen RC source + full release/compatibility/adversarial evidence | PASS_BOUNDED / CLOSED | Exact RC was built, consumed, destructively tested and upgrade/rollback-qualified |
| P9.7 | Stable release decision | Internal critical review under amended governance | `RELEASE_V1_0` / CLOSED | Stable v1.0.0 published with bounded claims; external independent validation is not claimed |

## P9.0 — frozen production criteria

The following criteria are mandatory unless a later governance decision changes them with explicit rationale and evidence.

### Independent usability

At least one current qualifying external execution/use record and one current qualifying external challenge/interoperability record must satisfy the already-frozen P8 rules. Internal AETHER X runs cannot substitute for this criterion.

### Stable consumer contract

Before `v1.0`, ExecSurface must explicitly define and test the stability boundary for:

- CLI command/argument behavior that is declared stable;
- PASS / ERROR / REVIEW / BLOCK exit-code semantics;
- baseline, policy and report compatibility;
- machine-readable schema/version negotiation;
- supported Linux architecture/kernel/environment boundary;
- deprecation and migration behavior;
- rollback behavior for a bad release.

### Fail-closed semantics

No supported path may convert incomplete, ambiguous, unsupported or lost evidence into a silent PASS. Baseline remains distinct from policy. Backend identity remains distinct from proposition-specific authority.

### Operational reliability

The production candidate must preserve repeatable evidence for:

- source formatting/lint/tests/lockfile integrity;
- public release binary installation and checksum verification;
- exact registry installation;
- stable GitHub Action behavior;
- adversarial regressions;
- release source/artifact binding;
- provenance/attestation where claimed;
- clean consumption on every declared supported environment.

### Supply-chain and repository control

Release-critical source and automation must be governed by controls that prevent an ordinary unreviewed push from becoming a production release solely because a release request file changed.

Required control categories include:

- default-branch protection or repository ruleset appropriate to release-critical changes;
- required CI/review checks for production-affecting changes;
- least-privilege workflow permissions;
- immutable release identity;
- dependency and lockfile discipline;
- documented release authority;
- emergency rollback/revocation procedure.

## P9.4 initial control audit

Audit date: 2026-10-03.

### Verified strengths

- Current ordinary CI uses `contents: read` permission.
- Public-consumer, technical-evaluation and adversarial workflows are intentionally separate from release promotion.
- Release identity is tied to explicit package/tag state and the release process verifies formatting, clippy, tests and lockfile integrity before tag creation.
- The published Alpha.5 release already has strong internal/public-artifact reproducibility and provenance evidence.

### Material blocker — default branch protection

At program start, GitHub reports `main` as `protected: false`, and the repository rulesets endpoint returns an empty ruleset list.

This means P9.4 is **not closed**.

The current `promote-release.yml` is triggered by a push to `main` that changes `.release/release-request.json`, and the promotion workflow receives `contents: write` and `actions: write` so it can create the immutable version tag and dispatch the release workflow.

The workflow itself performs meaningful source/release validation, but without an appropriate branch/ruleset control plane, the repository-level authorization boundary remains weaker than required for a production release process.

### Required administrator action

Configure repository protection/rules for `main` before P9.4 closeout. The exact GitHub settings must be verified after application, but the intended minimum is:

- require pull requests before merge for release-affecting changes;
- require the relevant green CI/review checks;
- prevent force pushes and branch deletion;
- restrict bypass of release-critical controls;
- ensure release-request changes cannot reach `main` through an unreviewed direct push;
- preserve emergency recovery through an explicitly authorized governance path rather than an undocumented bypass.

If the connected automation lacks repository-administration permission, this item remains an explicit human-admin blocker and must not be marked complete by documentation alone.

## P9.1 / P9.5 external-evidence boundary

External evidence remains governed by P8 (#114). The following do not close production gates by themselves:

- stars or download counts;
- outbound emails or invitations;
- community membership or routing;
- acknowledgements or praise;
- AETHER X internal workloads;
- assisted demonstrations where the unassisted initial result was not preserved.

Negative, partial, unsupported, reproduction-failure, interoperability and no-fit outcomes remain valid evidence.

## Current decision

ExecSurface is not authorized to claim production-ready or stable `v1.0` status from the current evidence set.

The current sequence after the explicit internal-qualification governance amendment is:

1. preserve the frozen v1 stable contract and all Stage-2/Productization evidence;
2. complete V1-R1 stable release-control-plane hardening;
3. complete the strengthened IQ0-IQ9 internal qualification chain on the exact v1 RC;
4. preserve external evidence honestly when it exists, but do not make P9.1/P9.5 release blockers;
5. issue a separate internal critical-board decision that explicitly states external independent validation is not claimed;
6. only then authorize or reject v1.0.

## Release-decision rule

P9 may end only as one of:

- `RELEASE_V1_0`
- `REWORK_REQUIRED`
- `REMAIN_ALPHA_BETA`

No calendar date, feature count, internal confidence score or marketing objective can override missing gate evidence.
