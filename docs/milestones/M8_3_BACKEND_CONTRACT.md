# M8.3a — Backend Capability & Completeness Contract

Date: 2026-09-26
Status: **IMPLEMENTATION GATE — CONTRACT FREEZE**
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

## Capability universe

M8.3 uses a typed capability vocabulary that is deliberately more granular than the earlier prose list:

- process spawn / lineage;
- process exec;
- process exit;
- pathname access intent;
- successful open -> fd identity;
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
- process exec occurrence metadata;
- successful open -> returned fd metadata;
- explicit loss/truncation visibility.

It does **not** yet establish product-grade semantics for:

- process exit;
- pathname access intent/path identity;
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

## Acceptance for M8.3a

M8.3a closes only when code proves:

- a stable backend identifier exists for ptrace and experimental libbpf;
- the capability vocabulary is typed;
- each descriptor partitions the full capability universe without overlap or omission;
- completeness state is explicit at the backend handoff;
- existing `observe_command*` callers continue to use ptrace with the same return type and behavior;
- no libbpf dependency is added to the normal workspace install path;
- repository CI remains green.

## Next gate — M8.3b

Connect the selected libbpf-rs collector behind this contract as an explicit experimental path. It must return its real descriptor and collection-health state; it may not fabricate unsupported observations or silently fall back to ptrace under the eBPF identity.

## Labels

- M8.3a contract: **OPEN UNTIL CI EVIDENCE**
- libbpf collector integration: **NOT STARTED IN THIS SUBGATE**
- cross-backend parity: **OPEN / M8.5**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace default/reference: **RETAINED**
