# ExecSurface — P4-A0 Independent Contract Review Result

Date: 2026-09-29
Parent program: #100
Protocol: `P4_BACKEND_AUTHORITY_PROTOCOL.md`
Inventory: `P4_A0_PROPOSITION_INVENTORY.md`
Review matrix: `P4_A0_REVIEW_MATRIX.md`
Decision protocol: `P4_A0_DECISION_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED**

## Review team

Fixed roles:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team
- Independent Milestone Reviewer

Dynamic specialists used for this contract review:
- PL / evidence semantics
- Linux ptrace and clone/fd lifecycle
- causal provenance / process identity
- filesystem object identity
- schema/versioning
- Rust adapter architecture

## Review result

Decision:
`P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`

The frozen A0 contract is sufficient to begin a **research-only ptrace reference adapter**. This decision validates the proposition/authority/completeness contract structure, not any adapter implementation and not any backend equivalence claim.

## Falsification matrix disposition

### A0-01 pathname TOCTOU
Safe classification exists:
- pathname observation may support `P4.PATH.ACCESS_ATTEMPT` as `attempt_only`;
- it cannot establish `P4.FILE.OPEN_OBJECT` as `direct` solely from the entry pathname.

No false object-identity promotion is permitted by the contract.

### A0-02 failed open
Safe classification exists:
- attempted pathname access can remain visible;
- successful-open proposition is not satisfied.

### A0-03 failed exec
Safe classification exists:
- failed attempt cannot satisfy `P4.EXEC.SUCCESS`.

### A0-04 clone flags unavailable/unreadable
Safe classification exists:
- `P4.FDTABLE.RELATION` becomes unknown/ambiguous;
- affected FD attribution cannot remain complete.

### A0-05 CLONE_FILES with close/reuse by peer
Safe classification exists:
- FD attribution requires lifecycle evidence across the shared table;
- stale binding cannot remain `direct+complete` merely because an earlier FD identity existed.

### A0-06 exec/de-threading identity shape
Safe classification exists:
- causal-lineage proof must survive the identity transition explicitly;
- otherwise the proposition becomes ambiguous rather than inferred from task shape.

### A0-07 observer/resource loss
Safe classification exists:
- loss is first-class proposition evidence;
- affected propositions must be downgraded to incomplete/lost.

### A0-08 actor substitution
Safe classification exists:
- same target does not transfer actor-sensitive authority.

### A0-09 causal-chain substitution
Safe classification exists:
- same actor+target does not transfer `P4.CAUSAL.EXEC_LINEAGE` authority without the declared chain evidence.

### A0-10 partial backend support
Safe classification exists:
- one proposition may be `direct+complete` while another remains unsupported/ambiguous;
- there is no global backend-equivalence state in the contract.

### A0-11 unsupported backend emits no event
Safe classification exists:
- unsupported/no-event cannot be interpreted as proof of absence.

### A0-12 evidence reference missing
Safe classification exists:
- a proposition record without the required raw-evidence or bounded-derivation reference cannot satisfy the frozen requirements for `direct+complete`.

## Anti-drift findings

The review found no contract path that authorizes:
- global ptrace/eBPF/external-backend equivalence;
- pathname-attempt to object-identity promotion;
- frequency/recurrence to authority promotion;
- unsupported/no-event to proof-of-absence conversion;
- public use of the research shared-FD certificate;
- silent whole-session completeness inference from partial proposition states.

## Innovation finding

The strongest architectural direction is to make **the proposition record, not the collector event, the backend boundary**. This keeps collection replaceable while preserving explicit authority/completeness and creates a clean path for P5 attestation later.

The adapter should therefore expose evidence records shaped around semantic claims rather than trying to normalize all backend events into one generic syscall schema.

## Boundary

A0 authorizes only:
**P4-A1 — research-only ptrace reference-adapter implementation.**

A0 does not authorize:
- a new BPF/eBPF backend;
- external trace import;
- public API/schema migration;
- public learn/check behavior changes;
- global backend interchangeability.

P4-A1 must preserve public alpha.4 behavior and must be falsified against the A0 matrix plus current M11/pathname identity limitations before any A2 authority-gap decision.
