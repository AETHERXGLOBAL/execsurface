# M8.3c — Isolated Real libbpf Collector

Date: 2026-09-26
Status: **OPEN — IMPLEMENTATION EVIDENCE REQUIRED**
Tracking: #37
Parent contract: `docs/milestones/M8_3_BACKEND_CONTRACT.md`

## Objective

Build a real, opt-in libbpf-rs collector that launches a target command, observes only metadata relevant to the launched process tree, writes machine-readable evidence to a separate report file, and preserves the normal ptrace installation path unchanged.

This is still an experimental backend. It is not a parity gate and cannot authorize eBPF-derived PASS.

## Architecture decision

The native libbpf implementation remains outside the default Cargo workspace under:

`experiments/m8-ebpf/libbpf-observer/`

This keeps libbpf/libelf/zlib/native build requirements out of ordinary ptrace-only builds.

The collector report is an M8.3 experimental protocol. It deliberately does not masquerade as the stable raw Observation schema where the semantic mapping is not yet proved.

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

## Process-tree scope

The root PID is known from the command launched by the collector. Fork/clone/vfork exit evidence extends the process-tree scope. Only evidence attributable to that tree is retained in the report.

## Completeness

The M8.3c collector is intentionally not PASS-eligible. Even on an otherwise healthy run it reports `incomplete_capability` because major ptrace semantic classes remain unsupported.

Higher-priority health failures override that state:

- producer drop counter > 0 -> `incomplete_loss`;
- event budget exceeded -> `incomplete_limit`;
- report/collection protocol failure -> error/failure;
- event-specific path resolution failure is explicitly recorded and cannot be treated as established path identity.

## Required evidence

1. clean isolated build on Rust 1.82-compatible dependencies;
2. unprivileged load denial is explicit and launches no target;
3. controlled target exec path resolves from a post-success exec event;
4. deterministic successful-open identity resolves to `/dev/null` while live;
5. process-tree lineage is recorded for a controlled child workload;
6. command exit status is recorded;
7. drop accounting is present;
8. privacy sentinel is absent from report;
9. normal repository CI remains green;
10. no changes to default ptrace selection or public verdict semantics.

## Non-claims

- no ptrace/eBPF parity;
- no cross-backend evidence equivalence;
- no performance superiority;
- no universal kernel support;
- no automatic backend selection;
- no production/public stable eBPF release;
- no eBPF-derived PASS authority.
