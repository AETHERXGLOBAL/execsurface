# M8.3c — Isolated Real libbpf Collector

Date: 2026-09-26
Status: **M8.3c1 CLOSED / PROVED ON REFERENCE CI · M8.3c2 OPEN**
Tracking: #37
Parent contract: `docs/milestones/M8_3_BACKEND_CONTRACT.md`
M8.3b resolver evidence: `docs/milestones/M8_3B_PATH_RESOLUTION_BRIDGE.md`

## Objective

Build a real, opt-in libbpf-rs collector that launches a target command, observes only metadata relevant to the launched process tree, writes machine-readable evidence to a separate report file, and preserves the normal ptrace installation path unchanged.

This remains an experimental backend. M8.3c1 is not a parity gate and does not authorize eBPF-derived PASS.

## Architecture decision

The native libbpf implementation remains outside the default Cargo workspace under:

`experiments/m8-ebpf/libbpf-observer/`

This keeps libbpf/libelf/zlib/native build requirements out of ordinary ptrace-only builds.

The collector report is an M8.3 experimental protocol. It deliberately does not masquerade as the stable raw `Observation` schema where semantic mapping has not been proved.

The serialized report is checked against the central typed `BackendDescriptor` in `execsurface-observe`. Capability drift, duplicate capability declarations, `observation_complete=true`, or `completeness=complete` are rejected by the M8.3c contract checker.

## Exec event hardening

M8.3c observes exec occurrence using `sched_process_exec`, not `sys_enter_execve`.

Reason: the userspace resolver must not race against the pre-exec executable image and accidentally read the old `/proc/<pid>/exe` target. `sched_process_exec` is a post-success exec event and therefore provides the stronger event boundary for prompt userspace executable-path resolution.

The exact reference-host tracepoint availability is audited in CI.

## Successful open semantics

Successful `openat` is observed at `sys_exit_openat` using the audited return-value layout. The collector persists:

- pid;
- returned fd;
- conditionally resolved `/proc/<pid>/fd/<fd>` path when still live.

It does **not** convert this evidence into `FilePathAccess(Open)` because the existing semantic means path access intent and requires flags. M8.3c preserves successful-open identity as backend-specific evidence until a later semantic gate explicitly maps it.

M8.3b separately proved that `/proc` path resolution is lifecycle-sensitive. The bridge is accepted only as a conditional fail-closed resolver; an unresolved event never becomes a guessed path.

## Process-tree scope

The root PID is known from the command launched by the collector. Fork/clone/vfork exit evidence extends the process-tree scope. Only evidence attributable to that tree is retained in the report.

## Completeness

The M8.3c collector is intentionally not PASS-eligible. Even on an otherwise healthy run it reports `incomplete_capability` because major ptrace semantic classes remain unsupported.

Higher-priority health failures override that state:

- producer drop counter > 0 -> `incomplete_loss`;
- event budget exceeded -> `incomplete_limit`;
- report/collection protocol failure -> error/failure;
- event-specific path resolution failure is explicitly recorded and cannot be treated as established path identity.

## M8.3c1 evidence closure

### Real collector and central-contract gate

Latest accepted real-collector rerun:

- workflow: `M8.3 Real libbpf Observer`
- run: `36245871938`
- source head: `c22626741ded6cc9c93715ba0ab2662cd58b4a99`
- reference host: GitHub-hosted Ubuntu 24.04.5, Linux `6.17.0-1022-azure`, x86_64
- result: **SUCCESS**

The run proved:

1. Rust 1.82-compatible isolated builds for the real collector, central-contract checker, and deterministic fixture;
2. `sched_process_exec` availability and `sys_exit_openat.ret` layout at offset 16 / size 8;
3. unprivileged BPF load denial fails before the target launches (`M8_3C_UNPRIVILEGED_PRELAUNCH_DENIAL_PASS exit=1`), with no report and no target side effect;
4. a privileged real libbpf run captured post-success exec evidence and successful-open `/dev/null` identity (`M8_3C_REAL_EXEC_OPEN_REPORT_PASS`);
5. the normal controlled run reported `incomplete_capability`, `observation_complete=false`, and `dropped_events=0`;
6. privacy sentinel content was absent from the persisted report (`M8_3C_REPORT_PRIVACY_PASS`);
7. the serialized report exactly matched the central typed capability contract: 4 supported and 12 unsupported capabilities (`M8_3C_CENTRAL_CONTRACT_PASS`);
8. launched-tree lineage was established for a controlled child workload (`M8_3C_LINEAGE_REPORT_PASS`);
9. an event budget of one produced `incomplete_limit`, one retained event, explicit `event_limit_exceeded`, and remained rejected from complete/PASS semantics (`M8_3C_EVENT_LIMIT_FAIL_CLOSED_PASS`).

Measured binaries from the accepted rerun:

