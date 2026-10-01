# OpenSSF / Linux Foundation Technical Community Engagement

Tracking: #94
Status: `POST-RELEASE TECHNICAL ENGAGEMENT — EXTERNAL EVIDENCE OPEN`
Current public release: `v0.1.0-alpha.5`
Release source: `9e73b925d55557e33de1b0813995609aaefdc037`
Stable Action: `AETHERXGLOBAL/execsurface@v0.1`
Current baseline: `docs/external/OPENSSF_CURRENT_STATE_BASELINE.md`

## Relationship classification

Treat OpenSSF as:

`EXTERNAL TECHNICAL COMMUNITY / OPEN-SOURCE SECURITY ECOSYSTEM`

Do not describe OpenSSF / Linux Foundation as customer, validator, partner, approver, or endorser unless a later explicit agreement supports that wording.

## Outreach history

AETHER X previously asked OpenSSF for independent technical evaluation of ExecSurface. The historical request referenced an earlier alpha and explicitly asked for independent installation/reproduction, including negative and friction evidence rather than endorsement.

OpenSSF / Linux Foundation responded that OpenSSF projects are community-driven, recommended participating directly in OpenSSF Working Groups for technical insight, invited AETHER X to join the community, and noted that membership can be discussed separately.

That routing is historical context only. All new technical engagement is governed by the current Alpha.5 baseline and the post-release P8 evidence program.

## Current engagement objective

The objective is not a broad endorsement request.

The objective is to put ExecSurface's bounded runtime-evidence model in front of engineers who can falsify it, compare it with existing ecosystem mechanisms, identify interoperability opportunities, or conclude that part of the model is redundant/no-fit.

Useful outcomes include:

- zero-assistance reproduction or reproduction failure;
- architecture criticism or counterexample;
- standards/interoperability guidance;
- external real-workload evidence;
- implementation defect or usability friction;
- explicit no-fit/redundancy conclusion;
- a concrete contribution or integration path.

Praise, routing, membership, or participation alone does not qualify as validation.

## Current OpenSSF path

`PRIMARY_OPENSSF_PATH = ORBIT Working Group`

Reason: ExecSurface's strongest ecosystem fit is interoperable security-relevant evidence, baseline semantics, provenance, and tooling rather than generic runtime threat detection.

`SECONDARY_OPENSSF_PATH = Supply Chain Integrity Working Group`

Use this path when there is a concrete question about linking runtime evidence to artifact provenance, in-toto/SLSA/Sigstore, GUAC-like data, or downstream supply-chain integrity.

Do not split initial technical engagement across many groups without a specific question.

## Questions for ORBIT

1. Is a command-scoped runtime execution-surface baseline/drift model useful as reusable security evidence, or too product-specific/noisy?
2. Is the evidence model sufficiently explicit about observer capability, completeness, and authority?
3. Should runtime evidence be represented as a standalone artifact, mapped into an existing security-data model, or remain product-local?
4. Does the proposition/authority distinction adequately prevent syscall/path metadata from being treated as kernel-object proof?
5. Which negative/incomplete states need standard representation for interoperability?
6. Are existing ORBIT projects already solving the useful part of this problem?
7. Is there a credible contribution path, or should ExecSurface only consume existing standards?

## Questions for Supply Chain Integrity

1. Can command-scoped runtime evidence complement SLSA/in-toto provenance after an artifact is built or consumed?
2. What minimum source/artifact/workflow binding is required before runtime evidence is useful downstream?
3. Can the current ExecSurface verification-result attestation compose cleanly with existing predicates without inventing a competing format?
4. Would this add integrity context or simply duplicate existing runtime telemetry?

## Public review target

External reviewers should evaluate the published Alpha.5 release, not an obsolete candidate:

- release: `v0.1.0-alpha.5`
- immutable Action: `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`
- registry package: `execsurface = 0.1.0-alpha.5`
- platform scope: Linux x86_64
- public reference observer: native `ptrace`
- public review hub: issue #118
- P8 evidence tracking: issue #114

Alpha.5 publication was owner-authorized with P8 waived only as a pre-publication condition. This is not a statement that P8 passed. Independent external validation remains open post-release.

## Community-entry rule

Participation must remain contribution-first and non-promotional:

- introduce the technical problem and bounded experiment, not the company story;
- ask a small number of falsifiable questions;
- provide the public self-service reproduction path;
- explicitly welcome negative/no-fit conclusions;
- do not ask for endorsement;
- do not call a working-group discussion "validation";
- preserve the first independent result before any assisted follow-up.

## Membership status

`MEMBERSHIP_OPTION_AVAILABLE — NOT REQUIRED FOR TECHNICAL ENGAGEMENT`

OpenSSF community and Working Group participation can proceed independently of a paid membership decision. No financial or legal commitment is implied by technical participation.

## Feedback processing

Every substantive external criticism is recorded in `docs/external/OPENSSF_FEEDBACK_LEDGER.md` and processed as:

`CLAIM -> CRITICISM/RESULT -> QUALIFICATION -> TEST/REPRODUCTION -> EVIDENCE -> DECISION`

Do not defend the implementation by default. Reproduce, falsify, or bound the criticism.

## Claim boundary

OpenSSF/community participation, routing guidance, acknowledgement, working-group activity, or positive discussion must not be represented as:

- OpenSSF validation;
- Linux Foundation endorsement;
- production-readiness certification;
- proof of software safety;
- independent adoption without qualifying evidence.
