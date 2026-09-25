# Changelog

## Unreleased

### 2026-09-25 — M6 GitHub Action accepted

- added root `action.yml` composite Action for Linux x86_64;
- added source-build execution with locked Cargo dependencies;
- added JSON verdict evidence and deterministic Markdown job summaries;
- added immutable evidence artifact upload with artifact URL/digest outputs;
- added PASS / REVIEW / BLOCK / ERROR workflow enforcement;
- REVIEW is non-failing by default and can be enforced with `fail-on-review=true`;
- passed Action smoke tests for PASS, REVIEW, enforced REVIEW, BLOCK and ERROR;
- passed an Action-level argv privacy sentinel test for JSON and Markdown evidence;
- passed adversarial Markdown escaping tests;
- command input is transported through an environment variable and quoted argv to `/bin/bash -lc` rather than interpolated into generated shell source;
- SARIF findings were killed/deferred because runtime effects do not yet prove causal repository source locations;
- preserved the initial Action smoke failure and its correction.


### 2026-09-25 — M5 Policy / Verdict accepted

- added `execsurface-policy` crate and policy schema v1;
- added independent ALLOW / REVIEW / BLOCK finding actions;
- added PASS / REVIEW / BLOCK / ERROR verdict model;
- built-in policy reviews unmatched drift rather than blocking by default;
- made rule resolution order-independent with BLOCK > REVIEW > ALLOW;
- ensured default action applies only when no rule matches;
- ensured no drift is PASS even with a blocking default;
- added matchers for change/effect/path class/path prefix/executable family/network IP/port;
- made path-prefix matching component-boundary aware;
- added stable exit codes: PASS=0, ERROR=2, REVIEW=10, BLOCK=20;
- retained M4 policy-free behavior through `--diff-only`;
- added policy JSON Schema and strict example policy;
- preserved formatting and Clippy failure evidence.


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
