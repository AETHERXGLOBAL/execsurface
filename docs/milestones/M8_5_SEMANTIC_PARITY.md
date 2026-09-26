# M8.5 — ptrace/eBPF Semantic Parity & Cross-Backend Comparability

Date: 2026-09-26
Status: **M8.5a UNDER EXECUTION — CONTRACT FROZEN / HARNESS NEXT**
Tracking: #42
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
Reference backend: `linux-ptrace-metadata-v2`
Candidate backend: `linux-libbpf-metadata-experimental-v1`

## Objective

Determine, per semantic capability, whether the experimental eBPF evidence is actually equivalent to the ptrace correctness reference, a strict subset, a representation difference, non-comparable, contradicted, or blocked by incomplete evidence.

M8.5 does **not** compare raw event counts, raw PIDs/TIDs, or raw sequence numbers as evidence of semantic parity.

## Fixed governance roles

- **Innovation Scientist / Architect:** define the strongest backend-independent semantic projection and comparability model without coupling product semantics to ptrace or eBPF implementation details.
- **Deviation Prevention / Scientific Integrity:** reject event-name/count similarity as proof, preserve counterexamples, classify every mismatch, and prevent premature eBPF PASS authority.

Independent parity red-team review is mandatory before closure.

## Parity states

Machine-readable parity verdicts are:

- `equivalent` — the selected semantic class has the same backend-independent meaning and the controlled witnesses agree.
- `reference_subset` — the ptrace evidence is a proved subset of candidate evidence for the selected class.
- `candidate_subset` — the eBPF evidence is a proved subset of ptrace evidence for the selected class.
- `representation_difference` — both backends establish related facts, but the current evidence interfaces do not expose an equivalent semantic projection.
- `non_comparable` — one backend does not claim the semantic class or the current proof obligations cannot be aligned.
- `contradicted` — both backends claim the class, evidence is sufficiently complete, and the controlled semantic projections disagree.
- `blocked_incomplete` — loss, truncation, decode, collector, lifecycle, or equivalent evidence-health failure prevents a parity conclusion.

## Structural identity rule

Separate ptrace and eBPF runs have different runtime process identifiers and scheduling sequences. Therefore:

- raw PID/TID equality is forbidden as a parity criterion;
- raw event sequence equality is forbidden as a parity criterion;
- process lineage is projected into structural roles such as `root`, `root/fork#1`, `root/fork#1/clone#1`;
- occurrence indexes are local to one parent and spawn mechanism and are used only in deterministic fixtures where that ordering is part of the workload contract;
- later adversarial fixtures must challenge any ordering assumption before it is generalized.

## Health gate

Parity is evaluated only after evidence-health classification.

Hard blockers for selected-class parity:

- `incomplete_loss`;
- `incomplete_limit`;
- `incomplete_decode`;
- `incomplete_collector`;
- `incomplete_lifecycle`;
- ptrace `complete=false` when the warning is not solely an explicitly out-of-scope capability declaration.

The eBPF collector currently reports `incomplete_capability` even when a selected shared capability was collected without known loss, because the candidate intentionally supports only a strict subset of the full ExecSurface surface. That state:

- does **not** authorize full-surface equivalence;
- may permit class-local comparison only for capabilities explicitly declared supported by both backends and only when no hard blocker is present.

Full-surface cross-backend comparability remains **false** while any required capability is unsupported or unproved.

## Capability intersection at M8.5 start

The ptrace descriptor supports a much broader surface. The experimental eBPF descriptor currently supports only:

1. `ProcessSpawnLineage`
2. `ProcessExecOccurrence`
3. `SuccessfulOpenFdIdentity`
4. `LossTruncationVisibility`

Every other declared class is initially `non_comparable` unless a later M8.5 subgate expands and proves the candidate capability contract.

## Initial semantic projections

### ProcessSpawnLineage

For deterministic single-threaded fork fixtures, project each backend to a structural edge:

`parent_role + spawn_mechanism -> child_role`

Numeric process identities are discarded after establishing the edge inside each individual run.

Initial acceptance workload:

`root fixture -> fork -> child exec helper`

A later clone/thread counterexample is mandatory before any claim extends fork parity to generic clone/thread semantics.

### ProcessExecOccurrence

Compare exec occurrence by structural process role, not path string and not numeric PID/TID.

This is deliberately narrower than `ProcessExecPathIdentity`.

A path-string mismatch cannot contradict occurrence parity because the candidate does not yet claim unconditional `ProcessExecPathIdentity`.

### SuccessfulOpenFdIdentity — representation gap at entry

This class is **not equivalent at M8.5 start** despite both descriptors declaring support.

Current ptrace behavior establishes a successful open internally on syscall exit and stores the returned FD/path in its FD table. The v2 raw observation model does not expose a dedicated successful-open event; its public open event is a pathname **attempt** and the successful identity becomes observable later only through attributed FD activity.

The experimental eBPF report, by contrast, explicitly emits `successful_open_identity { pid, fd, path? }` after successful `openat`.

Therefore direct raw-event equality would compare different propositions. Initial verdict: `representation_difference` pending a defensible backend-independent successful-open projection or an evidence-schema evolution under a separate gate.

### LossTruncationVisibility

M8.4 already proved eBPF loss/truncation visibility and fail-closed behavior. M8.5 treats this as a health prerequisite rather than assuming event-count equality proves transport parity.

A controlled same-workload candidate run with a low event limit must yield `blocked_incomplete` rather than any positive parity verdict.

## First differential harness gate — M8.5a

The first harness must:

1. run one deterministic fixture independently under ptrace and eBPF;
2. verify backend identities;
3. map runtime identities to structural roles separately in each run;
4. compare `ProcessSpawnLineage` and `ProcessExecOccurrence` only;
5. emit `SuccessfulOpenFdIdentity=representation_difference` rather than fabricate a projection;
6. emit all unsupported classes as `non_comparable`;
7. report `full_surface_comparable=false`;
8. preserve eBPF PASS authority as false;
9. reject a deliberately altered same-count/different-semantics candidate as `contradicted`;
10. reject a same-workload incomplete candidate as `blocked_incomplete`.

## Non-negotiable boundaries

- ptrace remains the correctness reference.
- eBPF PASS authority remains **NOT AUTHORIZED** during M8.5.
- selected-scenario parity cannot be generalized to unsupported capability classes.
- same event count is not semantic equivalence.
- raw runtime identity equality is not semantic equivalence.
- a representation gap is not silently converted into equivalence.
- incomplete evidence cannot prove parity.
