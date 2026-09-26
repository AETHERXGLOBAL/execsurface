# Roadmap

## M0 — Architecture Freeze

**CLOSED / ACCEPTED**

## M1 — Minimal Linux Observer

**CLOSED / ACCEPTED**

Metadata-only native ptrace backend on Linux x86_64. CI proves descendant exec observation, argv non-capture sentinel, path-based file observation, loopback connect destination, strict Clippy, and locked dependency reproducibility.

Evidence: `docs/milestones/M1_EVIDENCE.md`.

## M2 — Canonicalization

**CLOSED / ACCEPTED**

Deterministic canonical surface across PID/sequence/interleaving and declared machine-root variance. Explicit semantic roots, credential-shadow protection, unresolved-path semantics, deterministic deduplication, open-intent preservation, and observer→canonical repeatability are covered by CI.

Evidence: `docs/milestones/M2_EVIDENCE.md`.

## M3 — Baseline Lock

**CLOSED / ACCEPTED**

`execsurface learn -- COMMAND` creates a deterministic `execsurface.lock.json` with a frozen SHA-256 digest contract, privacy-safe command identity, explicit schema/versioning, atomic publication and no silent overwrite.

Evidence: `docs/milestones/M3_EVIDENCE.md`.

## M4 — Diff Engine

**CLOSED / ACCEPTED**

Verified baseline ingestion, comparability gating, deterministic added/removed findings, conservative changed pairing, JSON/text reporting, and real `learn → check` integration are covered by CI.

Evidence: `docs/milestones/M4_EVIDENCE.md`.

## M5 — Policy / Verdict

**CLOSED / ACCEPTED**

Independent policy v1 maps M4 findings to ALLOW / REVIEW / BLOCK using deterministic order-independent rule resolution. No drift is always PASS; operational/comparability failures remain ERROR.

Evidence: `docs/milestones/M5_EVIDENCE.md`.

## M6 — GitHub Action

**CLOSED / ACCEPTED**

Linux x86_64 composite Action with deterministic JSON evidence, Markdown job summary, artifact upload, PASS/REVIEW/BLOCK/ERROR enforcement, configurable REVIEW failure and privacy smoke evidence.

SARIF finding emission is **KILLED / DEFERRED for M6 v1** because current runtime evidence does not establish a causal repository source file/line. ExecSurface does not invent source locations for Code Scanning.

Evidence: `docs/milestones/M6_EVIDENCE.md`.

## M6.5 — Semantic Fidelity & Completeness Hardening

**CLOSED / ACCEPTED**

Hardened Linux ptrace evidence with syscall entry/exit pairing, successful-open fd identity, fd-attributed read/write effects, close/dup/fork/CLONE_FILES handling, openat/openat2 semantics, bounded causal execution chains, explicit event-budget truncation, fault injection, v2 schema migration and reproducible performance evidence.

The controlled 128-file burst remained complete with a median absolute ptrace overhead of 51.569 ms on the final shared-runner measurement. This is below the pre-M7 500 ms absolute-overhead trigger. ptrace therefore remains the reference correctness backend for M7; eBPF is not authorized as an interchangeable backend without equivalent lost-event/completeness semantics.

Evidence: `docs/milestones/M6_5_EVIDENCE.md`.
Backend decision: `docs/milestones/M6_5_BACKEND_DECISION.md`.

## M6.6 — Public Self-Service & Distribution Gate

**CLOSED / ACCEPTED**

ExecSurface is now distributed as a Linux x86_64 public alpha with a GitHub Release binary, SHA-256 checksum, build-provenance attestation, fresh Cargo Git fallback, `execsurface doctor`, conservative `execsurface init`, product-first documentation and a gated `@v0.1` GitHub Action channel.

The immutable `v0.1.0-alpha.1` release and moving `v0.1` Action channel both point to the release-gated source commit `d6919e2b3e23b196965bc7bba87598f09265599f`. The release workflow proved checksum/provenance verification, zero-contact binary use, fresh Cargo tag installation, immutable Action consumption, and stable-channel PASS/REVIEW/BLOCK/ERROR behavior.

M6.6 did not change v2 evidence semantics, verdict semantics, baseline/policy separation, the Linux x86_64 support boundary, or the M6.5 ptrace/eBPF decision.

