# M9.1 Batch 2R — Harness Repair Pre-Registration

Status: **FROZEN BEFORE REPAIR EXECUTION**

Batch 2 exposed two mechanical harness defects after the protocol was frozen:

1. for `just` and `ripgrep`, the user-like baseline used the external executable as the root command, while the controlled-drift case changed the root command to `/bin/bash -lc`; ExecSurface correctly rejected the candidate as non-comparable before producing a drift report;
2. for `sharkdp/fd`, the external checkout was nested under the ExecSurface Cargo workspace, so Cargo rejected setup before any ExecSurface observation.

The original Batch 2 artifacts remain preserved and are not replaced by this repair run.

## Frozen repair rule

Only `ZC-04`, `ZC-06`, and `ZC-07` are rerun. `ZC-05-FZF-CLI` is not rerun because its original Batch 2 harness was already comparable and completed successfully.

For every repaired case:

- the pinned external repository is copied unchanged to `$RUNNER_TEMP` before build;
- build/setup occurs outside observation;
- the root observed command is `/bin/bash -lc` for **baseline, unchanged checks, controlled drift, and performance**;
- baseline and controlled-drift scripts therefore retain the same root executable and the same argument count;
- no normalization, ignore rule, baseline semantic change, or product code change is permitted;
- two unchanged checks must be exact zero-finding PASS before the controlled-drift case is attempted;
- controlled drift must surface added `sha256sum` process execution;
- the frozen performance protocol remains 3 warmups/mode and 15 measured samples/mode with alternating direct/ptrace order.

## ZC-04R — casey/just CLI

Revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Setup:

```bash
cargo +1.90.0 build --locked --release
```

Frozen baseline / unchanged root command:

```bash
/bin/bash -lc "./target/release/just --list >/dev/null"
```

Frozen controlled drift:

```bash
/bin/bash -lc "./target/release/just --list >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null"
```

## ZC-06R — BurntSushi/ripgrep CLI

Revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`

Setup toolchain: Rust `1.96.0`.

Setup:

```bash
cargo +1.96.0 build --locked --release
```

Frozen baseline / unchanged root command:

```bash
/bin/bash -lc "./target/release/rg --files . >/dev/null"
```

Frozen controlled drift:

```bash
/bin/bash -lc "./target/release/rg --files . >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null"
```

## ZC-07R — sharkdp/fd CLI

Revision: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`

Setup:

```bash
cargo +1.90.0 build --locked --release --all-features
```

Frozen baseline / unchanged root command:

```bash
/bin/bash -lc "./target/release/fd --hidden --type f --exclude target . >/dev/null"
```

Frozen controlled drift:

```bash
/bin/bash -lc "./target/release/fd --hidden --type f --exclude target . >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null"
```

## Acceptance per repaired case

A case receives `COMPUTATIONAL_EVIDENCE` only if all of the following hold:

1. pinned provenance verified;
2. external setup succeeds outside the ExecSurface workspace;
3. direct baseline command succeeds;
4. ExecSurface doctor succeeds;
5. learn succeeds;
6. unchanged check #1 = PASS with zero findings;
7. unchanged check #2 = PASS with zero findings;
8. controlled drift = REVIEW or stronger and includes added `sha256sum` process execution;
9. pre-performance observe is complete and exit 0;
10. 3+15 performance protocol completes with no excluded samples;
11. evidence validator passes.

Any failure remains `PARTIAL`; no post-hoc repair is allowed inside the same run.

## Interpretation

Batch 2R repairs only test-harness comparability and workspace placement. It does not erase or relabel the original Batch 2 results, and it does not count as independent adoption.
