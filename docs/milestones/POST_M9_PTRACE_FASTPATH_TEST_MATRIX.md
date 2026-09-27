# Post-M9 — Ptrace Phase-Aware Fast-Path Adversarial Test Matrix

Date: 2026-09-27
Status: **FROZEN BEFORE RUNTIME IMPLEMENTATION**
Parent: GitHub issue #84
Protocol: `docs/milestones/POST_M9_PTRACE_PHASE_AWARE_EXIT_FASTPATH_PROTOCOL.md`

The candidate runtime implementation must not be accepted unless every applicable row below is exercised by automated evidence. Existing tests may satisfy a row only when their assertions cover the stated semantic boundary; otherwise a new fixture/test is required.

| ID | Adversarial case | Required invariant |
| --- | --- | --- |
| FP-01 | irrelevant syscall entry -> exit | candidate may use fast exit path only after authoritative entry and must return to entry-expected state |
| FP-02 | selected syscall requiring exit result | exit must use authoritative syscall info and preserve result-dependent fd/evidence semantics |
| FP-03 | failed selected syscall | no success-only fd/effect state may be created |
| FP-04 | signal interruption/restart around irrelevant syscall | phase must not drift; ambiguity must fall back or fail closed |
| FP-05 | signal-delivery stop between syscall activity | signal stop must not be consumed as syscall phase |
| FP-06 | fork/vfork event interleaving | parent/child phase state remains independent and lineage evidence unchanged |
| FP-07 | clone/clone3 with and without `CLONE_FILES` | fd-sharing semantics unchanged |
| FP-08 | successful exec/TID remap | phase is reconciled/reset; exec/path/close-on-exec evidence unchanged |
| FP-09 | failed exec then continued execution | no stale phase/path state corrupts later observation |
| FP-10 | `exit_group` + bounded ESRCH lifecycle | existing lifecycle hardening and fail-closed rules unchanged |
| FP-11 | preregistration child stop ordering | no unknown/duplicate child is accepted silently |
| FP-12 | event-budget exhaustion | incomplete evidence remains fail-closed at the same boundary |
| FP-13 | open -> read/write -> dup/close | exact fd-attributed effects and lifecycle parity |
| FP-14 | rename/delete path effects | exact event and fd-path update parity |
| FP-15 | connect destination | exact network evidence parity |
| FP-16 | existing full ptrace fixture suite | no regression in event/warning/completeness/outcome semantics |

## Performance is gated after semantics

No timing improvement can compensate for failure of any semantic row. If a row cannot be proved under the candidate implementation, Candidate A is killed or returned to design.
