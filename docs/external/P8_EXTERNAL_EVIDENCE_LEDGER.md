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

## Current external contribution records

These records arrived after P8-A0 and are retained even where they do **not** satisfy the A5 execution/use or challenge/interoperability minimum.

| ID | Evidence class | Source | Relationship | Assistance | Qualification | Scientific state | Boundary / action |
|---|---|---|---|---|---|---|---|
| P8-EXT-0001 | `EXTERNAL_CONTRIBUTION` | PandaHUN777 PR #126 from fork `PandaHUN777/execsurface`: deterministic `hello-drift` smoke example; merged 2026-10-02 | `CONTRIBUTOR_TO_EXECSURFACE` | `NOT_APPLICABLE` for contribution classification | `QUALIFYING_EXTERNAL_EVIDENCE` as an external contribution only | `ACCEPTED` | Contribution reported build/format/clippy/test validation and executable PASS→REVIEW behavior. The contributor implemented the tested contribution area, so this record is **not** promoted to A1 zero-assistance reproduction. No A1 relationship/assistance claim is inferred from the contribution. |
| P8-EXT-0002 | `EXTERNAL_CONTRIBUTION` | astrogilda PR #128 from fork `astrogilda/execsurface`: Python/pytest Alpha.5 integration recipe and retained execution report | `CONTRIBUTOR_TO_EXECSURFACE` | `NOT_APPLICABLE` for contribution classification | `QUALIFYING_EXTERNAL_EVIDENCE` as an external contribution only | `OPEN` | The PR preserves a reported checksum-selected Alpha.5 run with doctor/learn/PASS/REVIEW and unchanged baseline, plus a later restricted-environment doctor refusal and native tracing-test failures. Negative results are retained. The workload is a synthetic fixture authored for this contribution, so it is **not** P9.5/A4 external real-workload evidence. Because the contributor implemented the recipe and no separately frozen A1 independence/assistance declaration exists, the execution is not promoted to A1 zero-assistance reproduction. |
| P8-EXT-0003 | `EXTERNAL_CONTRIBUTION` | PandaHUN777 PR #127 from fork `PandaHUN777/execsurface`: copy-ready GitHub Action consumer example | `CONTRIBUTOR_TO_EXECSURFACE` | `NOT_APPLICABLE` for contribution classification | `QUALIFYING_EXTERNAL_EVIDENCE` as an external contribution only | `OPEN` | Maintainer review identified a copy-portability defect in a relative documentation link and requested changes. The open finding is retained rather than counted as a successful consumer proof. It does not satisfy A1/A2/A3/A4. |

### Qualification note

`EXTERNAL_CONTRIBUTION` is a qualifying P8 evidence class under A0, but it is not interchangeable with the specific current evidence classes required by A5 closeout.

The records above therefore establish genuine current external engineering participation and technically useful results, **not** current independent validation. In particular:

- no contributor record is relabeled `ZERO_ASSISTANCE_REPRODUCTION` without the A1 relationship, assistance, target and initial-result fields required by the preregistered protocol;
- a fixture authored specifically for an ExecSurface example is not relabeled `EXTERNAL_REAL_WORKLOAD_REPORT`;
- maintainer review findings are not relabeled external architecture criticism merely because the PR author is external;
- open/negative results remain visible and do not reduce the evidence value of the contribution.

## Current-state rule

Current external contributions now exist and are recorded above. The A5 closeout minimum is still **not** satisfied because the ledger does not yet contain both:

1. a current qualifying execution/use record from A1 zero-assistance reproduction/failure/independent partial attempt or A4 external real workload; and
2. a current qualifying challenge/interoperability record from A2 architecture criticism/counterexample/no-fit or A3 interoperability guidance.

Do not infer failure, rejection, validation or adoption from the absence of those specific classes.

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
