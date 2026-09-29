# ExecSurface — P3 V5-C C0/C1 Shared-FD Certification Protocol

Date: 2026-09-29
Tracking: #104
Parent P3: #103
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — C1 REAL-WORKLOAD RELATION CENSUS**

## Why this gate exists

Frozen V5 Stage L ended `P3_INCOMPLETE_EVIDENCE`. The independent diagnostic run `36560525523` then established:

- `observe_rc=0`;
- `complete=false`;
- 8,601 raw events;
- exactly one warning code in the summary: `shared_fd_table_ambiguity`.

The public v2 guard is therefore doing what it was designed to do: it refuses to certify clone-concurrent observations when raw v2 does not retain enough clone/fd-table relationship evidence.

P2/S5 separately established a bounded three-way semantic relation:
`Shared | IndependentCopy | Unknown`.

C1 tests whether that relation is actually observable on the real pinned FZF workload before any collector-side certificate is designed.

## Critical ptrace event caveat

Linux ptrace event type is not itself sufficient to identify the originating creation syscall. Linux documents that clone calls may be reported as `PTRACE_EVENT_VFORK` when `CLONE_VFORK` is set, or as `PTRACE_EVENT_FORK` when the clone exit signal is `SIGCHLD` and the corresponding ptrace option is enabled.

Therefore C1 MUST pair the creation event with the pending syscall-entry record. It is prohibited to infer fd-table relation from `PTRACE_EVENT_FORK/VFORK/CLONE` alone.

This is also a falsification target for the later collector-side certificate: a direct event-type mapping must not launder a clone-origin transition into fork/vfork authority.

## Team

Fixed:
1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team

Dynamic C1 specialists:
- Linux ptrace + clone/clone3 semantics
- fd-table / `CLONE_FILES` semantics
- Go runtime/build concurrency
- Rust systems implementation
- reproducibility / evidence sealing

## Frozen relation contract

For a child-creation transition:

- `Shared`: originating syscall is clone/clone3, flags are observed, and `CLONE_FILES` is set.
- `IndependentCopy`: originating syscall is fork/vfork, or clone/clone3 flags are observed and `CLONE_FILES` is clear.
- `Unknown`: clone/clone3 flags are unreadable/unavailable, pending syscall cannot be causally paired, or origin cannot be proved.

`CLONE_THREAD`, event type, TID/TGID shape, or workload behavior MUST NOT substitute for missing `CLONE_FILES` evidence.

## Frozen workload

Repository:
`junegunn/fzf`

Revision:
`b1be3a8be1b833ce5b92fbbac11637643d60a046`

Host:
GitHub-hosted `ubuntu-24.04`.

Target:

```bash
/bin/bash -lc "cd '$WORK_ROOT' && go test ./... >/dev/null 2>&1"
```

Before census, execute exactly 3 direct priming runs.

Then execute exactly 3 independent C1 census runs. No replacement.

## C1 tracer requirements

The research tracer is independent from public ExecSurface output semantics. It may observe only the evidence required by this question:

- syscall entry for `fork`, `vfork`, `clone`, `clone3`;
- clone/clone3 flags, reading only the first flags field of `clone_args`;
- ptrace child-creation event and child TID;
- causal pairing between pending creation syscall and child-creation event;
- target terminal outcome.

Per transition record:
- parent TID;
- child TID;
- ptrace event type;
- originating syscall kind;
- flags if available;
- relation classification;
- evidence source / ambiguity reason.

Per run summary:
- total child-creation transitions;
- clone-origin transitions;
- `Shared` count;
- `IndependentCopy` count;
- `Unknown` count;
- clone3 flag-read failures;
- origin/event mismatches or unpaired events;
- root exit result.

## C1 classification

`C1_RELATION_OBSERVABLE_ALL_RUNS` iff:
- all 3 census runs execute to target exit 0;
- all child-creation events are causally paired;
- every clone/clone3-origin transition has readable flags;
- `Unknown == 0` in every run.

`C1_RELATION_PARTIAL_UNKNOWN` if any valid run contains one or more Unknown transitions.

`C1_EXECUTION_INCOMPLETE` if tracer/target evidence is incomplete or a run cannot be admitted.

No C1 outcome changes public v2 or authorizes V5 re-run. Only `C1_RELATION_OBSERVABLE_ALL_RUNS` permits C2 certificate prototyping.

## Preserved safety rule

The frozen V5 learning samples are not replaced by C1. The current public `shared_fd_table_ambiguity` guard remains authoritative until a later separately accepted integration/release gate, if any.
