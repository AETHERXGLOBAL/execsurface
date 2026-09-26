# M9.1 User-Like Zero-Contact External Evidence

Status: **SEALED — COMPUTATIONAL_EVIDENCE for four pinned user-like CLI cases; no adoption claim**

This document synthesizes the preregistered Batch 2 / Batch 2R user-like external workload evidence after preserving all original harness failures separately.

The evidence class is `ZERO_CONTACT_EXTERNAL_REPRO`. It is compatibility, drift-detection, and performance evidence only. It is **not** independent user adoption.

## Why Batch 2R exists

Original Batch 2 was frozen before execution. It produced useful results but exposed two harness defects:

- `just` and `ripgrep` achieved exact zero-finding PASS on two unchanged reruns, but the controlled-drift harness changed the root executable from the learned CLI to `/bin/bash`, so ExecSurface correctly rejected the candidate as non-comparable;
- `sharkdp/fd` setup was nested under the ExecSurface Cargo workspace and Cargo rejected the external package before observation.

The original immutable artifacts were recovered and validated without rewriting those outcomes. Batch 2R was then separately preregistered before execution to repair only root-command comparability and external-workspace placement.

## Accepted user-like records

### ZC-04R — casey/just CLI

Pinned revision:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Observed command:

```text
/bin/bash -lc "./target/release/just --list >/dev/null"
```

Accepted evidence:

- evidence ID: `M9-ZC-04R-JUST-CLI-001`
- status: `COMPUTATIONAL_EVIDENCE`
- direct: PASS
- baseline: PASS
- unchanged check #1: PASS / 0 findings
- unchanged check #2: PASS / 0 findings
- controlled drift: REVIEW / 38 findings
- added `sha256sum` process execution: visible
- complete ptrace performance series: PASS
- median direct: `113.922361 ms`
- median ptrace: `514.780499 ms`
- absolute overhead: `400.858138 ms`
- slowdown: `4.518695842337748x`
- M6.5 trigger: **TRUE**
- evidence JSON SHA-256: `f9ada3eb7d7400ca19cbc654a007905cd0193ae64b28a172902219d69d2b972c`
- artifact ID: `10918483474`
- artifact ZIP SHA-256: `bbbc1c647789c9cb3dc551014f0809dabb72b9b7232e908e8c3ceaa271bb837e`

### ZC-05 — junegunn/fzf deterministic filter CLI

Pinned revision:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Observed command: deterministic compiled `fzf --filter beta` pipeline, with build outside observation.

Accepted evidence:

- evidence ID: `M9-ZC-05-FZF-CLI-ORIGINAL-001`
- status: `COMPUTATIONAL_EVIDENCE`
- direct: PASS
- baseline: PASS
- unchanged check #1: PASS / 0 findings
- unchanged check #2: PASS / 0 findings
- controlled drift: REVIEW / 38 findings
- added `sha256sum` process execution: visible
- complete ptrace performance series: PASS
- median direct: `113.940715 ms`
- median ptrace: `514.789188 ms`
- absolute overhead: `400.848473 ms`
- slowdown: `4.5180442127294x`
- M6.5 trigger: **TRUE**
- evidence JSON SHA-256: `4256ae2ff8b855775691c7a2d5c03fcd41193f52e895d0aec33705e03d23b441`
- recovered evidence artifact ID: `10918163602`
- recovered artifact ZIP SHA-256: `fcf19b466fb47e4e8b44e40ffefc972234a99b177310c6141876e4e56922d044`
- original Batch 2 artifact ID: `10917958670`
- original Batch 2 artifact ZIP SHA-256: `b51ce2167a9d95a52b2ea09ead1e9c9f6fb474b7e6ce72dc57d0d4cc9f215794`

### ZC-06R — BurntSushi/ripgrep CLI

Pinned revision:

`BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`

Build toolchain: Rust `1.96.0`, as established by the retained Batch 1 direct-only toolchain diagnostic.

Observed command:

```text
/bin/bash -lc "./target/release/rg --files . >/dev/null"
```

Accepted evidence:

