# ExecSurface — P4 Backend Adapter / Authority Architecture Closeout

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P4_BACKEND_AUTHORITY_PROTOCOL.md`

## Status

`SCIENTIFICALLY CLOSED — BOUNDED RESEARCH-ONLY`

P4 closes without public integration or release promotion.

## Protocol-conformant outcomes

The evidence supports the following P4 outcomes within their stated scope:

- `P4_PTRACE_ADAPTER_CONTRACT_ESTABLISHED_BOUNDED`
- `P4_EXTERNAL_IMPORT_PATH_ESTABLISHED_BOUNDED`
- `P4_BACKEND_EQUIVALENCE_NOT_ESTABLISHED`

For the three A2 success-authority gaps actually tested, the evidence also supports the narrower operational conclusion that **no second backend is justified by those gaps**. This is not a claim that no future proposition can ever justify another backend.

`P4_PROPOSITION_SCOPED_MULTI_BACKEND_PATH_ESTABLISHED` is **not** claimed: no second collection backend received success authority for the tested propositions.

## What P4 established

### A — authority architecture

P4 separated semantic authority from collection-backend identity:

- proposition-scoped evidence rather than bare event presence;
- explicit authority and completeness;
- deterministic evidence/proof identity;
- unsupported, ambiguous, pending, failed and lost evidence remain distinct from success;
- backend/profile names cannot upgrade authority;
- representation gaps do not automatically justify another collector.

### B — ptrace success-authority requalification

The three A2 gaps were requalified under bounded research-only ptrace experiments:

1. `P4.FILE.OPEN_OBJECT`
2. `P4.FILE.RENAME_DELETE`
3. `P4.NET.CONNECT_DESTINATION`

B1, B2 and B3 each passed their mandatory live Linux x86_64 corpus and model/falsification gates. Cross-proposition falsification then passed **12/12**, including replay, substitution, FD reuse, proposition confusion, attempt/pending/failure laundering, observer loss, causal substitution and deterministic identity attacks.

Bounded P4-B decision:

`P4_B_PTRACE_SUCCESS_AUTHORITY_ESTABLISHED_BOUNDED`

Boundary:

`NO_SECOND_BACKEND_JUSTIFIED_BY_THE_TESTED_P4_B_GAPS`

### C — external trace interoperability

P4-C demonstrated a schema-level import path from one pinned mature external source without treating producer identity or event presence as global authority.

Accepted P4-C evidence:

- source SHA: `7db008ad64aad0782b7438ea8f225739ab942fc5`
- run: `36739233309`
- job: `109968900093`
- artifact: `11110450417`
- artifact digest: `sha256:3aaaf91a7ea3f6dc28636bd864be7a227b3129545a39d885f305f30f7bcda043`
- frozen import corpus: **14/14 PASS**
- P4-B cross reproof: **12/12 PASS**
- B1/B2/B3 live reproof: **10/10 / 10/10 / 10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11: **6/6 PASS**

Decision:

`P4_C_EXTERNAL_IMPORT_PATH_ESTABLISHED_BOUNDED`

The external-import result remains incomplete at session/transport level unless stronger evidence proves those dimensions. It does not establish Tetragon/ptrace equivalence or authorize Tetragon as a required backend.

## Negative evidence retained

P4 closeout does not erase failed runs or counterexamples. Retained evidence includes, among other items:

- P1/eBPF negative/incomplete evidence that prevented unjustified backend promotion;
- B3 zombie-process lint failure;
- B3 loopback `ETIMEDOUT` 9/10 run;
- B3 test-adequacy defect and fixture byte-order correction;
- P4-B rustfmt-only failure;
- P4-C rustfmt-only failure;
- P4-C `large_enum_variant` clippy failure;
- P4-C 13/14 test-fixture failure where the mismatch fixture accidentally mutated both identities.

Each correction remained bounded to the demonstrated defect and did not weaken acceptance criteria.

## Immutable/public boundary reverified

P4 does not modify or promote:

- public release `v0.1.0-alpha.4`;
- immutable release source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable Action `AETHERXGLOBAL/execsurface@v0.1`;
- public default observer;
- raw/canonical/baseline v2 meaning;
- historical evidence.

No merge to `main` is authorized by this closeout.

## Anti-drift conclusion

P4's useful result is not “more collectors.” It is a bounded authority architecture in which evidence sources remain interchangeable only where proposition-level proof exists.

The next scientific phase is P5 Runtime Attestation / Provenance Interoperability. P5 must reuse existing ecosystem standards wherever they fit and must not invent a new predicate merely to create project-specific surface area.
