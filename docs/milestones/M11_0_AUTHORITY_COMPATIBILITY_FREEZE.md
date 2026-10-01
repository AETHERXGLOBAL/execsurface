# M11.0 — Authority / Compatibility Freeze

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — ARCHITECTURE FROZEN FOR IMPLEMENTATION**

## Purpose

Freeze the next-version evidence-authority and compatibility contract before runtime implementation.

M11.0 exists to prevent a technically stronger observer from silently changing baseline meaning, serialized schemas, digest identity, PASS eligibility, or existing public behavior.

## Review team

### Fixed roles

- **Innovative Systems Architect** — design the strongest evidence contract that can support portable ptrace and stronger managed kernel-hook evidence without collapsing their meanings.
- **Anti-Deviation / Skeptical Reviewer** — reject any design that hides M10 counterexamples, rewrites baseline meaning, equates capability with authority, or lets incomplete evidence retain PASS authority.

### Dynamic specialists

Rust API/schema evolution, baseline/digest compatibility, canonicalization, Linux ptrace semantics, BPF-LSM/tracepoints, evidence logic, backward compatibility, Marketplace/GitHub Actions, privacy, and red-team review.

## Current code facts

The current code already contains useful boundaries that M11 MUST preserve:

1. `ObservationCapability` distinguishes event/capability classes.
2. `BackendDescriptor` declares supported vs unsupported capabilities and validates a total/disjoint partition.
3. `CollectionCompleteness` is distinct from the `Observation` payload and only `Complete` is pass-eligible at the internal handoff.
4. The public observer API still returns the existing `Observation` schema and selects ptrace implicitly.
5. Raw observation schema is currently version `2`.
6. Canonical surface schema is currently version `2` and normalization profile version `3`.
7. Baseline lock schema is currently version `2` and digest format version `2`.
8. `BaselinePayload` includes observer identity/capabilities/limitations inside the digested payload.

These facts mean silent changes to observer identity, capabilities, canonical effects, or baseline payload semantics can change digest/comparability behavior even when the JSON shape remains parseable.

## Core architectural split

M11 freezes the following as separate concepts.

### A. Observation source

Where/how evidence was collected.

Examples:

- `linux-ptrace-metadata-v2`;
- future explicit hybrid kernel authority backend;
- future platform-specific backend.

Source identity does **not** by itself determine proposition authority.

### B. Proposition

The exact fact being asserted.

Initial proposition vocabulary:

- `process_spawn_occurrence`;
- `process_exec_attempt_path`;
- `process_exec_success`;
- `process_exec_object_identity`;
- `file_path_access_intent`;
- `file_open_success`;
- `file_open_object_identity`;
- `fd_read_effect`;
- `fd_write_effect`;
- `fd_lifecycle`;
- `rename_delete_effect`;
- `network_connect_attempt_destination`;
- `network_connect_success`;
- `trace_relative_path`;
- `causal_executable_chain`.

A single event may support more than one proposition, but authority is evaluated per proposition.

### C. Evidence authority class

Frozen internal vocabulary for M11 implementation:

- `ArgumentObserved` — userspace syscall argument/metadata observed at an instrumentation boundary; may describe intent but is not kernel-object identity.
- `KernelObjectCandidate` — kernel-mediated object/destination identity observed before final operation success is known.
- `KernelSuccessConfirmed` — kernel lifecycle/syscall completion confirms the operation succeeded.
- `KernelObjectSuccessBound` — object/destination identity and successful completion are both established and correlated for the proposition.
- `DerivedLifecycleModel` — proposition inferred from maintained observer state/lifecycle bookkeeping rather than one authoritative kernel object callback.
- `Unsupported` — backend cannot support the proposition with the required semantics.

Authority classes are **not a global ranking**. Their meaning is proposition-specific.

Examples fixed by M10:

- ptrace entry-copied open pathname: `ArgumentObserved`, not `KernelObjectSuccessBound`;
- BPF-LSM `file_open` object identity after accepted security hook semantics in the tested fixture: kernel-object evidence for file-open object proposition;
- BPF-LSM exec candidate alone: `KernelObjectCandidate`;
- `sched_process_exec` success confirmation correlated with candidate: supports `KernelObjectSuccessBound` for bounded exec proposition;
- BPF-LSM `socket_connect` destination alone: `KernelObjectCandidate`;
- `sys_exit_connect == 0` correlated with destination: supports `KernelObjectSuccessBound` for bounded blocking connect proposition;
- current ptrace fd read/write attribution: `DerivedLifecycleModel`, with M10.2 proving current completeness can be false under shared-fd race.

### D. Collection health/completeness

Frozen health states:

- `Complete`;
- `IncompleteLoss`;
- `IncompleteLimit`;
- `IncompleteCapability`;
- `IncompleteAmbiguity`;
- `Error`.

Only `Complete` may be PASS-eligible.

`IncompleteAmbiguity` is added by M11 to represent cases such as the M10.2 shared-fd race where the observer cannot prove its lifecycle attribution remained complete even if no transport loss counter fired.

