# ADR-0002 — Metadata-Only Native ptrace for M1

Status: **ACCEPTED — SUPERSEDES ADR-0001 FOR THE PRODUCTION M1 OBSERVER**

Date: 2026-09-25

## Context

ADR-0001 selected strace ingestion as the M1 reference backend because it offered a short, reproducible path to Linux syscall observation.

During the M1 privacy red-team gate, that choice failed a hard project invariant:

> Observe metadata, not secrets.

The strace interface is designed to display system calls and their arguments. Its normal decoding dereferences string arguments. For `execve`, those arguments can include argv values supplied by a child process. Redacting them after collection is insufficient for ExecSurface's default privacy contract because sensitive values may already have been decoded and captured by the observer.

This is not a defect in strace. It is a mismatch between a diagnostic tracer's output contract and ExecSurface's metadata-only collection contract.

## Evidence

Official strace documentation states that strace records system calls, arguments, and results, and that character pointers are normally dereferenced and printed as strings:

https://man7.org/linux/man-pages/man1/strace.1.html

Linux ptrace exposes process tracing primitives that allow a tracer to choose which tracee memory it reads, including syscall-entry state and fork/clone/exec events:

https://man7.org/linux/man-pages/man2/ptrace.2.html

A strace raw-argument mode avoids string decoding but no longer supplies the executable pathname needed by the M1 product contract. Process-name augmentation from `/proc/<pid>/comm` is not an equivalent full executable-path observation.

## Decision

M1 production observation will use a **minimal native ptrace backend written in Rust**, initially scoped to Linux x86_64.

The backend will:

- launch the declared target under ptrace;
- follow descendants using ptrace fork/vfork/clone events;
- observe selected syscall-entry metadata;
- read only specifically required pointer targets;
- read the `execve` pathname but **never dereference argv or envp**;
- read file path arguments only for selected filesystem syscalls;
- read only the sockaddr bytes required for `connect`;
- never read file contents, stdin, environment values, network payloads, or arbitrary process memory;
- surface unsupported architecture/observer failures explicitly.

## M1 scope

The backend is deliberately small.

### Process

- `process.spawn` from ptrace fork/vfork/clone events;
- `process.exec_attempt` from selected exec syscalls;
- executable pathname only;
- no argv values;
- no environment values.

### Files

M1 observes selected **path-based syscall attempts** such as open/openat, unlink and rename.

M1 does **not** claim reliable `file.read` / `file.write` semantics from fd activity until fd lifecycle attribution is implemented and tested.

### Network

M1 observes outbound `connect` attempts by decoding only the destination socket address structure.

No payload capture.

## Why not a strace + redaction layer?

**KILLED.**

Post-capture sanitization does not satisfy the default collection boundary because secret-bearing strings may exist in the trace before redaction.

## Why not raw strace + procfs command name?

**KILLED.**

Raw syscall arguments avoid string decoding but do not provide the required executable path. `/proc/<pid>/comm` is not a full-path substitute and introduces a separate snapshot/race model.

## Why not eBPF now?

**DEFERRED.**

The M0 reasons remain valid: GitHub Actions portability, privilege/capability variance and kernel/backend complexity are not justified before the core execution-surface abstraction is proven.

## Consequences

Positive:

- privacy boundary is enforced at collection time rather than repaired afterward;
- the event schema can represent exactly what was intentionally read;
- no secret-bearing argv/env data is required for executable observation;
- backend limitations remain explicit.

Negative:

- ptrace state-machine correctness becomes M1 engineering work;
- x86_64 register/syscall semantics are initially architecture-specific;
- ptrace can perturb scheduling/timing;
- trace-aware programs may change behavior;
- syscall coverage starts narrow.

## Required privacy invariant

For every M1 integration test, serializing the full observer result must not reveal a sentinel passed only as a child argv value.

A dedicated test must include a high-entropy sentinel and fail if it appears anywhere in output.

## Evidence status

- strace default-output privacy mismatch: **KILLED BY EVIDENCE**
- native ptrace metadata-only design: **ACCEPTED**
- implementation correctness: **OPEN until M1 tests**
- Linux x86_64 support: **OPEN until CI**
- performance: **OPEN**
