# Post-M9 — Ptrace Fast-Path Design Record

Date: 2026-09-27
Status: **CANDIDATE A SELECTED FOR ADVERSARIAL IMPLEMENTATION TESTING**
Parent: GitHub issue #84
Protocol: `docs/milestones/POST_M9_PTRACE_PHASE_AWARE_EXIT_FASTPATH_PROTOCOL.md`

## Candidate considered first

Candidate A is a per-TID phase-aware exit-side syscall-info fast path.

The observer continues to stop at every syscall under `PTRACE_SYSCALL`. Syscall-entry stops remain authoritative and continue using `PTRACE_GET_SYSCALL_INFO`. A corresponding exit-side syscall-info read may be skipped only when the preceding authoritative entry established that current semantics do not require the exit result and phase certainty has not been invalidated.

## Why not start with selective stop filtering

A larger theoretical saving could come from avoiding irrelevant syscall stops entirely. However a generic selective-stop design commonly requires additional target-side/kernel filtering state and therefore has a larger semantic and privilege surface. Phase A did not prove that such state would be transparent to the observed command.

Candidate A is intentionally tested first because it can be evaluated without intentionally changing target privilege state, seccomp state, baseline format, or public backend selection.

This record does not claim Candidate A is safe or valuable. Those are separate gates in the preregistered protocol.

## Innovation / deviation review

- Innovation objective: reduce observer control-plane work without discarding authority-relevant events.
- Deviation-prevention rule: any ambiguous syscall phase falls back to authoritative classification or fails closed; it never silently skips evidence.
- Red-team objective: force phase drift using signals, restart behavior, fork/clone events, exec/TID remapping and exit-group teardown.

## Rejected shortcuts

The following are not Candidate A and are not authorized by this record:

- suppressing frequent syscall numbers;
- blanket skipping of syscall exits;
- seccomp filters or `no_new_privs` changes;
- event sampling;
- path/temp suppression;
- eBPF substitution;
- reducing event budget;
- changing canonicalization, policy or verdict semantics.

## Next action

Implement Candidate A behind the existing native ptrace backend with explicit per-TID phase state, add the mandatory adversarial fixtures, and run semantic parity gates before any performance result is considered.
