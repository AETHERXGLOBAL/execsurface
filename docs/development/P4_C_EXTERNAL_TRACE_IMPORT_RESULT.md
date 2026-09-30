# ExecSurface — P4-C External Trace Import Result

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P4_C_EXTERNAL_TRACE_IMPORT_PROTOCOL.md`

## Decision

`P4_C_EXTERNAL_IMPORT_PATH_ESTABLISHED_BOUNDED`

Boundary:

`RESEARCH_ONLY — NO_PUBLIC_INTEGRATION_OR_RELEASE_PROMOTION_AUTHORIZED`

This result establishes only that the pinned Tetragon JSON event schema can be imported through the ExecSurface proposition/authority contract for the preregistered `P4.EXEC.SUCCESS` experiment without backend-name, event-presence, or completeness laundering under the frozen corpus.

It does **not** establish global Tetragon/ptrace equivalence, complete-session authority, backend adoption, runtime dependency adoption, public semantics v3 integration, or release promotion.

## Accepted source and reproducible evidence

- accepted source SHA: `7db008ad64aad0782b7438ea8f225739ab942fc5`
- workflow: `P4 C external trace import gate`
- run: `36739233309`
- job: `109968900093`
- runner: Ubuntu 24.04 / Linux x86_64
- Rust: `1.90.0`
- evidence artifact ID: `11110450417`
- evidence artifact digest: `sha256:3aaaf91a7ea3f6dc28636bd864be7a227b3129545a39d885f305f30f7bcda043`
- artifact name: `p4-c-import-36739233309-1`

Pinned external schema identity:

- repository: `cilium/tetragon`
- source commit: `666efe6f91e3605ad58683ad226d759d9cf970ca`
- schema path: `api/v1/tetragon/events.proto`
- schema blob: `d6bd56769241da983f7e0042a54d5b3a81f40817`

## Frozen gate results

All mandatory conditions passed on the accepted source:

- immutable ExecSurface boundary check: PASS
- exact external schema identity check: PASS
- `rustfmt --check`: PASS
- `clippy -D warnings`: PASS
- frozen external-import falsification corpus: **14/14 PASS**
- P4-B cross-proposition falsification reproof: **12/12 PASS**
- backend-name anti-inflation reproof: **1/1 PASS**
- B1 live ptrace reproof: **10/10 PASS**
- B2 live ptrace reproof: **10/10 PASS**
- B3 live ptrace reproof: **10/10 PASS**
- Semantics v3 reproof: **7/7 PASS**
- public M11 fail-closed reproof: **6/6 PASS**
- `v0.1.0-alpha.4` and stable `v0.1` remained bound to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- no `crates/` modification was introduced after the accepted P4-B source by this gate

## What the experiment proved

Within the preregistered scope:

1. a qualifying imported `process_exec` event can be represented as proposition-scoped evidence without making the producer/backend name an authority source;
2. imported success evidence remains coarse `Incomplete` when complete session/transport evidence is not established;
3. imported evidence cannot satisfy a proof requirement demanding complete session scope;
4. `procFS` bootstrap, contradictory flags, truncation/error/unknown/miss markers, missing or mismatched causal identity, malformed executable identity, unsupported event types and schema substitution fail closed;
5. producer/profile field injection cannot upgrade authority or completeness;
6. canonical JSON key reordering preserves evidence identity;
7. raw evidence mutation changes both the raw-evidence digest and imported-record digest;
8. the external schema is cryptographically pinned by source commit and Git blob identity in CI.

## Negative/failure history retained

The following failed runs remain first-class evidence and were not deleted or rewritten:

### Run `36737957630`

- classification: formatting/static defect
- failed at `rustfmt --check`
- scientific corpus did not execute
- immutable-boundary and external-schema checks passed
- evidence artifact ID: `11107054641`

### Run `36738414887`

- classification: implementation/layout defect
- `rustfmt` passed
- `clippy -D warnings` rejected `ImportDecision` because of `large_enum_variant`
- correction: box only the large `Evidence` payload; no authority/completeness/test criterion change
- scientific corpus did not execute
- evidence artifact ID: `11108650624`
- artifact digest: `sha256:ec445b510bd7e99265b61988f188f71c9d7cf728efa8102dc5c228ea464da8d6`

### Run `36738772644`

- classification: test-fixture defect
- static gates passed
- frozen corpus result: **13/14 PASS**
- failing test: `parent_exec_id_mismatch_fails_closed`
- root cause: the fixture replacement mutated both compared parent identities, so it did not construct a mismatch
- correction: mutate only `process.parent_exec_id` with `replacen(..., 1)`; assertion, threshold and semantic rule unchanged
- evidence artifact ID: `11107833288`
- artifact digest: `sha256:c4378c869ebfab6b6961c9dc7f7119d68a9a3a9607788c22fe79760331683ea6`

## Scientific interpretation

The result supports a bounded interoperability claim: ExecSurface can ingest one pinned mature external trace schema through proposition-scoped, explicit-completeness semantics without treating external event presence or backend identity as permission or global authority.

The result does not justify adding Tetragon as a required backend. P4-B already established bounded ptrace success authority for the tested gaps, so external import remains an interoperability path rather than a backend-count objective.

## Promotion boundary

No public code, public alpha.4 semantics, stable Action tag, baseline v2 meaning, or default observer is promoted by this result.

Any public use requires a separate promotion/release gate with migration, compatibility, reproducibility and independent review evidence.
