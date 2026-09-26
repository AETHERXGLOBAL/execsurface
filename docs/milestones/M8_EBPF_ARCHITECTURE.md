# M8 — Pluggable Observation Backends & eBPF Evidence Architecture

Date: 2026-09-26
Status: **OPEN — M8.0–M8.6 CLOSED / M8.7 PERSISTENT OBSERVER NEXT**
Tracking: #34, #37, #40, #42, #44
M8.2 decision: `docs/milestones/M8_2_STACK_DECISION.md`
M8.2 evidence: `docs/milestones/M8_2_EVIDENCE.md`
M8.4 evidence: `docs/milestones/M8_4_LOSS_SEMANTICS.md`
M8.5 evidence: `docs/milestones/M8_5_SEMANTIC_PARITY.md`
M8.5 red-team: `docs/milestones/M8_5_RED_TEAM_REVIEW.md`
M8.6 evidence: `docs/milestones/M8_6_PERFORMANCE_COMPATIBILITY.md`
M8.6 root-cause evidence: `docs/milestones/M8_6D_POST_TARGET_LATENCY.md`
M8.6 red-team: `docs/milestones/M8_6_RED_TEAM_REVIEW.md`

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

M8.5 proved only bounded semantic parity for explicitly controlled projections. It did not authorize full-surface equivalence or eBPF PASS authority.

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

M8.1 introduced an internal backend contract with these conceptual responsibilities.

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

The backend-to-core handoff exposes an explicit completeness state. Minimum states:

- `COMPLETE` — all selected event classes were collected under the backend's declared capability model with no known loss/truncation;
- `INCOMPLETE_LOSS` — backend reports event loss, ring/perf buffer loss, dropped records, or equivalent collection loss;
- `INCOMPLETE_LIMIT` — an ExecSurface event/buffer/resource budget was exceeded;
- `INCOMPLETE_CAPABILITY` — the host/backend cannot establish a required selected semantic;
- `ERROR` — collection protocol, attachment, verifier, decoding, or lifecycle failure prevents reliable evidence.

M8.4 operationalized the non-complete/error family in the experimental collector with explicit machine-readable states including `incomplete_loss`, `incomplete_limit`, `incomplete_capability`, `incomplete_lifecycle`, `incomplete_decode`, and `incomplete_collector`. These remain experimental collection-health states until the public integration gate defines the stable external schema.

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

An eBPF backend may support a strict subset. Unsupported capability must be explicit and must participate in comparability gating.

## Cross-backend comparability

A baseline learned under one backend must not automatically compare as evidence-equivalent with another backend.

M8 distinguishes:

- **same-backend comparable** — same backend family and compatible capability/privacy contract;
- **cross-backend parity-proven** — explicit parity evidence exists for the selected semantic class, proposition, versions, and controlled conditions;
- **cross-backend non-comparable** — parity has not been established or required capabilities differ.

M8.5 proved bounded class-local parity for recorded controlled process-lineage, exec-occurrence, and focused positive successful-open witness propositions. It explicitly did not prove full-surface parity. Ptrace-learned and eBPF-learned evidence therefore remain non-interchangeable for normal PASS.

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

The implementation path uses BPF ring buffer transport because it supports shared multi-producer ordering properties useful for process lifecycle streams.

M8.4 proved how producer reservation failure, dropped records, user-space consumer lag, buffer saturation, attachment/setup failure, decode failure, post-start collector failure, and teardown races become visible and fail closed in the experimental collector. A post-start collector failure is additionally bounded by target process-group containment before returning failure.

A condition that can lose selected events without detection is a **KILLED** design for PASS-capable operation.

## Portability strategy

M8.2 compared Aya and libbpf/libbpf-rs with executable feasibility evidence.

### Aya

Observed strengths:

- Rust-native development model;
- direct code sharing potential between userspace and eBPF components;
- CO-RE/BTF support;
- no runtime dependency on libbpf.

Measured costs included newer/nightly Rust-side build requirements and BPF toolchain complexity relative to the existing product MSRV.

### libbpf / libbpf-rs

Observed strengths:

- close alignment with upstream Linux BPF/CO-RE ecosystem;
- mature BTF/CO-RE model;
- compatibility with the product's Rust 1.82 line when dependency resolution is performed under the recorded fallback procedure.

Measured costs include the native clang/libelf/libbpf build surface and packaging complexity.

M8.2 selected libbpf-rs/libbpf for the experimental implementation path. Aya remains preserved as a viable alternative rather than being declared technically invalid.

## Attachment strategy