Evidence: `docs/milestones/M6_6_EVIDENCE.md`.

## M7 — External Real-World Proof

**CLOSED / ACCEPTED — COMPATIBILITY PROOF**

External target:

`sharkdp/fd@ce97e473ebaec49697c07daa50a7bc2b32f713d2`

The pinned upstream suite reproduced with 118 tests passing. ExecSurface public release `v0.1.0-alpha.1` learned a real `fd` runtime baseline with 517 canonical effects, produced repeated no-drift PASS results, and detected a controlled runtime expansion as REVIEW with an added `sha256sum` process execution plus fd-attributed read evidence for the fixture file.

The stable public Action channel `AETHERXGLOBAL/execsurface@v0.1` also proved PASS for the baseline command and REVIEW for the controlled expansion.

No upstream modification or maintainer contact occurred before the proof passed. This is compatibility evidence, **not third-party adoption**.

Evidence: `docs/milestones/M7_EVIDENCE.md`.

## M7.1 — Registry Distribution Parity

**CLOSED / ACCEPTED**

ExecSurface `0.1.0-alpha.2` is published on crates.io under the public package name `execsurface`, together with its publishable runtime workspace crates.

The publication chain was resumed safely after two preserved infrastructure failures:

- crates.io account email verification initially blocked the first upload;
- crates.io new-crate rate limits temporarily interrupted the multi-crate publication chain.

The idempotent recovery path skipped versions already present in the registry, completed all eight packages, and then proved a zero-contact fresh install directly from crates.io:

```bash
cargo install execsurface --version "=0.1.0-alpha.2" --locked
```

The installed binary passed `--version`, `doctor`, `learn`, and a no-drift `check` with PASS / 0 findings on a fresh Ubuntu 24.04 runner.

The normal Rust-native user path is now:

```bash
cargo install execsurface --locked
```

GitHub Release binaries and the stable `AETHERXGLOBAL/execsurface@v0.1` Action channel remain supported. No observer, evidence-schema, canonicalization, baseline, policy, or verdict semantics changed during M7.1.

## M8 — Pluggable Observation Backends & eBPF Evidence Path

**CLOSED / ACCEPTED AS RESEARCH PROGRAM — PUBLIC eBPF EXPOSURE DEFERRED**

M8 built and adversarially evaluated an eBPF observation path without replacing or weakening the native ptrace correctness reference.

Gate status:

1. **M8.0 — CLOSED / ACCEPTED:** backend contract, capability/completeness/loss/privacy/comparability architecture.
2. **M8.1 — CLOSED / ACCEPTED:** ptrace moved behind a backend abstraction while existing CI/distribution/action behavior remained stable.
3. **M8.2 — CLOSED / ACCEPTED:** Aya and libbpf-rs passed feasibility hard gates; **libbpf-rs / libbpf selected for the experimental implementation**. Apache-2.0 and packaging constraints remain explicit.
4. **M8.3 — CLOSED / ACCEPTED:** typed partial-capability contract, conditional fail-closed path resolution, isolated libbpf observer, launched-tree scoping, explicit loss/limit handling and observation-only CLI bridge proved. Default `observe`, `learn`, and `check` retained ptrace semantics.
5. **M8.4 — CLOSED / PROVED:** producer loss, truncation, consumer lag, post-root descendant drain, lifecycle timeout, machine-readable collector/decode failures and post-start containment were proved fail-closed.
6. **M8.5 — CLOSED / ACCEPTED:** bounded cross-backend parity was proved for selected process/exec and focused successful-open propositions, including counterexamples and fail-closed incomplete-state handling. Full-surface equivalence was not established.
7. **M8.6 — CLOSED / ACCEPTED — MEASUREMENT GATE:** exact-host work showed per-invocation libbpf teardown dominated measured lifecycle cost and motivated persistent attachment rather than timing shortcuts.
8. **M8.7 — CLOSED / ACCEPTED — BOUNDED PERSISTENT SESSION ARCHITECTURE FEASIBLE:** persistent attachment, session epochs, root-registration barrier, task-creation propagation, descendant drain, reset checks, crash/restart isolation, routing failure, producer loss, event-budget exhaustion and live stale-epoch rejection were proved for the tested single-active-session architecture. M8.7c reduced the conservative two-session amortized libbpf cost by about 16.7% on the exact host, but remained about 50.5x the ptrace median.
9. **M8.8 — CLOSED / DEFER:** public eBPF exposure is not justified by current product evidence. The original M6.5 external-workload value trigger has not been met, the current eBPF path remains slower end-to-end on the measured workload, its semantic surface is narrower, and persistent privilege/service boundaries remain open. The research asset is retained with explicit reopen conditions.

