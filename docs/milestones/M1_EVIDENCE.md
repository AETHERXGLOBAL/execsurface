# M1 — Minimal Linux Observer Evidence

Date: 2026-09-25

Gate verdict: **ACCEPTED**

Branch: `milestone/m1-minimal-linux-observer`

## Scope proven

M1 establishes a narrow raw-observation backend, not an execution-surface baseline or policy engine.

Implemented:

- Rust workspace with `execsurface-model`, `execsurface-observe`, and `execsurface-cli`;
- Linux x86_64 native ptrace backend;
- descendant fork/vfork/clone tracking;
- confirmed executable pathname after successful exec event;
- selected path-based file syscall attempts;
- outbound `connect(2)` destination metadata;
- backend capability/limitation metadata;
- experimental `execsurface observe -- COMMAND` JSON output;
- metadata-only privacy contract: observer never dereferences argv or envp;
- one observation session per host process to prevent `waitpid(-1, __WALL)` cross-reaping;
- committed Cargo.lock and locked CI.

## Privacy evidence

Integration fixture passes the sentinel:

`AX_EXEC_SURFACE_SECRET_SENTINEL_9f62d6f2`

only through child argv.

The complete serialized Observation is asserted not to contain that sentinel.

Status: **COMPUTATIONAL_EVIDENCE**

This proves non-capture for the implemented M1 path exercised by the fixture. It is not a universal non-leakage proof.

## Runtime fixtures

GitHub Actions run #8:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36169541898

Environment:

- Ubuntu 24.04 hosted runner
- Linux kernel `6.17.0-1022-azure`
- x86_64
- `rustc 1.98.1 (48a229cea 2026-09-01)`
- `cargo 1.98.1 (797e8a9bc 2026-08-05)`

Passing checks:

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`
- `cargo test --locked --workspace --all-targets`
- Cargo.lock integrity check

Runtime tests:

- `captures_descendant_exec_without_argv_values` — PASS
- `captures_path_based_file_open` — PASS
- `captures_local_network_connect_destination` — PASS
- `backend_declares_scope_and_limitations` — PASS
- IPv4 sockaddr parser — PASS
- IPv6 sockaddr parser — PASS

## Preserved negative evidence

### Privacy gate

Original M0 strace-ingestion production plan was rejected after review showed decoded trace output can include string syscall arguments before redaction.

Status: **KILLED BY EVIDENCE**

See ADR-0002.

### CI run #1

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36168569055

Failure: rustfmt gate.

Status: **KILLED / FIXED**

### CI run #2

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36168719678

Failure: libc ptrace event constants were `i32` while the internal event value was typed `u32`.

Status: **KILLED / FIXED**

### CI run #3

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36168787081

Failure: strict Clippy rejected a needless return.

Status: **KILLED / FIXED**

### CI run #4

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36168867012

Observation: format and Clippy passed, but parallel runtime tests remained active while the corrected run completed rapidly.

Root-cause hypothesis: concurrent observer sessions in the same host test process can compete on `waitpid(-1, __WALL)` and cross-reap tracee events.

Corrective design: process-wide observer-session mutex.

Status: **COMPUTATIONAL_EVIDENCE / MITIGATED IN M1**

The repository does not claim a formal proof that this was the only possible cause of the stalled run.

### CI run #7

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36169462342

Failure: a manually transcribed serde_json checksum in Cargo.lock did not exactly match the generated artifact.

Corrective action: restored the checksum from the CI-generated lock artifact and reran with `--locked`.

Status: **KILLED / FIXED**

## Boundaries retained

M1 does **not** claim:

- file read/write truth from fd lifecycle;
- complete syscall coverage;
- canonical stable surfaces;
- baseline learning;
- diff semantics;
- policy verdicts;
- low overhead;
- hostname intent;
- eBPF support;
- macOS/Windows support;
- sandboxing;
- malware detection;
- program safety.

## Carry-forward OPEN items

- fd lifecycle attribution for true read/write effects;
- symlink/path-resolution semantics;
- `openat2` and broader selected syscall coverage;
- explicit fault-injection tests for observer-internal failure;
- architecture abstraction beyond x86_64;
- canonicalization/noise taxonomy implementation;
- performance measurement.

## Gate decision

M1 is accepted because the minimal observer hypothesis has independent CI evidence for process, file-path and network-destination metadata while preserving the privacy boundary.

M2 is authorized next, but has not started in this gate.
