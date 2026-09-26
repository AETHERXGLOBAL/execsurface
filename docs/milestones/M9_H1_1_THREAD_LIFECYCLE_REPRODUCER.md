# M9-H1.1 — Minimal Thread-Lifecycle Reproducer

Status: IN PROGRESS

## Objective

Isolate and close the Linux ptrace child pre-registration ordering gap observed during M9.1 on the pinned `casey/just` workload.

The specific observed pattern is:

1. a traced parent creates a child/thread;
2. the kernel may make the automatically attached child waitable in its initial `SIGSTOP` before the tracer has processed the parent's `PTRACE_EVENT_FORK` / `PTRACE_EVENT_VFORK` / `PTRACE_EVENT_CLONE` and `PTRACE_GETEVENTMSG` registration path;
3. treating that child stop as an ordinary already-registered tracee can create stale or untracked lifecycle state;
4. later `PTRACE_EVENT_EXIT`, restart `ESRCH`, or outer-loop `ECHILD` then exposes the bookkeeping defect.

No product fix is accepted merely because an errno disappears.

## Team for this gate

### Fixed roles

- **Innovation Scientist / Architecture Challenger** — look for a smaller, more general lifecycle model rather than workload-specific suppression.
- **Deviation Prevention / Scientific Integrity Reviewer** — enforce fail-closed semantics, preserve negative evidence, and reject claims not supported by reproducible artifacts.

### Dynamic specialists

- **Linux ptrace / process-lifecycle specialist** — `waitpid(__WALL)`, `PTRACE_O_TRACE*`, clone/fork/exec/exit ordering and TID/TGID identity.
- **Concurrency / thread-runtime engineer** — rapid pthread creation/exit, thread-group behavior, and stress determinism.
- **Runtime semantics engineer** — descendant completeness, root outcome, syscall pairing, fd-table inheritance and event-budget semantics.
- **CI / reproducibility engineer** — pinned toolchain, repeatable fixture, evidence capture and SHA-256 records.
- **Independent Red Team** — attempts to create false PASS via dropped stops, blanket `ESRCH` ignore, arbitrary retries, sleeps, or premature tracee deletion.

These are functional review roles for the gate; they do not imply external independent review.

## Minimal reproducer

`tests/fixtures/m9/thread_lifecycle_repro.c`

The fixture intentionally creates two forms of rapid descendant churn:

- `pthread_create` / immediate thread exit to exercise `CLONE_THREAD`-style lifecycle pressure;
- `fork` / immediate child exit to exercise process-child registration pressure.

It contains no sleeps and no timing-based success criterion.

## Candidate model under test

A newly auto-attached child that becomes waitable in the expected initial plain `SIGSTOP` before parent-side registration may be held in a bounded **pre-registration stop buffer** keyed by TID.

The candidate is acceptable only if the later parent `PTRACE_EVENT_*` event names exactly that TID via `PTRACE_GETEVENTMSG`, at which point the child state is created from the parent's actual semantics and the buffered stop is reconciled exactly once.

Anything else remains fail-closed.

## Prohibited fixes

- blanket ignore of `ESRCH`, `ECHILD`, unknown TIDs, or `PTRACE_EVENT_EXIT`;
- fixed sleeps;
- arbitrary retry loops;
- inserting an untracked child with guessed fd-table or TGID semantics;
- deleting stale tracees merely because `waitpid` reached `ECHILD`;
- changing baseline/diff/policy/report semantics to hide observer incompleteness;
- substituting eBPF for ptrace.

## Acceptance gate

M9-H1.1 is **PROVED** only if all are true:

1. the minimal fixture succeeds directly;
2. the fixture succeeds under the candidate ptrace observer repeatedly without observer ERROR;
3. every observation is `complete=true` and root outcome is exit code 0 with no signal;
4. no untracked stop, untracked terminal wait, untracked EXIT event, unresolved pre-registration TID, or stale tracee remains;
5. existing `execsurface-observe` regression tests pass unchanged;
6. existing event-budget fail-closed semantics pass unchanged;
7. the same pinned `casey/just` workload succeeds under observation three consecutive times;
8. evidence and hashes are preserved;
9. Red Team review finds no silent evidence loss or guessed descendant semantics.

If the minimal fixture does not reproduce the relevant ordering, that is evidence about the fixture only; it does not close issue #55. The gate then remains OPEN and the fixture must be refined from the preserved `casey/just` evidence.

## Current evidence basis

The prior external diagnostic evidence already showed many `M9_UNTRACKED_STOP_BEFORE_REGISTRATION` events, including plain `SIGSTOP` (`signal=19`, `event=0`), followed by an untracked `PTRACE_EVENT_EXIT` for the same thread-group. That evidence motivates this gate but does not by itself authorize the candidate fix.
