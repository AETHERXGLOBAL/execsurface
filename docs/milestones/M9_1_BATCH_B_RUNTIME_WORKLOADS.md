# M9.1 Batch B — Frozen External Runtime Workloads

Date: 2026-09-26
Status: **FROZEN BEFORE EXECUTION**
Class: `ZERO_CONTACT_EXTERNAL_REPRO`
Base protocol: `docs/milestones/M9_PROTOCOL.md`

## Purpose

Batch A exposed two distinct external-workload facts that must be preserved rather than rewritten:

- the pinned ripgrep test-suite command was non-comparable under the frozen Rust 1.90 harness because that upstream revision requires Rust 1.96;
- the pinned fzf `go test ./...` workload completed under ptrace, but unchanged checks produced REVIEW because Go build/vet creates run-specific `$TMP/go-build*` paths.

Batch B does **not** replace or erase either result. It answers a different predeclared question: can the same pinned, unmodified external projects be built with their declared toolchain requirements and then execute stable real CLI runtime workloads under the existing ExecSurface ptrace product with repeatable baseline/check semantics?

No ExecSurface semantics, thresholds, canonicalization, privacy rules, or authority rules are changed for Batch B.

## Frozen cases

### ZC-04-RIPGREP-RUNTIME

- repository: `BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- upstream package version at revision: `15.2.0`
- external build toolchain: Rust `1.96.0` because the pinned upstream workspace declares `rust-version = "1.96"`
- build performed before measurement: `cargo +1.96.0 build --locked --release`
- measured runtime command, from the pinned checkout root:
  - `./target/release/rg --no-heading --line-number 'ripgrep' README.md >/dev/null`
- project source modification by AETHER X: none

### ZC-05-FZF-RUNTIME

- repository: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- external build prerequisite: upstream `BUILD.md` declares Go `1.23` or above
- build performed before measurement using the upstream documented command: `make`
- measured runtime command, from the pinned checkout root:
  - `printf 'alpha\nbeta\ngamma\n' | ./target/fzf --filter=beta --select-1 --exit-0 >/dev/null`
- project source modification by AETHER X: none

The fixed input is only workload input; the external project source remains pinned and unmodified. ExecSurface's M9 privacy boundary still forbids collecting stdin contents.

## Frozen acceptance

For each case, preserve and validate a schema-conformant M9 evidence record containing:

1. exact repository revision and exact command;
2. successful direct runtime command;
3. `execsurface doctor`;
4. successful baseline creation;
5. two unchanged checks, each `PASS` with zero findings;
6. complete ptrace observation and root exit semantics;
7. frozen performance protocol: 3 warmups per mode and 15 measured samples per mode, alternating direct/ptrace ordering;
8. raw artifacts and SHA-256 manifest;
9. explicit `ZERO_CONTACT_EXTERNAL_REPRO` attribution;
10. any failure retained as `PARTIAL` rather than hidden or relabeled.

Crossing the frozen M6.5 performance trigger is evidence only; it does not authorize eBPF or any backend change.

## Governance note

These cases are additive. Batch A failures remain first-class evidence and must be cited in M9.3 synthesis. Batch B results cannot be used to claim that build/test workflows are stable if those workflows showed nondeterministic drift.
