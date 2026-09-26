# M8.3 Negative Evidence 001 — Shell Open Fixture Did Not Establish Held FD Resolution

Date: 2026-09-26
Status: **KILLED AS TEST HARNESS**
Tracking: #37

## Run

Workflow: `M8.3 Path Resolution Bridge`
Run: `36244595612`
Head: `5be8750450dbf8043b399587f1cf834c6ab2f7a2`

## What passed before the failure

- host/BTF contract — PASS;
- `sys_exit_openat` return layout — PASS;
- Rust 1.82-compatible dependency resolution — PASS;
- Rust 1.82 build — PASS;
- unprivileged BPF denial — explicit PASS;
- held exec `/proc/<pid>/exe` resolution — PASS (`/usr/bin/sleep`);
- reaped short-lived exec resolution unavailable — PASS negative lifecycle evidence.

## Failure

The shell fixture:

`/bin/sh -c 'exec 9</dev/null; sleep 2'`

produced no observed successful-open event that the harness could resolve to `/dev/null` for the shell PID before timeout.

## Classification

This result does **not** prove the userspace fd-resolution bridge is impossible. The shell adds implementation uncertainty about which syscall/process performs the redirection and how descriptor inheritance/execution is handled.

Therefore the shell fixture is killed as an evidence harness.

## Required repair

Replace it with a deterministic child mode in the Rust probe that invokes `openat(AT_FDCWD, "/dev/null", O_RDONLY)` directly, then either:

- holds the returned fd open for the positive case; or
- closes/exits before resolution for the negative lifecycle case.

The failed run remains preserved and is not rewritten as success.
