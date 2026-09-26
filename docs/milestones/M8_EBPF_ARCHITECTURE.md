# M8 — Pluggable Observation Backends & eBPF Evidence Architecture

Date: 2026-09-26
Status: **OPEN — M8.0 ARCHITECTURE GATE**
Tracking: #34

## Objective

Add a high-performance eBPF observation path without weakening the semantics, privacy boundary, fail-closed behavior, or evidence quality already established by the native Linux ptrace reference backend.

M8 is an additive architecture. It is not a rewrite.

The governing invariant is:

> A faster observer may improve collection cost, but it may not silently change what ExecSurface means by observed, complete, comparable, or PASS.

## Existing authority

M6.5 remains authoritative until M8 closes an explicit replacement or equivalence gate:

- native Linux ptrace remains the correctness reference backend;
- existing canonicalization, baseline, diff, policy, verdict, and report semantics remain authoritative;
- an eBPF backend is not evidence-equivalent merely because it emits similar event names;
- incomplete or uncertain observation cannot silently produce PASS.

## Team model

### Fixed role — Innovation Scientist / Architect

Responsibilities:

- search for a stronger backend abstraction than a ptrace/eBPF special case;
- challenge unnecessary coupling between collection mechanism and evidence semantics;
- identify interoperability opportunities with Linux observability primitives;
- seek performance improvements that preserve semantic rigor;
- propose future expansion paths without expanding product scope into EDR, antivirus, sandboxing, or generic runtime security.

### Fixed role — Deviation Prevention / Scientific Integrity

Responsibilities:

- preserve the product objective: accepted execution surface -> later execution -> deterministic drift evidence;
- reject unsupported security, completeness, performance, portability, or novelty claims;
- audit trust boundaries and privacy invariants;
- require negative evidence and failed approaches to remain recorded;
- block any change that makes backend uncertainty indistinguishable from no drift.

### Dynamic M8 specialists

- Linux kernel / eBPF lead;
- Rust systems / API architecture lead;
- observation semantics and canonicalization specialist;
- BTF / CO-RE / kernel compatibility specialist;
- performance and benchmarking specialist;
- privacy/security red team;
- CI / release / supply-chain engineer;
- independent cross-backend parity reviewer.

## Architecture

```text
                         +---------------------+
Command / target  ------>| Observation Session |
                         +----------+----------+
                                    |
                         +----------v----------+
                         | Backend Selection   |
                         +-----+----------+-----+
                               |          |
                         +-----v---+  +---v------+
                         | ptrace  |  | eBPF     |
                         | REF     |  | candidate|
                         +-----+---+  +---+------+
                               |          |
                               +----+-----+
                                    |
                         +----------v----------+
                         | Raw Backend Evidence|
                         +----------+----------+
                                    |
                         +----------v----------+
                         | Completeness Gate   |
                         +----------+----------+
                                    |
                         +----------v----------+
                         | Canonicalization    |
                         +----------+----------+
                                    |
                         +----------v----------+
                         | Baseline / Diff     |
                         +----------+----------+
                                    |
                         +----------v----------+
                         | Policy / Verdict    |
                         +----------+----------+
                                    |
                         +----------v----------+
                         | JSON / Markdown     |
                         +---------------------+
```

The backend boundary ends before canonical semantic interpretation. A collection backend supplies typed raw observations plus explicit collection-health metadata; it does not decide policy or verdict.

## Backend contract

M8.1 must introduce an internal backend contract with these conceptual responsibilities.

### `BackendDescriptor`

Must identify at minimum:

- stable backend identifier;
- backend implementation version;
- host OS and architecture;
- kernel release;
- relevant kernel/BPF capability information;
- selected event classes;
- declared unsupported event classes;
- collection mechanism metadata;
- privacy profile version.

### `ObservationSession`

A session must bind:

- target command identity;
- backend selection;
- collection configuration;
- event budget / buffer budget;
- process-tree scope;
- start and termination state;
- backend health state.

### `RawObservationBatch`

A backend may emit backend-specific raw records internally, but the handoff into canonicalization must include:

- typed observation records;
- stable ordering metadata where semantically required;
- target process identity/context sufficient for causal reconstruction;
- backend capability metadata;
- explicit loss/truncation/error accounting.

The backend must not invent evidence for an event class it cannot establish.

## Completeness states

The backend-to-core handoff must expose an explicit completeness state. Minimum states:

- `COMPLETE` — all selected event classes were collected under the backend's declared capability model with no known loss/truncation;
- `INCOMPLETE_LOSS` — backend reports event loss, ring/perf buffer loss, dropped records, or equivalent collection loss;
- `INCOMPLETE_LIMIT` — an ExecSurface event/buffer/resource budget was exceeded;
- `INCOMPLETE_CAPABILITY` — the host/backend cannot establish a required selected semantic;
- `ERROR` — collection protocol, attachment, verifier, decoding, or lifecycle failure prevents reliable evidence.

Only `COMPLETE` may be eligible to proceed toward a normal PASS. Other states must remain fail-closed under existing comparability/verdict semantics.

## Capability model

Backend capability is explicit, versioned evidence, not an implicit implementation detail.

Initial capability classes to model:

1. process start / fork / clone / exec / exit;
2. pathname access intent;
3. successful open -> file-descriptor identity;
4. fd-attributed read/write effect;
5. fd duplication / close lifecycle;
6. fork inheritance / shared descriptor tables;
7. close-on-exec semantics;
8. rename/delete effects;
9. network connect destination;
10. trace-time relative path semantics;
11. causal executable chain;
12. loss/truncation visibility.

