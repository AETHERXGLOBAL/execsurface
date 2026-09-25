# Changelog

## Unreleased

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
