# Changelog

## Unreleased

### 2026-09-25 — M4 Diff Engine accepted

- added `execsurface-diff` crate;
- added verified baseline-to-candidate comparison;
- added explicit comparability gates for platform, observer capabilities, canonical schema, normalization profile/root labels and command identity;
- added deterministic added/removed findings;
- added conservative changed pairing that refuses ambiguous one-to-many or many-to-many pairing;
- added `execsurface check` with text and JSON output;
- kept drift policy-free: successful comparison exits 0 even when drift exists;
- proved real `learn → check` no-drift behavior;
- proved controlled process drift is surfaced;
- proved corrupted baseline rejection and argv privacy in JSON reports;
- preserved failed formatting CI evidence.


### 2026-09-25 — M3 Baseline Lock accepted

- added `execsurface-baseline` crate and lock schema v1;
- added `execsurface learn -- COMMAND` with default `execsurface.lock.json` output;
- froze digest-format v1 as SHA-256 over deterministic minified JSON excluding the digest field;
- pinned `sha2 = 0.10.9` and generated Cargo.lock through GitHub Actions rather than manually inventing checksums;
- added fixed serialization and SHA-256 test vectors;
- added privacy-safe command identity using canonical executable, argument count and optional explicit label;
- excluded full argv values from the lockfile and proved an argv sentinel is absent;
- added atomic publication and explicit `--overwrite` semantics;
- reject failed/signaled target commands without creating a baseline;
- proved repeat learning produces byte-identical lockfiles for a controlled command;
- proved a controlled canonical effect change changes the baseline digest;
- added JSON Schema for lockfile v1;
- preserved M3 bootstrap and CI failures in the evidence ledger.


### 2026-09-25 — M2 Canonicalization accepted

- added backend-independent canonical execution-surface model;
- added versioned deterministic normalization profile;
- added explicit semantic roots for workspace, home, temp, run-temp and named caches;
- rejected generic random-looking path heuristics;
- removed PID/TID/sequence from canonical identity while preserving causal replay;
- added deterministic effect ordering and deduplication;
- preserved executable/path suffixes and remote destination ports;
- marked relative and parent-traversal paths unresolved instead of guessing;
- prevented semantic roots from shadowing credential-sensitive namespaces;
- preserved Linux open access intent rather than silently dropping flags;
- proved repeated real ptrace observation of the same command canonicalizes identically in CI;
- retained M2 failed CI evidence.


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
