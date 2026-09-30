# P5-A1 — Runtime Trace Mapping Gate

Status: **PREREGISTERED — EXACTLY 12 TESTS / RESEARCH-ONLY**

Parent protocol: `docs/development/P5_ATTESTATION_PROVENANCE_PROTOCOL.md`

Accepted prerequisite:

- P5-A0 accepted source: `9ff34d9baaa01a90cf6d2cb95e1930dc4c011e11`
- P5-A0 run: `36743125582`
- decision: `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`

## Question

Can ExecSurface map its bounded runtime evidence into in-toto Runtime Trace v0.1 while preserving exact subject/job/command/policy/evidence identity and while keeping ExecSurface proposition authority/completeness outside generic Runtime Trace semantics?

## Frozen hypothesis

A bounded mapping is acceptable only if Runtime Trace is treated as a content-bound observation representation and cannot independently create, upgrade or preserve ExecSurface PASS authority when its bound semantic context is changed, truncated, replayed or substituted.

No backend name, monitor name, signature, provenance reference or generic Runtime Trace field may create ExecSurface authority.

## Mapping contract

For this bounded prototype:

1. Statement type remains `https://in-toto.io/Statement/v1`.
2. Predicate type remains `https://in-toto.io/attestation/runtime-trace/v0.1`.
3. `monitor.type` identifies **ExecSurface**. It does not identify ptrace, Tetragon or another collection backend as semantic authority.
4. `monitor.configSource` is the exact content-addressed policy ResourceDescriptor used by verification.
5. `monitor.tracePolicy` is producer-defined metadata that must exactly bind the ExecSurface observer profile, authority label, capability-state digest, completeness-state digest and evidence digest. These fields mirror already-established semantics; Runtime Trace does not interpret them as generic authority.
6. `monitoredProcess.type` identifies the bounded ExecSurface command-operation type.
7. `monitoredProcess.hostID` is cross-bound to an explicit `hostIdentity` in the detailed SCAI conditions. It is not inferred from workflow name or backend name.
8. `monitoredProcess.event` exactly equals the SCAI-bound command identity.
9. The bounded A1 fixture contains exactly one producer-defined process log record. That record must bind command identity, evidence digest and observer profile to the SCAI conditions.
10. Subject identity is identical across Runtime Trace, SCAI and SVR.
11. Workflow identity remains an explicit SCAI-bound ResourceDescriptor and must be checked by a consumer-context verifier when a caller supplies the expected workflow. It is not silently conflated with host identity.
12. Observer loss/incompleteness remains an ExecSurface semantic state in SCAI. A syntactically intact Runtime Trace can never upgrade it to PASS.

The exact-one-process-record constraint is an **ExecSurface bounded-prototype invariant**, not a claim that Runtime Trace v0.1 generically requires exactly one process record.

## Frozen 12-test falsification corpus

Exactly **12 tests** are required:

1. a complete bounded fixture maps to the frozen Statement/Runtime Trace types, exact ExecSurface monitor identity, exact process type, exact host/command binding, exact policy config source and one bound process record;
2. missing or substituted `monitor.type` fails closed;
3. Runtime Trace subject substitution fails closed against SCAI/SVR;
4. empty/truncated bounded process log fails closed;
5. backend/profile-name substitution cannot upgrade authority/completeness/verdict, and a Runtime-Trace profile mismatch against SCAI fails closed;
6. process-log mutation of command, evidence digest or observer profile fails closed;
7. `monitoredProcess.event` command substitution fails closed;
8. `monitoredProcess.hostID` substitution fails closed against the SCAI `hostIdentity` binding;
9. replay under a different expected workflow or command is rejected by the consumer-context verifier;
10. policy/configSource substitution or policy-cardinality corruption fails closed without panic;
11. observer-loss/incomplete evidence cannot become PASS even when the Runtime Trace structure itself is otherwise intact;
12. identical bounded mapping is deterministic, and changing the mapped runtime observation changes the Runtime Trace digest and therefore the SCAI evidence binding.

Acceptance: **12/12 PASS**, with no weakened fixture, no post-result threshold change and A0 remaining `18/18`.

## Required implementation behavior

The A1 verifier must explicitly validate cross-field mapping. Digest mismatch alone is not sufficient as the only defense because a malicious producer could recompute a self-consistent but semantically substituted bundle.

At minimum, it must validate:

- exact monitor type;
- exact bounded process type;
- exact policy/config source binding;
- exact trace-policy bindings to SCAI conditions;
- exact host and command cross-binding;
- exact process-log bindings;
- subject equality;
- consumer-supplied expected workflow/command/host context where requested;
- fail-closed policy cardinality before indexing a policy element.

## Decision space

If all 12 tests pass with A0 and critical P4 predecessors re-proved:

`P5_RUNTIME_TRACE_MAPPING_ESTABLISHED_BOUNDED`

If a representation/cross-binding gap remains:

`P5_RUNTIME_TRACE_MAPPING_GAP_RETAINED`

A failing A1 does not authorize a new predicate automatically. Any new-predicate question remains separately gated by retained representation evidence.

## Boundaries

A1 does not authorize:

- public integration;
- signing;
- a new predicate;
- SLSA level claims;
- generic-consumer understanding of ExecSurface authority;
- backend-name authority;
- promotion of incomplete/ambiguous/lost/unsupported evidence to PASS.
