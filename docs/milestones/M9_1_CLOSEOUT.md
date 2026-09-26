# M9.1 — Zero-Contact External Workload Expansion Closeout

Date: 2026-09-26
Status: **CLOSED — MIXED EVIDENCE PRESERVED**
Primary classification: `ZERO_CONTACT_EXTERNAL_REPRO`
Authority: `docs/milestones/M9_PROTOCOL.md`

## Decision

M9.1 is closed with two accepted, fully comparable external runtime cases and retained PARTIAL build/test cases. The evidence supports a narrow conclusion:

> The post-hardening ptrace product can complete baseline -> unchanged check workflows with zero findings on pinned, unmodified external CLI runtime workloads, while full build/test workflows can expose large legitimate run-to-run execution-surface variation from temporary compiler/test artifacts.

This is **not** independent adoption evidence. M9.2 remains required.

No observer semantics, canonicalization, thresholds, privacy rules, or backend authority were changed to improve M9.1 outcomes.

## Post-hardening prerequisite

The M9 ptrace lifecycle hardening was merged to `main` as `6234da919d1e46b60564114929db593bbce8ef31`. Main CI and the dedicated ptrace lifecycle regression passed, and issue #55 was closed before the post-hardening M9.1 evidence was accepted.

## Accepted external runtime evidence

### M9-ZC-04-RIPGREP-RUNTIME-001 — COMPUTATIONAL_EVIDENCE

- project: `BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- external build toolchain: Rust 1.96.0, matching the pinned upstream `rust-version = "1.96"`
- build: PASS
- direct runtime: PASS
- `execsurface doctor`: PASS
- baseline: PASS
- unchanged check 1: PASS, 0 findings
- unchanged check 2: PASS, 0 findings
- observation/performance completeness: PASS
- artifact: `10918647233`
- artifact digest: `sha256:1ff225748f654ca04c400baf3797370fab5ef48141a59e855401e94153bed9c0`
- evidence validator SHA-256: `ee461865e5ae3f25d9ec3d70d0f9d3cdcaf392ea45f05fafa308d87c188f51c5`

Performance, exact host only:

- median direct: `114.008558 ms`
- median ptrace: `514.993235 ms`
- median absolute overhead: `400.984677 ms`
- slowdown ratio: `4.517145414645101`
- frozen M6.5 trigger: **CROSSED** (`D >= 100 ms` and ratio `> 2.0`)

### M9-ZC-05-FZF-RUNTIME-003 — COMPUTATIONAL_EVIDENCE

- project: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- upstream build: `make`, with full git history/tags required for upstream version metadata
- corrected executable path was frozen before execution in `4eb2e20446298cf5869103b113d36458b07c4585`, grounded in the pinned upstream Makefile (`target/fzf-linux_amd64` on x86_64 Linux)
- build: PASS
- direct runtime: PASS
- `execsurface doctor`: PASS
- baseline: PASS
- unchanged check 1: PASS, 0 findings
- unchanged check 2: PASS, 0 findings
- observation/performance completeness: PASS
- artifact: `10918691829`
- artifact digest: `sha256:850cc6883ebdbb76f0800288ec5d5d81a5f9bf611e25ae3b0f7fccd4a6143ea6`
- evidence validator SHA-256: `06b722460c8d105c500551cd2fdba098af94f24e35ba6e1ae3fcf864d34b7094`

Performance, exact host only:

- median direct: `114.110023 ms`
- median ptrace: `565.135575 ms`
- median absolute overhead: `451.025552 ms`
- slowdown ratio: `4.952549829912838`
- frozen M6.5 trigger: **CROSSED** (`D >= 100 ms` and ratio `> 2.0`)

## Retained PARTIAL external evidence

### M9-ZC-01-JUST-POSTHARDENING-002 — PARTIAL

- project: `casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- direct: PASS
- doctor: PASS
- baseline: PASS
- unchanged checks: REVIEW, about `49.9k` findings per run
- dominant variation: test-generated temporary executables/files under `/run/user/.../just-*` plus build/test artifacts
- performance protocol: complete
- artifact: `10917857559`
- artifact digest: `sha256:0a22ae1a75ab83b7bc7efd15705bde99c8cf72619f5ac8dddbb391966a8e7756`
- evidence validator SHA-256: `7f4ff51ae1588f0194355d42249e79e3a3ce76d64660db7184dfed7b22fedbd6`
- median direct: `5137.341042 ms`
- median ptrace: `19229.852469 ms`
- slowdown ratio: `3.743152792813945`
- M6.5 trigger: **CROSSED**

