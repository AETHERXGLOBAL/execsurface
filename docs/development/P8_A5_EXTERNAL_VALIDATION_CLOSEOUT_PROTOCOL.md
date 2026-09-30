# ExecSurface — P8-A5 External Validation Closeout Protocol

Date: 2026-09-30
Parent program: #100
P8 issue: #114
Status: **PREREGISTERED — P8 CANNOT CLOSE FROM INTERNAL EVIDENCE ALONE**

## Purpose
Freeze the minimum evidence required to close P8 before current external results arrive, preventing convenient post-hoc closure after a favorable acknowledgement, single praise message, or internal reproduction.

## Fixed roles
1. Innovation Scientist / Systems Architect — synthesize external evidence into the smallest justified architecture/product decisions.
2. Anti-Drift / Scientific Integrity Reviewer — blocks weak evidence promotion, cherry-picking and endorsement/adoption inflation.
3. Independent Falsifier / Red Team — attacks independence, reproduction, assistance and source claims for every candidate closeout item.
4. Independent Critical-Milestone Reviewer — independently rechecks evidence locators, target identities, negative-result retention and claims wording.

## Mandatory prerequisites

### A0 — pack integrity
Must be closed as:
`P8_A0_EXTERNAL_VALIDATION_PACK_READY_BOUNDED`

A0 is necessary but contributes zero independent-validation evidence by itself.

### Current external evidence requirement
P8 closeout requires **at least two distinct current qualifying external evidence records** after P8-A0 freeze, from at least **two distinct evidence classes**, with these coverage requirements:

1. at least one current record from the execution/use side:
   - A1 `ZERO_ASSISTANCE_REPRODUCTION` / `REPRODUCTION_FAILURE` / independent partial/unsupported attempt; or
   - A4 `EXTERNAL_REAL_WORKLOAD_REPORT`;

AND

2. at least one current record from the challenge/interoperability side:
   - A2 `ARCHITECTURE_CRITICISM`, `COUNTEREXAMPLE`, or `NO_FIT_OR_REDUNDANCY`; or
   - A3 `INTEROPERABILITY_GUIDANCE`.

The two records may come from the same external person only if they are substantively independent evidence classes with separate technical content and locators. Prefer distinct evaluators; if the same evaluator supplies both, the closeout reviewer must state that limitation explicitly.

## Historical evidence boundary

Historical `P8-HIST-0001` / `EXT-0001` is qualifying prior external architecture criticism already incorporated into the system. It remains part of the synthesis but **does not satisfy the two-current-record closeout minimum** because P8 is testing the current public/research state after those changes.

## Symmetric outcomes

P8 can close scientifically even if the current evidence is negative.

Examples of valid closeout evidence:
- zero-assistance reproduction failure + accepted architecture criticism;
- successful independent reproduction + no-fit finding;
- external real-workload partial/unsupported result + interoperability gap;
- independent reproduction success + counterexample that constrains a separate claim.

P8 does not require endorsement, adoption, positive review, or product-market fit.

## Negative-evidence retention

Before closeout:
- every qualifying failed/partial/no-fit result must exist in `P8_EXTERNAL_EVIDENCE_LEDGER.md` or a linked finding record;
- assisted retries must remain separate from initial attempts;
- no original evidence locator may be replaced by a cleaned-up internal summary;
- unresolved/inconclusive external findings remain listed.

## Security-sensitive evidence

A security-sensitive report may satisfy an evidence-class requirement if the critical-milestone reviewer can verify its existence, qualification and disposition without publishing exploit details. Public closeout wording must not leak sensitive content.

## Immutable product/research claims

Closeout must independently verify:
- public alpha.4/stable tag identity has not been silently reinterpreted;
- P2-P7 research-only results are not described as alpha.4 public features;
- no reviewer affiliation/name is used as proof beyond the technical evidence;
- no standards-body endorsement is claimed from interoperability feedback;
- adoption is claimed only if separate evidence actually establishes adoption.

## Closeout decisions

After satisfying the minimum current external evidence requirement, P8 may close as one of:

### `P8_EXTERNAL_VALIDATION_COMPLETE_BOUNDED`
Current qualified external evidence is sufficient for bounded claims and all external findings are processed/retained.

### `P8_EXTERNAL_VALIDATION_COMPLETE_WITH_MATERIAL_GAPS`
The minimum external evidence exists, but one or more material findings remain unresolved/inconclusive. P8 can still close scientifically if the gaps and claim restrictions are explicit.

### `P8_EXTERNAL_NO_FIT_DECISION_BOUNDED`
Current external evidence establishes a concrete no-fit/redundancy conclusion for the intended niche or a major portion of it. This is a valid scientific outcome and must not be reframed as success.

## Cannot-close states

- `P8_EXTERNAL_EVIDENCE_INSUFFICIENT`
- `P8_EXTERNAL_INDEPENDENCE_NOT_ESTABLISHED`
- `P8_EXTERNAL_SECURITY_REVIEW_PENDING`

Silence, routing, Slack participation, downloads/stars, or internal tests leave P8 in an insufficient-evidence state.

## Current preregistered state

A0 is pack-ready. No two-current-record minimum has been satisfied yet.

Therefore:

**`P8_EXTERNAL_EVIDENCE_INSUFFICIENT — A1/A2/A3/A4 CURRENT EXTERNAL INPUT PENDING`**
