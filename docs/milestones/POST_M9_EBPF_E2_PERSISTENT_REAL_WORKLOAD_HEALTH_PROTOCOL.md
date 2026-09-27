# Post-M9 — eBPF E2 Persistent Real-Workload Health Protocol

Date: 2026-09-27
Status: **PREREGISTERED — NOT YET EXECUTED**
Tracking: GitHub issue #85
Baseline HEAD: `80722931e29d375f489307fcfcc3e11ec2e678b5`

## Objective

Test whether the **unchanged M8.7 persistent lifecycle architecture** can carry the two pinned real workloads that triggered the Post-M9 eBPF research path, without reusing or patching the killed M8.4d syscall-exit membership design.

This is a lifecycle/health gate only. It is not a performance experiment and does not establish semantic parity with ptrace's full observation surface.

## Why this gate

E1 established that:

- the historical M9.1 blocker was intermittent `incomplete_lifecycle`, not `incomplete_capability`;
- the six-run E1 diagnostic did not reproduce the timeout;
- M9.1 used the older M8.4d per-invocation collector;
- M8.7 had already superseded that collector's child-membership boundary with session epochs, root registration before release, and `sched_process_fork` propagation.

Therefore E2 must test the stronger architecture directly rather than modifying timeout thresholds or adding heuristics to the old tracker.

## Frozen implementation rule

The committed source under:

`experiments/m8-ebpf/persistent-observer/`

must remain unchanged for this gate.

No runner-only patch to the persistent collector is permitted.

A small external **barrier adapter fixture** may be compiled inside the GitHub Actions runner. It exists only to present the already-supported M8.7 fixture contract to a generic shell command:

1. the persistent observer spawns the adapter with `barrier <fd>`;
2. the adapter blocks on that inherited file descriptor;
3. the persistent observer inserts the root TID -> epoch membership and starts its userspace tracker;
4. the observer releases the barrier;
5. the adapter closes the barrier descriptor and `exec`s `/bin/bash -lc <frozen workload command>`;
6. the root TID is preserved across `exec` and descendants are propagated by the unchanged M8.7 BPF `sched_process_fork` path.

The adapter must not inspect or alter workload files, environment secrets, stdin, network payloads, or unrestricted argv. Its only control-plane function is the launch barrier and `exec` handoff.

## Frozen targets

### E2-JUST

Repository/revision:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Command:

```text
cargo +1.90.0 test --all >/dev/null 2>&1
```

### E2-FZF

Repository/revision:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Command:

```text
go test ./... >/dev/null 2>&1
```

## Execution shape

For each target:

- same GitHub-hosted Ubuntu 24.04 runner class;
- exact pinned target revision;
- prime the exact workload once before the health gate;
- build the unchanged M8.7 persistent observer using the repository's established Rust 1.82-compatible lock-resolution procedure;
- compile the runner-local barrier adapter;
- run **3 independent persistent-observer invocations**;
- each invocation retains M8.7's existing **2 sessions** with distinct non-zero epochs;
- total accepted session opportunities per target: `6`;
- no performance timing acceptance criterion;
- no timeout changes;
- no event-limit changes;
- no fault injection;
- no retry/replacement of failed invocations or sessions.

A failed invocation/session remains evidence and kills E2 for that target.

## Required per-session health gates

Every one of the 6 sessions per target must satisfy the unchanged M8.7 report semantics:

- command outcome exit code `0`, signal `null`;
- `lifecycle_drain_complete == true`;
- `active_remaining == 0`;
- `stale_epoch_events == 0`;
- `integrity_errors == 0`;
- `decode_error_delta == 0`;
- `producer_drop_delta == 0`;
- `routing_error_delta == 0`;
- `membership_map_empty == true`;
- `pending_mechanism_map_empty == true`;
- `event_limit_hit == false`;
- `clean_for_m8_7a == true`.

The report must also retain the existing structural evidence required by M8.7, including non-trivial exec/spawn/exit lifecycle activity.

## Required per-invocation/final gates

Each persistent report must satisfy:

- exactly 2 sessions;
- `unexpected_without_session == 0`;
- `decode_errors_total == 0`;
- final membership map empty;
- final pending mechanism map empty;
- authority block unchanged:
  - `ptrace_correctness_reference == true`;
  - `full_surface_comparable == false`;
  - `ebpf_pass_authorized == false`;
  - `product_integration_authorized == false`.

## Acceptance

E2 is **PASS** only if all 3 invocations / all 6 sessions pass on **both** pinned targets.

Any one failed session makes E2 fail closed. No threshold or session count may be changed after observing results.

## What E2 PASS would mean

Only this:

> The already-proved M8.7 persistent session/lifecycle discipline successfully carries the two pinned real workloads through repeated health-complete lifecycle sessions under this frozen gate.

It would **not** mean:

- full semantic parity with ptrace;
- eBPF PASS authority;
- eBPF `learn/check` authorization;
- baseline interchangeability;
- backend auto-selection;
- public integration;
- a performance improvement claim.

If E2 passes, E3 may define a separate proposition/health parity gate before any value timing.

## Negative-evidence rule

Historical M9.1 failures and E1 non-reproduction evidence remain authoritative regardless of E2 outcome. They must not be removed, rewritten, or reclassified as if the old collector had passed.
