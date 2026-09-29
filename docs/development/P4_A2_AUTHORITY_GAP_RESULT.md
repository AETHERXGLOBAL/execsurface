# P4 A2 — Authority-Gap Matrix Result

Status: **CLOSED — PASS BOUNDED**

Formal decision:

`P4_A2_PROPOSITION_GAPS_ESTABLISHED_BOUNDED`

Boundary:

`NO_BACKEND_PROMOTION_AUTHORIZED`

## Accepted source and evidence

- Development branch: `development/post-alpha4-behavioral-integrity`
- Accepted source: `43d6c05e47b23b5a3f0a1ebb9f98558af22f4ea6`
- Workflow: `36590722299`
- Job: `109482793575`
- Artifact: `11044125563`
- Artifact SHA-256: `af92dfac5c2cb0f4d58ab4536ea1e82be9dcdbbbb503520ad1c3ff37e9377e5d`
- Strongest accepted A1.4 ancestor: `08969deddf8c0cac154e73922672c60b8c0d3660`
- Frozen public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

The workflow verified that both `v0.1.0-alpha.4` and stable `v0.1` still resolve to the frozen alpha.4 source and that no runtime/public crate drift occurred after the accepted A1.4 source.

## Gate evidence

All final A2 gates passed:

- rustfmt: PASS
- clippy `-D warnings`: PASS
- deterministic A2 matrix audit: **16/16 PASS**
- canonical A0 authority model: **8/8 PASS**
- A1.3U attempt-authority contract: **5/5 PASS**
- A1 adversarial/mapping corpus: **18/18 PASS**
- A1.2 mapping corpus: **10/10 PASS**
- Semantics v3 reproof: **7/7 PASS**
- public M11 shared-FD contract: **6/6 PASS**

The matrix contains exactly the ten preregistered propositions and contains no aggregate backend score.

## Proposition-by-proposition result

| Proposition | Best bounded authority | Primary gap | Product-usable now | Second backend plausibly useful |
| --- | --- | --- | --- | --- |
| `P4.PROC.CREATE_RELATION` | Direct | Representation gap | No | No |
| `P4.EXEC.SUCCESS` | Direct | Representation gap | No | No |
| `P4.PATH.ACCESS_ATTEMPT` | AttemptOnly | No gap | Yes | No |
| `P4.FILE.OPEN_OBJECT` | Unsupported | Observation gap | No | **Yes** |
| `P4.FD.IO_ATTRIBUTION` | Direct conditionally | Representation gap under raw-v2 clone | No | No |
| `P4.FILE.RENAME_DELETE` | AttemptOnly | Success-semantics gap | No | **Yes** |
| `P4.NET.CONNECT_DESTINATION` | AttemptOnly | Success-semantics gap | No | **Yes** |
| `P4.CAUSAL.EXEC_LINEAGE` | DerivedBounded | No gap | Yes | No |
| `P4.OBSERVER.HEALTH_LOSS` | Direct/lost bounded by observed state | No gap | Yes | No |
| `P4.FDTABLE.RELATION` | DerivedBounded for fork/vfork; ambiguous for clone | Representation gap | No | No |

Exactly three propositions are classified as plausible targets for additional observation authority:

1. `P4.FILE.OPEN_OBJECT`
2. `P4.FILE.RENAME_DELETE`
3. `P4.NET.CONNECT_DESTINATION`

Representation gaps are explicitly **not** evidence that a second backend is required. The process-identity and clone/fd-table gaps should first be addressed by retaining evidence already available to ptrace in a future versioned evidence schema.

## Exact missing evidence for the three targeted gaps

### `P4.FILE.OPEN_OBJECT`

Missing: successful open result, returned-FD identity, and a post-open object/path binding at the successful transition, including the case where no later covered I/O occurs.

### `P4.FILE.RENAME_DELETE`

Missing: successful syscall/effect result bound to the source/target identities. A pathname attempt alone remains AttemptOnly.

### `P4.NET.CONNECT_DESTINATION`

Missing: connect-result semantics, including explicit bounded treatment of asynchronous `EINPROGRESS`, bound to socket/destination identity and the originating execution chain.

## Anti-drift interpretation

A2 does not justify a broad eBPF/BPF-LSM implementation. The matrix narrows the next scientific question: determine whether targeted ptrace syscall-exit/state evidence can close the three success/observation gaps before introducing any second backend.

No backend is globally better or worse. Authority remains proposition-scoped.

## Preserved negative evidence

The following failed attempts remain part of the evidence history and are not relabeled:

- formatting-only A2 workflow failures before measurement;
- isolated A2 audit formatting failures;
- contract-correction compile failure caused by an unnecessary `Deserialize` derive on borrowed static-slice evidence structures;
- a correction workflow that proved the strengthened matrix tests but could not push a workflow-file modification because of GitHub Actions token policy;
- anti-race correction attempts that aborted when the development branch advanced and the targeted change already existed.

None of those failures changed a scientific threshold, expected outcome, proposition inventory, classification rule, or public runtime contract.

## Product and release boundary

This result authorizes **no public integration**, **no backend promotion**, **no stable-channel movement**, and **no alpha.4 reinterpretation**.

The next bounded research gate is P4-B: targeted success-authority requalification. It must test ptrace first for the three identified gaps and may justify a second-backend prototype only proposition-by-proposition if ptrace fails a preregistered falsification gate.