An eBPF backend may initially support a strict subset. Unsupported capability must be explicit and must participate in comparability gating.

## Cross-backend comparability

A baseline learned under one backend must not automatically compare as evidence-equivalent with another backend.

M8 will distinguish:

- **same-backend comparable** — same backend family and compatible capability/privacy contract;
- **cross-backend parity-proven** — explicit parity evidence exists for the required semantic classes and versions;
- **cross-backend non-comparable** — parity has not been established or required capabilities differ.

Until an explicit M8 parity gate is accepted, ptrace-learned and eBPF-learned evidence are not interchangeable for PASS.

## Privacy boundary

The current metadata-only boundary is preserved or tightened.

The eBPF path must not capture or persist by default:

- file contents;
- environment values;
- stdin contents;
- network payloads;
- secret material;
- unrestricted child argv values.

Any kernel-side temporary data required to resolve metadata must be minimized and must not broaden persisted evidence without a separate privacy gate.

## eBPF transport and event loss

The implementation spike must evaluate BPF ring buffer as the primary event transport because it supports shared multi-producer ordering properties useful for process lifecycle streams. However, transport choice is evidence-driven rather than assumed.

M8.4 must prove how producer reservation failure, dropped records, user-space consumer lag, buffer saturation, attachment failure, and teardown races become visible to ExecSurface.

A condition that can lose selected events without detection is a **KILLED** design for PASS-capable operation.

## Portability strategy

The feasibility spike must compare at least:

### Aya

Potential strengths:

- Rust-native development model;
- direct code sharing between user-space and eBPF Rust components;
- CO-RE support;
- no runtime dependency on BCC/libbpf.

Risks to measure:

- build/toolchain complexity;
- verifier/debugging ergonomics;
- kernel feature variance;
- packaging and reproducible release implications;
- CI privilege/capability availability.

### libbpf / libbpf-rs

Potential strengths:

- close alignment with upstream Linux BPF/CO-RE ecosystem;
- mature BTF/CO-RE model;
- strong compatibility with kernel-native tooling.

Risks to measure:

- C/libbpf build and distribution surface;
- Rust FFI/dependency complexity;
- static/dynamic linking and release portability;
- reproducibility and supply-chain implications.

No library is selected by preference. M8.2 must record a reproducible decision matrix and preserve losing-path evidence.

## Attachment strategy

Prefer stable kernel tracepoints or other documented BPF attachment mechanisms when they establish the required semantics.

Dynamic kprobe/fentry/fexit use must be justified per semantic class and kernel compatibility target. Internal kernel implementation details are not treated as stable product semantics without a compatibility strategy.

## Rootless / privilege boundary

The current ptrace path remains the ordinary rootless-friendly reference for commands launched by ExecSurface.

An eBPF path may require capabilities, privileged execution, or host policy changes depending on kernel configuration. ExecSurface must:

- detect requirements explicitly;
- never silently elevate privileges;
- never weaken sysctls/security controls automatically;
- expose why eBPF is unavailable;
- retain ptrace as a safe fallback while M8 is experimental.

`auto` backend selection is not authorized until its fallback and comparability semantics are proved.

## Release compatibility

M8 must not invalidate current public alpha users.

Required compatibility rules:

- current ptrace behavior remains available;
- existing lock/policy/report files remain readable under their declared schemas;
- no existing `PASS / REVIEW / BLOCK / ERROR` meaning is silently changed;
- eBPF remains opt-in/experimental until the public-alpha integration gate closes;
- release artifacts identify backend support and capability boundaries clearly.

## Gate sequence

### M8.0 — Architecture contract

Acceptance requires:

- backend boundary documented;
- completeness/loss model documented;
- capability and cross-backend comparability model documented;
- privacy and privilege boundaries documented;
- dynamic team and fixed governance roles documented.

### M8.1 — Non-breaking observer abstraction

Acceptance requires:

- current ptrace implementation behind the new backend interface;
- no intended user-visible behavior change;
- existing ptrace tests preserved;
- baseline/diff/policy/verdict semantics unchanged;
- code review confirms no silent completeness weakening.

### M8.2 — eBPF feasibility decision

Acceptance requires:

- Aya vs libbpf-rs/libbpf reproducible spikes;
- BTF/CO-RE and kernel compatibility evidence;
- privilege/CI/release analysis;
- verifier and diagnostics analysis;
- selected path plus preserved rejected-path evidence.

### M8.3 — Metadata-only eBPF observer

Acceptance requires a minimum declared capability set with explicit unsupported classes.

### M8.4 — Fail-closed loss gate

Acceptance requires controlled proof that known event loss, truncation, buffer pressure, attachment failure, and teardown failure cannot produce silent PASS.

### M8.5 — Parity gate

Acceptance requires a controlled ptrace-vs-eBPF workload matrix with semantic differences classified and machine-readable comparability rules.

### M8.6 — Performance / compatibility gate

Acceptance requires representative direct/ptrace/eBPF measurements and a declared kernel/platform support matrix.

### M8.7 — Public-alpha integration gate

Acceptance requires safe backend selection, documentation, technical evaluation updates, release evidence, and no regression of the stable current path.

## Current status

- M8 objective: **OPEN**
- M8.0 architecture: **PARTIAL — document created, implementation review pending**
- M8.1 observer abstraction: **NOT STARTED**
- M8.2 eBPF implementation selection: **NOT STARTED**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace correctness reference: **PROVED / RETAINED under existing M6.5 evidence**
