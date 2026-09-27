# Post-M9 — Phase-Aware Exit Fast-Path Semantic Comparator

Date: 2026-09-27
Status: **PREREGISTERED ON EXPERIMENT BRANCH — BEFORE CANDIDATE EXECUTION**
Parent protocol: `docs/milestones/POST_M9_PTRACE_PHASE_AWARE_EXIT_FASTPATH_PROTOCOL.md`
Tracking: GitHub issue #84

## Purpose

Freeze how the reference ptrace observer and Candidate A will be compared before the candidate is executed.

The candidate remains experimental. This record does not authorize merge, release, or any public performance claim.

## Reference

Reference source is the branch base commit:

`fdf668af61ba08a11f0bc7d3dac44cdc484bbcf2`

The reference binary is built and copied before any experiment-only source patch is applied in the workflow job.

## Semantic projection

Two independent executions cannot have identical kernel TIDs or raw sequence numbers. Those are run-local identities, not the semantic payload being optimized.

For each reference/candidate observation, compare:

1. backend metadata: exact equality;
2. `complete`: exact equality;
3. command outcome: exact equality;
4. warnings: exact multiset of `{code, message}`; warning `tid` is excluded as run-local identity;
5. events: exact multiset of each `RawEventKind` JSON payload; raw `tid` and `sequence` are excluded as run-local identity.

No event kind, path, fd, operation, endpoint, flag, resolve value, child relationship payload, warning code/message, completeness state or command outcome may be normalized away.

Multiset comparison is used because independent executions, especially threaded/forking fixtures, may interleave event order differently even when semantic evidence is equivalent. Candidate A is not allowed to rely on ordering normalization to hide a missing or added payload.

## Frozen semantic fixture classes

The experiment must include at minimum:

- irrelevant syscall traffic with no authority-relevant event growth;
- selected open/read/write/close behavior;
- failed selected open;
- signal interruption around irrelevant syscall activity followed by selected file activity;
- fork/vfork/clone or thread lifecycle coverage;
- successful exec;
- failed exec followed by continued execution;
- exit-group/lifecycle coverage;
- existing observer integration tests, including event-budget fail-closed, fd inheritance/sharing, openat2 and network destination evidence.

Any fixture that cannot be made deterministic enough for the frozen semantic projection must remain an explicit gap and cannot be silently omitted from the protocol-level acceptance claim.

## Fast-path activation diagnostic

Separately from semantic parity, an uninstrumented candidate is not assumed to have exercised the intended path.

On a synthetic raw-`getpid` workload, run reference and candidate under separate observer-process `strace` diagnostics and count exact `PTRACE_GET_SYSCALL_INFO` request arguments.

Expected structural signal:

- reference: approximately two syscall-info reads per added irrelevant syscall (entry + exit);
- candidate: approximately one syscall-info read per added irrelevant syscall after phase establishment, because the eligible exit-side info read is skipped;
- ExecSurface semantic event projection must remain equal.

This counter diagnostic is not a performance sample.

## Acceptance

Semantic eligibility requires all of the following before any external performance gate is run:

- standard repository tests pass on the candidate working tree;
- dedicated ptrace observer tests pass;
- every frozen differential fixture passes the semantic projection;
- the candidate fast path is actually exercised in the synthetic diagnostic;
- no warning, incomplete state, protocol error or negative evidence is hidden;
- no public/default branch runtime behavior has changed.

Any semantic mismatch, unresolved phase ambiguity or missing required fixture keeps Candidate A unaccepted regardless of speed.
