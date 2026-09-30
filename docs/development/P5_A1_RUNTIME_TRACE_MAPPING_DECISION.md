# ExecSurface — P5-A1 Runtime Trace Mapping Decision

Date: 2026-09-30
Branch: `development/post-alpha4-behavioral-integrity`
Parent protocol: `docs/development/P5_ATTESTATION_PROVENANCE_PROTOCOL.md`
Gate protocol: `docs/development/P5_A1_RUNTIME_TRACE_MAPPING_GATE.md`

## Decision

`P5_RUNTIME_TRACE_MAPPING_ESTABLISHED_BOUNDED`

The bounded research mapping from ExecSurface verification evidence to in-toto Runtime Trace v0.1 survived the preregistered A1 gate. This decision does not authorize public integration, signing, release promotion, or any reinterpretation of public alpha.4 semantics.

## Accepted source and executable evidence

- accepted source SHA: `74dc06a3f5a091f2f70e5f73290adf37ef3f2f44`
- GitHub Actions run: `36745097898`
- job: `109989156382`
- evidence artifact: `11111718460`
- uploaded artifact SHA-256: `26d9265620d7cfc5cd06c79ff922df23574b20ba2ea38596e54317612164345d`
- pinned in-toto attestation source: `fd2609c16bcb0ac53443e2b4612977f997e8f9a5`
- accepted P5-A0 source used by the gate: `9ff34d9baaa01a90cf6d2cb95e1930dc4c011e11`
- P4 closeout source used by the gate: `320c865d3e81071a8214ad726dca7a587b0fc479`
- immutable public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

## Gate evidence

All required checks passed on the accepted source:

- frozen boundaries / preregistration: PASS
- pinned Runtime Trace standards snapshot: PASS
- `cargo fmt --check`: PASS
- `clippy -D warnings`: PASS
- P5-A1 Runtime Trace mapping corpus: **12/12 PASS**
- accepted P5-A0 standards-composition corpus: **18/18 PASS**
- P4 cross-proposition falsification: **12/12 PASS**
- P4 B1 live ptrace authority: **10/10 PASS**
- P4 B2 live ptrace authority: **10/10 PASS**
- P4 B3 live ptrace authority: **10/10 PASS**
- Semantics v3 regression: **7/7 PASS**
- public M11 shared-FD regression: **6/6 PASS**

The A1 corpus retained the required attacks on monitor identity, subject substitution, bounded trace truncation/cardinality, backend-name inflation, process-log mutation, command/host substitution, workflow/command/host replay, policy/config substitution, loss/incompleteness laundering, and deterministic rebinding.

## Retained negative / pre-closure evidence

The earlier run `36744377118` failed at `cargo fmt --check` before the scientific corpus executed. It remains retained as a formatting/static-gate failure, not a scientific counterexample.

- failed run: `36744377118`
- retained artifact: `11111332815`
- retained artifact SHA-256: `5952bf36c3662374f75b19f9465673780acca1ba346855c33c39a541829519bb`

The correction was rustfmt-equivalent only. No semantic assertion, threshold, verdict rule, authority rule, or adversarial fixture was weakened.

## Bounded interpretation

Established:

- ExecSurface can truthfully map the bounded monitored-operation identity and monitor/configuration material into Runtime Trace v0.1;
- ExecSurface-specific authority/completeness semantics remain separately bound rather than being implied by generic Runtime Trace fields;
- backend/profile names cannot upgrade semantic authority;
- observer loss or incompleteness cannot be laundered into PASS through the mapping;
- identical semantic inputs produce deterministic mapping identities within the tested model.

Not established:

- that a generic Runtime Trace consumer understands ExecSurface proposition authority;
- that Runtime Trace alone is a complete ExecSurface verification record;
- that a signature implies behavioral correctness;
- public-v2/v3 interchangeability;
- public product integration or release promotion.

## Next gate

Proceed to **P5-A2 — detailed SCAI v0.3 verification assertion + SVR v0.2 concise verification summary**, over the same subject identity, with a separately preregistered cross-binding/falsification corpus before execution.
