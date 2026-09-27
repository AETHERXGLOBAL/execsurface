# Post-M9 — Ptrace Fast-Path A2 Parity v2 Result

Date: 2026-09-27
Status: **BLOCKED — COMPARATOR IMPLEMENTATION DEFECT; NO CANDIDATE CONCLUSION**
Parent: GitHub issue #84
Workflow run: `36288875048`
Source SHA: `7abef67d917f01cd6c34925a50acdbffd8fbb10a`
Artifact ID: `10921406769`
Artifact digest: `sha256:526a7dc3f09bbdb5f3c1ac26cf2342d444c4847798e1374dac92e386b14e3429`

## Decision

Candidate A2 did **not** fail semantic parity in this run.

The gate stopped before comparing A2 on the `restart` case because the reference/reference comparison reported:

`reference nondeterminism under frozen raw comparator: restart`

The failed run and its artifact remain preserved unchanged. It is not relabeled as PASS and is not deleted.

## What passed before the block

- frozen helper built successfully;
- Candidate A2 applied only inside the runner;
- strict workspace Clippy passed;
- `execsurface-observe` tests passed;
- full workspace tests passed;
- the comparator reached `restart` only after the earlier cases completed.

The activation diagnostic, bidirectional production baseline/diff parity, and all performance-value measurements were skipped by design.

## Subsequent comparator audit — correction

A later audit found that the Python implementation did **not** implement the already-frozen TID-only comparator correctly for the actual flattened observation JSON.

The observation schema emits process-spawn identity as flattened fields:

- `event_type = "process_spawn"`
- `tid = <parent tid>`
- `child_tid = <child tid>`

The v2 projection remapped `event["tid"]` but attempted to find `child_tid` inside a nonexistent nested `kind.ProcessSpawn` object. Therefore `child_tid` remained as the raw kernel TID and naturally differed across independent reference launches.

Commit `a25a03a479bee38aee6c52b1853e892a5ee9e74f` subsequently corrected this implementation defect and added a projection self-test proving that parent TID, spawn `child_tid`, later child event TID, and warning TID all map through the same local token map.

### Consequence

The earlier statement that v2 proved reference **event-order** nondeterminism is superseded. The run proves only that the then-current comparator implementation reported inequality. Because an allowed TID identity was left unprojected, the run cannot distinguish scheduling/order variance from this harness defect.

The correct classification is therefore:

**HARNESS / COMPARATOR IMPLEMENTATION DEFECT — NO A2 SEMANTIC CONCLUSION.**

This correction does not rewrite the old run or artifact; it corrects the scientific interpretation in a later commit.

## Prospective rule

A rerun must:

1. keep the frozen semantic rule unchanged: numeric runtime TID identity may be locally renamed; event order, sequence and all non-TID payloads remain exact;
2. correctly project flattened `process_spawn.child_tid` through the same TID token map;
3. execute a comparator self-test before any reference/candidate data are accepted;
4. perform reference/reference determinism before reference/A2 comparison;
5. preserve all previous failed runs and artifacts;
6. continue blocking activation/value measurement on any genuine reference nondeterminism or candidate mismatch.

Both the original concurrent `restart` fixture and the later single-process restart fixture may be tested prospectively after the comparator implementation is corrected; neither prior failure is silently converted into a pass.

## Authority boundary

- public runtime on `main`: unchanged;
- public release: `v0.1.0-alpha.3`;
- ptrace correctness authority: unchanged;
- eBPF authority: unchanged;
- Candidate A2 semantic eligibility: **OPEN / BLOCKED**;
- Candidate A2 performance value: **NOT EVALUATED**.
