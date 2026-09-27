# Post-M9 — eBPF E3 Shared Lifecycle Proposition Parity Result

Date: 2026-09-27
Status: **E3 CLOSED — PASS**
Tracking: GitHub issue #85
Source SHA: `196024408e497ec3448b0eced4f3350b098ac9d2`
Workflow run: `36301688725`
Artifact: `10925328847`
Artifact digest: `sha256:6709544a4e3023d661c52d0cc81c8e40e66a4d861cf2db627e5115cf7dda9cc2`
Protocol: `docs/milestones/POST_M9_EBPF_E3_SHARED_LIFECYCLE_PARITY_PROTOCOL.md`

## Decision

E3 passes the preregistered shared lifecycle proposition gate.

The public ptrace correctness-reference backend and the unchanged M8.7 persistent eBPF BPF architecture agree on the controlled fixture's shared lifecycle propositions:

- exactly two root-created child roles;
- the first child produces exactly two successful exec occurrences;
- the second child performs a deliberately failed exec attempt and produces zero successful exec occurrences;
- workload outcome is exit `0`;
- the persistent eBPF sessions remain lifecycle-health complete.

This result authorizes E4 **performance measurement design/execution only**. It does not authorize public eBPF exposure, PASS authority, `learn/check`, backend auto-selection, baseline interchangeability, full-surface equivalence, or product integration.

## Pre-execution protocol correction

Before any E3 workflow existed or ran, the original one-exec success fixture was corrected to a two-stage successful exec chain because the unchanged M8.7 health contract requires `exec_count >= 2` per accepted session.

The health gate was not weakened. No observed E3 result existed when the fixture correction was committed.

## Reference determinism

The public ptrace observer was executed twice against the same deterministic fixture.

Both runs were:

- complete;
- warning-free;
- target exit `0`, no signal;
- proposition-identical.

Frozen ptrace proposition projection:

```json
{
  "failed_exec_child_exec_occurrences": 0,
  "outcome": "exit_0",
  "root_spawn_count": 2,
  "spawn_roles": ["success_child", "failed_exec_child"],
  "success_child_exec_occurrences": 2
}
```

## Persistent eBPF evidence

The M8.7 BPF source was not modified.

Runner-only userspace instrumentation recorded the already-accepted current-epoch lifecycle transcript without changing event acceptance, tracker state, timeouts, polling, event limits, authority, or target behavior. The exact patch is preserved in the artifact.

### Epoch 1

Accepted transcript topology:

1. root -> child 1 spawn;
2. child 1 exec;
3. child 1 exec;
4. child 1 exit;
5. root -> child 2 spawn;
6. child 2 exit;
7. root exit.

Counts:

- event count: `7`;
- spawn count: `2`;
- exec count: `2`;
- exit count: `3`;
- `clean_for_m8_7a == true`.

Proposition projection exactly matched ptrace.

### Epoch 2

The second distinct-epoch session produced the same role-level transcript and projection:

- event count: `7`;
- spawn count: `2`;
- exec count: `2`;
- exit count: `3`;
- `clean_for_m8_7a == true`.

Proposition projection exactly matched ptrace.

## Health and authority

Both persistent sessions passed:

- lifecycle drain complete;
- active remaining `0`;
- stale epoch events `0`;
- integrity errors `0`;
- decode error delta `0`;
- producer drop delta `0`;
- routing error delta `0`;
- membership map empty;
- pending mechanism map empty;
- event limit not hit;
- target exit `0`;
- `clean_for_m8_7a == true`.

Final report retained:

- `ptrace_correctness_reference == true`;
- `full_surface_comparable == false`;
- `ebpf_pass_authorized == false`;
- `product_integration_authorized == false`.

## Classification

`E3_SHARED_LIFECYCLE_PROPOSITION_PARITY_PASS`

## What is proved

Within this controlled deterministic lifecycle fixture:

> ptrace and the M8.7 persistent eBPF architecture agree on the preregistered child-spawn and successful-exec occurrence propositions, including fail-closed non-promotion of a failed exec attempt.

## What remains unproved

E3 does not establish:

- full ptrace/eBPF runtime-surface equivalence;
- file/FD/path/network equivalence;
- public eBPF correctness authority;
- performance value on real workloads;
- baseline interchangeability;
- public integration readiness.

## Next gate — E4

E4 may now preregister same-workload performance requalification on the pinned `casey/just` and `junegunn/fzf` workloads.

Every accepted eBPF timing sample must first satisfy lifecycle/health completeness. Timing incomplete evidence is prohibited.