- evidence ID: `M9-ZC-06R-RIPGREP-CLI-001`
- status: `COMPUTATIONAL_EVIDENCE`
- direct: PASS
- baseline: PASS
- unchanged check #1: PASS / 0 findings
- unchanged check #2: PASS / 0 findings
- controlled drift: REVIEW / 38 findings
- added `sha256sum` process execution: visible
- complete ptrace performance series: PASS
- median direct: `113.904530 ms`
- median ptrace: `514.712956 ms`
- absolute overhead: `400.808426 ms`
- slowdown: `4.518810235203112x`
- M6.5 trigger: **TRUE**
- evidence JSON SHA-256: `724d3822e6c71c665a997c3a3a0cb3b017dd7c6dce994c13ddf51662a44732fb`
- artifact ID: `10918541992`
- artifact ZIP SHA-256: `20e87eb6414cea182d2b855bf882193d53d1853c986bb756ab61c78333231858`

### ZC-07R — sharkdp/fd CLI

Pinned revision:

`sharkdp/fd@ce97e473ebaec49697c07daa50a7bc2b32f713d2`

Observed command:

```text
/bin/bash -lc "./target/release/fd --hidden --type f --exclude target . >/dev/null"
```

Accepted evidence:

- evidence ID: `M9-ZC-07R-FD-CLI-001`
- status: `COMPUTATIONAL_EVIDENCE`
- direct: PASS
- baseline: PASS
- unchanged check #1: PASS / 0 findings
- unchanged check #2: PASS / 0 findings
- controlled drift: REVIEW / 38 findings
- added `sha256sum` process execution: visible
- complete ptrace performance series: PASS
- median direct: `113.910428 ms`
- median ptrace: `664.969642 ms`
- absolute overhead: `551.059214 ms`
- slowdown: `5.837653792328829x`
- M6.5 trigger: **TRUE**
- evidence JSON SHA-256: `8f52542840dee7302eee03f74d3354f7ba8da14f6b94eb2bbeecc4711d04b723`
- artifact ID: `10918263846`
- artifact ZIP SHA-256: `ac1036cc3097d2a6d9943d698998496ca362e277b3316b062e41f38b3cf3d731`

## What this batch proves

Within the exact pinned host/protocol scope:

1. The hardened ptrace observer completed all four user-like external CLI cases.
2. Each case supported baseline learning followed by two unchanged zero-finding PASS checks.
3. Each case surfaced a preregistered additional `sha256sum` process as controlled runtime expansion.
4. The large instability seen in Batch 1 full build/test commands did **not** reproduce in these user-like compiled CLI commands.
5. Therefore the current evidence supports a narrower distinction: exact baseline stability is strong for these user-like CLI cases, while build/test graphs can contain large ephemeral execution-surface variation.
6. This does not justify retroactive normalization of Batch 1. Issue #59 remains the prospective design venue for build/test surface semantics.

## Performance finding

All four user-like cases crossed the predeclared M6.5 performance trigger.

The three shell-wrapped short Rust CLI cases show a roughly `400–551 ms` median absolute ptrace overhead on this GitHub-hosted runner class. Because direct runtime is only about `114 ms`, slowdown ratios are large even when semantic stability is excellent.

This is evidence of a real product-performance pressure point. It is not evidence that eBPF is semantically equivalent to ptrace.

## eBPF consequence

M8.8 V1 is satisfied by real external workloads. A same-workload eBPF value screen is therefore permitted and has been separately preregistered.

The following authority facts remain unchanged regardless of its timing result:

- ptrace is the correctness reference;
- eBPF full-surface comparability is false;
- eBPF learn/check are unauthorized;
- eBPF PASS authority is unauthorized;
- backend auto-selection is unauthorized;
- ptrace/eBPF baselines are not interchangeable;
- public eBPF integration remains unauthorized unless future evidence closes the explicit semantic and operational gates.

## M9.1 interpretation

This batch materially strengthens external compatibility evidence but does not close M9.2. No case here is an `INDEPENDENT_USER` adoption event.
