# ExecSurface — P4-A0 Proposition Inventory / Authority Contract Freeze

Date: 2026-09-29
Parent program: #100
Protocol: `docs/development/P4_BACKEND_AUTHORITY_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **FROZEN FOR A0 REVIEW — IMPLEMENTATION NOT YET AUTHORIZED**

## Purpose

Freeze the minimum proposition vocabulary that every future evidence backend adapter must use. A backend is evaluated proposition-by-proposition; no global backend-equivalence claim exists.

## Authority vocabulary

Each proposition record must declare exactly one authority state:

- `direct` — backend evidence directly establishes the proposition under the declared identity and lifecycle assumptions;
- `derived_bounded` — proposition follows from explicit evidence plus a declared bounded derivation rule;
- `attempt_only` — evidence proves an attempted operation/path reference, not successful object-level effect;
- `unsupported` — backend does not support the proposition;
- `ambiguous` — evidence exists but cannot uniquely establish the proposition;
- `lost` — observer/resource loss invalidates the proposition for the affected scope.

`unsupported`, `ambiguous`, and `lost` are never equivalent to absence of behavior.

## Completeness vocabulary

For each proposition scope:
- `complete` — all declared prerequisites are satisfied and no relevant loss/ambiguity is present;
- `incomplete` — at least one declared prerequisite is missing or failed;
- `not_applicable` — proposition is outside the backend's declared capability and must pair with `unsupported` authority.

Whole-session completeness must not be inferred by averaging proposition states.

## Frozen proposition inventory

### P4.PROC.CREATE_RELATION
Question: did a declared parent create a declared child process/thread relation?

Minimum identity:
- parent execution identity;
- child execution/process identity;
- creation mechanism when observable;
- causal event binding.

Ptrace reference expectation:
- direct when the creation event is causally paired;
- ambiguous if causal pairing is lost;
- lost on observer/resource loss.

### P4.EXEC.SUCCESS
Question: did a process successfully transition to a specific executable identity?

Minimum evidence:
- process identity;
- successful exec transition;
- executable identity at the supported resolution level;
- causal lineage position.

Failed exec attempts MUST NOT satisfy this proposition.

### P4.PATH.ACCESS_ATTEMPT
Question: did an actor attempt an operation against a pathname/reference?

Minimum evidence:
- actor identity;
- operation;
- pathname/reference captured at observation time;
- causal position where available.

This is `attempt_only` by default and MUST NOT be promoted to successful object identity solely from a pathname pointer.

### P4.FILE.OPEN_OBJECT
Question: did an actor successfully open a file object that can be bound to a post-open FD/object identity?

Minimum evidence:
- actor identity;
- successful syscall/result;
- FD identity;
- post-open object/path resolution evidence at the backend's declared level;
- fd-table relation/completeness prerequisites.

Pathname-at-entry alone is insufficient.

### P4.FD.IO_ATTRIBUTION
Question: can a covered read/write be attributed to the correct currently bound FD object?

Minimum evidence:
- actor/process identity;
- FD number;
- FD lifecycle state;
- open/dup/close/reuse lineage;
- shared-fd-table relation where concurrency exists.

Unknown shared-FD relation => ambiguous/incomplete.

### P4.FILE.RENAME_DELETE
Question: did a rename/delete effect occur for the declared target identity at the backend's supported resolution?

Minimum evidence:
- actor identity;
- successful effect result;
- source/target identities as applicable;
- declared path/object authority distinction.

Attempted and successful operations remain distinct.

### P4.NET.CONNECT_DESTINATION
Question: did an actor successfully initiate/establish a covered outbound connect to the declared destination identity?

Minimum evidence:
- actor identity;
- socket/destination identity;
- success/result semantics;
- address-family/endpoint normalization contract.

Attempted connect and successful connection must not be conflated.

### P4.CAUSAL.EXEC_LINEAGE
Question: is the execution/effect causally attributable to the declared ancestor chain?

Minimum evidence:
- explicit process creation/exec lineage;
- no missing causal edge in the claimed chain;
- stable identity rules across exec/de-threading cases.

A matching actor name/path without chain proof is insufficient.

### P4.OBSERVER.HEALTH_LOSS
Question: is there observer loss, truncation, dropped events, resource exhaustion, decode failure or other evidence invalidating a proposition scope?

This proposition is itself first-class evidence. Positive loss/health failure MUST downgrade affected propositions rather than be discarded as telemetry.

### P4.FDTABLE.RELATION
Question: for a creation transition, is the child FD table known to be shared, independently copied, or unknown?

States:
- `shared` only with explicit `CLONE_FILES` evidence;
- `independent_copy` only with explicit clear `CLONE_FILES` evidence or separately justified fork/vfork semantics;
- `unknown` when flags/evidence are unavailable, unreadable or causally unpaired.

`CLONE_THREAD`, TID/TGID shape, workload behavior or frequency cannot substitute for missing `CLONE_FILES` evidence.

## Cross-cutting record requirements

Every adapter-produced proposition record must include:
- proposition ID;
- backend/observer ID and version/schema identity;
- subject identity;
- object/effect identity as required;
- authority state;
- completeness state;
- raw evidence reference/digest or bounded derivation reference;
- causal binding reference where required;
- loss/ambiguity reason when not complete;
- deterministic serialization identity.

## Explicit non-equivalences frozen at A0

- pathname attempt != successful object identity;
- actor path/family match != causal-lineage proof;
- event presence != completeness;
- no event != proof of no behavior when proposition is unsupported/incomplete/lost;
- backend support for one proposition != backend equivalence for another proposition;
- research certificate != public v2 semantic authority;
- recurrence/frequency != authorization or authority.

## A0 falsification questions

The independent reviewer must try to break the contract with at least:
1. pathname TOCTOU;
2. failed open versus successful-open identity;
3. failed exec versus successful exec;
4. shared-FD reuse under known shared relation;
5. clone relation with missing/unreadable flags;
6. exec/de-threading causal identity;
7. observer event/resource loss;
8. actor substitution with same target;
9. causal-chain substitution with same actor/target;
10. backend claims one supported proposition while another remains unsupported.

Any case that can produce a false `direct+complete` proposition requires contract revision before P4-A1.

## A0 acceptance condition

P4-A0 may close only if:
- all ten propositions have explicit identity/authority/completeness contracts;
- the falsifier cannot create a declared false `direct+complete` state under the frozen rules;
- unsupported/ambiguous/lost states remain first-class;
- no public/runtime behavior changes;
- public alpha.4 regressions remain untouched.

Allowed A0 decisions:
- `P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`
- `P4_A0_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A0_INCOMPLETE_CONTRACT`

Only the first authorizes P4-A1 ptrace reference-adapter implementation.
