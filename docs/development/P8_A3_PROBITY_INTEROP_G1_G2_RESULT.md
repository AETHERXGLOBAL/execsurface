# P8-A3 Probity Interoperability Finding — G1/G2 Result

Date: 2026-10-04  
Tracking: #145  
External source: `probityai/agent-evidence-observer#41`  
ExecSurface target: `v0.1.0-alpha.5`  
Reproduction PR: #147  
Accepted reproduction run: `37194823782`  
Evidence artifact: `11299629771`  
Artifact digest: `sha256:6a85b156d2b48753c0b4e8f7ed8660dfad1fec427b4fc5b113e8096ab4d0a4dd`

## Decision

`P8_A3_INTEROPERABILITY_GAP_REPRODUCED_BOUNDED — G2_MAPPING_COMPLETE`

The external finding is real in the tested Alpha.5 raw-observation scope.

No product/schema change is authorized by this result.

## G1 — Exact Alpha.5 reproduction

GitHub-hosted Ubuntu 24.04 reproduced the finding against the immutable published Alpha.5 artifact.

Pinned identities:

- release source: `9e73b925d55557e33de1b0813995609aaefdc037`;
- release archive SHA-256: `6a27ee3245acec67fc153cca3f6715f4395eadcf8c6358b415ecc6a9347efdb7`;
- binary SHA-256: `11d1f70d3e6bd6526eff4889b90cba4a0ad95ce09d489643e7f3b703e455f646`;
- model source SHA-256: `26b9f431249a3cce574f27a1048699334d776d15ec1c0d8fe4e9f4271f92bb85`;
- ptrace source SHA-256: `afda9661804ae99dffd7116adc91cbc1672256da29ec0b29556f38a5702b6d03`.

Observed bounded result:

- raw schema version: `2`;
- backend: `linux-ptrace-metadata-v2`;
- target command completed successfully;
- one target `file_descriptor_access/write` event was retained;
- retained write-event keys were exactly:
  `event_type, fd, operation, path, sequence, tid`;
- no transferred-byte / syscall-result field was carried in the event;
- observation health was exposed through global `complete=true` and `warnings=[]`;
- no typed collection-health envelope was present.

Accepted result:

`P8_A3_G1_GAP_REPRODUCED_BOUNDED`

## G2 — Mapping to existing Semantics v3 research model

### 1. Typed collection health

External need:
a consumer needs to distinguish typed observation-health/completeness properties rather than treat one global Boolean as all semantic authority.

Current public Alpha.5:
- global `complete`;
- warning list;
- backend capabilities/limitations.

Research-only Semantics v3 already contains:
- `CompletenessDimension`;
- `CompletenessState::{Complete, Incomplete, Ambiguous, Unsupported, NotRequired}`;
- `ObserverHealthObserved`;
- fail-closed proof requirements over required completeness dimensions.

Classification:

`REPRESENTED_IN_RESEARCH_V3 — NOT EXPOSED_BY_PUBLIC_ALPHA5`

This is external evidence that the existing research direction addresses a real interoperability need. It is not evidence that the prototype is ready for promotion.

### 2. Successful file-write result semantics

External need:
the consumer must know that the retained fd-write event represents a successful positive-byte I/O rather than merely a pre-operation attempt.

Current public Alpha.5:
- the native backend emits fd read/write attribution only for successful positive-byte I/O;
- that result semantics is not explicit in the retained event fields themselves.

Research-only Semantics v3 already contains:
- `Proposition::FileFdEffectObserved`;
- `ObservationPoint::SyscallResultPostOperation`;
- `TemporalBinding::SuccessfulOperationResult`;
- `IdentityBasis::RuntimeFdPathCorrelated`;
- proposition-scoped proof guarantees.

Classification:

`SUCCESS_SEMANTICS_REPRESENTED_IN_RESEARCH_V3 — NUMERIC_BYTE_COUNT_NOT_REPRESENTED`

The external integration needed a nine-byte state statement for its own state-binding protocol. ExecSurface does **not** need to expose content bytes or a numeric byte count merely to communicate that the observed fd effect followed a successful positive-byte syscall result.

### 3. Before/after bytes and state roots

The Probity companion captures explicit before/after fixture bytes and derives a state root.

ExecSurface's accepted metadata-only privacy contract explicitly avoids file-content collection.

Classification:

`INTENTIONALLY_OUT_OF_CORE_SCOPE`

Do not solve this finding by adding default file-content capture, filesystem state roots, or Merkle-state semantics to ExecSurface.

### 4. Signature, commitment, custody and trusted time

The external companion creates same-operator Ed25519 commitments and explicitly states that they do not establish independent custody or trusted time.

ExecSurface P5 research already keeps cryptographic validity, provenance identity, semantic authority and observer completeness distinct.

Classification:

`OUTSIDE_THIS_PRODUCT_GATE / COMPOSITION_LAYER_CONCERN`

No general custody/signing subsystem is authorized in ExecSurface by this finding.

### 5. Causal source-code provenance

The Alpha.5 backend explicitly states that runtime fd-path evidence is not causal source-code provenance.

Classification:

`EXPLICITLY_NOT_CLAIMED`

No expansion is authorized.

## G2 design conclusion

The evidence currently favors testing a **report/evidence-layer typed bridge** rather than changing the raw schema immediately.

Candidate only:

`REPORT_LAYER_TYPED_HEALTH_BRIDGE`

The candidate would aim to expose proposition-scoped guarantees such as `successful_operation_result` plus typed health/completeness derived under explicit proof rules, while preserving:

- raw schema-v2 evidence unchanged for Alpha.5 compatibility;
- metadata-only privacy;
- baseline/policy separation;
- fail-closed incompleteness;
- no content capture;
- no general state/custody system.

## Not authorized yet

The candidate remains unapproved until G3 adversarial tests show that raw-v2 facts can be mapped into typed guarantees without authority inflation or completeness laundering.

Next gate: **G3 failure-first adversarial mapping matrix**.\n\nG3 implementation is tracked in PR #147 and the dedicated `p8_a3_typed_bridge_redteam` test target.