This result is retained as external build/test nondeterminism evidence. The findings are not normalized away merely to force PASS.

### M9-ZC-03-FZF-POSTHARDENING-002 — PARTIAL

- workload: pinned `go test ./...`
- direct: PASS
- doctor: PASS
- baseline: PASS
- unchanged checks: REVIEW due run-specific Go `$TMP/go-build*` / vet/build paths
- performance protocol: complete
- artifact: `10918690918`
- artifact digest: `sha256:4b13f724052d0ac0401af65449da2c81326add0e85ac9ff60d2f205a03b26bb2`
- evidence validator SHA-256: `5d34486c0067e0c2a5adbad29b858de613a93764765e64ea5b8ff33d35685ed3`
- median direct: `365.411697 ms`
- median ptrace: `1468.653486 ms`
- slowdown ratio: `4.019174804905054`
- M6.5 trigger: **CROSSED**

Again, the nondeterministic build graph is retained rather than hidden by post-hoc normalization.

### M9-ZC-02-RIPGREP-POSTHARDENING-002 — PARTIAL / NON-COMPARABLE

- original Batch A harness forced Rust 1.90.0
- pinned ripgrep upstream requires Rust 1.96
- direct upstream test command therefore failed before ExecSurface comparison
- artifact: `10918232231`
- artifact digest: `sha256:5399181db38b55292f914f17ac2b7babaf92c6d3c8e0d7f6286e336790e82167`
- evidence validator SHA-256: `e8e23e3199c3d78d026b174cf6770375aa042da8d1d666fb7d4dc8c82efd3677`

This is harness/toolchain evidence, not an ExecSurface compatibility failure. The later pre-frozen runtime case corrected the external toolchain and passed fully.

## Preserved fzf Batch B harness corrections

The fzf runtime workload required two transparent harness corrections. Neither prior record was deleted or rewritten.

1. `M9-ZC-05-FZF-RUNTIME-001` — PARTIAL: shallow checkout omitted tags needed by upstream `make` to derive version metadata. Run `36279671507`, artifact `10918905123`, digest `sha256:c37b0f1345be05bde2b0729d353066301acd1b92b3dd9ef826af0b5dfb3e9891`, validator SHA-256 `13966c00bd62a9e79c5202d8fe94c07c59b6507f0e44bce8baec9bb9659793dd`.
2. `M9-ZC-05-FZF-RUNTIME-002` — PARTIAL: upstream `make` succeeded but the first frozen command referenced the wrong output filename (`target/fzf` rather than the Makefile-derived `target/fzf-linux_amd64`). Run `36279861164`, artifact `10918252014`, digest `sha256:ba93f5ef59fae0700d4d854f6ae30d23474b6da5fca634bbf95a0e3df3c62480`, validator SHA-256 `8fc474155ad7f89ae39c1787473b16c46390ca1104144d9e5753d3e7a5ecd0d8`.

## Scientific interpretation

### PROVED / COMPUTATIONAL_EVIDENCE within the tested scope

- The lifecycle hardening removed the previously blocking ptrace ESRCH failure on the pinned external workload and is merged to main.
- Two distinct pinned external CLI runtime workloads (`ripgrep`, `fzf`) complete baseline and two unchanged checks with zero findings under ptrace.
- The frozen 3+15 protocol shows substantial ptrace overhead on multiple exact-host external workloads.

### PARTIAL / OPEN

- Full build/test workloads can create large legitimate run-to-run execution-surface variation. Whether a future product should offer an explicitly scoped build-ephemeral abstraction is **OPEN** and must not be solved by blanket temp-path suppression.
- The M6.5 performance reopen condition is now satisfied by multiple external workloads. Per protocol, this permits a separate same-workload eBPF value experiment only. It does **not** authorize eBPF `learn/check`, eBPF PASS, backend auto-selection, baseline interchangeability, or public eBPF exposure.
- Independent user adoption is still **OPEN** and belongs to M9.2.

## M9.1 gate result

**CLOSED.**

Proceed to:

1. **M9.2 — Independent adoption evidence**, preserving the strict `INDEPENDENT_USER` definition; and
2. a bounded, separate same-workload eBPF value experiment may now be opened because the predeclared M6.5 trigger was crossed. Any such experiment remains research-only and cannot change public authority without its own evidence/security/product gates.
