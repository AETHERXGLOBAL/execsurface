# Post-M9 — Ptrace Fast-Path Candidate A2 Hardening

Date: 2026-09-27
Status: **PREREGISTERED BEFORE A2 EXECUTION**
Parent: GitHub issue #84
Base protocol: `docs/milestones/POST_M9_PTRACE_PHASE_AWARE_EXIT_FASTPATH_PROTOCOL.md`
Raw comparator: `docs/milestones/POST_M9_PTRACE_FASTPATH_PARITY_COMPARATOR.md`
Canonical parity method: `docs/milestones/POST_M9_PTRACE_FASTPATH_PARITY_METHOD.md`

## Decision

Candidate A is tightened to **Candidate A2** before any external value measurement.

A2 keeps the same bounded optimization objective: preserve every `PTRACE_SYSCALL` stop and skip only an eligible exit-side `PTRACE_GET_SYSCALL_INFO` request. It does not filter syscall stops and does not change target seccomp, privilege, policy, baseline, canonicalization, verdict, backend identity, or eBPF authority.

## Mandatory A2 hardening

The optional exit fast path is allowed only when the immediately preceding authoritative syscall-entry classification established all of the following for the same TID:

- `pending_syscall` is `None`;
- `pending_exec` is `None`;
- `exit_group_pending` is false;
- `retired_by_exec` is false;
- `exit_event_status` is `None`;
- `newborn` is false.

Additionally:

1. **Any non-syscall stop invalidates the optimization phase before the tracee is resumed.** This includes signal-delivery stops. The next syscall stop must use authoritative `PTRACE_GET_SYSCALL_INFO` rather than assuming it is the previously expected exit.
2. Any ptrace event invalidates phase before event-specific reconciliation.
3. Any exec identity remap/reconciliation resets phase.
4. Bounded ESRCH lifecycle recovery resets phase.
5. Any restart/resume ESRCH resets phase.
6. If A2 reaches an exit fast-path state while any protected condition above is present, it must fail closed rather than skip classification.

## Why A2 replaces A for the next gate

The original isolated Candidate A passed its existing semantic smoke suite, but the frozen protocol requires conservative handling of phase ambiguity. Signal/non-syscall stops can occur between syscall stops and therefore cannot be assumed to preserve the optional entry->exit phase state.

This tightening is made **before adversarial differential parity and before any value benchmark**. It is not a response to favorable or unfavorable performance results.

## Frozen acceptance sequence

A2 may proceed only in this order:

1. apply A2 in an isolated workflow working tree; public `main` runtime remains unchanged;
2. formatting + Clippy + existing observer and workspace tests;
3. adversarial reference-vs-A2 raw parity using the frozen TID-only comparator;
4. fast-path activation diagnostic showing the intended exit-side GETINFO reduction on synthetic irrelevant syscalls;
5. production canonical/baseline-diff parity with unchanged existing machinery;
6. only after all semantic gates pass, evaluate the separately preregistered >=10% per-target value threshold on pinned `ripgrep` and `fzf`.

Any semantic mismatch, unexpected warning/incompleteness, phase ambiguity, or harness uncertainty blocks value measurement and is preserved as negative evidence.

## Authority boundary

Candidate A2 is experimental only. This record does not modify `crates/execsurface-observe/src/linux_ptrace.rs` on `main`, does not authorize a merge or release, and does not establish a public performance claim.
