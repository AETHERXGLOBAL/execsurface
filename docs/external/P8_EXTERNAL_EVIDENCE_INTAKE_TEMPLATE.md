# P8 External Evidence Intake Template

Use one record per initial external result. Never overwrite an initial attempt with a later assisted result.

## Identity

- Evidence ID: `P8-EXT-XXXX`
- Evidence class: `ZERO_ASSISTANCE_REPRODUCTION | REPRODUCTION_FAILURE | ARCHITECTURE_CRITICISM | COUNTEREXAMPLE | NO_FIT_OR_REDUNDANCY | INTEROPERABILITY_GUIDANCE | EXTERNAL_REAL_WORKLOAD_REPORT | EXTERNAL_CONTRIBUTION`
- Source/channel:
- Source locator / evidence locator:
- External actor / handle:
- Relationship: `UNAFFILIATED | UPSTREAM_OR_ECOSYSTEM_MAINTAINER | USER_OR_EVALUATOR | PRIOR_CONTACT_NO_PROJECT_ROLE | CONTRIBUTOR_TO_EXECSURFACE | AETHER_X_AFFILIATED | UNKNOWN`
- Received at:

## Target

- Product/research target:
- Release/tag:
- Commit/SHA:
- Documentation revision if known:

## Attempt conditions

- Assistance: `ZERO_ASSISTANCE | DOCS_ONLY_CLARIFICATION | ASSISTED | COLLABORATIVE | NOT_APPLICABLE`
- OS / architecture:
- CI/runtime environment:
- Workload origin: `AETHER_X_CONTROLLED | EXTERNAL | UNKNOWN | NOT_APPLICABLE`
- Workload identity/command if shareable:

## Raw external result

Preserve the evaluator's meaning before response or repair.

- Result summary:
- Reproduction steps / reasoning:
- Logs/evidence excerpt or locator:
- Reported false PASS / false REVIEW / incompleteness:
- Setup/usability/privacy observations:
- Proposed claim/architecture change, if any:

## Safety

- Security-sensitive: `yes | no | uncertain`
- If yes/uncertain: stop public technical details and route through SECURITY.md.

## Qualification

- Qualification state: `QUALIFYING_EXTERNAL_EVIDENCE | TECHNICALLY_USEFUL_NONINDEPENDENT | ROUTING_ONLY | PARTICIPATION_ONLY | INSUFFICIENT_DETAIL | SECURITY_PRIVATE_PROCESS | DISQUALIFIED_SELF_EVIDENCE`
- Qualification rationale:

## Scientific processing

- Initial state: `OPEN`
- Internal reproduction protocol/commit/run:
- Result: `REPRODUCED | NOT_REPRODUCED_TESTED_SCOPE | ACCEPTED | REJECTED_WITH_EVIDENCE | INCONCLUSIVE | NO_FIT | OPEN`
- Resulting action:
- Claim/docs/code changed:

## Follow-up separation

If an assisted retry occurs, create a separate linked record. Do not replace this record.

- Follow-up evidence ID:
- Assistance introduced:
- Relationship to initial result:

## Claims boundary

- Endorsement claimed: `false`
- Adoption claimed: `false` unless exact independent adoption evidence exists
- Independent reproduction claimed: `true` only if qualification + assistance conditions support it
