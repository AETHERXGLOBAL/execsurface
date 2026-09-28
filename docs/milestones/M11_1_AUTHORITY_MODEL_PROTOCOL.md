# M11.1 — Authority-Aware Internal Evidence Model Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — IMPLEMENTATION AUTHORIZED**

## Objective

Implement the M11.0 authority vocabulary as an internal-only Rust model without changing the current public observation schema, canonical schema, baseline v2 serialization/digest, CLI behavior, backend selection, Marketplace behavior, or release artifacts.

## Fixed review roles

- **Innovative Systems Architect** — seek a model that can represent both portable ptrace evidence and stronger managed kernel-hook evidence without falsely collapsing their meanings.
- **Anti-Deviation / Skeptical Reviewer** — reject any implementation that hides the M10 counterexamples, silently strengthens ptrace evidence, or changes current public compatibility.

## Dynamic specialists

Rust type/API design, evidence semantics, baseline/digest compatibility, ptrace capability modeling, formal invariants, backward compatibility, privacy, and adversarial test design.

## Frozen implementation surface

A new internal crate, `execsurface-authority`, may define:

- `EvidenceProposition`;
- `EvidenceAuthority`;
- `EvidenceContractVersion`;
- `EvidenceHealth` including `IncompleteAmbiguity`;
- `ComparabilityRelation`;
- `KernelCapabilityMode`;
- `CapabilityFingerprint`;
- a total ptrace-v2 proposition/authority mapping.

The crate must not serialize into the public v2 baseline by default in this gate.

## Frozen ptrace-v2 authority rules

At minimum:

- syscall-entry pathname metadata -> `ArgumentObserved`;
- network destination read from syscall arguments -> `ArgumentObserved`;
- fd read/write and fd lifecycle reconstruction -> `DerivedLifecycleModel`;
- successful-operation facts derived from ptrace entry/exit/lifecycle bookkeeping -> at most `DerivedLifecycleModel`;
- kernel-object identity propositions must not be represented as `KernelObjectSuccessBound` merely because a pathname or later procfs fd resolution exists;
- unsupported propositions remain explicitly `Unsupported`.

## Health invariants

1. only `Complete` is PASS-eligible;
2. `IncompleteLoss` is not PASS-eligible;
3. `IncompleteLimit` is not PASS-eligible;
4. `IncompleteCapability` is not PASS-eligible;
5. `IncompleteAmbiguity` is not PASS-eligible;
6. `Error` is not PASS-eligible.

## Totality invariant

Every value in `EvidenceProposition::ALL` must have exactly one authority classification in every accepted capability fingerprint. Missing or duplicate proposition classification is a gate failure.

## Compatibility checks

The accepted gate must run the full locked workspace test suite and therefore preserve existing baseline serialization/digest tests. No public/default schema version or release version may change.

The following files are explicitly outside M11.1 mutation authority except where required to wire the new internal crate into the research workspace/tests:

- public CLI behavior;
- `schemas/execsurface-lock-v1.schema.json`;
- `schemas/execsurface-lock-v2.schema.json`;
- `action.yml`;
- `action/release-tag.txt`;
- release tags;
- `main`.

## Acceptance criteria

M11.1 closes only if:

1. the internal authority crate compiles with `#![forbid(unsafe_code)]`;
2. the proposition universe is exhaustive and deterministic;
3. ptrace-v2 mapping is total;
4. `file_path_access_intent` is `ArgumentObserved`;
5. `fd_read_effect` and `fd_write_effect` are `DerivedLifecycleModel`;
6. kernel-object identity is not silently claimed for ptrace pathname evidence;
7. `IncompleteAmbiguity.pass_eligible() == false`;
8. invalid/incomplete fingerprints fail validation;
9. `cargo fmt --all -- --check` passes;
10. `cargo clippy --locked --workspace --all-targets -- -D warnings` passes;
11. `cargo test --locked --workspace --all-targets` passes;
12. lockfile remains stable during the accepted run;
13. no public product/release mutation occurs.

## Stop rule

Any failing invariant blocks M11.2. The failure must be preserved and fixed without weakening the criterion.
