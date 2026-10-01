# M11.4 — Explicit Hybrid Capability Adapter Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — PASS**

## Classification

`M11_4_EXPLICIT_HYBRID_AUTHORITY_ADAPTER_ACCEPTED`

## Accepted evidence

Accepted implementation SHA: `3cdb9c43bdec89c0c193b2d9c0a6469e3f0056e1`.

Accepted GitHub Actions run: `36448590359` — **SUCCESS**.

The accepted gate passed:

- formatting for both M11 authority crates;
- full-workspace Clippy with warnings denied;
- authority-model tests;
- explicit hybrid adapter tests;
- ptrace pathname-authority tests;
- shared-FD ambiguity tests;
- frozen baseline-v2 digest compatibility test;
- full locked workspace tests;
- Cargo.lock integrity.

## Accepted adapter contract

The hybrid path is internal and explicitly selected. Default selection remains `Disabled`.

Activation fails explicitly unless every declared prerequisite is present:

- Linux;
- x86_64;
- active BPF LSM;
- readable kernel BTF;
- BPF operation permitted;
- privilege explicitly acknowledged by the caller;
- producer-loss accounting available;
- exec-success confirmation available;
- connect-completion confirmation available.

There is no automatic ptrace fallback, no privilege escalation, no daemon installation, and no public backend mutation.

## Frozen bounded hybrid authorities

Only propositions directly supported by accepted M10 evidence are exposed:

- `FileOpenObjectIdentity` -> `KernelObjectCandidate`;
- `ProcessExecSuccess` -> `KernelSuccessConfirmed`;
- `ProcessExecObjectIdentity` -> `KernelObjectSuccessBound`;
- `NetworkConnectAttemptDestination` -> `KernelObjectCandidate`;
- `NetworkConnectSuccess` -> `KernelSuccessConfirmed`.

All other M11 propositions remain `Unsupported` in this adapter.

The adapter also preserves explicit health behavior: producer loss becomes `IncompleteLoss`; malformed/session/role ambiguity becomes `IncompleteAmbiguity`; neither is PASS-eligible.

## Compatibility boundary

No change was made to:

- `main`;
- `v0.1.0-alpha.3`;
- Marketplace behavior;
- stable GitHub Action channel;
- default ptrace selection;
- raw schema v2;
- canonical schema v2;
- baseline lock schema v2;
- baseline digest format v2.

The hybrid source has a distinct fingerprint and M11.4 grants **no cross-backend baseline equivalence**.

## Decision

**M11.4 CLOSED — PASS.**

Authorized next gate: **M11.5 — proposition-level comparability**.

M11.5 must decide comparability per proposition, default to fail-closed, and must not infer backend-wide equivalence from source identity or from a stronger authority label alone.
