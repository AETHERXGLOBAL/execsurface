# ExecSurface — P4-A1.4 Public Anti-Drift Result

Date: 2026-09-29
Parent: #108 / #107 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P4_A1_4_PUBLIC_ANTIDRIFT_PROTOCOL.md`
Status: **CLOSED — PASS**

## Decisions

`P4_A1_PUBLIC_ANTIDRIFT_PASS`

`P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`

A final anti-drift reproof was executed after the accepted A1.3U attempt-authority correction and after the A1.4 protocol was strengthened to require that correction explicitly. The result establishes that the research-only proposition-authority work did not alter the public/runtime crates or the immutable alpha.4 release references.

## Strongest accepted evidence

Accepted source:
`08969deddf8c0cac154e73922672c60b8c0d3660`

Workflow:
`36587685968`

Job:
`109472277417`

Artifact:
`11042292788`

Artifact SHA-256:
`sha256:5f51fae16e7ec7c2486d62fe8438df72c8e0178ec82ef5774df4e80317bd8597`

Required A1.3U correction ancestor:
`7dfdd3958ac7e6c38cb52859d883035fa6e9ace2`

## Gates passed

- development branch boundary PASS;
- explicit A1.3 result boundary PASS;
- explicit A1.3U result boundary PASS;
- accepted A1.3U correction commit verified as an ancestor of the tested source;
- `v0.1.0-alpha.4` resolves to immutable source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable `v0.1` resolves to the same immutable alpha.4 source;
- post-A1.2 path audit found **0 public/runtime crate drift**;
- bounded M12 fixtures compiled successfully;
- full workspace rustfmt PASS;
- full workspace clippy `-D warnings` PASS;
- full `execsurface-observe --all-targets` regression PASS;
- observer unit suite: 21 passed, 0 failed, 3 dedicated-live tests ignored by the general gate;
- ptrace Linux integration suite **11/11 PASS**;
- explicit M11 shared-FD fail-closed replay **6/6 PASS**;
- explicit M12 adversarial replay **6/6 PASS** with `M12_PATH_RUNS=300`;
- full workspace regression PASS;
- Semantics v3 model tests **7/7 PASS** inside the workspace reproof;
- `Cargo.lock` integrity PASS.

The workflow explicitly recorded:

`P4_A1_PUBLIC_ANTIDRIFT_PASS`

`P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`

## Retained earlier evidence and pre-test boundary failure

The earlier successful A1.4 run is retained as supporting evidence:
- source `28a5bfdb4682bc9aaa9d3580b9085e697bd91980`;
- workflow `36587339898`;
- job `109471050275`;
- artifact `11042357244`;
- artifact SHA-256 `sha256:1187080e0d965ec6ada431b7515ae87a4dc04bee32281c27b67bccf80173b819`.

After the protocol was strengthened to require the A1.3U correction explicitly, workflow `36587517026` stopped before regression execution because the workflow still checked the older protocol marker. That attempt remains retained as a boundary-sync engineering failure; no scientific regression result was produced from it. The workflow boundary check was then corrected without changing any regression criterion.

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