Prefer stable kernel tracepoints or other documented BPF attachment mechanisms when they establish the required semantics.

Dynamic kprobe/fentry/fexit use must be justified per semantic class and kernel compatibility target. Internal kernel implementation details are not treated as stable product semantics without a compatibility strategy.

M8.6 measured that the current per-invocation libbpf design pays a dominant teardown cost on the tested host. M8.7 therefore investigates persistent attachment with explicit bounded observation sessions before any public-alpha integration.

## Rootless / privilege boundary

The current ptrace path remains the ordinary rootless-friendly reference for commands launched by ExecSurface.

An eBPF path may require capabilities, privileged execution, or host policy changes depending on kernel configuration. ExecSurface must:

- detect requirements explicitly;
- never silently elevate privileges;
- never weaken sysctls/security controls automatically;
- expose why eBPF is unavailable;
- retain ptrace as a safe fallback while M8 is experimental.

`auto` backend selection is not authorized until its fallback and comparability semantics are proved in a later integration gate.

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

Acceptance requires controlled proof that known event loss, truncation, buffer pressure, attachment failure, decode/lifecycle failure, and post-start collector failure cannot produce silent PASS or leave unbounded target execution.

### M8.5 — Parity gate

Acceptance requires a controlled ptrace-vs-eBPF workload matrix with semantic differences classified, machine-readable comparability rules, preserved counterexamples, an explicit bounded comparability decision, and independent red-team closure.

**Status: CLOSED / ACCEPTED — bounded semantic parity only.** See `M8_5_SEMANTIC_PARITY.md` and `M8_5_RED_TEAM_REVIEW.md`.

### M8.6 — Performance / compatibility gate

Acceptance requires representative direct/ptrace/eBPF measurements, phase attribution, a declared kernel/platform support matrix, preserved negative performance evidence, and independent red-team review.

**Status: CLOSED / ACCEPTED — exact-host measurement only.** The current per-invocation libbpf path is slower end-to-end than ptrace on the measured host/workloads, and isolated measurement attributes the dominant lifecycle cost to per-invocation detach. See `M8_6_PERFORMANCE_COMPATIBILITY.md`, `M8_6D_POST_TARGET_LATENCY.md`, and `M8_6_RED_TEAM_REVIEW.md`.

### M8.7 — Persistent observer architecture gate

Acceptance requires executable proof that persistent attachment can amortize the measured teardown cost without weakening evidence semantics. At minimum:

- explicit single-active-session epoch and root/descendant attribution;
- rejection of stale/cross-session events;
- per-session loss/truncation accounting or fail-closed ambiguity;
- preservation of M8.4 lifecycle/quiescence semantics;
- bounded and explicit privilege lifetime;
- deterministic session state reset;
- crash/restart invalidation of ambiguous in-flight work;
- no default install/CLI regression;
- no eBPF PASS promotion;
- independent security/evidence Red Team.

Multi-session concurrency is not authorized until independent attribution is proved.

### M8.8 — Public-alpha integration gate

Acceptance requires safe backend selection, expanded incomplete-state CLI/schema surfacing, documentation, technical evaluation updates, release evidence, and no regression of the stable current path. M8.8 may not begin until M8.7 closes or the persistent path is explicitly killed and an alternative architecture is selected.

## Current status

- M8 objective: **OPEN — M8.7 PERSISTENT OBSERVER NEXT**
- M8.0 architecture: **CLOSED / ACCEPTED**
- M8.1 observer abstraction: **CLOSED / ACCEPTED**
- M8.2 eBPF implementation selection: **CLOSED / ACCEPTED — libbpf-rs / libbpf selected**
- M8.3 metadata-only eBPF observer: **CLOSED / ACCEPTED — experimental observation-only path**
- M8.4 fail-closed loss/lifecycle gate: **CLOSED / PROVED**
- M8.5 ptrace/eBPF parity gate: **CLOSED / ACCEPTED — bounded semantic parity only**
- M8.6 performance / compatibility gate: **CLOSED / ACCEPTED — exact-host measurement; per-invocation detach dominant measured lifecycle cost**
- M8.7 persistent observer architecture gate: **NEXT / NOT STARTED**
- M8.8 public-alpha integration gate: **NOT STARTED**
- eBPF full-surface comparability: **FALSE**
- eBPF PASS authority: **NOT AUTHORIZED**
- automatic cross-backend baseline interchangeability: **NOT AUTHORIZED**
- ptrace correctness reference: **PROVED / RETAINED under existing M6.5 evidence**
