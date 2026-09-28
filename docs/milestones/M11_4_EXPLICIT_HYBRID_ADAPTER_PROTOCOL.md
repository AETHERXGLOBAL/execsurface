# M11.4 — Explicit Hybrid Capability Adapter Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — IMPLEMENTATION AUTHORIZED**

## Objective

Represent the bounded kernel-authority capabilities validated by M10 as an explicit, versioned, internal adapter without changing the public/default observer, without silent privilege, without a daemon, and without automatic fallback.

M11.4 is an authority-contract integration gate. It is **not** a public BPF backend launch and does not authorize `learn/check`, baseline reuse, Marketplace behavior, or automatic backend selection.

## Frozen evidence inherited from M10

M10 validated, in bounded controlled fixtures:

- file-open kernel object candidate identity from BPF LSM `file_open`, matching returned-fd `fstat(2)` truth;
- executable object candidate from BPF LSM plus distinct successful-exec confirmation from `sched_process_exec`;
- kernel-mediated connect destination candidate plus distinct success/failure completion from `sys_exit_connect`;
- real producer ring-buffer loss detection that forced evidence completeness and PASS authority false;
- boot-controlled/root environments with active BPF LSM can support the stronger path, while standard hosted/default-container environments cannot be assumed to do so.

M10 did **not** authorize public BPF PASS, backend auto-selection, baseline interchangeability, ptrace replacement, silent elevation, or privileged daemon installation.

## Fixed roles

- **Innovative Systems Architect** — expose the strongest validated M10 authority without coupling it to default product selection.
- **Anti-Deviation / Skeptical Reviewer** — reject any API that silently activates, silently elevates, silently falls back, or upgrades an unproved proposition.

## Dynamic specialists

Linux BPF/LSM capability detection, privilege boundaries, evidence authority, loss accounting, Rust API design, compatibility, and adversarial state-machine testing.

## Adapter state machine

The internal adapter has two selection states:

1. `Disabled` — the default.
2. `ExplicitResearchV1` — only entered by an explicit caller decision.

Requesting a hybrid evidence contract while disabled must fail explicitly.

The adapter may expose a contract only when all declared prerequisites for this bounded research contract are true:

- Linux;
- x86_64;
- active BPF LSM;
- readable kernel BTF;
- BPF operation permitted in the selected execution context;
- privilege requirement explicitly acknowledged by the caller;
- producer-loss accounting available;
- exec-success confirmation source available;
- connect-completion confirmation source available.

A missing prerequisite must return an explicit error. There is no automatic fallback to ptrace inside this adapter.

## Frozen proposition mapping

The M11.4 hybrid contract must remain conservative:

- `FileOpenObjectIdentity` -> `KernelObjectCandidate`.
  - Reason: M10.3D proved kernel-object grounding for the bounded file-open fixture but did not establish a general independent open-success confirmation contract.
- `ProcessExecSuccess` -> `KernelSuccessConfirmed`.
- `ProcessExecObjectIdentity` -> `KernelObjectSuccessBound`.
  - Reason: M10.4 correlated a kernel object candidate with distinct successful-exec confirmation.
- `NetworkConnectAttemptDestination` -> `KernelObjectCandidate`.
- `NetworkConnectSuccess` -> `KernelSuccessConfirmed`.
  - Reason: M10.5 separated LSM destination candidate from syscall completion and prevented failed-connect promotion.

All other propositions remain `Unsupported` in the M11.4 hybrid adapter unless a prior accepted M10 result directly proves them.

This adapter does not claim pathname equivalence, fd read/write lifecycle completeness, causal executable chains, universal protocol coverage, or portable deployment.

## Health rule

The adapter contract is authority-bearing only under explicit complete health. Any recorded producer loss, capability failure, ambiguity, or error remains non-PASS-eligible under the M11 health model.

M11.6 will separately validate runtime fail-closed behavior for integrated authority-bearing event classes; M11.4 only establishes the explicit adapter contract and selection semantics.

## Acceptance tests

1. default adapter selection is `Disabled`;
2. disabled selection cannot yield a hybrid contract;
3. every prerequisite is fail-closed when absent;
4. explicit research selection with all prerequisites yields the exact frozen proposition mapping above;
5. unsupported propositions remain unsupported;
6. hybrid and ptrace contracts are not silently declared equivalent;
7. no adapter API performs privilege escalation, daemon installation, backend fallback, or public backend mutation;
8. ptrace contract remains unchanged;
9. frozen baseline-v2 digest test remains green;
10. full workspace Clippy/tests remain green.

## Stop rule

Any implementation path that makes hybrid activation implicit, hides a missing privilege/capability, falls back to ptrace as if evidence-equivalent, or promotes an unproved proposition blocks M11.5.
