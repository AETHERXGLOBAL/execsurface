# M8.3b — Userspace Path-Resolution Bridge

Date: 2026-09-26
Status: **OPEN — EXPERIMENTAL EVIDENCE REQUIRED**
Tracking: #37
Parent: `docs/milestones/M8_3_BACKEND_CONTRACT.md`

## Question

Can the M8.2 numeric libbpf evidence be converted into the path-bearing metadata required by the existing ExecSurface raw model without fabricating semantics?

The bridge under test is deliberately narrow:

- exec occurrence `(pid)` -> `/proc/<pid>/exe`;
- successful open `(pid, fd)` -> `/proc/<pid>/fd/<fd>`;
- resolution failure -> explicit incomplete evidence, never a guessed path.

The kernel-side event remains metadata-only. No argv, environment value, file content, stdin, or network payload is persisted.

## Why this gate exists

M8.2 proved exec occurrence and successful-open fd identity. It did **not** prove exec path identity or open path identity. The current raw model requires real path metadata for `ProcessExec` and path-bearing file semantics. PID/FD numbers cannot be promoted to path claims by assumption.

## Dynamic team

Fixed:

- Innovation Scientist / Architect
- Deviation Prevention / Scientific Integrity

Dynamic:

- Linux eBPF / tracepoint engineer
- Rust/libbpf-rs engineer
- `/proc` lifecycle/race specialist
- observation-semantics reviewer
- privacy red team
- CI/reproducibility reviewer

## Experiment matrix

### E1 — held exec

Attach the libbpf collector, launch a controlled process that remains alive, observe its exec occurrence, then resolve `/proc/<pid>/exe` and verify the resolved executable identity matches the launched executable.

### E2 — reaped short-lived exec

Observe a controlled short-lived exec, wait until the process is reaped, then attempt `/proc/<pid>/exe` resolution.

Expected result: resolution is unavailable. This negative case proves that userspace resolution is lifecycle-sensitive.

### E3 — held successful open

Launch a controlled process that opens `/dev/null` and holds the returned fd open. Observe successful-open `(pid, fd)` metadata, resolve `/proc/<pid>/fd/<fd>`, and verify `/dev/null` identity.

### E4 — closed/reaped short-lived open

Observe a controlled process that opens `/dev/null` and exits, then attempt fd-path resolution after reaping.

Expected result: resolution is unavailable. This negative case proves that successful-open fd identity does not guarantee later path resolvability.

### E5 — privacy

Use an environment sentinel in controlled children. The persisted experiment output must not contain the sentinel value.

## Decision rule

The bridge is **not allowed** to claim unconditional path identity.

It may be accepted only as a **conditional fail-closed resolver** if:

1. positive held-object cases establish correct identity;
2. short-lived/closed cases visibly demonstrate the lifecycle race;
3. every unresolved event can be converted into an explicit incomplete state;
4. no guessed or stale path is emitted;
5. normal ptrace installation remains untouched.

If a stale/wrong path can be accepted as successful resolution, the bridge is **KILLED**.

## Non-claims

This experiment does not establish ptrace/eBPF parity, multi-kernel portability, performance superiority, public eBPF readiness, or eBPF-derived PASS authority.
