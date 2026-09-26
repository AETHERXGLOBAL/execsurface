# M8.3a — Backend Capability & Completeness Contract

Date: 2026-09-26
Status: **CLOSED / ACCEPTED**
Tracking: #34
Parent architecture: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
M8.2 decision: `docs/milestones/M8_2_STACK_DECISION.md`

## Objective

Freeze the machine-readable boundary that every observation backend must satisfy before the selected libbpf-rs collector is connected to the product path.

The purpose is to prevent a partial eBPF collector from becoming indistinguishable from a complete ptrace observation.

M8.3a changes **no default backend**, **no public verdict meaning**, and **no baseline compatibility rule**.

## Governing invariants

1. Native ptrace remains the default and correctness reference.
2. libbpf-rs/libbpf is the selected **experimental** M8.3 implementation stack.
3. eBPF-derived evidence-equivalent PASS remains **NOT AUTHORIZED**.
4. Backend identity, supported capabilities, unsupported capabilities, privacy profile, and completeness are explicit data.
5. Supported and unsupported capability sets must be disjoint and together cover the declared capability universe.
6. Known loss, event-budget truncation, missing required capability, or collection error must never be represented as `COMPLETE`.
7. Normal ptrace installation must not acquire libbpf/native build prerequisites from this contract work.
8. Observing that an operation occurred is not equivalent to establishing its path identity.

## Capability universe

M8.3 uses a typed capability vocabulary that separates occurrence from path identity where the evidence differs:

- process spawn / lineage;
- process exec occurrence;
- process exec path identity;
- process exit;
- pathname access intent;
- successful open -> fd identity;
- successful open path identity;
- fd-attributed read/write effects;
- fd duplication / close lifecycle;
- fork/shared descriptor-table inheritance;
- close-on-exec tracking;
- rename/delete effects;
- network connect destination;
- trace-time relative-path semantics;
- causal executable chain;
- loss/truncation visibility.

A backend must explicitly classify every capability as supported or unsupported for the declared backend contract version.

## Completeness states

The internal backend handoff distinguishes:

- `Complete`
- `IncompleteLoss`
- `IncompleteLimit`
- `IncompleteCapability`
- `Error`

Only `Complete` can ever be considered for ordinary downstream PASS eligibility, and cross-backend PASS remains separately prohibited until M8.5 parity is accepted.

## Initial experimental libbpf capability declaration

The M8.2 proof supports only the following minimum M8.3 starting claims:

- process spawn / lineage metadata;
- process exec **occurrence** metadata;
- successful open -> returned fd metadata;
- explicit loss/truncation visibility.

It does **not** yet establish:

- process exec path identity;
- successful-open path identity;
- process exit;
- pathname access intent;
- fd-attributed read/write effects;
- fd duplication/close lifecycle;
- shared fd-table inheritance;
- close-on-exec;
- rename/delete;
- network connect destination;
- trace-time relative path semantics;
- causal executable chain.

Those classes remain explicit unsupported capabilities until separately implemented and proved.

## Ptrace declaration

The ptrace reference descriptor is mapped from already-established behavior and remains subject to the existing observer warnings/completeness flag. M8.3a does not expand its semantic claims.

Its descriptor distinguishes and declares both exec occurrence and exec path identity, and both successful-open fd identity and open path identity, because the existing ptrace implementation already supplies path-bearing raw events under its established evidence contract.

## Implementation

`crates/execsurface-observe/src/lib.rs` contains:

- typed `ObservationCapability` values;
- a total `ALL_OBSERVATION_CAPABILITIES` universe;
- separate `ProcessExecOccurrence` / `ProcessExecPathIdentity` capabilities;
- separate `SuccessfulOpenFdIdentity` / `OpenPathIdentity` capabilities;
- `BackendDescriptor` with backend identity, implementation version, platform, architecture, kernel release, privacy profile, supported set, and unsupported set;
- a validator that rejects duplicate, overlapping, or incomplete capability declarations;
- explicit `CollectionCompleteness` states;
- an internal `BackendObservation` envelope;
- a ptrace descriptor preserving the existing backend identity;
- an experimental libbpf descriptor limited to semantics actually proved by M8.2;
- a hard check that incomplete observations are not PASS-eligible.

The existing public `observe_command*` functions still select ptrace and still return the existing `Observation` model.

## CI evidence

Initial implementation head:

`400e808eff570592afe56668956aafa20f16df25`

CI run `36243547337` failed at **format only** before clippy/tests. The rustfmt diff was applied without semantic changes.

Corrected initial contract head:

`44a57b3622930369334a18e642c738187a483c8b`

CI run `36243629555` — **SUCCESS**.

Anti-drift review then identified that a single `ProcessExec` capability overclaimed the M8.2 evidence because occurrence did not establish path identity. The vocabulary was refined before collector integration.

Refined contract head:

`c0931922a8eaf65e58f8b8a1c6713c2e3312e521`

CI run `36243919035` — **SUCCESS**.

Verified on the refined contract:

- format — PASS;
- clippy — PASS;
- tests — PASS;
- lockfile integrity — PASS.

The contract work added no libbpf dependency to the normal workspace install path.

## M8.3b semantic blocker

The M8.2 eBPF feasibility record intentionally persisted numeric metadata only. That is sufficient to prove an exec occurrence and a successful `openat` returned fd, but it is **not yet sufficient to populate the existing ExecSurface raw model faithfully**:

- existing `RawEventKind::ProcessExec` requires an executable path;
- existing file semantics require path identity/attribution, not merely an fd number.

M8.3b must not fabricate these values or silently claim semantic equivalence.

The next experiment tests a userspace metadata-resolution bridge:

- exec occurrence `(pid)` -> promptly resolve `/proc/<pid>/exe`;
- successful open `(pid, fd)` -> promptly resolve `/proc/<pid>/fd/<fd>`;
- resolution failure/race -> explicit incomplete state, never fabricated evidence;
- kernel-side event schema remains numeric/metadata-only;
- no argv/environment/content capture;
- no license change merely to obtain GPL-restricted kernel helper access.

The experiment must include deliberately short-lived/replaced executable and fd cases. A positive held-open case alone is not sufficient to prove path identity semantics.

## Next gate — M8.3b

Prove or kill the userspace metadata-resolution bridge, then connect the selected libbpf-rs collector behind the M8.3 contract only if the required semantics are reproducible and fail closed.

## Labels

- M8.3a contract: **CLOSED / PROVED ON CI**
- refined occurrence-vs-path capability split: **PROVED ON CI**
- libbpf collector integration: **OPEN / M8.3b**
- userspace metadata-resolution bridge: **OPEN / NEXT EXPERIMENT**
- cross-backend parity: **OPEN / M8.5**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace default/reference: **RETAINED**
