# ExecSurface — P4 Backend Adapter / Authority Architecture Protocol

Date: 2026-09-29
Parent program: #100
Predecessor: P3 `P3_EPHEMERAL_ONLY_VALUE_ESTABLISHED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — P4-A0 CONTRACT FREEZE**

## Objective

Separate ExecSurface semantic/product logic from collection implementation without pretending that different observers are equivalent.

The P4 goal is not "add more collectors". It is to define and falsify an adapter contract where each backend can support only the propositions for which it has explicit evidence, authority and completeness semantics.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — seek the smallest architecture that makes backend authority composable without turning ExecSurface into a collector clone.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks backend-count goals, parity theater, silent authority upgrades and claim inflation.
3. **Independent Falsifier / Red Team** — constructs proposition-equivalence, loss, identity, causal-lineage and backend-mismatch counterexamples.
4. **Independent Milestone Reviewer** — verifies frozen contracts, exact source/evidence, negative results and promotion boundaries.

## Dynamic specialists for P4-A0/A1

- programming-languages / evidence-semantics specialist
- Linux ptrace specialist
- Linux kernel / BPF-LSM specialist
- causal provenance / process identity specialist
- filesystem object-identity specialist
- network endpoint identity specialist
- schema/versioning engineer
- Rust API/trait design specialist
- reproducibility / CI evidence engineer

External-trace specialists are added only when P4 reaches the external-import gate.

## Immutable boundaries

- public `v0.1.0-alpha.4` and stable `@v0.1` remain unchanged;
- public/default ptrace behavior remains the correctness reference until a separate promotion gate;
- no backend is described as equivalent to ptrace or to another backend globally;
- equivalence can exist only per proposition and only after evidence;
- unsupported, ambiguous, incomplete and lost evidence cannot silently become supported/complete;
- no broad host daemon or event collector is built for competitive parity;
- no eBPF/BPF-LSM PASS/learn/check authority promotion from P4 architecture work alone;
- raw backend evidence and adapter-produced evidence remain distinguishable;
- adapter failure must fail closed for affected propositions;
- P2 Semantics-v3 authority/completeness concepts are reused rather than replaced.

## P4-A0 — proposition inventory and adapter contract freeze

Before implementation, freeze the minimum proposition inventory already relevant to ExecSurface:

1. process creation relation;
2. successful executable transition;
3. pathname access attempt;
4. successful-open file-object identity where available;
5. covered FD read/write attribution;
6. rename/delete effect;
7. outbound connect destination;
8. causal execution lineage;
9. observer loss/health state;
10. clone/fd-table relationship evidence needed for completeness.

For each proposition define:
- canonical proposition identifier;
- subject/object identity requirements;
- evidence payload minimum;
- authority class;
- completeness prerequisites;
- explicit unsupported/ambiguous/lost states;
- causal binding requirements;
- backend-specific proof obligations.

Adapter output MUST NOT be a bare event stream. It must be proposition-scoped evidence records.

## P4-A1 — ptrace reference adapter

Implement a research-only adapter over existing ptrace evidence without changing public observation bytes or verdict behavior.

Acceptance:
- adapter can represent the frozen proposition inventory;
- existing ptrace limitations remain explicit;
- pathname attempts are never upgraded to object identity;
- shared-FD/clone certainty uses only proven internal evidence and remains research-scoped;
- unsupported propositions are emitted as unsupported rather than absent-success;
- deterministic serialization under identical evidence;
- public regression suite unchanged.

Required falsification:
- pathname TOCTOU counterexample;
- shared-FD ambiguous case;
- incomplete/loss case;
- failed exec versus successful exec;
- failed open versus successful-open identity;
- causal-lineage substitution;
- unknown proposition must not certify complete.

## P4-A2 — authority-gap matrix

Using the ptrace reference adapter, produce a factual matrix of where ptrace authority is insufficient for specific propositions.

No new backend is authorized unless at least one concrete proposition gap is demonstrated and the proposed backend can plausibly strengthen that proposition.

Allowed A2 outputs:
- `NO_AUTHORITY_GAP_REQUIRING_NEW_BACKEND`
- `PROPOSITION_GAPS_ESTABLISHED_BOUNDED`
- `INCOMPLETE_FOR_BACKEND_DECISION`

## P4-B — proposition-scoped native kernel/BPF-LSM research adapter

Only if A2 establishes a concrete gap, prototype the smallest native backend needed for the selected proposition(s).

No generic syscall/event coverage target. No full-host daemon.

Each selected proposition requires:
- preregistered authority claim;
- same-effect identity mapping;
- loss/health semantics;
- adversarial comparison against ptrace reference evidence;
- explicit non-equivalence for all unproved propositions.

## P4-C — external trace import experiment

Only after the internal adapter contract is stable, assess whether a mature external trace source can be mapped without semantic laundering. Candidate sources include Tetragon or cicd-sensor only where licensing/schema/access permits.

This gate is import/interoperability research, not dependency adoption.

## P4 decision outcomes

- `P4_PTRACE_ADAPTER_CONTRACT_ESTABLISHED_BOUNDED`
- `P4_PROPOSITION_SCOPED_MULTI_BACKEND_PATH_ESTABLISHED`
- `P4_EXTERNAL_IMPORT_PATH_ESTABLISHED_BOUNDED`
- `P4_BACKEND_EQUIVALENCE_NOT_ESTABLISHED`
- `P4_NO_NEW_BACKEND_VALUE`
- `P4_INCOMPLETE_EVIDENCE`

No P4 positive outcome alone authorizes public integration or release.

## First execution step

Execute **P4-A0 only**:
1. audit P2 semantics-v3 artifacts and current ptrace model;
2. freeze the proposition inventory and evidence contract;
3. run independent red-team review on the contract;
4. only then authorize P4-A1 implementation.
