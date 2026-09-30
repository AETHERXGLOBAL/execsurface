# ExecSurface — P8 External Evidence Ledger

Tracking: #114
External engagement: #94
Protocol: `docs/development/P8_A0_EXTERNAL_EVIDENCE_PROTOCOL.md`

Purpose: preserve the qualification and scientific state of every technically meaningful external result without promoting routing, participation, internal rehearsal or prestige into validation.

## State vocabulary

Qualification:
- `QUALIFYING_EXTERNAL_EVIDENCE`
- `TECHNICALLY_USEFUL_NONINDEPENDENT`
- `ROUTING_ONLY`
- `PARTICIPATION_ONLY`
- `INSUFFICIENT_DETAIL`
- `SECURITY_PRIVATE_PROCESS`
- `DISQUALIFIED_SELF_EVIDENCE`

Scientific processing:
- `OPEN`
- `REPRODUCED`
- `NOT_REPRODUCED_TESTED_SCOPE`
- `ACCEPTED`
- `REJECTED_WITH_EVIDENCE`
- `INCONCLUSIVE`
- `NO_FIT`

## Frozen baseline at P8 start

| ID | Evidence class | Source | Relationship | Assistance | Qualification | Scientific state | Boundary / action |
|---|---|---|---|---|---|---|---|
| P8-HIST-0001 | `ARCHITECTURE_CRITICISM` | Greg Kroah-Hartman ptrace-vs-LSM criticism; original record `EXT-0001` in `OPENSSF_FEEDBACK_LEDGER.md` | `UPSTREAM_OR_ECOSYSTEM_MAINTAINER` | `NOT_APPLICABLE` | `QUALIFYING_EXTERNAL_EVIDENCE` | `ACCEPTED` | Historical external criticism already acted upon: bound ptrace authority, PATH-TOCTOU/shared-FD falsification, fail-closed hardening, kernel-hook/hybrid research. Retained as prior evidence; does not substitute for current-state reproduction/review. |
| P8-ROUTE-0001 | routing / community-path guidance | Linux Foundation / OpenSSF response recorded in #94 | `PRIOR_CONTACT_NO_PROJECT_ROLE` | `NOT_APPLICABLE` | `ROUTING_ONLY` | `INCONCLUSIVE` | Directed engagement to community/working-group paths. Not validation, adoption or endorsement. |
| P8-ROUTE-0002 | routing guidance | Rust Foundation response recorded in public repository current-state history | `PRIOR_CONTACT_NO_PROJECT_ROLE` | `NOT_APPLICABLE` | `ROUTING_ONLY` | `INCONCLUSIVE` | Directed project-review request to Rust users Code Review. Not a project review or validation. |
| P8-PART-0001 | community participation | AETHER X joined OpenSSF Slack and posted in `#wg-orbit`, recorded in #94 | `AETHER_X_AFFILIATED` | `NOT_APPLICABLE` | `PARTICIPATION_ONLY` | `OPEN` | Valid outreach channel only; cannot satisfy external validation. |
| P8-SELF-0001 | internal clean-environment rehearsal | AETHER X GitHub Actions, recorded in #94 | `AETHER_X_AFFILIATED` | `NOT_APPLICABLE` | `DISQUALIFIED_SELF_EVIDENCE` | `ACCEPTED` for pack integrity only | Demonstrates public-pack executability under AETHER X-controlled CI; never independent reproduction/adoption. |

## Current-state rule

No current P8 A1 zero-assistance external reproduction has been received at this ledger freeze.

Do not infer failure, rejection, validation or adoption from silence.

## New-record rule

When a new external result arrives:
1. preserve the original source/result before troubleshooting;
2. create a `P8-EXT-XXXX` record using `P8_EXTERNAL_EVIDENCE_INTAKE_TEMPLATE.md`;
3. qualify independence/assistance separately from scientific correctness;
4. if security-sensitive, stop public technical detail and route privately;
5. if reproduction/testing is appropriate, preregister the finding-specific test before changing claims/code;
6. retain failed/partial/no-fit outcomes permanently.

## Claim boundary

A ledger entry can be technically valuable without constituting independent reproduction, endorsement or adoption.