- observer SHA-256: `9ade711dfd97d95b7ee489e0f061b2916616e239fb8f89fcf700c7b79c2675e3`;
- central-contract checker SHA-256: `df3c30eca041bc81447c5789081a5134848d297be335e917754ed0e1fb18d71b`.

The measured non-vendored experimental observer links system `libelf`, `libz`, and `libzstd`. That dependency surface is accepted only inside this isolated experiment. M8.2 separately proved a vendored packaging path. No libbpf/native dependency is added to the root workspace or ordinary ptrace installation.

### Deterministic unresolved-exec fail-closed gate

Source head:

`23be3c174e1d3d3335817298d3936836ed40e6e6`

Workflow:

- `M8.3 Exec Resolution Failure`
- run `36245897942`
- result: **SUCCESS**

The gate uses an external `LD_PRELOAD` fault injector so the production-shaped collector binary itself is unchanged. It forces only `/proc/<pid>/exe` `readlink` resolution to fail.

Observed evidence:

- real target execution completed successfully;
- collector remained `completeness=incomplete_capability` with `dropped=0`;
- the root exec occurrence remained in evidence;
- the root exec path was `null`, not fabricated;
- `exec_path_resolution_unavailable` was emitted;
- `conditional_path_resolution` was emitted;
- `M8_3C_FORCED_EXEC_RESOLUTION_FAIL_CLOSED_PASS`;
- central contract still passed with 4 supported / 12 unsupported capabilities.

Fault-injector SHA-256:

`9643b6d2479374d9419324192daa8680dc2e0c19eb35b8e653e027823dac1808`

### Normal repository CI

On head `23be3c174e1d3d3335817298d3936836ed40e6e6`:

- `CI` run `36245897897` — **SUCCESS**;
- format — SUCCESS;
- clippy — SUCCESS;
- tests — SUCCESS;
- lockfile integrity — SUCCESS.

Repository comparison against `main` confirms M8.3 changes are confined to M8.3 workflows/docs, the internal observer contract, and isolated experiment directories. The root `Cargo.toml` and root `Cargo.lock` are unchanged.

## Current capability boundary

Supported by the experimental backend contract:

- process spawn / lineage;
- process exec occurrence;
- successful open -> fd identity;
- loss / truncation visibility.

Still unsupported as unconditional capabilities include:

- process exec path identity;
- successful-open path identity;
- process exit;
- path access intent;
- fd read/write effects;
- fd duplication / close lifecycle;
- fork fd inheritance;
- close-on-exec;
- rename/delete effects;
- network connect destination;
- trace-time relative path semantics;
- causal executable chain.

## M8.3c1 decision

**ACCEPTED ON THE DECLARED REFERENCE CI ENVIRONMENT.**

This proves a real isolated libbpf collector with explicit partial capability, launched-tree scoping, producer loss accounting, conditional path enrichment, deterministic failure handling, and exact binding to the central typed backend contract.

It does **not** prove ptrace/eBPF parity, cross-backend evidence equivalence, universal kernel portability, performance superiority, or eBPF-derived PASS authority.

## Next subgate — M8.3c2

M8.3c2 may expose an explicit experimental CLI observation route only while preserving the same boundaries. Intended form:

`execsurface observe --backend experimental-libbpf --collector <PATH> -- COMMAND [ARGS...]`

Requirements:

- existing `execsurface observe -- COMMAND` remains ptrace and unchanged;
- no automatic privilege elevation;
- no silent fallback to ptrace;
- the companion collector path is explicit;
- missing collector, permission denial, collector failure, or invalid report is an explicit error;
- `learn` and `check` remain ptrace-only until the later parity gate;
- ordinary `cargo install execsurface --locked` remains free of libbpf/native build requirements;
- the experimental route surfaces incomplete capability/loss/limit states rather than translating them into PASS;
- no automatic backend selection is introduced.

## Labels

- M8.3c1 real libbpf companion collector: **PROVED ON REFERENCE CI**
- deterministic unresolved-exec fail-closed behavior: **PROVED ON REFERENCE CI**
- M8.3b conditional `/proc` path resolver: **PROVED / FAIL-CLOSED ON REFERENCE CI**
- central typed contract binding: **PROVED ON REFERENCE CI**
- default ptrace path: **RETAINED**
- ordinary install dependency graph: **UNCHANGED**
- M8.3c2 explicit CLI route: **OPEN**
- automatic backend selection: **NOT AUTHORIZED**
- eBPF PASS authority: **NOT AUTHORIZED**
- cross-backend parity: **OPEN / M8.5**

## Non-claims

- no ptrace/eBPF parity;
- no cross-backend evidence equivalence;
- no performance superiority;
- no universal kernel support;
- no automatic backend selection;
- no production/public stable eBPF release;
- no eBPF-derived PASS authority.
