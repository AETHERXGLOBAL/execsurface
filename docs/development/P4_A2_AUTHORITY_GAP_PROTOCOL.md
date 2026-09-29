# ExecSurface — P4-A2 Authority-Gap Matrix Protocol

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Predecessor: `P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — MATRIX IMPLEMENTATION MAY START**

## Objective

Determine, proposition-by-proposition, what the accepted research-only ptrace raw-v2 adapter can actually prove, where it can prove only a bounded derivation or attempt, where authority is ambiguous/lost/unsupported, and which remaining gaps are concrete enough to justify investigating a second evidence backend.

A2 is a measurement/classification gate. It does not authorize a new collector/backend by itself.

## Fixed team

1. **Innovation Scientist / Systems Architect** — search for the smallest architecture that fills only evidence-backed proposition gaps and reuses mature external/native evidence sources where possible.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks global backend scores, capability inflation, absence-as-negative-proof, and post-hoc redefinition of propositions.
3. **Independent Falsifier / Red Team** — attacks each matrix row for false `direct+complete`, unsupported-as-absence, attempt-to-success laundering, identity laundering, causal substitution, and loss masking.
4. **Independent Milestone Reviewer** — verifies source SHA, frozen inputs, executable checks, matrix determinism, and allowed decision before A2 closure.

## Dynamic specialists

- Linux ptrace / syscall lifecycle specialist
- eBPF / BPF-LSM / kernel-hook specialist
- filesystem pathname-versus-object identity specialist
- process lineage / clone / fd-table specialist
- network connect semantics specialist
- formal semantics / proof-obligation specialist
- attestation / external-trace interoperability specialist
- CI reproducibility / evidence-engineering reviewer

## Frozen inputs

A2 must use only the already-frozen P4 contracts and accepted A1 evidence:

- `docs/development/P4_A0_PROPOSITION_INVENTORY.md`
- `docs/development/P4_A0_REVIEW_RESULT.md`
- `docs/development/P4_A1_PTRACE_ADAPTER_PROTOCOL.md`
- `docs/development/P4_A1_2_PTRACE_MAPPING_PROTOCOL.md`
- `docs/development/P4_A1_3_ADVERSARIAL_RESULT.md`
- `docs/development/P4_A1_3U_ATTEMPT_AUTHORITY_RESULT.md`
- `docs/development/P4_A1_4_PUBLIC_ANTIDRIFT_RESULT.md`
- canonical executable model: `experiments/p4-backend-authority`

No proposition may be reworded merely to improve ptrace coverage.

## Frozen proposition inventory

Exactly these ten product propositions are in scope:

1. `P4.PROC.CREATE_RELATION`
2. `P4.EXEC.SUCCESS`
3. `P4.PATH.ACCESS_ATTEMPT`
4. `P4.FILE.OPEN_OBJECT`
5. `P4.FD.IO_ATTRIBUTION`
6. `P4.FILE.RENAME_DELETE`
7. `P4.NET.CONNECT_DESTINATION`
8. `P4.CAUSAL.EXEC_LINEAGE`
9. `P4.OBSERVER.HEALTH_LOSS`
10. `P4.FDTABLE.RELATION`

## Required matrix fields

Each row must record, independently:

- proposition ID;
- raw-v2 evidence source(s);
- mapped proposition shape;
- best authority state actually justified by the accepted adapter:
  `direct | derived_bounded | attempt_only | ambiguous | lost | unsupported`;
- completeness dimensions satisfied;
- completeness dimensions not proven;
- identity basis;
- temporal binding;
- causal binding;
- explicit capability-gap / ambiguity reason codes;
- whether the proposition is usable for the frozen product question;
- whether a second backend could plausibly strengthen the proposition;
- what exact missing evidence would be required to strengthen it.

No row may inherit authority from another row merely because the same backend emitted both.

## Gap classes

A2 must classify each product proposition into exactly one decision-oriented gap class:

- `NO_GAP` — current ptrace evidence satisfies the frozen product proposition under the bounded declared scope;
- `REPRESENTATION_GAP` — the live observer can obtain relevant evidence but raw-v2 discards or cannot express a prerequisite;
- `OBSERVATION_GAP` — required evidence is not observed with sufficient authority by the current ptrace path;
- `SUCCESS_SEMANTICS_GAP` — an attempt is observed, but successful effect/result authority required by the product proposition is not established;
- `IDENTITY_GAP` — evidence exists but the required object/subject identity basis is weaker than the product proposition requires;
- `COMPLETENESS_GAP` — evidence may be authoritative locally but required session/lifecycle/resource/fd-table/causal completeness is not established;
- `NO_PRODUCT_REQUIREMENT` — reserved only if the A0 inventory itself explicitly makes the row descriptive rather than a product requirement. This class may not be invented during A2.

Multiple technical deficiencies may be recorded, but exactly one primary gap class must be selected by a deterministic precedence frozen below.

## Deterministic primary-gap precedence

When more than one deficiency applies, select the first applicable class in this order:

1. `REPRESENTATION_GAP` if accepted prior evidence shows the observer obtains a prerequisite that raw-v2 loses;
2. `OBSERVATION_GAP` if the prerequisite is not observed at the needed observation point;
3. `SUCCESS_SEMANTICS_GAP` if attempt evidence exists but success semantics are missing;
4. `IDENTITY_GAP` if success/effect evidence exists but identity grounding is insufficient;
5. `COMPLETENESS_GAP` if authority/identity exist but a required completeness dimension is unresolved;
6. `NO_GAP` only when all frozen product requirements are satisfied.

This precedence is classification-only and is not a severity ranking.

## Executable A2 audit requirements

A2 must add a research-only deterministic audit that consumes or reconstructs the accepted ptrace adapter contract and fails if any of the following occurs:

1. fewer or more than the ten frozen product proposition IDs are represented;
2. `P4.PATH.ACCESS_ATTEMPT` is not usable as explicit attempt authority;
3. rename/delete or connect attempt evidence is treated as successful product authority;
4. pathname attempt evidence satisfies `P4.FILE.OPEN_OBJECT`;
5. clone raw-v2 without retained flags becomes exact fd-table relation;
6. dependent FD IO remains `direct+complete` when fd-table relation is ambiguous;
7. unsupported/ambiguous/lost is interpreted as proof that behavior did not occur;
8. causal lineage is upgraded from bounded derivation to direct without new evidence;
9. backend name/profile alone changes authority;
10. matrix serialization or row order is nondeterministic.

The audit must include at least one positive and one negative/falsification fixture for each gap-bearing proposition class discovered from the frozen evidence.

## Evidence-source rule

A2 may use prior accepted live/research evidence to establish that a gap is representational versus observational, but must cite the exact prior gate/result. A2 must not infer live kernel capability from source-code possibility alone.

## Backend-candidate rule

A2 may name a **candidate evidence source** only after a concrete missing evidence requirement is written for a row. Candidate examples may include existing internal certificate evidence, BPF-LSM, tracepoints, Tetragon-compatible events, or imported attestations, but:

- a candidate name is not a recommendation;
- no backend is globally stronger;
- no implementation begins in A2;
- reuse/import is preferred over rebuilding a broad collector when it can satisfy the proposition contract;
- P4-B may prototype only the smallest evidence source that targets one or more established gaps.

## Falsification set

Independent review must try at minimum:

- failed open versus successful-open identity;
- pathname TOCTOU;
- failed exec versus successful exec;
- rename failure versus successful rename/delete;
- connect failure versus successful connect;
- clone with missing/unreadable flags;
- shared-FD close/dup/reuse dependency;
- actor substitution;
- causal-chain substitution;
- observer loss/resource truncation;
- backend/profile-name substitution;
- matrix row omission/duplication/order perturbation.

## Public boundary

A2 must not modify:

- public/raw-v2 schema or bytes;
- default observer behavior;
- public learn/check/baseline/policy semantics;
- `main`;
- `v0.1.0-alpha.4`;
- stable `@v0.1`;
- release artifacts.

## Allowed A2 decisions

- `P4_A2_NO_AUTHORITY_GAP_REQUIRING_NEW_BACKEND`
- `P4_A2_PROPOSITION_GAPS_ESTABLISHED_BOUNDED`
- `P4_A2_INCOMPLETE_FOR_BACKEND_DECISION`
- `P4_A2_FALSE_AUTHORITY_PATH_FOUND`

Only `P4_A2_PROPOSITION_GAPS_ESTABLISHED_BOUNDED` may authorize a P4-B prototype, and only for the explicitly established gap rows. It does not authorize public integration, release, or backend promotion.
