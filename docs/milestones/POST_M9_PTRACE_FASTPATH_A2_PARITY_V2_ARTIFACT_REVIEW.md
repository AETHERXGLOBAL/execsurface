# Post-M9 — Candidate A2 Parity v2 Artifact Review

Date: 2026-09-27
Status: **ADDITIVE CORRECTION — COMPARATOR IMPLEMENTATION DEFECT IDENTIFIED**
Parent: GitHub issue #84
Preserved prior result: `docs/milestones/POST_M9_PTRACE_FASTPATH_A2_PARITY_V2_RESULT.md`
Workflow run reviewed: `36288875048`
Artifact ID: `10921406769`
Artifact digest: `sha256:526a7dc3f09bbdb5f3c1ac26cf2342d444c4847798e1374dac92e386b14e3429`

## Purpose

Preserve the prior blocked result unchanged while correcting its interpretation after direct artifact inspection against the already-frozen comparator contract.

No history is rewritten. The prior result remains evidence of a blocked run. This review records why that run cannot be interpreted as established reference nondeterminism.

## Frozen comparator authority

`POST_M9_PTRACE_FASTPATH_PARITY_COMPARATOR.md`, frozen before adversarial execution, permits exactly three trace-local identifier rewrites:

- `event.tid`;
- `ProcessSpawn.child_tid`;
- `warning.tid`.

No other field may be normalized.

The production raw event schema serializes `RawEventKind` with a flattened `event_type` tag. Therefore a spawn event is represented directly as fields such as:

```json
{
  "sequence": 7,
  "tid": 6391,
  "event_type": "process_spawn",
  "child_tid": 6392,
  "mechanism": "fork"
}
```

It is not represented as a nested `kind.ProcessSpawn` object.

## Artifact finding

For the `restart` row, the two reference observations in run `36288875048` were complete, warning-free and had identical command outcomes. They each contained seven events with the same event types, order, sequence numbers and non-TID fields.

The observed difference at the spawn event was trace-local identity only:

- reference run 1: parent `tid=6391`, `child_tid=6392`;
- reference run 2: parent `tid=6394`, `child_tid=6395`.

The parity runner correctly normalized the top-level parent `event.tid`, but its implementation looked for a nested `kind.ProcessSpawn` structure that does not exist in the actual flattened schema. Consequently `child_tid` remained unnormalized and the reference/reference comparison failed.

## Classification

The v2 block is therefore reclassified prospectively as:

**COMPARATOR/HARNESS SCHEMA FAILURE — NOT ESTABLISHED REFERENCE NONDETERMINISM, NOT CANDIDATE A2 SEMANTIC FAILURE.**

The old blocked record is not deleted or relabeled in place.

## Prospective correction

Before changing the concurrent restart fixture, rerun the exact existing rows with a comparator implementation that correctly applies the already-frozen TID-only rule to the actual flattened event schema:

- normalize top-level `event.tid`;
- when `event_type == "process_spawn"`, require `child_tid` and normalize it;
- normalize `warning.tid` when present;
- change nothing else.

The runner must fail closed if a `process_spawn` event lacks `child_tid` or if the expected event representation is ambiguous.

No event may be sorted, removed or rewritten beyond those trace-local identifiers. Event sequence numbers, event order, paths, mechanisms, warnings, completeness, outcomes and backend metadata remain exact.

Only if reference/reference still differs after the frozen comparator is implemented correctly may the concurrent restart fixture be classified as genuinely nondeterministic under that comparator and the previously proposed deterministic replacement be considered.

## Authority boundary

Unchanged:

- Candidate A2 remains experimental and runner-only;
- public runtime on `main` remains unchanged;
- performance value remains not evaluated;
- ptrace public correctness authority remains unchanged;
- eBPF authority remains unchanged.
