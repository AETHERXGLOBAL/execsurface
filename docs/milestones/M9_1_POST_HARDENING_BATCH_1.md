# M9.1 Post-Hardening Zero-Contact Batch 1

Status: **SEALED — PARTIAL external evidence, no adoption claim**

This document records the first predeclared post-hardening zero-contact batch after the ptrace lifecycle fix merged to `main`.

It does **not** rewrite the observed outcomes, does **not** convert zero-contact evidence into adoption, and does **not** normalize unstable build/test surfaces after the fact.

## Authority boundary

- Public authority remains Linux x86_64 `ptrace`.
- eBPF remains research-only and cannot authorize PASS, learn/check, backend auto-selection, or baseline interchangeability.
- `ZERO_CONTACT_EXTERNAL_REPRO` is compatibility/performance evidence only and is not independent adoption.

## Source state

Ptrace lifecycle hardening was merged to `main` at verified merge commit:

`6234da919d1e46b60564114929db593bbce8ef31`

The production-clean observer source used for hardening proof had SHA-256:

`3dc656358a3675a0d78135bb83046bc45c33dca2718c8bd8b68fa090e18eac8b`

Issue #55 is closed. Batch 1 did not reproduce the old ptrace lifecycle crash.

## Frozen batch

### ZC-01-JUST

- project: `casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- frozen command: `cargo +1.90.0 test --all >/dev/null 2>&1`
- direct: PASS
- doctor: PASS
- baseline: PASS
- unchanged check 1: REVIEW / 50,774 findings
- unchanged check 2: REVIEW / 50,783 findings
- performance series: complete
- median direct: `4034.020824 ms`
- median ptrace: `14906.453411 ms`
- absolute overhead: `10872.432587 ms`
- slowdown: `3.6951850427532644x`
- M6.5 trigger: **TRUE**
- accepted status: `PARTIAL`
- evidence ID: `M9-ZC-01-JUST-POSTHARDENING-001`
- evidence JSON SHA-256: `5b98d08d079034c218289846aaa84c1b853fc3b0e4631cd50d826edd5658ecea`
- original artifact ID: `10918581109`
- original artifact ZIP SHA-256: `e20a0ed1a1566b287032096a925894e50537dd1f836ed9e5dc8f36f1411122ab`
- recovered evidence artifact ID: `10918201663`

Interpretation: observer execution completed; this is not a #55 recurrence. The full Cargo test workload produces a very large runtime surface that changes between unchanged reruns.

### ZC-02-RIPGREP

- project: `BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- frozen command: `cargo +1.90.0 test --workspace --all-targets >/dev/null 2>&1`
- ExecSurface install: PASS
- direct external command: FAIL / rc=101
- doctor: PASS
- baseline/check/performance: NOT RUN by fail-closed protocol
- accepted status: `PARTIAL`
- evidence ID: `M9-ZC-02-RIPGREP-POSTHARDENING-001`
- evidence JSON SHA-256: `0dbbfc55daf842be90522f00c0240afc6ed18341f19986c6aadc5041ede4e16c`
- original artifact ID: `10918306814`
- original artifact ZIP SHA-256: `b328178150d34d78ad8fce458e37d0155a47409da83f72ef4b084d1f30558943`
- recovered evidence artifact ID: `10917704525`

Independent direct-only diagnostic run `36279218262` reproduced the same direct failure with `execsurface_involved=false`: the pinned ripgrep dependency set requires Rust 1.96 while this frozen batch used Rust 1.90.0.

Interpretation: external toolchain incompatibility, not an ExecSurface regression.

### ZC-03-FZF

- project: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- frozen command: `go test ./... >/dev/null 2>&1`
- direct: PASS
- doctor: PASS
- baseline: PASS
- unchanged check 1: REVIEW / 701 findings
- unchanged check 2: REVIEW / 699 findings
- most findings were randomized Go temporary build paths with additional cache/toolchain variation
- performance series: complete
- median direct: `465.870373 ms`
- median ptrace: `1970.512132 ms`
- absolute overhead: `1504.641759 ms`
- slowdown: `4.2297433925895955x`
- M6.5 trigger: **TRUE**
- accepted status: `PARTIAL`
- evidence ID: `M9-ZC-03-FZF-POSTHARDENING-001`
- evidence JSON SHA-256: `09aac8194d1b46597d932d1fff3d4d73aae99cc17977d564124e064b838eefdf`
- original artifact ID: `10918495777`
- original artifact ZIP SHA-256: `17c9e09dd91d9cebdd3d9497e130ba3bdb358df931866fbdeeba53afaeba4a5e`
- recovered evidence artifact ID: `10918272587`

Interpretation: complete ptrace execution with substantial performance overhead and unstable exact baseline behavior on an ephemeral Go build/test surface.

## Batch conclusions

1. The ptrace lifecycle hardening is compatible with completing both `just` and `fzf` workloads that previously would have been high-risk process/thread cases.
2. Batch 1 does **not** support a broad success-rate claim: all three records are `PARTIAL` for different reasons.
3. `ripgrep` is excluded from product-regression interpretation because its direct command failed before ExecSurface due to the frozen Rust toolchain mismatch.
4. `just` and `fzf` reveal a separate product-design question: exact baseline stability on ephemeral build/test surfaces. This is tracked prospectively in issue #59. Batch 1 remains unchanged.
5. `just` and `fzf` independently cross the predeclared M6.5 performance trigger. This permits same-workload eBPF **value experiments only**; it does not authorize eBPF as a public backend.

## Next gates

- Predeclare Batch 2 before execution using stable, user-like runtime commands with build/setup outside the observed command.
- Run same-workload eBPF value experiments only for trigger-crossing workloads under the existing M8.8 research boundary.
- Continue M9.1 diversity without converting zero-contact evidence into adoption.
- M9.2 independent adoption remains a separate required stage.
