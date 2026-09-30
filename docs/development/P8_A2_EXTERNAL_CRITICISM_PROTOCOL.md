# ExecSurface — P8-A2 Current External Architecture Criticism / Counterexample Protocol

Date: 2026-09-30
Parent program: #100
P8 issue: #114
Predecessor: P8-A0 pack ready; A1 may proceed independently.
Status: **PREREGISTERED — CURRENT EXTERNAL FINDING REQUIRED**

## Question
Can a technically substantive external reviewer identify a concrete surviving architecture gap, counterexample, no-fit/redundancy finding, or claim-boundary defect in the current public/research state, and can ExecSurface process it without defense-first reasoning or evidence loss?

## Fixed roles
1. Innovation Scientist / Systems Architect — turn criticism into hypotheses, not rebuttal.
2. Anti-Drift / Scientific Integrity Reviewer — blocks prestige weighting, cherry-picking and goal changes after feedback.
3. Independent Falsifier / Red Team — attempts to reproduce or strengthen the external counterexample.
4. Independent Critical-Milestone Reviewer — verifies original external wording/evidence is preserved before internal response.

Dynamic specialists are selected from Linux runtime/ptrace/LSM, evidence semantics, CI/supply-chain, filesystem/network/process identity, privacy, and standards interoperability according to the finding.

## Current-state target
A2 current evidence must target at least one materially current boundary:
- public alpha.4 behavior/claims; or
- the explicitly identified post-alpha.4 research architecture through P7.

Historical `P8-HIST-0001` / `EXT-0001` is retained as qualifying prior criticism already acted upon, but does not by itself close this current-state A2 gate.

## Qualifying inputs
- `ARCHITECTURE_CRITICISM`
- `COUNTEREXAMPLE`
- `NO_FIT_OR_REDUNDANCY`
- technically specific claim-overstatement finding

The input must identify a concrete proposition, failure mode, overlap, or claim boundary. Praise, generic concern, or organizational affiliation is insufficient.

## Required processing
1. Freeze the original external result in the P8 ledger before discussion/fix.
2. Classify security sensitivity; S4 routes privately.
3. State the exact claim/proposition under attack.
4. Preregister the smallest reproduction/falsification test where executable testing is meaningful.
5. Preserve a `NO_FIT` possibility; do not force every criticism into a product fix.
6. Record one of:
   - `REPRODUCED`
   - `NOT_REPRODUCED_TESTED_SCOPE`
   - `ACCEPTED`
   - `REJECTED_WITH_EVIDENCE`
   - `INCONCLUSIVE`
   - `NO_FIT`
7. Any code/docs change is linked to the finding; the original criticism remains unchanged.

## Prohibited
- `CRITICISM -> DEFENSE` without a test/evidence step;
- changing thresholds/claims before preserving the external result;
- calling an unreproduced criticism false universally;
- treating reviewer title/employer as proof;
- deleting a no-fit/redundancy result because it harms positioning;
- using internal red-team output as a substitute for current external criticism.

## A2 outcomes
A2 may close current-state evidence as one of:
- `P8_A2_CURRENT_EXTERNAL_CRITICISM_ACCEPTED_BOUNDED`
- `P8_A2_CURRENT_EXTERNAL_COUNTEREXAMPLE_REPRODUCED_BOUNDED`
- `P8_A2_CURRENT_EXTERNAL_NO_FIT_BOUNDED`
- `P8_A2_CURRENT_EXTERNAL_FINDING_NOT_REPRODUCED_TESTED_SCOPE`
- `P8_A2_CURRENT_EXTERNAL_FINDING_INCONCLUSIVE`

All of these are scientifically legitimate outcomes. None is an endorsement claim.

Current state: **`P8_A2_CURRENT_EXTERNAL_FINDING_PENDING`**.
