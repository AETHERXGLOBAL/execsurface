# ExecSurface — P4-A1.4 Public Anti-Drift Result

Date: 2026-09-29
Parent: #108 / #107 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P4_A1_4_PUBLIC_ANTIDRIFT_PROTOCOL.md`
Status: **CLOSED — PASS**

## Decisions

`P4_A1_PUBLIC_ANTIDRIFT_PASS`

`P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`

A fresh anti-drift reproof was executed after the accepted A1.3U attempt-authority correction. The result establishes that the research-only proposition-authority work did not alter the public/runtime crates or the immutable alpha.4 release references.

## Accepted evidence

Accepted source:
`28a5bfdb4682bc9aaa9d3580b9085e697bd91980`

Workflow:
`36587339898`

Job:
`109471050275`

Artifact:
`11042357244`

Artifact SHA-256:
`sha256:1187080e0d965ec6ada431b7515ae87a4dc04bee32281c27b67bccf80173b819`

## Gates passed

- development branch boundary PASS;
- `v0.1.0-alpha.4` still resolves to immutable source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable `v0.1` still resolves to the same immutable alpha.4 source;
- post-A1.2 path audit found **0 public/runtime crate drift**;
- bounded M12 fixtures compiled successfully;
- full workspace rustfmt PASS;
- full workspace clippy `-D warnings` PASS;
- full `execsurface-observe --all-targets` regression PASS;
- explicit M11 shared-FD fail-closed replay **6/6 PASS**;
- explicit M12 adversarial replay **6/6 PASS**;
- full workspace regression PASS;
- `Cargo.lock` integrity PASS.

The observer regression included the current fail-closed and research-certificate tests, while the public M11 and M12 contracts remained unchanged.

## A1 closure interpretation

A1 now has bounded executable evidence for:

1. canonical proposition-scoped authority/completeness invariants;
2. raw-v2 ptrace evidence mapping without changing collector/public bytes;
3. independent false-authority attacks;
4. bounded attempt-only usability without attempt-to-success laundering;
5. fresh public/runtime anti-drift after the final A1 authority correction.

Therefore A1 closes as:

`P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`

This is a research-only architecture result. It does not authorize a public backend selector, public Semantics-v3 migration, eBPF/BPF-LSM promotion, release, tag movement, or reinterpretation of raw-v2 evidence.

## Next authorized gate

P4-A2 — Authority-Gap Matrix.

A2 must classify the frozen proposition inventory against the accepted ptrace adapter proposition-by-proposition. It must identify concrete capability/authority gaps without assigning a scalar backend score and without authorizing a new backend merely because a gap exists.
