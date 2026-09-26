# M8.3c — Real libbpf Collector Integration

Date: 2026-09-26
Status: **IMPLEMENTATION GATE — OPEN UNTIL CI EVIDENCE**
Tracking: #37
Parent: `docs/milestones/M8_3_BACKEND_CONTRACT.md`

## Objective

Turn the M8.2/M8.3b libbpf experiments into a production-shaped **experimental companion collector** without adding libbpf/native build requirements to the normal ExecSurface install path.

M8.3c does not authorize cross-backend parity, eBPF-derived PASS, automatic backend selection, or a stable public eBPF release.

## Architecture decision

The native libbpf collector remains isolated under `experiments/m8-ebpf/libbpf-collector` and communicates through a versioned JSON report. The default `ptrace` path and normal workspace dependency graph remain unchanged.

The companion collector is responsible for:

- attaching the selected libbpf programs;
- launching the target command itself;
- filtering the global trace stream down to the launched process tree;
- resolving exec identity through `/proc/<pid>/exe` only after `sched_process_exec`;
- recording successful-open fd identity and an optional live `/proc/<pid>/fd/<fd>` resolution;
- exposing producer drop counts;
- exposing target exit/signal outcome;
- marking event-budget or path-resolution uncertainty explicitly incomplete;
- persisting no argv values, environment values, file contents, stdin, or network payloads.

## Important semantic boundary

`ProcessExecOccurrence` and `ProcessExecPathIdentity` remain distinct capabilities.

The kernel program reports exec occurrence after image replacement using `sched_process_exec`. Userspace emits a path-bearing exec record only when `/proc/<pid>/exe` resolves while the process is live. An unresolved exec occurrence is retained as a completeness failure, not converted into a guessed path.

Successful open fd identity is retained even when the optional fd path cannot be resolved. `OpenPathIdentity` therefore remains an unsupported **unconditional** capability.

## M8.3c1 acceptance

The dedicated CI gate must prove all of the following on the declared Ubuntu 24.04 reference runner:

1. the collector builds from declared inputs with the product Rust 1.82 target;
2. unprivileged BPF denial produces an explicit error report and nonzero exit;
3. a real libbpf run captures the launched target's exec identity;
4. a held successful `openat` produces fd metadata and resolves `/dev/null` while live;
5. argv/environment sentinels are absent from the persisted report;
6. producer drop count is exported;
7. forced exec-path resolution failure produces `incomplete_capability` and no fabricated path event;
8. an exceeded event budget produces `incomplete_limit`;
9. normal repository CI remains green;
10. the root workspace and ordinary ptrace installation acquire no libbpf dependency.

## Next subgate — M8.3c2

Only after M8.3c1 succeeds may the main CLI gain an explicit experimental observation route. The intended form is:

`execsurface observe --backend experimental-libbpf -- COMMAND [ARGS...]`

That route must invoke the companion collector without privilege escalation, must not silently fall back to ptrace, and must remain unavailable to `learn` / `check` until the later parity gate.

## Labels

- real libbpf companion collector: **OPEN UNTIL CI**
- default ptrace path: **RETAINED**
- ordinary install dependency graph: **MUST REMAIN UNCHANGED**
- automatic backend selection: **NOT AUTHORIZED**
- eBPF PASS authority: **NOT AUTHORIZED**
- cross-backend parity: **OPEN / M8.5**
