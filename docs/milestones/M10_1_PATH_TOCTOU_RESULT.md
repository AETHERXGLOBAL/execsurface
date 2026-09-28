# M10.1 — PATH-TOCTOU-001 Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer`
Protocol: `docs/milestones/M10_1_PATH_TOCTOU_PROTOCOL.md`
Source SHA: `3503a757e5230b54c6d01d77c7ed4e7715563e61`
Workflow run: `36420520715`
Artifact: `10968584240`
Artifact digest: `sha256:6f22d3fb413f1f1238984dea3a616d3e9e64185afc217efff058f0d3a3cc20f4`
Kernel: `6.17.0-1022-azure`

Status: **CLOSED — COUNTEREXAMPLE OBSERVED**
Classification: **`PTRACE_PATH_TOCTOU_COUNTEREXAMPLE_OBSERVED`**

## Decision

The preregistered universal hypothesis is **KILLED** within the declared scope:

> The pathname copied by the current ptrace observer at syscall entry is not universally identical to the pathname/object subsequently consumed by the kernel when another thread can mutate the shared userspace pathname buffer during the ptrace stop/resume interval.

This is an evidence-authority result, not a product-breakage result.

No runtime code, public release, baseline, policy, Marketplace behavior, or backend authority changed in M10.1.

## Frozen result

| Metric | Result |
|---|---:|
| requested runs | 500 |
| eligible complete runs | 500 |
| incomplete runs | 0 |
| warning runs | 0 |
| observer errors | 0 |
| fixture errors | 0 |
| missing/multiple relevant path intents | 0 |
| `PathAccessIntent` vs kernel truth mismatches | **243** |
| `FileDescriptorAccess(Read)` vs kernel truth mismatches | **0** |

Distributions:

| Evidence | A | B |
|---|---:|---:|
| kernel truth | 254 | 246 |
| ptrace pathname intent | 269 | 231 |
| fd-attributed read | 254 | 246 |

Observed mismatch fraction in this deliberately adversarial fixture: `243 / 500 = 48.6%`.

That fraction is **not** a real-workload frequency estimate. The fixture was designed to maximize the race window.

## Concrete counterexample

An accepted warning-free observation contained:

1. relevant path-attempt event:
   - `FilePathAccess(Open)` -> `.../A`
2. later successful positive-byte read attribution:
   - `FileDescriptorAccess(Read)` -> `.../B`
3. target exit code:
   - `12`, independently defined by the preregistered fixture as "the opened fd returned byte B"
4. observation state:
   - `complete == true`
   - warnings: none

Thus, in the same accepted observation, entry-time userspace pathname metadata named A while the kernel-opened object proved by the fd contents was B.

The preserved artifact contains bounded raw observation examples for both mismatch directions.

## Why the result is credible

- Protocol and classifications were committed before execution.
- The current production ptrace observer was not modified.
- The target used an atomic one-byte pathname element and a fixed NUL byte; the experiment does not rely on a torn multi-byte pathname.
- Ground truth came from the successfully opened fd's contents, not from a second pathname snapshot.
- Every interpreted run was observer-complete and warning-free.
- Both A->B and B->A directions were observed in the preserved examples.
- The post-open fd-read attribution matched independent ground truth in all 500 runs, which argues against a broken A/B truth decoder.

## What is killed

**KILLED:** treating current `PathAccessIntent` / entry-copied pathname as universally kernel-authoritative object identity under concurrent shared-memory mutation.

The correct semantic interpretation is narrower:

> `PathAccessIntent` is metadata observed from the tracee's userspace syscall argument at the ptrace entry stop. Under concurrent mutation it may differ from the bytes later consumed by the kernel.

## What survives

This result does **not** kill:

- ptrace as a useful per-command lifecycle/reference observer;
- process spawn/exec occurrence tracking as a whole;
- current fail-closed completeness machinery;
- post-success fd-attributed read/write evidence in this specific fixture;
- the public alpha within its bounded observer semantics.

Notably, `FileDescriptorAccess(Read)` matched the kernel-opened A/B object in all 500 accepted runs. This is evidence that later fd-grounded attribution can be stronger than entry-time pathname intent for this proposition. It does not prove fd attribution race-freedom generally; `FD-SHARE-RACE-001` remains mandatory.

## Architecture consequence

M10's hybrid direction is strengthened.

A kernel-hook/object-grounded backend should now be tested for propositions where the public model needs identity stronger than userspace argument intent. BPF LSM remains a candidate, not an accepted solution.

The next mandatory adversarial gate is:

`M10.2 — FD-SHARE-RACE-001`

Goal: attempt to break successful fd identity/read-write attribution under `CLONE_FILES` close/reuse concurrency before assigning stronger authority to the current fd model or to any replacement.

## Product impact

Current impact: **none**.

- `main`: unchanged by the experiment implementation/result.
- `v0.1.0-alpha.3`: unchanged.
- Marketplace Action: unchanged.
- baseline compatibility: unchanged.
- ptrace public backend: unchanged.
- BPF/LSM public authority: none.

A future mainline documentation change may clarify `PathAccessIntent` semantics, but M10 research remains isolated until closeout.
