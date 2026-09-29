# ExecSurface — P3 C1 Shared-FD Completeness Certification Protocol

Date: 2026-09-29
Tracking: #103
Parent diagnostic: `P3_V5_DIAG_SHARED_FD_AMBIGUITY_CONFIRMED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **C1 PREREGISTERED — OBSERVER CORRECTNESS GATE**

## Purpose

Reduce the known false-incompleteness of the conservative post-collection clone guard **without reopening the previously demonstrated shared-FD false-completeness risk**.

C1 is separate from the frozen V5 campaign. The six failed V5 Stage L samples remain `P3_INCOMPLETE_EVIDENCE` and are never replaced.

Public `v0.1.0-alpha.4`, raw observation schema v2, baseline/check semantics, normalization and stable `@v0.1` remain unchanged unless a later integration gate explicitly authorizes otherwise.

## Team

Fixed roles:
- Innovation Scientist / Systems Architect — seek structural proof retention, not guard deletion;
- Anti-Drift / Scientific Integrity Reviewer — preserve frozen V5 evidence and fail-closed semantics;
- Independent Falsifier / Red Team — owns ambiguity and lifecycle counterexamples.

Dynamic specialists:
- Linux ptrace / clone / clone3 / `CLONE_FILES` / `CLONE_THREAD` semantics;
- fd-table lifecycle state machines;
- Rust ownership/state modeling;
- process/thread exec semantics;
- reproducibility and concurrent workload testing;
- observer completeness certification.

## Current defect boundary

The current ptrace collector already attempts to read clone/clone3 flags and uses them internally:
- `CLONE_FILES` chooses shared parent fd-table identity versus cloned fd table;
- `CLONE_THREAD` chooses inherited thread-group identity;
- missing flags emit `clone_flags_unavailable` or `clone3_flags_unreadable`, which already mark the observation incomplete.

However the backend handoff receives only raw `Observation`. Raw v2 `ProcessSpawn` records the spawn mechanism but not clone flags. The post-collection `apply_shared_fd_ambiguity_guard` therefore treats **every** observed `SpawnMechanism::Clone` as uncertified and sets `shared_fd_table_ambiguity`, including sessions whose clone flags were successfully consumed by the internal fd-table state machine.

The fzf diagnostic run `36560525523` reproduced this conservative false-incompleteness:
- observer rc 0;
- 8,601 events;
- `complete=false`;
- warning code `shared_fd_table_ambiguity`.

## Candidate design

Do not add clone flags to the public raw schema in C1.

Instead, extend the internal ptrace result with a non-public completeness certificate produced by the same collector that consumes clone flags.

The certificate must contain at minimum:
- count of `PTRACE_EVENT_CLONE` events observed;
- count of clone events whose flags were positively correlated with the pending clone/clone3 syscall;
- count of certified `CLONE_FILES` shared-table transitions;
- count of certified non-`CLONE_FILES` cloned-table transitions;
- count of certified `CLONE_THREAD` transitions;
- an explicit boolean/enum stating whether clone fd semantics are fully certified for the session.

Certification may be positive only when **every** observed clone event has correlated flags and the collector has no ambiguity warning that affects fd lifecycle semantics.

The backend handoff may stop applying the generic `shared_fd_table_ambiguity` warning only when this internal certificate is positive.

If certification is absent, contradictory or incomplete, behavior remains fail-closed.

## Required invariants

1. `clone_count == correlated_clone_flags_count` is necessary, not merely desirable, for positive certification.
2. Every clone transition is classified exactly once as shared (`CLONE_FILES`) or cloned fd table.
3. `CLONE_THREAD` tracking remains independent of `CLONE_FILES`; neither bit may stand in for the other.
4. Missing `PendingSyscall::Clone` at `PTRACE_EVENT_CLONE` forces incomplete certification.
5. unreadable `clone3` flags force incomplete certification.
6. event-budget truncation, syscall-pairing loss, unreadable successful-open fd identity, or other existing observer warnings remain incomplete exactly as before.
7. No path/effect is invented to obtain certification.
8. Raw public event format is unchanged in C1.
9. Existing public fail-closed tests remain green.
10. A clone-free workload remains complete if otherwise healthy.

## Preregistered falsification suite

At minimum:

### C1-A — no clone
Synthetic bookkeeping with zero clone events must certify trivially, but must not claim any shared transition.

### C1-B — ordinary clone without `CLONE_FILES`
One correlated clone with flags lacking `CLONE_FILES` must create a distinct fd-table transition and may certify.

### C1-C — `CLONE_FILES`
One correlated clone carrying `CLONE_FILES` must share the exact parent fd-table identity and may certify.

### C1-D — `CLONE_THREAD` independent bit
A clone carrying `CLONE_THREAD` without `CLONE_FILES` must share tgid but not fd-table identity.

### C1-E — combined `CLONE_THREAD | CLONE_FILES`
Must share both tgid and fd-table identity.

### C1-F — missing pending clone flags
A clone event without correlated flags must increment clone count but not correlated count, produce/retain `clone_flags_unavailable`, and fail certification.

### C1-G — clone3 flags unreadable
Must retain `clone3_flags_unreadable` and fail observation completeness/certification.

### C1-H — conflicting lifecycle warning
Any pre-existing fd-lifecycle ambiguity/loss warning must prevent a PASS-eligible observation even if clone counts correlate.

### C1-I — exec + CLOEXEC under shared table
The state machine must preserve the accepted Linux exec semantics: an execing task must obtain the correct post-exec fd-table semantics without closing CLOEXEC descriptors in other users of a previously shared table.

### C1-J — dup/close/close_range under shared table
Mutations by one `CLONE_FILES` user must remain visible to all users mapped to the same internal table; independent cloned tables must diverge correctly.

### C1-K — fd-number reuse after close
Reused descriptor numbers must bind only to the new object/path; no stale attribution.

### C1-L — event-limit overflow
Still incomplete regardless of clone certification.

## Gate sequence

1. implement an internal certificate only; no public schema changes;
2. add deterministic unit/state-machine tests for C1-A through C1-L where feasible;
3. run full `execsurface-observe` tests + workspace fmt/clippy/tests required by affected crates;
4. run a synthetic real ptrace concurrency harness containing both `CLONE_FILES` and non-sharing clone cases;
5. only if all synthetic/adversarial gates pass, re-run **one diagnostic-only** pinned fzf observation using a new source hash;
6. require fzf to be complete with no `shared_fd_table_ambiguity` warning and no other warning before C1 can close positively.

The fzf diagnostic is not a V5 learning replacement and cannot retroactively change Stage L.

## Allowed outcomes

- `P3_C1_SHARED_FD_CERTIFICATION_PASS_BOUNDED`
- `P3_C1_FALSE_COMPLETENESS_FOUND`
- `P3_C1_CERTIFICATION_INCOMPLETE`
- `P3_C1_REAL_WORKLOAD_STILL_INCOMPLETE`

A bounded PASS only authorizes a **new preregistered requalification campaign**. It does not authorize public release integration by itself.
