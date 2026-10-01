# M11.2 — ptrace Pathname Semantics Hardening Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — IMPLEMENTATION AUTHORIZED**

## Objective

Make the M10.1 PATH-TOCTOU result executable as an internal invariant: pathname bytes copied from tracee userspace at syscall entry are request/intent metadata, not kernel-object identity.

## Fixed roles

- **Innovative Systems Architect** — preserve useful ptrace portability while exposing stronger evidence semantics cleanly.
- **Anti-Deviation / Skeptical Reviewer** — reject any bridge that re-labels entry paths as kernel-object authority or erases the M10.1 counterexample.

## Dynamic specialists

Linux ptrace/syscall semantics, TOCTOU, Rust API design, authority-model integration, backward compatibility, canonicalization, baseline/digest compatibility, and adversarial testing.

## Frozen facts

M10.1 demonstrated a bounded PATH-TOCTOU counterexample in which the pathname copied by ptrace at syscall entry could differ from the object actually resolved by the kernel. The public raw v2 schema nevertheless must remain unchanged during this gate.

Therefore M11.2 must add an internal bridge rather than silently rewriting current serialized events.

## Required invariants

1. `ProcessExecAttemptPath` from ptrace remains `ArgumentObserved`.
2. `FilePathAccessIntent` from ptrace remains `ArgumentObserved`.
3. `NetworkConnectAttemptDestination` from ptrace remains `ArgumentObserved`.
4. `ProcessExecObjectIdentity` remains unsupported by the current ptrace contract.
5. `FileOpenObjectIdentity` may remain lifecycle-derived but cannot satisfy a `KernelObjectSuccessBound` requirement.
6. A caller requesting exact kernel-object-bound authority for the entry pathname must receive an explicit rejection.
7. Existing public/default `Observation` JSON and baseline-v2 digest behavior remain unchanged.

## Acceptance tests

- exact-authority guard accepts only an exact supported proposition/authority pair;
- ptrace entry-path guard rejects `KernelObjectCandidate` and `KernelObjectSuccessBound` requirements;
- unsupported exec object identity fails explicitly;
- bridge tests connect the existing ptrace backend descriptor to the M11 authority contract;
- full M11 CI remains green, including frozen baseline digest test and full workspace tests.

## Stop rule

Any path by which a legacy ptrace path capability can be silently interpreted as kernel-object proof blocks M11.3.
