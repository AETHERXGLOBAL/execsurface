# M8.3 — Experimental libbpf-rs Backend Contract

Date: 2026-09-26
Status: **OPEN — IMPLEMENTATION GATE**
Tracking: #34
Parent decision: `docs/milestones/M8_2_STACK_DECISION.md`

## Objective

Implement the first real libbpf-rs observation backend without changing the default ptrace product path and without claiming semantic equivalence that M8.5 has not proved.

M8.3 is an additive implementation gate. It is not the public eBPF release gate.

## Fixed governance roles

- **Innovation Scientist / Architect** — seek the strongest extensible backend boundary and reject unnecessary coupling to the existing ptrace implementation.
- **Deviation Prevention / Scientific Integrity** — reject invented semantics, hidden fallbacks, privacy/licensing shortcuts, unsupported performance claims, and any accidental eBPF PASS authority.

## Dynamic M8.3 specialists

- Linux eBPF / libbpf engineer
- Rust systems / process-control engineer
- observation-model / canonicalization specialist
- privacy and licensing red team
- CI / packaging / reproducibility engineer
- independent fail-closed reviewer

## Architectural decision — isolated helper backend

The first libbpf backend is implemented as an **isolated non-workspace package/helper**, not as a default dependency of `execsurface-observe`.

Reasons:

1. normal `cargo install execsurface --locked` must remain ptrace-only and must not acquire clang/libelf/autotools or libbpf build requirements;
2. normal workspace CI must remain able to run without the eBPF native build chain;
3. the experimental backend needs a dedicated packaging and privilege boundary;
4. a process/protocol boundary lets later releases ship, version, replace, or sandbox the eBPF collector independently;
5. M8.7, not M8.3, decides the final public integration surface.

The helper must emit the existing raw `Observation` JSON contract, not a private second evidence model.

## Backend identity

Initial backend identity:

`linux-libbpf-ebpf-experimental-v1`

The identity is intentionally different from ptrace and therefore remains cross-backend non-comparable under the existing M4 observer-name gate.

## M8.3 minimum capability set

The first accepted implementation may claim only semantics it can produce without fabrication:

### Supported

- `process.root_exec_path.command_spec.v1`
  - the root executable path is resolved in userspace before launch;
  - argv values are not persisted;
  - no BPF user-memory string helper is required.
- `process.spawn.clone_family.syscall_exit.v1`
  - successful `clone`, `clone3`, `fork`, and `vfork` exits provide parent -> child lineage;
  - mechanisms are mapped conservatively to the existing `SpawnMechanism` enum.
- `loss.producer_drop_counter.v1`
  - ring-buffer reservation failures increment an explicit producer-side counter.
- `privacy.metadata_only.v1`
  - persisted events contain process/lineage metadata only.

### Explicitly unsupported in M8.3 v1

- descendant executable path after descendant `exec`;
- pathname access intent;
- successful open -> path/fd identity in the existing raw v2 model;
- fd-attributed read/write;
- fd duplication/close lifecycle;
- fork-inherited/shared fd-table semantics;
- close-on-exec semantics;
- rename/delete;
- network connect destination;
- trace-time relative path semantics;
- full causal executable chains after descendant exec;
- ptrace/eBPF semantic equivalence.

M8.2 proved an fd-only `openat` metadata signal is technically obtainable, but raw schema v2 requires path-bearing file semantics. M8.3 must **not** invent an empty/synthetic path merely to reuse that event.

## Exec path decision

The kernel currently marks `bpf_probe_read_user_str` as GPL-only on the tested tracing path. ExecSurface must not change its Apache-2.0 licensing declaration merely to obtain an exec filename.

For M8.3 v1:

- resolve the root executable path in userspace before launch;
- persist only that resolved executable path as the root `ProcessExec` event;
- do not claim descendant-exec-path support;
- later descendant exec semantics require a separately reviewed mechanism.

## Launch race requirement

A normal `Command::spawn()` is insufficient for the correctness contract because the root process could execute/spawn before the observer has registered its PID.

The helper must use a launch barrier:

1. load and attach BPF programs;
2. create a synchronization pipe;
3. fork the target child;
4. child blocks before `exec`;
5. parent inserts the child TGID into the backend's tracked-process map;
6. parent releases the child;
7. only then may target execution proceed.

A backend that cannot prove this ordering is not accepted as complete.

## Tree scoping

The BPF program must not emit a global host trace as product evidence.

Minimum scoping design:

- tracked-TGID map seeded with the root target PID before release;
- successful spawn from a tracked parent inserts the child TGID and emits lineage metadata;
- unrelated host activity does not become ExecSurface evidence;
- userspace tracks/reaps the launched process tree and must not silently retain ambiguous stale identities.

Any remaining PID-lifecycle limitation must be explicit and must prevent an unsupported stronger claim.

## Completeness mapping under raw schema v2

M8.3 does not introduce raw schema v3 merely for architectural neatness.

For the selected capability profile:

- `Observation.complete = true` means all **declared M8.3 v1 selected capabilities** completed with no known transport loss, event-budget truncation, attachment failure, protocol/decode failure, or launch-scope failure;
- any detected loss or collection uncertainty sets `complete = false` and records an `ObserverWarning`;
- unsupported capability classes are listed in `backend.limitations` and are not silently interpreted as observed absence.

Because observer name/capability mismatches already make M4 diff return incomparable, an eBPF v1 baseline cannot silently compare with a ptrace baseline.

## Privacy contract

The helper may receive the target argv because it must launch the command, but persisted evidence must not include argv values.

The eBPF program must not persist:

- argv strings;
- environment values;
- stdin or file contents;
- network payloads;
- arbitrary user-memory strings;
- secret material.

Tests must use argv/environment sentinels and assert they are absent from emitted Observation JSON.

## Privilege contract

- no automatic privilege escalation;
- no automatic sysctl/security weakening;
- no hidden fallback to ptrace inside an explicitly requested eBPF session;
- insufficient privilege is an explicit backend error;
- ptrace remains the normal/default path outside the experimental helper.

## Packaging contract

The helper is not a member of the default Cargo workspace in M8.3.

Its dedicated CI must prove:

- declared native build inputs;
- Rust 1.82 product-compatible userspace build where applicable;
- reproducible locked resolution;
- vendored-runtime packaging path;
- normal workspace `Cargo.toml` / `Cargo.lock` unchanged by the backend build;
- normal product CI remains green.

## Acceptance tests

M8.3 closes only when dedicated evidence proves at minimum:

1. backend identity/capability/limitation metadata is deterministic;
2. root executable path is real/resolved, not a placeholder;
3. launch barrier prevents pre-registration execution;
4. unrelated host process activity is excluded from emitted evidence;
5. controlled descendant spawn produces `ProcessSpawn` for the tracked tree;
6. no argv/environment sentinel appears in Observation JSON;
7. detected producer loss produces `complete=false` and an explicit warning;
8. insufficient privilege is explicit;
9. ptrace/default product tests and distribution probes remain unchanged;
10. ptrace baseline vs eBPF candidate remains incomparable;
11. no eBPF PASS-authority claim is introduced.

## Explicit non-goals

M8.3 does not prove:

- full ptrace semantic parity;
- production kernel portability;
- eBPF performance superiority;
- public self-service backend selection;
- automatic backend fallback;
- production readiness.

Those belong to M8.4–M8.7.

## Current labels

- libbpf-rs implementation path: **ACCEPTED from M8.2**
- M8.3 backend implementation: **OPEN**
- default backend: **ptrace — RETAINED**
- cross-backend parity: **OPEN**
- eBPF PASS authority: **NOT AUTHORIZED**
