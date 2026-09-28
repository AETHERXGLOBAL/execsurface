# OpenSSF / Linux Foundation Technical Community Engagement

Tracking: #94
Status: `PRE-SUBMISSION — DOCUMENTATION_REPAIR / SELF-SERVICE REHEARSAL`
Current public release: `v0.1.0-alpha.4`
Current public source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

## Relationship classification

Treat OpenSSF as:

`EXTERNAL TECHNICAL COMMUNITY / OPEN-SOURCE SECURITY ECOSYSTEM`

Do not describe OpenSSF / Linux Foundation as customer, validator, partner, approver or endorser unless a later explicit agreement supports that wording.

## Outreach history

AETHER X previously asked OpenSSF for an independent technical evaluation of ExecSurface. The historical request referenced `v0.1.0-alpha.3` and explicitly asked for independent installation/reproduction, including negative and friction evidence rather than endorsement.

OpenSSF / Linux Foundation responded that OpenSSF projects are community-driven, recommended participating directly in OpenSSF Working Groups for technical insight, invited AETHER X to join the community, and noted that membership can be discussed separately.

The alpha.3 reference is historical only. All new engagement is governed by `OPENSSF_CURRENT_STATE_BASELINE.md` and alpha.4.

## Engagement objective

The goal is not a broad "please review our project" request.

The goal is to put a bounded runtime-evidence model in front of engineers who can falsify it, compare it to existing ecosystem mechanisms, and identify whether it should interoperate with existing OpenSSF standards/tools rather than compete with them.

Success requires at least one substantive result: independent reproduction, meaningful criticism, architecture improvement, standards/interoperability opportunity, documented no-fit, maintainer discussion, external contribution, or credible adoption path.

## Current OpenSSF Working Group matrix

Current OpenSSF working groups were reviewed against the current public product and research boundary.

| Working Group / SIG | Relevance | Why | What ExecSurface can contribute | What we want reviewed |
|---|---|---|---|---|
| **ORBIT — Open Resources for Baselines, Interoperability and Tooling** | **HIGH / PRIMARY** | ORBIT explicitly focuses on interoperable resources for identification/presentation of security-relevant data, baselines, testing, integration and tooling. ExecSurface is primarily an evidence generator with an accepted runtime baseline/diff model. | Concrete runtime evidence schema/use case; a test case for interoperability between runtime evidence and security metadata/tooling; failure-first external evaluation methodology. | Whether runtime execution-surface evidence belongs in/interoperates with ORBIT resources; schema clarity; evidence provenance; baseline semantics; integration with Security Insights, Gemara/Minder/assessment tooling. |
| **Supply Chain Integrity WG** | **HIGH / SECONDARY** | Focuses on end-to-end software supply-chain integrity and projects such as SLSA, GUAC and gittuf. ExecSurface is post-build/runtime evidence that may complement build provenance rather than replace it. | Runtime evidence as a possible downstream signal attached to an artifact/workflow; concrete examples of where provenance alone does not describe observed runtime effects. | Whether runtime evidence should be linked to SLSA/in-toto/GUAC-like metadata; whether this is useful or redundant; artifact/command identity and trust semantics. |
| Security Baseline SIG (under ORBIT) | MEDIUM | OSPS Baseline defines project security controls. ExecSurface could eventually provide automation evidence for selected controls, but the current product is not itself a project-security compliance framework. | Potential machine evidence for future controls/assessment automation if a concrete mapping exists. | Whether any current OSPS control can legitimately consume runtime-drift evidence; avoid inventing a compliance mapping. |
| Securing Software Repositories WG | LOW-MEDIUM | Focuses on package repositories/registries and tools that rely on them. ExecSurface can monitor commands that consume packages but is not registry infrastructure. | Consumer-side runtime drift case studies against package/dependency workflows. | Whether package/repository ecosystems have a concrete use case for post-install runtime evidence. |
| AI/ML Security WG | LOW-MEDIUM / FUTURE | ExecSurface can monitor AI-assisted developer/agent workflows, but the current OpenSSF request is broader software integrity, not an AI security claim. | Bounded runtime evidence for agent/tool execution experiments. | Only if a concrete AI-agent workflow need emerges; do not lead with this path now. |
| Best Practices for Open Source Developers | LOW-MEDIUM | Could help with developer guidance if runtime evidence proves practical, but it is not the best architecture-review venue. | Reproducible self-evaluation workflow and lessons from fail-closed evidence. | Developer usability after technical fit is established elsewhere. |

