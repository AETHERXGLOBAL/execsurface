# Changelog

## Unreleased

### 2026-09-25 — M1 Minimal Linux Observer accepted

- implemented a Rust workspace with raw observation model, metadata-only native ptrace backend and experimental `execsurface observe -- COMMAND` CLI;
- added descendant exec, selected path-based filesystem, and connect-destination observation;
- added privacy sentinel test proving argv-only sentinel absence from serialized observation;
- serialized observer sessions within one host process after concurrency testing exposed cross-reaping risk;
- pinned the Rust dependency graph in Cargo.lock;
- passed format, strict Clippy, locked tests and lockfile-integrity checks on GitHub Actions;
- preserved failed CI iterations in the M1 evidence ledger.


### 2026-09-25 — M1 Privacy Gate

- red-team review found that decoded strace output can collect string syscall arguments before redaction;
- preserved the negative result instead of weakening the privacy contract;
- ADR-0002 supersedes ADR-0001 for the M1 production observer;
- M1 production backend changed to a minimal metadata-only native Linux ptrace implementation;
- strace may remain a controlled test/debug oracle only when no secrets are present.

### 2026-09-25 — M0 Architecture Freeze

- established problem statement, boundaries and threat/privacy model;
- separated raw observation from canonical execution surface;
- separated baseline from policy;
- defined normalization evidence requirements;
- defined PASS / REVIEW / BLOCK / ERROR;
- selected strace ingestion for the M1 reference observer at the M0 gate;
- killed procfs-only observation;
- deferred eBPF-first implementation;
- defined MVP and M1 authorization boundary.
