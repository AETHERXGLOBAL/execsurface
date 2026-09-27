# Post-M9 — Ptrace Fast-Path A2 Parity v3 Result

Date: 2026-09-27
Status: **BLOCKED — SAME COMPARATOR IMPLEMENTATION DEFECT; NO CANDIDATE CONCLUSION**
Parent: GitHub issue #84
Workflow run: `36289107167`
Source SHA: `df0eeec245284de0d6a0b57b01a5e477f8c038f0`
Artifact ID: `10921403460`
Artifact digest: `sha256:a5390c301b4bce92a4daadfdf94b0e06dcdd0cad483b482532fe766e1251ed10`

## Observed run result

The v3 gate completed all pre-parity engineering checks successfully:

- reference/helper build: PASS;
- Candidate A2 runner-only application: PASS;
- workspace Clippy with `-D warnings`: PASS;
- `execsurface-observe` tests: PASS;
- full workspace tests: PASS.

The strict raw parity step then stopped at:

`reference nondeterminism under frozen raw comparator: fork`

Activation diagnostics, bidirectional baseline/diff parity and performance value were skipped by design.

## Subsequent audit

The v3 script inherited the same projection implementation defect as v2: it remapped event `tid` but did not remap flattened `process_spawn.child_tid`. The `fork` case necessarily creates a process-spawn record with a newly allocated child TID, so two otherwise equivalent reference executions contain different raw `child_tid` values unless that allowed runtime identity is projected.

Commit `a25a03a479bee38aee6c52b1853e892a5ee9e74f` corrected the implementation to:

- require the flattened `event_type` field;
- remap `event.tid`;
- when `event_type == "process_spawn"`, remap flattened `child_tid` through the same local TID map;
- remap warning TIDs;
- self-test the projection before accepting data.

Therefore v3 does **not** establish true reference event-order nondeterminism in `fork`, and it does not establish a candidate mismatch.

Correct classification:

**HARNESS / COMPARATOR IMPLEMENTATION DEFECT — NO A2 SEMANTIC CONCLUSION.**

## Preserved evidence rule

The failed v3 run and artifact remain unchanged and visible. No prior failure is converted into PASS. A later run may only test the corrected implementation of the already-frozen TID-only rule; it may not relax event order, sequence, event payloads, warnings, completeness or outcome equality.

## Next gate

Execute a new versioned parity run with:

1. corrected flattened spawn-child TID projection;
2. projection self-test;
3. reference/reference comparison before candidate comparison;
4. both the original concurrent restart stress case and the single-process restart case, if included prospectively;
5. the same regression, activation and bidirectional production baseline/diff gates;
6. no external performance value measurement until semantic eligibility is established.

## Authority boundary

Public runtime, release, ptrace correctness authority and eBPF authority remain unchanged. Candidate A2 remains experimental and unmerged.