Official current references:

- OpenSSF Working Groups: https://openssf.org/community/openssf-working-groups/
- ORBIT: https://github.com/ossf/wg-orbit
- Supply Chain Integrity: https://openssf.org/groups/supply-chain-integrity/
- OpenSSF TAC initiative inventory: https://github.com/ossf/tac

## Selected path

`PRIMARY_OPENSSF_PATH = ORBIT Working Group`

Reason: the strongest current fit is not generic runtime security detection; it is **interoperable security-relevant evidence and baseline semantics**. ORBIT's scope gives the best chance of receiving useful criticism on whether ExecSurface's evidence model can integrate with existing OpenSSF data/tooling rather than becoming an isolated product-specific format.

`SECONDARY_OPENSSF_PATH = Supply Chain Integrity Working Group`

Reason: use only after the ORBIT discussion has clarified the evidence model, or when a concrete question exists about attaching runtime evidence to artifact provenance / SLSA / GUAC-like supply-chain data.

Do not split initial participation across several WGs.

## Why not lead with a "Security Tooling" group

The current OpenSSF WG inventory does not expose a standalone Working Group named simply "Security Tooling". Tooling/interoperability work is currently represented most directly by ORBIT and its technical initiatives. Do not invent a group name from historical correspondence or informal descriptions.

## Technical questions to take to ORBIT

1. Is a command-scoped runtime execution-surface baseline/drift model useful as security-relevant evidence, or is it too product-specific/noisy to be interoperable?
2. Is the baseline/evidence model sufficiently explicit about observer capability, completeness and authority?
3. Should runtime evidence be represented as a standalone artifact, a Security Insights/Gemara-compatible input, or not integrated at all?
4. Does the current proposition/authority distinction adequately separate ptrace argument metadata from kernel-object evidence?
5. What data model would make runtime evidence reusable without falsely turning observed behavior into a safety assertion?
6. Are there existing ORBIT projects/tools that already solve the useful part of this problem?
7. Which negative/incomplete states need to be standardized for interoperability?
8. Is there a credible contribution path, or should ExecSurface remain independent and only consume existing standards?

## Secondary questions for Supply Chain Integrity

If/when brought to SCI:

1. Can command-scoped runtime evidence complement SLSA/in-toto provenance after an artifact is built or consumed?
2. Is there a meaningful GUAC relationship for attaching observed runtime facts to artifact/source/workflow identities?
3. Would that add useful integrity context or merely duplicate runtime-security telemetry?
4. What minimum identity/provenance binding would be required before such evidence is trustworthy enough to ingest?

## Community-entry rule

Participation must be contribution-first and non-promotional:

- introduce the technical problem and bounded experiment, not the company story;
- ask a small number of falsifiable questions;
- provide a public, self-service reproduction pack;
- explicitly welcome negative/no-fit conclusions;
- do not ask for endorsement;
- do not call a WG discussion "validation";
- do not provide private implementation assistance before an evaluator has attempted the public path.

## Membership status

`MEMBERSHIP_OPTION_AVAILABLE — NOT YET EVALUATED FOR COMMITMENT`

OpenSSF community and Working Group participation is open to non-members, so membership is not required for the technical objective.

Current public fee information shows paid Premier and General tiers, with General pricing dependent on employee count, and free Associate eligibility limited to qualifying nonprofit/academic/government/open-source-foundation organizations. No financial or legal commitment is authorized here.

Membership will be evaluated only after free technical participation demonstrates a concrete benefit that cannot be obtained through normal community participation.

## Communication state

Do not send the historical alpha.3 request again.

External reply/send gate requires:

- alpha.4 evaluation docs repaired;
- `docs/OPENSSF_TECHNICAL_REVIEW.md` complete;
- zero-assistance public-artifact rehearsal PASS or a documented blocker;
- claims-boundary review complete.

## Feedback processing

Every external criticism is recorded in `docs/external/OPENSSF_FEEDBACK_LEDGER.md` and processed as:

`CLAIM -> CRITICISM -> TEST -> EVIDENCE -> DECISION`

No defensive-response shortcut.
