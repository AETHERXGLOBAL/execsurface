# Changelog

## Unreleased

### 2026-09-26 — M7.1 Registry Distribution Parity in progress

- prepared the public CLI package name `execsurface` for Rust-native installation;
- advanced product version to `0.1.0-alpha.2` without changing evidence schema v2;
- added exact registry versions to publishable workspace path dependencies;
- kept `execsurface-bench` explicitly non-publishable;
- added a read-only registry packaging/lockfile gate;
- added a manually dispatched, environment-protected crates.io publication workflow;
- documented the first-publication credential boundary and dependency-order publication;
- retained GitHub Release binaries and stable `@v0.1` Action distribution.


### 2026-09-26 — M7 External Real-World Compatibility Proof accepted

- pinned external target `sharkdp/fd@ce97e473ebaec49697c07daa50a7bc2b32f713d2`;
- reproduced the pinned upstream Linux/Rust test suite with 118 tests passing;
- downloaded ExecSurface `v0.1.0-alpha.1` from the public GitHub Release and verified SHA-256 plus GitHub build provenance;
- built the real `fd 10.5.0` release binary;
- learned a real `fd` runtime baseline containing 517 canonical effects;
- proved two repeated no-drift checks return PASS with zero findings;
- introduced a controlled additional `/usr/bin/sha256sum` execution in the same Bash wrapper;
- detected the controlled expansion as REVIEW with 38 added findings;
- confirmed the drift includes one added `process_exec`, four added `file_read`, 32 added `file_open`, and one added `file_write` finding;
- confirmed the fixture-file read is fd-attributed and carries the causal execution chain `bash → sha256sum`;
- proved public stable Action `AETHERXGLOBAL/execsurface@v0.1` returns PASS for the accepted command and REVIEW/10 for the controlled expansion;
- preserved two harness failures involving an omitted explicit baseline path; no ExecSurface semantic weakening or broad normalization was introduced;
- made no upstream modification and no third-party adoption claim.

### 2026-09-26 — M6.6 Public Self-Service & Distribution Gate accepted

- promoted product version to `0.1.0-alpha.1` without changing evidence schema v2;
- added `execsurface --version`, diagnostic `execsurface doctor`, and conservative `execsurface init`;
- added product-first README, five-minute quickstart, troubleshooting, support, examples and adoption/integration templates;
- established GitHub Release as the primary no-clone Linux x86_64 installation path;
- published `execsurface-v0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz` plus SHA-256 checksum;
- generated GitHub build-provenance attestation and verified it in the zero-contact consumer gate;
- proved a fresh release-binary consumer can run version/doctor/learn/check and observe controlled REVIEW drift without source checkout;
- proved fresh `cargo install --git ... --tag v0.1.0-alpha.1 ... --locked`;
- deferred crates.io publishing after a packaging probe confirmed the internal workspace dependency graph should not be distorted merely to publish the CLI;
- changed remote GitHub Action consumption to download and checksum-verify the pinned release binary while preserving source-build `uses: ./` for repository development;
- promoted stable `AETHERXGLOBAL/execsurface@v0.1` only after immutable release, binary, Cargo and Action gates passed;
- proved stable-channel PASS, REVIEW, BLOCK and ERROR behavior;
- preserved failed release-promotion evidence caused by literal `\\n` release-tag bytes; the gate stopped before tag creation;
- preserved release-page metadata and verifier failures caused by shell backtick substitution; fixes changed release notes/verification only and did not move tags or alter artifact bytes;
- verified final public release source metadata, archive digest and current main CI;
- retained Linux x86_64-only and `observed behavior ≠ all possible behavior` boundaries.

### 2026-09-26 — M6.5 Semantic Fidelity & Completeness Hardening accepted

- upgraded raw observation, canonical surface, baseline lock/digest, diff and verdict contracts to v2 rather than silently changing v1 meaning;
- added successful-open fd identity and actual fd-attributed `file_read` / `file_write` effects;
- added fd lifecycle handling for close/dup/dup2/dup3/fcntl duplication, fork inheritance, CLONE_FILES sharing and close-on-exec semantics;
- added trace-time relative/openat resolution and openat2 resolve metadata;
- added bounded causal execution chains without PID/TID in canonical identity;
- added a finite observation event budget; overflow marks evidence incomplete and canonicalization fails closed;
- added observer fault injection proving unreadable pathname metadata becomes incomplete evidence;
- added high-volume and threaded/forked fd stress tests;
- added self-proc normalization only for the proven tracee/TGID identity, preserving access to other PIDs as distinct;
- added policy v2 file-read/file-write matchers while preserving restricted v1 policy compatibility;
- added reproducible ptrace benchmark evidence and an evidence-based backend decision;
- final branch CI passed format, strict Clippy, 78 tests and Cargo.lock integrity;
- final GitHub Action smoke passed PASS, REVIEW, enforced REVIEW, BLOCK, ERROR and privacy cases;
- preserved the v2 Action PASS regression caused by volatile `/proc/<PID>/maps` identities and its narrow correction.


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
