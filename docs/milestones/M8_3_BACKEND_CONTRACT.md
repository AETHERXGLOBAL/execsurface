# M8.3 — Backend Capability, Completeness & Experimental libbpf Integration Contract

Date: 2026-09-26
Status: **M8.3a CLOSED / M8.3b CLOSED / M8.3c OPEN**
Tracking: #37
Parent architecture: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
M8.2 decision: `docs/milestones/M8_2_STACK_DECISION.md`
M8.3b evidence: `docs/milestones/M8_3B_PATH_RESOLUTION_BRIDGE.md`

## Objective

Freeze the machine-readable boundary that every observation backend must satisfy and connect the selected libbpf-rs collector without allowing partial eBPF evidence to become indistinguishable from complete ptrace observation.

M8.3 changes **no default backend**, **no public verdict meaning**, **no baseline compatibility rule**, and does not authorize eBPF-derived PASS.

## Governing invariants

1. Native ptrace remains the default and correctness reference.
2. libbpf-rs/libbpf is the selected **experimental** M8.3 implementation stack.
3. eBPF-derived evidence-equivalent PASS remains **NOT AUTHORIZED**.
4. Backend identity, supported capabilities, unsupported capabilities, privacy profile, and completeness are explicit data.
5. Supported and unsupported capability sets must be disjoint and together cover the declared capability universe.
6. Known loss, event-budget truncation, missing required capability, path-resolution failure, or collection error must never be represented as `COMPLETE`.
7. Normal ptrace installation must not acquire libbpf/native build prerequisites from M8.3.
8. Observing that an operation occurred is not equivalent to establishing its path identity.
9. The experimental eBPF path must never silently fall back to ptrace while retaining an eBPF identity.
10. `auto` backend selection is not authorized in M8.3.

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

## M8.3a — contract closure

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
- an experimental libbpf descriptor limited to semantics actually proved;
- a hard check that incomplete observations are not PASS-eligible.

The existing public `observe_command*` functions still select ptrace and still return the existing `Observation` model.

### M8.3a evidence

Initial contract head:

`400e808eff570592afe56668956aafa20f16df25`

CI run `36243547337` failed at format only before clippy/tests. The rustfmt difference was corrected without changing semantics.

Corrected contract head:

`44a57b3622930369334a18e642c738187a483c8b`

CI run `36243629555` — **SUCCESS**.

Anti-drift review then found that a single `ProcessExec` capability overclaimed M8.2 because occurrence did not establish path identity. The capability vocabulary was refined before collector integration.

Refined contract head:

`c0931922a8eaf65e58f8b8a1c6713c2e3312e521`

CI run `36243919035` — **SUCCESS**.

M8.3a is therefore **CLOSED / PROVED ON CI**.

## M8.3b — userspace path-resolution bridge closure

M8.2 numeric libbpf events proved exec occurrence and successful-open fd identity but not the path-bearing semantics required by the current raw model.

M8.3b tested a narrow userspace bridge:

- exec occurrence `(pid)` -> `/proc/<pid>/exe`;
- successful open `(pid, fd)` -> `/proc/<pid>/fd/<fd>`;
- unresolved lifecycle race -> explicit incomplete condition;
- no guessed path;
- no argv/environment/content capture.

### M8.3b decision

The bridge is **ACCEPTED ONLY AS A CONDITIONAL FAIL-CLOSED RESOLVER**.

Reference head:

`72ce6d144cfcc737e6af42a25629f04834481cdd`

Dedicated workflow run:

- `M8.3 Path Resolution Bridge` — `36244748914` — **SUCCESS**

Normal repository CI on the same head:

- `CI` — `36244748984` — **SUCCESS**

Observed proof markers:

- `M8_3_EXEC_HELD_RESOLUTION_PASS ... path=/usr/bin/sleep`
- `M8_3_EXEC_REAPED_RESOLUTION_UNAVAILABLE_PASS ...`
- `M8_3_OPEN_HELD_RESOLUTION_PASS ... path=/dev/null`
- `M8_3_OPEN_REAPED_RESOLUTION_UNAVAILABLE_PASS ...`
- `M8_3_PATH_BRIDGE_CONDITIONAL_FAIL_CLOSED_PASS`
- `M8_3_PATH_BRIDGE_PRIVACY_PASS`
- `M8_3_UNPRIVILEGED_DENIAL_PASS exit=1`

The result proves both sides of the contract: live object resolution can succeed accurately, while resolution after process/fd lifetime ends is unavailable and must remain incomplete.

### Preserved negative evidence

The first held-open shell fixture was unsuitable for deterministic evidence and timed out. It remains recorded in:

`docs/milestones/M8_3_NEGATIVE_001_SHELL_OPEN_FIXTURE.md`

That failure was not erased or rewritten as a successful run.

M8.3b is therefore **CLOSED / CONDITIONAL RESOLVER PROVED ON REFERENCE CI**.

## Experimental libbpf capability declaration after M8.3b

Unconditional capabilities remain limited to:

- process spawn / lineage metadata;
- process exec occurrence;
- successful open -> fd identity;
- explicit loss/truncation visibility.

Path identity is not promoted to an unconditional backend capability merely because the conditional `/proc` resolver succeeded in live cases.

M8.3c may emit a path-bearing raw event only when that individual event has a successful live resolution. If a required event cannot be resolved, collection must become incomplete.

Still unsupported as general product semantics:

- unconditional process exec path identity;
- unconditional successful-open path identity;
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

## Ptrace declaration

The ptrace reference descriptor remains based on already-established behavior and remains subject to existing observer warnings/completeness handling. M8.3 does not expand ptrace claims.

## M8.3c — real collector integration gate

The next subgate is to connect the selected libbpf-rs collector behind the backend contract as an explicit experimental path.

Acceptance requires:

1. explicit eBPF backend selection; ptrace remains default;
2. real libbpf collection, not a demo-only marker;
3. backend descriptor and collection completeness returned with the observation;
4. no silent ptrace fallback under eBPF identity;
5. path-bearing raw events emitted only after successful event-specific live resolution;
6. unresolved required path identity -> explicit incomplete collection;
7. explicit unsupported capability set retained;
8. unprivileged/attach/decode/lifecycle failures fail closed;
9. normal ptrace installation remains free of libbpf/native build prerequisites;
10. normal CI/distribution/action smoke remain green;
11. dedicated eBPF integration evidence succeeds on the declared reference host;
12. no claim of cross-backend parity or eBPF PASS authority.

## Labels

- M8.3a backend contract: **CLOSED / PROVED ON CI**
- occurrence-vs-path capability split: **PROVED ON CI**
- M8.3b userspace path bridge: **CLOSED / CONDITIONAL FAIL-CLOSED RESOLVER PROVED**
- failed shell open harness: **KILLED / PRESERVED NEGATIVE EVIDENCE**
- M8.3c real libbpf collector integration: **OPEN**
- cross-backend parity: **OPEN / M8.5**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace default/reference: **RETAINED**
