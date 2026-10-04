# P8-A3.1 — Typed Evidence Report Bridge Prototype

Date: 2026-10-04  
Tracking: #149  
Parent gate: #145  
Parent evidence: `P8-EXT-0004` / `probityai/agent-evidence-observer#41`  
Prototype PR: #150

## Final decision

`BRIDGE_PROTOTYPE_ACCEPTED_FOR_PUBLIC_API_DESIGN`

This authorizes **design work only** for a narrow public report surface. It does not authorize product integration, public exposure, release promotion, or P8 closeout.

## R0 — Internal-boundary freeze

The prototype is compiled only under `cfg(test)` and consumes the existing internal `BackendObservation` boundary.

No new acquisition path was added.

The prototype does not read or collect:

- file contents;
- argv/env;
- stdin;
- network payloads;
- arbitrary process memory;
- exact transferred byte counts beyond existing bounded semantics;
- before/after filesystem state roots.

Accepted result:

`R0_INTERNAL_BOUNDARY_FROZEN_NO_ACQUISITION_EXPANSION`

## R1 — Minimal deterministic report

The research report schema is:

`execsurface-research-typed-report-v0`

It exports only existing bounded facts:

- recognized backend/profile identity;
- implementation version;
- platform and architecture;
- metadata-only privacy profile;
- typed collection-health state;
- pass eligibility;
- warning codes;
- unsupported capability list;
- successful fd-write propositions with bounded guarantees;
- explicit non-claims.

Determinism is tested by serializing the same input twice and requiring byte-identical JSON.

Typed health includes:

- `complete`;
- `incomplete_loss`;
- `incomplete_limit`;
- `incomplete_capability`;
- `incomplete_ambiguity`;
- `error`.

Accepted result:

`R1_MINIMAL_DETERMINISTIC_TYPED_REPORT_PASS`

## R2 — Non-inflation attacks

The accepted prototype rejects or preserves the following boundaries:

1. incomplete/warning state cannot be laundered into complete/pass-eligible health;
2. backend naming cannot upgrade authority;
3. a raw fd effect without the declared fd-effect capability is rejected;
4. process/TID binding mismatch is rejected;
5. repeated fd effects remain distinct;
6. unknown/changed privacy profile is rejected;
7. forbidden state/byte fields are absent as JSON object keys;
8. explicit `does_not_assert` values are permitted and cannot be mistaken for asserted fields.

The report explicitly does **not** assert:

- exact transferred byte count;
- file contents;
- before/after state roots;
- custody;
- trusted time;
- signature as behavioral authority;
- causal source-code provenance.

Accepted result:

`R2_NON_INFLATION_MATRIX_PASS`

## R3 — Live native proof

The prototype invokes the actual internal native Linux x86_64 ptrace backend through `PTRACE_BACKEND.observe` in test scope.

A live shell workload writes a temporary target file.

The resulting `BackendObservation` is converted into the typed report without re-parsing serialized public raw-v2 JSON.

The accepted live test proves:

- collection health remains `complete`;
- the report remains pass-eligible;
- the target fd-write effect is present;
- its temporal guarantee is `successful_operation_result`.

Accepted run:

`37196488594`

Prototype corpus:

**10/10 PASS**

Accepted result:

`R3_LIVE_NATIVE_TYPED_REPORT_PASS_BOUNDED`

## Retained negative evidence

### Initial formatting-only failure

Run `37196376963` failed at `cargo fmt --check` before Clippy or the scientific corpus.

Classification:

`HARNESS_STATIC_GATE_FAILURE_ONLY`

### Initial scientific-corpus run

Run `37196421833` reached the prototype corpus:

- 9/10 tests passed;
- live native R3 passed;
- one test failed because its oracle searched the entire serialized JSON text for the substring `byte_count`.

The report correctly contained the explicit non-claim:

`does_not_assert: exact_transferred_byte_count`

The original oracle incorrectly treated that non-claim value as if it were a forbidden asserted field.

Correction:

- changed the oracle to recursively inspect **JSON object keys**;
- forbidden fields remain forbidden;
- explicit non-claim values remain allowed;
- no report-generation logic, authority rule, completeness rule, or test threshold was weakened.

Accepted rerun:

`37196488594` — **10/10 PASS**

## R4 — Compatibility and regressions

Accepted head before this documentation-only closeout:

`7798738f9ebd7a239223d83ac4e5d572ef0df0e9`

Accepted runs:

- typed report prototype `37196488594`: **SUCCESS**
- full CI `37196488568`: **SUCCESS**
- P9.3 Compatibility Contract `37196488586`: **SUCCESS**
- Adversarial Regression `37196488610`: **SUCCESS**
  - Ubuntu 22.04: PASS
  - Ubuntu 24.04: PASS

No public compatibility surface changed:

- raw observation schema v2 unchanged;
- baseline schema/digest unchanged;
- policy semantics unchanged;
- PASS/ERROR/REVIEW/BLOCK meanings unchanged;
- GitHub Action contract unchanged;
- public CLI unchanged;
- Linux x86_64 + native ptrace support boundary unchanged.

Accepted result:

`R4_COMPATIBILITY_AND_ADVERSARIAL_REGRESSION_PASS`

## R5 — Decision

The prototype establishes that a first-party typed report bridge can be produced at the internal backend boundary without adding data acquisition or changing Alpha.5 public semantics.

The external Probity integration establishes a concrete consumer need for this class of information.

Therefore the next authorized step is:

`BRIDGE_PROTOTYPE_ACCEPTED_FOR_PUBLIC_API_DESIGN`

### What this authorizes

A separate design gate may specify:

- exact public report schema;
- stability/versioning boundary;
- CLI/Action exposure options;
- privacy review;
- migration/compatibility rules;
- external-consumer usability test.

### What this does not authorize

It does not authorize:

- exposing the research report publicly now;
- adding a new stable CLI command;
- changing `observe` output;
- changing Action outputs;
- moving Semantics v3 wholesale into the public product;
- a vNext release;
- production-adoption claims;
- P8 A5 closeout.

P8 remains open because a qualifying current A1/A4 external execution/use record is still missing.
