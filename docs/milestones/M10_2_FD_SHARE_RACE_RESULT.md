# M10.2 — FD-SHARE-RACE-001 Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer`
Protocol: `docs/milestones/M10_2_FD_SHARE_RACE_PROTOCOL.md`
Source SHA: `6f6b3b35185dcbe111f4855901363f4dbe850769`
Workflow run: `36421409534`
Artifact: `10969715825`
Artifact digest: `sha256:38e538302613aa852575a861c387cac6576fa87ec6aec6ad88bf4ed32253a69c`
Kernel: `6.17.0-1022-azure`

Status: **CLOSED — COUNTEREXAMPLE OBSERVED**
Classification: **`PTRACE_FD_ATTRIBUTION_COUNTEREXAMPLE_OBSERVED`**

## Decision

The preregistered universal shared-fd attribution hypothesis is **KILLED** within the declared adversarial scope.

Under concurrent `CLONE_FILES`-equivalent pthread close/reuse of one numeric fd, the target independently recorded **2851 successful one-byte A/B reads**, while the unchanged ExecSurface ptrace observer emitted only **2018 relevant `FileDescriptorAccess(Read)` events** and still reported the observation as complete with zero warnings.

The decisive counterexample is therefore the **event-count divergence under accepted-complete evidence**.

No runtime code, public release, Marketplace behavior, baseline, policy, or backend authority changed in M10.2.

## Frozen result

| Metric | Result |
|---|---:|
| target exit code | 0 |
| observation complete | true |
| warning count | 0 |
| target-proven successful A/B reads | **2851** |
| emitted A/B fd-read events | **2018** |
| emitted minus truth count | **-833** |
| raw positional mismatch counter | 975 |
| observation event count | 4053 |
| invalid truth bytes | 0 |

The raw positional mismatch count (`975`) is retained as diagnostic evidence only. Once events are missing, naive index-by-index alignment is shifted and must not be interpreted as 975 independent wrong-object attributions.

The scientifically decisive fact is:

> 833 target-proven successful relevant reads had no corresponding relevant fd-read evidence, despite `Observation.complete == true` and zero warnings.

## Why the count divergence matters

The current observer records a read syscall at entry as a pending operation containing only the numeric fd. At syscall exit, when the read returned a positive byte count, it calls the observer's current internal fd-table lookup and emits an event only if the fd still has a tracked mapping.

In Linux, a read can acquire a reference to a `struct file` and later complete successfully even if another thread sharing the fd table closes or reuses the same numeric descriptor while that read is in flight.

The adversarial fixture intentionally exercises that condition:

1. main thread repeatedly executes `pread(N, ..., 1, 0)`;
2. mutator thread repeatedly closes and reopens the same numeric fd `N` as A or B;
3. main thread records every successful byte returned by the kernel in userspace memory;
4. after concurrency ends, the target writes the ordered truth sequence;
5. the harness compares that truth count with relevant ExecSurface fd-read events.

A successful read therefore can remain real at the kernel file-object level even when the numeric fd mapping has changed by the time ExecSurface performs its exit-time fd-table lookup.

## What is killed

**KILLED:** the universal claim that current positive-byte fd read/write attribution is complete under arbitrary shared-fd close/reuse concurrency merely because the observation is marked complete.

More narrowly, the present model can silently omit a successful read effect when:

- the syscall began with numeric fd `N`;
- the kernel acquired the underlying file object;
- another thread sharing the fd table closes/reuses `N` before ExecSurface's exit-time attribution;
- the read still completes successfully;
- the observer's current fd-table lookup no longer has the original mapping.

The current `complete` bit does not detect this semantic loss in the tested case.

## What is NOT established

M10.2 does not establish:

- that 833 events would be lost in ordinary workloads;
- a universal loss rate;
- that every positional mismatch is a wrong-object attribution;
- exploitability or security severity;
- that BPF LSM is correct or sufficient;
- that ptrace must be removed entirely;
- that existing public users have encountered this race.

The fixture is intentionally adversarial and designed to stress a concurrency boundary.

## Relationship to M10.1

M10.1 killed universal kernel authority for **entry-time pathname intent** under shared-memory mutation.

M10.2 independently kills universal completeness for **post-success fd read attribution** under shared-fd close/reuse concurrency.

Together they establish that two distinct authority-sensitive surfaces can diverge:

1. userspace pointer snapshot vs kernel-consumed pathname/object;
2. exit-time numeric-fd table state vs the `struct file` reference actually used by an in-flight I/O operation.

This materially strengthens the case for evaluating kernel-object-grounded hooks.

## Architecture consequence

The next gate must stop asking whether a different backend is merely faster and ask whether it can bind evidence closer to the kernel object/credential actually participating in the operation.

Next mandatory gate:

`M10.3 — BPF-LSM-FILE-001`

Objective:
- build an **audit-only**, research-only BPF LSM file/open evidence prototype;
- perform no enforcement (always allow);
- capture explicit session membership and loss/health state;
- determine whether the hook can provide object-grounded evidence that closes the M10.1/M10.2 authority gaps for selected propositions;
- keep public ptrace behavior unchanged.

## Product impact

Current product impact: **none**.

- `main`: unchanged by M10 implementation.
- `v0.1.0-alpha.3`: unchanged.
- Marketplace Action: unchanged.
- public ptrace backend: unchanged.
- baseline compatibility: unchanged.
- BPF/LSM public PASS authority: none.

Any future product correction requires M10 closeout and explicit migration/compatibility decisions.