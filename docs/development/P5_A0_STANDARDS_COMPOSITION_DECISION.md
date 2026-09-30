# P5-A0 — Standards Composition Decision

Status: **SCIENTIFICALLY CLOSED — PASS BOUNDED / RESEARCH-ONLY**

Decision:

`P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`

Accepted source:

- commit: `9ff34d9baaa01a90cf6d2cb95e1930dc4c011e11`
- workflow run: `36743125582`
- evidence artifact: `11110384934`
- artifact SHA-256: `b769b01ccd6051d23fbf998d37c30d9d7073f39505301f4cec5fabbdfada3595`

## What was established

For the bounded P5-A0 fixture, existing standard predicates are sufficient to represent the required ExecSurface verification graph without inventing a new in-toto predicate:

- in-toto Statement v1;
- Runtime Trace v0.1 for the bounded runtime observation representation;
- SCAI v0.3 for detailed ExecSurface-specific verification conditions and evidence binding;
- SVR v0.2 for the concise verification result over the same subject;
- optional exact reference to SLSA provenance/v1 by predicate type, statement digest and subject digest.

This establishes representational sufficiency only. It does **not** establish generic-consumer understanding of ExecSurface authority semantics, signature trust, SLSA level, public-release readiness or multi-backend equivalence.

## Frozen A0 gate result

The preregistered corpus remained exactly 18 tests. The accepted run passed:

- P5-A0: `18/18`;
- P4-C import corpus: `14/14`;
- P4 cross-proposition falsification: `12/12`;
- P4-B1 live ptrace authority: `10/10`;
- P4-B2 live ptrace authority: `10/10`;
- P4-B3 live ptrace authority: `10/10`;
- Semantics v3 reproof: `7/7`;
- public M11 fail-closed reproof: `6/6`;
- rustfmt and clippy `-D warnings`: PASS;
- immutable alpha.4 / stable `v0.1` boundaries: PASS;
- pinned in-toto standards snapshot identity: PASS.

The strengthened test #16 explicitly exercises post-build substitution of the embedded SLSA provenance digest and predicate type and requires verification to fail closed. The test count, threshold and semantic success criteria were not changed.

## Retained negative / pre-close evidence

No failed or weaker run is erased or reclassified as a scientific PASS.

- run `36741656523`: static rustfmt failure before scientific corpus; artifact `11109703891`, SHA-256 `d41c33fec5f8a0c12cd6b8f2bfa764772ffb34afd1d73e6dab8264931bb01090`;
- run `36742010054`: remaining test-file rustfmt failure before corpus; artifact `11109699513`, SHA-256 `3ae560b05261f4f294b4a27243cf5ecf3bb2a7a7f5681adf0c507331295e2dcc`;
- run `36742159476`: compile/type defect in bounded subject comparison before corpus;
- run `36742784570`: full green predecessor run, but retained as **PRE-CLOSURE / TEST-HARNESS-ADEQUACY-DEFECT** because the original test #16 proved identity sensitivity rather than post-build fail-closed substitution. Artifact `11111355567`, SHA-256 `5b0fb24720d68b918ddeeb1269e356ce4299c984b9c8c2bdd593eeac5b9aad63`.

The accepted run is `36743125582`, after the adequacy correction.

## Scientific boundaries

This decision does not authorize:

- a new ExecSurface predicate;
- signing or attestation authenticity as semantic authority;
- treating a SLSA provenance reference as runtime behavioral truth;
- assigning a SLSA level;
- promotion into public crates, public observer behavior, v2 semantics, `v0.1.0-alpha.4`, or stable `@v0.1`;
- interpreting backend/profile names as authority;
- treating incomplete, ambiguous, lost or unsupported evidence as PASS.

The research-only implementation remains isolated under `experiments/p5-attestation-provenance/`.

## Next gate

Proceed to **P5-A1 — Runtime Trace Mapping Gate** under the already frozen P5 protocol. A1 must prove the mapping itself, including fail-closed cross-binding of monitor identity, exact command/job context, monitor-log mutation, observer loss/truncation and replay. A0 is not reopened unless a new counterexample invalidates this decision.