### M8 authority boundary at closure

- ptrace correctness reference: **RETAINED**;
- default public backend: **ptrace**;
- eBPF public exposure: **DEFERRED**;
- eBPF full-surface comparability: **FALSE**;
- eBPF PASS authority: **NOT AUTHORIZED**;
- eBPF `learn` / `check`: **NOT AUTHORIZED**;
- backend auto-selection: **NOT AUTHORIZED**;
- cross-backend baseline interchangeability: **NOT AUTHORIZED**;
- production persistent daemon/service: **NOT AUTHORIZED**;
- default public install path: **UNCHANGED**.

The eBPF path may be reopened only when a real external workload crosses the predeclared ptrace cost trigger or a concrete external workflow requires a capability that eBPF can uniquely provide, followed by all required safety/product gates.

Architecture: `docs/milestones/M8_EBPF_ARCHITECTURE.md`.
M8.2 decision: `docs/milestones/M8_2_STACK_DECISION.md`.
M8.4 evidence: `docs/milestones/M8_4_LOSS_SEMANTICS.md`.
M8.5 evidence: `docs/milestones/M8_5_SEMANTIC_PARITY.md`.
M8.6 evidence: `docs/milestones/M8_6_PERFORMANCE_COMPATIBILITY.md` and `docs/milestones/M8_6_RED_TEAM_REVIEW.md`.
M8.7 evidence: `docs/milestones/M8_7_PERSISTENT_OBSERVER.md`, `docs/milestones/M8_7C_PERFORMANCE_PROTOCOL.md`, and `docs/milestones/M8_7_RED_TEAM_REVIEW.md`.
M8.8 decision: `docs/milestones/M8_8_PUBLIC_INTEGRATION_DECISION.md`.
Tracking: GitHub Issues #34, #37, #40, #42, #44, #46 and #49.

## M9 — Independent Adoption & Real-Workload Evidence

**OPEN — M9.0 CLOSED / PROVED; M9.1 NEXT**

The next highest-value program is independent use of the existing public product on real developer/CI workloads, not another speculative observer feature.

### M9.0 — Protocol & Intake Freeze

**CLOSED / PROVED**

The evidence protocol, machine-readable schema, independence provenance classes, privacy boundary, failure-preservation rules, exact-host performance protocol, and predeclared M6.5 eBPF reopen trigger were frozen before accepted M9 external evidence.

Red Team found and killed an initial schema design that could have allowed self-run compatibility evidence to be mislabeled as independent adoption. The hardened schema requires provenance and CI contains a negative sentinel that rejects this attack.

Closure evidence on protocol snapshot `ffff35904556cd020b06369a3b272da20598b53c`:

- `M9 Evidence Protocol` run `36266376676`: **SUCCESS**;
- normal `CI` run `36266376681`: **SUCCESS**.

No accepted M9 external/adoption evidence was collected before protocol freeze.

Protocol: `docs/milestones/M9_PROTOCOL.md`.
Schema: `docs/milestones/M9_EVIDENCE_SCHEMA.json`.
Tracking: GitHub Issue #51.

### M9.1 — Zero-Contact External Workload Expansion

**NEXT**

Before observing results, freeze selection criteria and pinned revisions for multiple public, unmodified external workloads. Run the unchanged public ptrace product without maintainer coordination and preserve all successes and failures. Every self-run external result remains `ZERO_CONTACT_EXTERNAL_REPRO`; it must never be represented as independent adoption.

### M9.2 — Independent Adoption Evidence

**PLANNED**

Accept independent-adoption claims only when third-party initiation/execution and provenance are auditable under the frozen schema.

### M9.3 — Evidence Synthesis / Product Decision

**PLANNED**

Conclude whether external evidence supports continued ptrace productization, identifies concrete UX/reliability work, or satisfies an explicit eBPF reopen condition. M9 must not manufacture an eBPF need in advance.
