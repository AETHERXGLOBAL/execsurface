# ExecSurface Production Readiness

Program: `P9 — Production Readiness & v1.0 Qualification`
Tracking issue: `#129`
Program start HEAD: `88284764c70dddf137e89a34b48519328e530ad9`
Current public release at program start: `v0.1.0-alpha.5`

Current gap-analysis refresh: 2026-10-08  
Current public release at refresh: `v0.1.0-alpha.6`  
Current gap analysis: `docs/development/V1_PRODUCTION_READINESS_GAP_ANALYSIS.md`

The original gate criteria below remain frozen. The 2026-10-08 gap analysis updates the factual state after Stage-2 remediation, Alpha.6 release, and the Alpha.6 Productization Gate without weakening any production criterion.

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
| P9.0 | Freeze the production qualification contract | This document + issue #129 | ACTIVE | Criteria are versioned, bounded and linked from current roadmap/docs |
| P9.1 | Independent external evidence | P8 issue #114 + public intake #118 | OPEN / BLOCKING | P8 current external-evidence minimum is genuinely satisfied |
| P9.2 | Public-consumer reliability | Operational CI, public artifact, crates.io and Action consumption | STRONG INTERNAL EVIDENCE | Supported consumer paths repeatedly pass without semantic relaxation |
| P9.3 | Compatibility & stability contract | Versioning/schema/CLI/migration evidence | OPEN | Stable compatibility promises and deprecation/migration rules are explicit and tested |
| P9.4 | Repository & supply-chain controls | GitHub rules/protection, workflow permissions, release/provenance evidence | OPEN / ADMIN BLOCKER PRESENT | Release-critical changes cannot bypass the approved control plane |
| P9.5 | Real-workload adoption evidence | External workloads not created solely for demonstration | OPEN / BLOCKING | Qualified independent workload evidence exists, including negative/no-fit outcomes |
| P9.6 | v1.0 RC qualification | Frozen RC source + full release/compatibility/adversarial evidence | BLOCKED | Exact public RC is reproducibly built, consumed and upgrade-tested |
| P9.7 | Stable release decision | Independent critical review of P9.0–P9.6 | BLOCKED | `RELEASE_V1_0`, `REWORK_REQUIRED`, or `REMAIN_ALPHA_BETA` recorded with evidence |

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

The immediate sequence is:

1. merge this production-criteria freeze after review/CI;
2. apply and verify repository protection/rules for P9.4;
3. freeze/test the P9.3 compatibility contract;
4. continue P8 independent validation without contaminating independence;
5. gather qualified external real-workload evidence;
6. only then construct and qualify a v1.0 release candidate.

## Release-decision rule

P9 may end only as one of:

- `RELEASE_V1_0`
- `REWORK_REQUIRED`
- `REMAIN_ALPHA_BETA`

No calendar date, feature count, internal confidence score or marketing objective can override missing gate evidence.
