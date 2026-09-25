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

**NOT STARTED**

`execsurface learn -- COMMAND` → deterministic `execsurface.lock.json` + digest.

## M4 — Diff Engine

**NOT STARTED**

Added/removed/changed canonical effects.

## M5 — Policy / Verdict

**NOT STARTED**

PASS / REVIEW / BLOCK / ERROR with policy independent of baseline.

## M6 — GitHub Action

**NOT STARTED**

PR summary, JSON, Markdown, semantically appropriate SARIF, evidence artifact.

## M7 — External Real-World Proof

**NOT STARTED**

Compatibility proof against an external open-source project before requesting maintainer integration.
