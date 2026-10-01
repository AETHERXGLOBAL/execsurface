# ExecSurface — Post-alpha.4 Semantics v3 Rework Protocol

Date: 2026-10-01
Tracking: #115, #116
Status: PREREGISTERED — REWORK ONLY / NO PUBLIC PROMOTION

## Frozen source and public boundary

- broken promotion candidate under repair: `5079a990b924d8ccd7ac6414f8a9a2571b54e240`
- rework branch: `integration/post-alpha4-semantics-v3-rework`
- public alpha.4 source remains immutable: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action remains `AETHERXGLOBAL/execsurface@v0.1`
- P8 remains external-evidence pending; this work cannot satisfy P8.

## Fixed review roles

1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team
4. Independent Critical-Milestone Reviewer

Dynamic specialists: PL/formal semantics, Rust type/API design, evidence authority, adversarial verification, schema migration/backward compatibility.

## Counterevidence that must be repaired

Issue #116 reproduced three material false-admission paths:

- D01 — an unbound generic proof requirement can be satisfied by the wrong proposition/subject;
- D02 — an explicit invalidating ambiguity can coexist with a claimed `Complete` dimension and remain admissible;
- D03 — an empty/default proof requirement can admit an arbitrary v3 record.

No threshold, acceptance criterion, or semantic weakness may be relaxed to obtain PASS.

## Rework design constraints

### R1 — proposition-bound proof contracts

A semantic proof requirement must carry the exact proposition/subject it is intended to prove. Admission must reject proposition mismatch before guarantee/completeness evaluation.

The original D01 test fixture cannot remain byte-for-byte source-compatible with a principled fix because its `ProofRequirement` contains no expected proposition/subject at all. It is therefore retained as historical counterevidence, while the rework test must preserve the **same attack and same kill criterion** using the corrected bound requirement API. No implicit global state, call-order binding, backend-name inference, or other hidden binding mechanism is allowed.

### R2 — ambiguity/completeness consistency

Known invalidating ambiguity must dominate a conflicting `Complete` claim for the affected completeness dimension. At minimum, `object_identity_conflict` invalidates `ObjectIdentity`. Unknown ambiguity codes must not silently grant stronger authority.

### R3 — non-vacuous requirements

An empty/default `ProofRequirement` is never sufficient semantic proof. Admission must fail closed if the requirement has no bound proposition or has no meaningful guarantee/completeness obligations.

### R4 — fail-closed schema boundary

- schema v2 remains v2;
- schema v3 remains explicit and side-by-side;
- unsupported schema fails closed;
- v2↔v3 remains `INCOMPARABLE_SCHEMA` by default;
- backend name/profile never raises authority;
- no automatic migration or baseline reinterpretation.

## Frozen falsification rule

The rework is rejected if any reproducible path allows:

- wrong proposition/subject to satisfy a bound contract;
- invalidating ambiguity to remain PASS-admissible for an affected required dimension;
- empty/default requirement to admit evidence;
- authority to be raised by backend/profile metadata alone;
- existing fail-closed M11/M12/P4/P5 behavior to regress;
- public alpha.4/stable `v0.1` identity or semantics to move.

## Execution order

1. repair the proof-contract API with the smallest explicit semantic change;
2. run the D01–D03 attack family with D01 expressed through the corrected explicit binding API and D02/D03 unchanged in semantic intent;
3. rerun the prior A1 9-test corpus;
4. rerun surviving P3/P4/P5/M11/M12 adversarial controls;
5. recheck immutable alpha.4/stable-tag anchors;
6. only then decide whether A1 may be requalified.

All failed runs and harness corrections remain retained. A PASS here is internal bounded requalification only, never external validation, adoption, endorsement, production readiness, or P8 closure.
