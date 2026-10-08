# ExecSurface — Stage-2 R8 Full Qualification / FINAL_INTERNAL_GATE Protocol

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Predecessors: R0–R7  
Qualification candidate at gate open: `70a680d35ee1ac516b69002d1bd6fb73d4f0da66`  
Status: **PREREGISTERED — FINAL INTERNAL QUALIFICATION OPEN**

## Objective

R8 is the final internal qualification gate for the Stage-2 remediation program.

It does not reopen closed gates merely to repeat work. It asks a stronger composed question:

> Do the independently repaired R0–R7 invariants remain true when custody, policy-v3 semantics, runtime observation, object identity, report materialization, compatibility and CI triggering are exercised together on the same candidate?

R8 is a qualification gate, not release authorization.

## Fixed review roles

1. **Execution / Runtime Verification** — owns exact reproduction, Linux behavior and deterministic qualification.
2. **Innovation / Architecture** — searches specifically for cross-gate semantic gaps not visible in isolated tests.
3. **Anti-Drift / Goal Alignment** — blocks any weakening, history rewrite, automatic migration or release-scope inflation.
4. **Independent Destruction / Falsification** — owns composed attacks and may expand the hostile corpus while R8 is open.
5. **Independent Critical Reviewer** — may close the gate only from exact source SHA + executable evidence + retained negative results.

The destruction role is logically independent from the implementation role: a newly discovered counterexample returns R8 to RED even if every predecessor gate remains individually GREEN.

## Source-of-truth freeze

At R8 opening:

- `main = c7c9d317e95ec4837dcce64b311cfdc13b512676`
- remediation branch candidate = `70a680d35ee1ac516b69002d1bd6fb73d4f0da66`
- PR #166 remains DRAFT / not merge-authorized
- Alpha.5 remains immutable historical evidence
- all nine adjacent workflows on the opening candidate completed SUCCESS, including CI, Adversarial Regression, Ptrace Lifecycle Regression, P9.3 Compatibility, packaging and typed-evidence gates.

Any newer branch head must be qualified independently. Success on an older SHA does not transfer automatically.

## R8 composed hostile corpus

### R8-C1 — custody + policy-v3 + verdict materialization

Use a valid externally pinned baseline and exact-byte pinned schema-v3 policy. Introduce runtime drift containing a truncating file open. A v3 `open_intent.truncate=true` BLOCK rule must survive the full CLI path and produce BLOCK/20 while JSON report materialization remains disjoint from protected inputs.

This composes R4 + R5 + R7.

### R8-C2 — custody mismatch dominates target execution

With a valid v3 policy and selected verdict output, supply a well-formed but unauthorized baseline digest. The command must not execute, no target marker may appear, and no policy or output semantics may downgrade custody failure into REVIEW/PASS.

This composes R4 + R5 + CLI verdict precedence.

### R8-C3 — protected policy/output alias dominates target execution

With correct custody pins, select the trusted policy itself as verdict output. The alias must be rejected before target execution, policy bytes must remain unchanged and no marker may appear.

This composes R4 + R7 under schema-v3 policy loading.

### R8-C4 — trusted-object move remains blocked under v3 policy

With correct custody pins and a valid v3 allow policy, let the target move the verified baseline inode onto the report-output pathname. Postflight object identity must detect the moved trusted object and return ERROR/2 without overwriting its bytes.

This composes R4 + R5 + the R7-X2 correction.

### R8-C5 — post-preflight policy mutation cannot rewrite the policy already consumed

Load and externally pin a schema-v3 policy whose default action is BLOCK. The target then replaces the on-disk policy bytes with a permissive policy while generating runtime drift. Evaluation must still use the already verified in-memory policy and return BLOCK/20.

This tests the custody/evaluation temporal boundary and rejects a file-re-read TOCTOU interpretation.

## Required predecessor replay

The dedicated final gate must execute, without weakening:

- Stage-2 external falsification acceptance corpus;
- R2 FD state-machine corpus;
- R3 object-identity corpus;
- R4 CLI custody corpus;
- R4 Action custody corpus;
- R5 policy-expressiveness corpus;
- R6 CI-trigger contract;
- R7 cross-gate destruction corpus;
- R8 composed hostile corpus;
- full workspace tests;
- workspace Clippy with warnings denied;
- rustfmt check.

The gate must run on both Ubuntu 22.04 and Ubuntu 24.04.

## Adjacent qualification requirements

On the exact R8 qualified source, all existing required workflows must also be SUCCESS:

- CI
- Adversarial Regression
- Ptrace Lifecycle Regression
- P9.3 Compatibility Contract
- Registry Packaging Gate
- Public Consumer Smoke
- P8 A3.4 consumer contract red team
- P8 A3 typed report prototype
- P8 A3 typed evidence output
- Stage-2 Final Internal Gate

A green dedicated R8 workflow cannot mask a regression in an adjacent gate.

## Anti-drift rules

R8 MUST NOT:

- weaken any R0–R7 acceptance assertion;
- rewrite an earlier RED run into GREEN history;
- modify or retag Alpha.5;
- silently migrate profile-3 baselines into profile 4;
- reinterpret policy schemas v1/v2 as v3;
- grant authority from backend name, signature presence or CI provider identity;
- convert incomplete/ambiguous evidence into PASS;
- merge PR #166 or authorize a release automatically.

New hostile cases found during R8 are recorded as explicit in-gate expansions, not back-edited into this preregistration.

## Decision rule

R8 may close only when:

1. R8-C1..C5 pass on the same source SHA;
2. every predecessor replay passes unchanged on that SHA;
3. both Ubuntu qualification jobs pass;
4. all adjacent workflows pass on that SHA;
5. independent destruction has no unresolved counterexample;
6. exact residual assumptions are documented;
7. the critical reviewer records **FINAL_INTERNAL_GATE_PASS_BOUNDED**.

Passing R8 authorizes only the next governance decision about merge/release. It is not itself a release.
