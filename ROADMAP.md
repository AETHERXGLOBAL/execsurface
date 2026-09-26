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

Hardened Linux ptrace evidence with syscall entry/exit pairing, successful-open fd identity, fd-attributed read/write effects, close/dup/fork/CLONE_FILES handling, openat/openat2 trace-time path semantics, bounded causal execution chains, explicit event-budget truncation, fault injection, v2 schema migration and reproducible performance evidence.

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

**CODE READY — FIRST crates.io PUBLICATION REQUIRES ACCOUNT CONFIGURATION**

Prepare the Rust workspace for first-class crates.io distribution while preserving all M6.5/M7 runtime semantics. Target user path after publication: `cargo install execsurface --locked`. Initial publication requires explicit crates.io maintainer authentication; no registry-availability claim is made before the public package is verified.

GitHub release/package preparation evidence is complete. The remaining blocker is tracked in Issue #27 and requires one-time crates.io maintainer authentication before the first registry publication.