### E. Capability fingerprint

A capability fingerprint is a deterministic description of the evidence contract actually available in one observation session.

The initial M11 fingerprint MUST include at least:

- backend/source ID;
- backend implementation contract version;
- platform + architecture;
- privacy profile;
- supported proposition -> authority-class mapping;
- unsupported propositions;
- collection-health/loss-accounting contract version;
- relevant kernel capability mode for explicit hybrid operation.

Kernel release may be recorded diagnostically, but MUST NOT be assumed by itself to prove proposition comparability.

### F. Baseline comparability

Baseline equality and evidence comparability are separate concepts.

M11 freezes the rule:

> Two observations/baselines are comparable only when every proposition used by canonicalization/policy has an accepted comparability relation between their evidence contracts.

No backend-wide statement such as `ptrace == hybrid` is allowed.

Comparability states:

- `EquivalentForProposition`;
- `OneWayRefinement`;
- `NotComparable`;
- `Unknown`.

Default is `Unknown` / fail closed.

## Versioning and migration rules

### Rule V1 — current public schemas remain frozen by default

Until an explicit migration gate authorizes otherwise:

- raw observation schema remains `2`;
- canonical surface schema remains `2`;
- normalization profile remains `3`;
- baseline lock schema remains `2`;
- digest format remains `2`.

### Rule V2 — authority envelope starts internal

M11.1 MUST introduce authority/capability metadata outside the current serialized v2 baseline by default.

The existing public/default `learn/check` path MUST continue producing the same v2 baseline semantics unless an explicit opt-in next-version format is selected later.

### Rule V3 — no silent observer identity mutation in v2 baseline

Because observer metadata is part of the digested `BaselinePayload`, the default v2 path MUST NOT silently rename the observer, rewrite capabilities, or add authority fields into the digested payload.

Any such change requires an explicit versioned migration decision.

### Rule V4 — no silent canonical strengthening

A stronger observation source MUST NOT silently replace a weaker proposition in canonicalization if doing so can alter canonical effects, diff output, baseline digest, or policy result.

### Rule V5 — explicit opt-in for hybrid capability

Future hybrid mode must be explicit. Unsupported environment/capability must return a clear non-success state; it must not silently fall back while claiming the same evidence authority.

### Rule V6 — existing baselines remain readable/verifiable

All valid v2 baselines created by the current public release remain parseable and digest-verifiable throughout M11.

### Rule V7 — no cross-backend baseline reuse by source name alone

Backend IDs do not establish comparability. Only proposition-level rules may do so.

## PASS eligibility invariants

The following are frozen invariants for M11 implementation:

1. `health != Complete => pass_eligible == false`.
2. any real producer/transport loss => `IncompleteLoss`.
3. known unresolved lifecycle ambiguity affecting an authority-bearing proposition => `IncompleteAmbiguity`.
4. unsupported required proposition => `IncompleteCapability` or explicit unsupported error; never silent PASS.
5. an attempt proposition cannot satisfy a success proposition.
6. a pathname argument cannot satisfy a kernel-object-identity proposition.
7. failed exec/connect/open MUST NOT be promoted to successful-operation evidence.
8. cross-backend comparison with `Unknown`/`NotComparable` proposition relation MUST fail closed before PASS.
9. no public backend selection may silently request elevated privilege or install a persistent privileged component.

## Privacy invariants

M11 preserves the current metadata-only privacy boundary unless a later explicit gate changes it.

In particular:

- no argv/envp content capture;
- no file-content capture;
- no socket payload capture;
- no secret-value capture as a requirement for authority;
- stronger kernel evidence should use object metadata/identity, not content inspection.

## M11.1 implementation target

M11.1 is authorized to add an **internal-only authority envelope** with types equivalent to:

- `EvidenceProposition`;
- `EvidenceAuthority`;
- `EvidenceContractVersion`;
- `CapabilityFingerprint`;
- `EvidenceHealth` including `IncompleteAmbiguity`;
- proposition-level comparability relation.

The public/default `Observation`, canonical surface, baseline v2 JSON, and digest format must remain unchanged in M11.1.

## M11.1 acceptance criteria

M11.1 closes only if all are true:

1. new authority vocabulary compiles and has exhaustive tests;
2. current ptrace descriptor maps every proposition to exactly one supported authority or unsupported state;
3. the mapping explicitly marks ptrace syscall-entry path metadata as `ArgumentObserved`;
4. fd read/write is represented as `DerivedLifecycleModel`, not kernel-object proof;
5. `IncompleteAmbiguity` is not PASS-eligible;
6. default public observation return schema is unchanged;
7. existing baseline v2 fixture serialization/digest tests remain unchanged/passing;
8. no `main`, tag, Marketplace, or public release change occurs.

## Decision

**M11.0 CLOSED.**

Architecture and compatibility rules are frozen. Authorized next gate: **M11.1 — Authority-aware internal evidence model**.
