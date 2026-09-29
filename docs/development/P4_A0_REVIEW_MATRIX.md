# ExecSurface — P4-A0 Independent Review Matrix

Date: 2026-09-29
Protocol: `P4_BACKEND_AUTHORITY_PROTOCOL.md`
Contract: `P4_A0_PROPOSITION_INVENTORY.md`
Status: **PREREGISTERED REVIEW MATRIX — RESULTS NOT YET RECORDED**

The following review matrix is frozen before any A0 implementation or acceptance decision.

| Case | Attack / ambiguity | Required safe classification |
|---|---|---|
| A0-01 | pathname pointer observed, object changes before successful open identity is resolved | PATH.ACCESS_ATTEMPT may be supported; FILE.OPEN_OBJECT must not become direct solely from entry pathname |
| A0-02 | open attempt fails | PATH.ACCESS_ATTEMPT only; no successful FILE.OPEN_OBJECT |
| A0-03 | exec attempt fails | no EXEC.SUCCESS |
| A0-04 | clone relation flags unavailable/unreadable | FDTABLE.RELATION = unknown; affected FD attribution incomplete |
| A0-05 | explicit CLONE_FILES + FD close/reuse by sharing peer | FD attribution must follow lifecycle or become ambiguous/incomplete; never stale-direct |
| A0-06 | exec/de-threading changes task identity shape | causal lineage must remain evidence-bound or become ambiguous |
| A0-07 | resource/event loss | OBSERVER.HEALTH_LOSS positive; affected propositions cannot remain complete |
| A0-08 | same target with substituted actor | actor-sensitive proposition cannot inherit authority |
| A0-09 | same actor+target with substituted causal chain | causal proposition cannot inherit authority |
| A0-10 | backend supports exec but not file-object identity | EXEC.SUCCESS may be direct; FILE.OPEN_OBJECT remains unsupported/ambiguous; no global backend equivalence |
| A0-11 | unsupported backend emits no event | absence cannot become proof of no behavior |
| A0-12 | proposition record loses raw evidence/derivation reference | record cannot be direct+complete |

## Reviewer roles

- Innovation Scientist: attempts a smaller, more composable evidence contract without broadening claims.
- Anti-Drift Reviewer: checks every case against P2 semantics and prevents global-equivalence language.
- Independent Falsifier: attempts to produce false `direct+complete` records.
- Milestone Reviewer: checks the exact frozen files/SHAs and confirms no runtime/public path changes.

## Decision rule

A0 passes only if every row is representable without semantic contradiction and no review case admits a false `direct+complete` proposition.

Allowed decisions remain:
- `P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`
- `P4_A0_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A0_INCOMPLETE_CONTRACT`
