# M8.3c — Real libbpf Observer Integration

Date: 2026-09-26
Status: **M8.3c1 CLOSED / PROVED ON REFERENCE CI — M8.3c2 CLI WIRING OPEN**
Tracking: #37
Parent: `docs/milestones/M8_3_BACKEND_CONTRACT.md`

## Objective

Turn the M8.2/M8.3b libbpf experiments into a production-shaped **experimental companion observer** without adding libbpf/native build requirements to the normal ExecSurface install path.

M8.3c does not authorize cross-backend parity, eBPF-derived PASS, automatic backend selection, or a stable public eBPF release.

## Accepted implementation

The accepted real observer is isolated under:

`experiments/m8-ebpf/libbpf-observer`

It communicates through a versioned JSON report and is checked against the central typed backend contract by the independent `contract-check` binary. The default ptrace path and normal workspace dependency graph remain unchanged.

The companion observer:

- attaches real libbpf programs;
- launches the target command itself only after BPF load/attach succeeds;
- filters the global trace stream to the launched process tree;
- observes post-image exec occurrences through `sched_process_exec`;
- conditionally resolves `/proc/<pid>/exe` while the object is live;
- records successful-open fd identity and conditionally resolves `/proc/<pid>/fd/<fd>`;
- exports producer drop counts;
- exports target exit/signal outcome;
- marks event-budget, event-loss, and capability/path uncertainty explicitly incomplete;
- persists no argv values, environment values, file contents, stdin, or network payloads.

## Important semantic boundary

`ProcessExecOccurrence` and `ProcessExecPathIdentity` remain distinct capabilities.

A path-bearing exec/open record is emitted only when that individual live `/proc` resolution succeeds. Path resolution success does not promote path identity to an unconditional backend capability. No path is guessed.

The observer intentionally keeps `observation_complete=false` and `completeness=incomplete_capability` during ordinary M8.3c runs because the backend still supports only a strict subset of the ptrace reference capability universe. Therefore M8.3c cannot produce evidence-equivalent PASS.

## M8.3c1 evidence

Reference implementation head:

`95ec2b7460f8c18a8e1fb5034ac7457c12127f8a`

Dedicated workflow:

- workflow: `M8.3 Real libbpf Observer`
- run: `36245671015`
- result: **SUCCESS**
- job: `isolated real libbpf observer` — **SUCCESS**

Normal repository CI on the same head:

- run: `36245671113`
- result: **SUCCESS**

Reference host evidence:

- Ubuntu 24.04.5;
- Linux `6.17.0-1022-azure` x86_64;
- kernel BTF readable;
- unprivileged BPF disabled by host policy (`2`);
- `sched_process_exec` tracepoint audited;
- `sys_exit_openat` return field verified at offset 16, size 8.

Build evidence:

- product-compatible compiler: Rust `1.82.0`;
- dependency resolver: Cargo `1.87.0` with incompatible-Rust fallback;
- libbpf-rs/libbpf-cargo `0.27.0`;
- observer SHA-256: `9ade711dfd97d95b7ee489e0f061b2916616e239fb8f89fcf700c7b79c2675e3`;
- contract-check SHA-256: `df3c30eca041bc81447c5789081a5134848d297be335e917754ed0e1fb18d71b`.

The measured M8.3c1 observer binary dynamically links libelf/zlib/zstd on the reference CI host. M8.2 separately proved a vendored packaging path; this M8.3c1 run does not claim that packaging has already been applied to the real observer artifact.

### Fail-closed and semantic proof markers

Unprivileged execution:

- BPF load failed explicitly with EPERM;
- target launch sentinel remained absent;
- report remained absent;
- `M8_3C_UNPRIVILEGED_PRELAUNCH_DENIAL_PASS`.

Real exec/open evidence:

- target exit code `0`;
- dropped events `0`;
- process exec occurrence included the actual fixture executable path;
- successful open identity resolved `/dev/null` while live;
- persisted privacy sentinel remained absent;
- `M8_3C_REAL_EXEC_OPEN_REPORT_PASS`;
- `M8_3C_REPORT_PRIVACY_PASS`.

Central contract synchronization:

- backend id `linux-libbpf-metadata-experimental-v1`;
- supported capability count `4`;
- unsupported capability count `12`;
- `M8_3C_CENTRAL_CONTRACT_PASS`.

Launched-tree lineage:

- a child spawn rooted at the launched process was observed;
- a child exec matched the rooted child identity;
- `M8_3C_LINEAGE_REPORT_PASS`.

Event-budget fail-closed behavior:

- controlled `--event-limit 1` run produced `incomplete_limit`;
- `observation_complete=false`;
- `event_limit_exceeded` warning present;
- at most one retained event;
- `M8_3C_EVENT_LIMIT_FAIL_CLOSED_PASS`.

## M8.3c1 decision

**CLOSED / PROVED ON THE DECLARED REFERENCE CI ENVIRONMENT.**

This proves a real, isolated, contract-checked libbpf observer with process-tree scoping and explicit fail-closed behavior. It does not prove parity with ptrace, general kernel portability, lower overhead, or PASS authority.

## M8.3c2 — explicit CLI observation bridge

The next subgate may expose only:

`execsurface observe --backend experimental-libbpf --collector PATH -- COMMAND [ARGS...]`

Requirements:

1. default `execsurface observe -- COMMAND` remains ptrace and unchanged;
2. the companion path is explicit — no hidden discovery and no silent fallback;
3. ExecSurface never invokes `sudo` or changes host security policy;
4. companion protocol/backend identity is validated before its report is accepted;
5. the CLI rejects any experimental report claiming `observation_complete=true` before the later parity/authority gates;
6. `learn` and `check` remain ptrace-only;
7. missing/failed companion execution is an explicit error;
8. normal CI, distribution probe, and Action smoke remain green.

## Labels

- M8.3a typed backend contract: **CLOSED / PROVED**
- M8.3b conditional `/proc` path bridge: **CLOSED / PROVED FAIL-CLOSED**
- M8.3c1 real libbpf observer: **CLOSED / PROVED ON REFERENCE CI**
- M8.3c2 explicit CLI observation bridge: **OPEN**
- default ptrace path: **RETAINED**
- ordinary install dependency graph: **UNCHANGED**
- automatic backend selection: **NOT AUTHORIZED**
- eBPF PASS authority: **NOT AUTHORIZED**
- cross-backend parity: **OPEN / M8.5**
