# Post-M9 — Ptrace Phase-Aware Exit Fast-Path Protocol

Date: 2026-09-27
Status: **PREREGISTERED — CANDIDATE NOT YET AUTHORIZED**
Parent tracking: GitHub issue #84
Evidence basis: `docs/milestones/POST_M9_PTRACE_COST_ATTRIBUTION_RESULT.md`

## Candidate

Evaluate one narrowly scoped native-ptrace optimization:

> after a syscall-entry stop has been authoritatively identified with `PTRACE_GET_SYSCALL_INFO`, avoid a second `PTRACE_GET_SYSCALL_INFO` on the corresponding syscall-exit stop **only when the observer has already proved that no current ExecSurface exit-result semantic depends on that exit**.

The candidate does **not** remove syscall stops, change `PTRACE_SYSCALL`, change the event budget, change path/fd semantics, add seccomp, add eBPF, or change the public backend.

The intended reduction is one observer ptrace request on eligible syscall exits. This is deliberately smaller than selective kernel stop filtering because it does not intentionally modify target process privilege/security state.

## Why this candidate is first

Phase A established a causal all-syscall stop tax and measured roughly 20k `PTRACE_GET_SYSCALL_INFO` requests per accepted external diagnostic run. The hardened `strace -T` decomposition also shows that syscall-info requests are a non-trivial observer operation, but those request durations remain `DIAGNOSTIC_PERTURBED` and are not a production performance claim.

Therefore the first optimization experiment targets a measurable request class while preserving every existing stop/resume boundary.

## Frozen semantic rule

The candidate may suppress exit-side syscall-info retrieval only when all of the following are true for the tracked TID:

1. the immediately corresponding syscall-entry stop was read successfully with `PTRACE_GET_SYSCALL_INFO`;
2. that entry did not create a `PendingSyscall` whose result is required by current semantics;
3. it is not in a lifecycle state that requires explicit exit-side classification for fail-closed handling;
4. no ptrace event, exec identity remap, registration anomaly, protocol error, or terminal reconciliation invalidated the phase state;
5. the observer can still prove whether the next syscall-stop is the expected paired exit without guessing across an ambiguous transition.

If phase certainty is lost, the candidate must fall back to the current authoritative `PTRACE_GET_SYSCALL_INFO` path. Ambiguity may reduce optimization benefit; it may not become silent evidence loss.

## Explicitly protected semantics

The candidate must preserve exactly the current behavior for at least:

- process spawn lineage and fork/vfork/clone events;
- exec path capture and exec identity remapping;
- `exit_group` lifecycle handling and bounded ESRCH recovery;
- open/openat/openat2/creat success-dependent fd identity;
- read/write and transfer syscalls whose positive result creates fd-attributed effects;
- close/close_range success-dependent fd lifecycle;
- dup/dup2/dup3/fcntl fd lifecycle;
- rename success-dependent fd-path updates;
- clone/clone3 flag handling;
- unlink/delete attempts;
- connect destination observation;
- event-budget truncation and all completeness/warning/error behavior;
- privacy boundary: no argv/env/file-content/stdin/network-payload capture.

## Required implementation shape

- introduce explicit per-TID syscall-phase state; do not infer phase from global ordering;
- default state after registration/reconciliation must be conservative;
- ptrace events and exec/TID remapping must explicitly reconcile or invalidate phase state;
- signal-delivery stops must not silently advance syscall phase;
- any unexpected syscall-stop phase must use authoritative syscall-info retrieval or fail closed;
- do not change public CLI flags, baseline schema, backend ID, policy, verdict, or release metadata merely for this experiment.

## Mandatory adversarial semantic fixtures

Before performance acceptance, automated tests must cover at minimum:

1. irrelevant syscall entry -> paired exit;
2. selected syscall with required exit result;
3. failed selected syscall;
4. signal interruption and restart around an irrelevant syscall;
5. signal delivery between traced syscall activity;
6. fork/vfork/clone event interleaving;
7. clone/clone3 fd-sharing semantics;
8. successful exec and TID remap;
9. failed exec followed by continued execution;
10. `exit_group` lifecycle and the existing bounded ESRCH path;
11. child preregistration ordering;
12. event-budget fail-closed behavior;
13. path/fd/connect evidence parity on the existing observer fixture.

The independent Red Team must attempt to construct a case where phase drift suppresses an exit result needed for evidence or causes entry/exit misclassification.

## Semantic acceptance gate

A candidate implementation is semantically eligible only if:

- all existing repository tests pass;
- all mandatory adversarial fixtures pass;
- reference and candidate observations for the parity fixtures are exactly equivalent in events, warnings, completeness, and command outcome;
- no new warning is hidden and no existing negative fixture is deleted or weakened;
- current ptrace lifecycle regression evidence remains green.

Any unresolved phase ambiguity or evidence mismatch kills the candidate for this gate.

## Performance protocol

Only after semantic eligibility:

Use the same pinned external runtime targets as Phase A:

- `BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`
- `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Compare a frozen reference binary from pre-candidate source against the candidate on the same runner/job and workload revision.

For each target/mode:

- 3 warmups;
- 15 measured samples;
- alternating reference/candidate order;
- no clean-sample removal;
- raw samples retained;
- complete, warning-free observations required;
- exact command/outcome and evidence parity required.

Primary metric:

`ptrace observer wall time` for reference vs candidate.

Predeclared engineering value gate:

- candidate median ptrace wall time must be lower on **both** pinned targets; and
- median ptrace wall-time reduction must be at least `10%` on **each** target; and
- no semantic/parity gate may fail.

If semantic gates pass but the value threshold fails, classify the candidate `SEMANTICALLY_VALID / VALUE_NOT_ESTABLISHED` and do not present it as a performance improvement.

## Prohibited shortcuts

- no syscall/event dropping based only on frequency;
- no seccomp/no-new-privs change in this candidate;
- no eBPF substitution;
- no threshold weakening after results;
- no sleeping/retry tricks for benchmark appearance;
- no selective sample exclusion;
- no baseline/canonicalization/policy change to force parity;
- no public speed claim from this gate alone.

## Authority boundary

Even if this candidate passes:

- ptrace remains the same public correctness-reference backend;
- eBPF authority does not change;
- public release remains unchanged until a separate release gate;
- results remain exact-host/workload evidence, not a universal Linux performance claim.
