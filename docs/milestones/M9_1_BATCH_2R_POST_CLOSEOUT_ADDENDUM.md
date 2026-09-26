# M9.1 — Batch 2R Post-Closeout Evidence Addendum

Date: 2026-09-27
Status: **SEALED — COMPUTATIONAL_EVIDENCE**
Classification: `ZERO_CONTACT_EXTERNAL_REPRO`
Authority: `docs/milestones/M9_PROTOCOL.md`

## Purpose

`docs/milestones/M9_1_CLOSEOUT.md` closed M9.1 using the accepted evidence available at that point. The preregistered Batch 2R run was still completing. This addendum preserves the subsequently completed Batch 2R results without rewriting the earlier closeout or deleting any prior PARTIAL evidence.

Batch 2R repaired harness mechanics only, under the preregistration in `docs/milestones/M9_1_BATCH_2R_HARNESS_REPAIR_PREREGISTRATION.md`:

- the baseline and controlled-drift commands retained the same `/bin/bash -lc` root shape;
- external projects were staged outside the parent ExecSurface Cargo workspace;
- no observer semantics, canonicalization, thresholds, baseline semantics, policy semantics, privacy rules, or backend authority were changed;
- each case required two unchanged zero-finding PASS checks, visible controlled `sha256sum` process expansion, complete observation, the frozen 3+15 performance protocol, and evidence-schema validation.

The run was:

- GitHub Actions run: `36280138226`
- ExecSurface source SHA: `aa38b648e307f453f23ee5a637e870899b590e33`
- product version string: `0.1.0-alpha.2`
- backend: Linux x86_64 `ptrace`

A comparison from merged hardening main `6234da919d1e46b60564114929db593bbce8ef31` to the Batch 2R source SHA shows only M9.1 workflows, documentation and evidence-support scripts; no `crates/` product source changed. Therefore these cases exercise the same product core that was already merged to main.

## ZC-04R — casey/just CLI

Evidence ID: `M9-ZC-04R-JUST-CLI-001`
Claim status: **COMPUTATIONAL_EVIDENCE**

- project: `casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- runtime command: `/bin/bash -lc "./target/release/just --list >/dev/null"`
- direct runtime: PASS
- `execsurface doctor`: PASS
- baseline: PASS
- unchanged check 1: PASS, 0 findings
- unchanged check 2: PASS, 0 findings
- controlled drift: REVIEW, rc=10, 38 findings, added `sha256sum` process execution visible
- pre-performance observation: complete, exit 0
- performance protocol: PASS, 3 warmups/mode + 15 measured/mode, no excluded samples
- median direct: `113.922361 ms`
- median ptrace: `514.780499 ms`
- median absolute overhead: `400.858138 ms`
- slowdown ratio: `4.518695842337748x`
- M6.5 trigger: **CROSSED** (`D >= 100 ms` and `P/D > 2.0`)
- evidence validator SHA-256: `f9ada3eb7d7400ca19cbc654a007905cd0193ae64b28a172902219d69d2b972c`
- artifact ID: `10918483474`
- artifact ZIP digest: `sha256:bbbc1c647789c9cb3dc551014f0809dabb72b9b7232e908e8c3ceaa271bb837e`

## ZC-06R — BurntSushi/ripgrep CLI

Evidence ID: `M9-ZC-06R-RIPGREP-CLI-001`
Claim status: **COMPUTATIONAL_EVIDENCE**

- project: `BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- external build toolchain: Rust `1.96.0`
- runtime command: `/bin/bash -lc "./target/release/rg --files . >/dev/null"`
- direct runtime: PASS
- `execsurface doctor`: PASS
- baseline: PASS
- unchanged check 1: PASS, 0 findings
- unchanged check 2: PASS, 0 findings
- controlled drift: REVIEW, rc=10, 38 findings, added `sha256sum` process execution visible
- pre-performance observation: complete, exit 0
- performance protocol: PASS, 3 warmups/mode + 15 measured/mode, no excluded samples
- median direct: `113.904530 ms`
- median ptrace: `514.712956 ms`
- median absolute overhead: `400.808426 ms`
- slowdown ratio: `4.518810235203112x`
- M6.5 trigger: **CROSSED** (`D >= 100 ms` and `P/D > 2.0`)
- evidence validator SHA-256: `724d3822e6c71c665a997c3a3a0cb3b017dd7c6dce994c13ddf51662a44732fb`
- artifact ID: `10918541992`
- artifact ZIP digest: `sha256:20e87eb6414cea182d2b855bf882193d53d1853c986bb756ab61c78333231858`

## ZC-07R — sharkdp/fd CLI

Evidence ID: `M9-ZC-07R-FD-CLI-001`
Claim status: **COMPUTATIONAL_EVIDENCE**

- project: `sharkdp/fd`
- revision: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`
- runtime command: `/bin/bash -lc "./target/release/fd --hidden --type f --exclude target . >/dev/null"`
- direct runtime: PASS
- `execsurface doctor`: PASS
- baseline: PASS
- unchanged check 1: PASS, 0 findings
- unchanged check 2: PASS, 0 findings
- controlled drift: REVIEW, rc=10, 38 findings, added `sha256sum` process execution visible
- pre-performance observation: complete, exit 0
- performance protocol: PASS, 3 warmups/mode + 15 measured/mode, no excluded samples
- median direct: `113.910428 ms`
- median ptrace: `664.969642 ms`
- median absolute overhead: `551.059214 ms`
- slowdown ratio: `5.837653792328829x`
- M6.5 trigger: **CROSSED** (both the ratio condition and absolute-overhead condition are satisfied)
- evidence validator SHA-256: `8f52542840dee7302eee03f74d3354f7ba8da14f6b94eb2bbeecc4711d04b723`
- artifact ID: `10918263846`
- artifact ZIP digest: `sha256:ac1036cc3097d2a6d9943d698998496ca362e277b3316b062e41f38b3cf3d731`

## Updated M9.1 interpretation

The original closeout remains historically valid and is not rewritten. Batch 2R strengthens the completed M9.1 evidence set:

- four distinct pinned external CLI projects now have accepted, stable user-like runtime evidence across the M9.1 work (`just`, `ripgrep`, `fzf`, `fd`);
- the repaired Batch 2R cases independently demonstrate exact unchanged stability (two PASS / zero-finding checks) while preserving detection of an intentionally added process execution;
- prior PARTIAL build/test evidence remains retained and continues to show that ephemeral compiler/test surfaces are a separate OPEN product-design problem;
- multiple external workloads cross the frozen M6.5 performance trigger.

These are compatibility, stability, controlled-drift and exact-host performance results only. They are **not independent adoption evidence** and must not be counted as `INDEPENDENT_USER` cases.

## Gate consequence

M9.1 remains **CLOSED**, now with the Batch 2R evidence sealed as an addendum.

The next product gate remains:

**M9.2 — Independent Adoption Evidence**

Separately, the already-crossed M6.5 trigger permits bounded same-workload eBPF value experiments under the existing research-only authority boundary. It does not authorize eBPF PASS, learn/check, backend auto-selection, baseline interchangeability, or public eBPF exposure.
