# Post-M9 — eBPF E2 Persistent Real-Workload Health Result

Date: 2026-09-27
Status: **E2 CLOSED — PASS**
Tracking: GitHub issue #85
Source SHA: `9edc66b9e860e8cf474bb169b966951accebe2b9`
Workflow run: `36301162578`
Protocol: `docs/milestones/POST_M9_EBPF_E2_PERSISTENT_REAL_WORKLOAD_HEALTH_PROTOCOL.md`

## Decision

The unchanged M8.7 persistent lifecycle architecture passed the preregistered real-workload health gate on both pinned targets.

- `casey/just`: 3/3 persistent invocations PASS, 6/6 sessions PASS.
- `junegunn/fzf`: 3/3 persistent invocations PASS, 6/6 sessions PASS.

No session was replaced or retried. No lifecycle timeout, stale epoch, routing error, producer drop, decode error, integrity error, event-limit hit, active-task leak, or residual kernel-map membership was accepted.

This closes E2 and authorizes E3 proposition/health parity design only. It does **not** authorize performance timing, full semantic parity, public eBPF exposure, PASS authority, `learn/check`, backend auto-selection, or baseline interchangeability.

## Architecture under test

The committed source under `experiments/m8-ebpf/persistent-observer/` was unchanged.

A runner-local barrier adapter implemented only the already-supported M8.7 fixture contract:

1. block on the inherited barrier descriptor;
2. allow the persistent observer to insert root TID -> session epoch membership;
3. receive the release byte;
4. close the barrier descriptor;
5. `exec /bin/bash -lc <frozen workload command>`.

The root TID is preserved across the exec handoff and descendant membership is propagated by the unchanged M8.7 `sched_process_fork` path.

## `casey/just` result

Target:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Command:

`cargo +1.90.0 test --all >/dev/null 2>&1`

Artifact:

- ID: `10926175328`
- digest: `sha256:9de1bd5dd4b005b2eec79144277df1616eb29ec3576fd7bad66f3ce74fd64941`

All 3 invocations returned zero and each contained exactly 2 accepted sessions.

All 6 sessions had:

- lifecycle drain complete;
- active remaining `0`;
- stale epoch events `0`;
- integrity errors `0`;
- decode error delta `0`;
- producer drop delta `0`;
- routing error delta `0`;
- membership map empty;
- pending-mechanism map empty;
- event limit not hit;
- `clean_for_m8_7a == true`;
- workload exit `0` / no signal.

Each of the 6 sessions recorded exactly:

- event count: `19,540`;
- spawn count: `8,045`;
- exec count: `3,449`;
- exit count: `8,046`.

The repeat identity of these aggregate counts is additional reproducibility evidence for this pinned workload, but is not generalized beyond the declared run.

## `junegunn/fzf` result

Target:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Command:

`go test ./... >/dev/null 2>&1`

Artifact:

- ID: `10925861295`
- digest: `sha256:3afee45753c3d823a26b3e640fae7b66a720e8fc10b9b96c887b0d55a2c56edd`

All 3 invocations returned zero and each contained exactly 2 accepted sessions.

All 6 sessions passed every frozen health gate. Across the six sessions:

- event count ranged from `463` to `487`;
- spawn count ranged from `206` to `217`;
- exec count ranged from `50` to `52`;
- exit count ranged from `207` to `218`;
- no lifecycle, loss, routing, decode, stale-epoch, integrity, map-cleanup or authority-boundary failure occurred.

The count variation is retained as evidence of legitimate workload/runtime concurrency variation and is not normalized away.

## Final authority checks

Every accepted report retained:

- `ptrace_correctness_reference == true`;
- `full_surface_comparable == false`;
- `ebpf_pass_authorized == false`;
- `product_integration_authorized == false`;
- `unexpected_without_session == 0`;
- `decode_errors_total == 0`;
- final membership map empty;
- final pending mechanism map empty.

## What E2 proves

Within the frozen targets and runner class:

> The M8.7 session-epoch/root-barrier/`sched_process_fork` lifecycle discipline can repeatedly carry the two real workloads that triggered Post-M9 eBPF research without the lifecycle-drain failures observed historically in the older M8.4d value-screen path.

## What E2 does not prove

E2 does not establish:

- why each historical M8.4d timeout occurred;
- full runtime-surface equivalence to ptrace;
- file/FD/path/network semantic parity;
- eBPF performance value;
- eBPF public readiness;
- public PASS authority;
- baseline interchangeability.

## Next gate — E3

E3 must define and test only **shared lifecycle propositions** that both the ptrace correctness reference and the M8.7 persistent observer can support without pretending the narrower eBPF collector has ptrace's full surface.

Performance timing remains blocked until E3 closes successfully.
