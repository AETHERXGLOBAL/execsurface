# Post-M9 — Ptrace Fast-Path A2 External Value Attempt 1

Date: 2026-09-27
Status: **HARNESS FAILURE — NO PERFORMANCE VALUE RESULT**
Parent: GitHub issue #84
Protocol: `docs/milestones/POST_M9_PTRACE_FASTPATH_A2_EXTERNAL_VALUE_PROTOCOL.md`
Workflow run: `36289829323`
Source SHA: `76a10fbc95601b4605a4275d8dee6a990a8c9f1e`

## Decision

Attempt 1 produced **no accepted semantic or timing result** for Candidate A2.

Both target jobs failed at the same pre-measurement harness step while applying/building A2. The failure occurred before the frozen production-semantic prerequisite and before any warmup or measured sample was executed.

Therefore this run is not evidence that A2 is faster, slower, semantically different, or value-negative.

## Exact failure

The workflow installed Rust `1.90.0` using the `minimal` profile, then later invoked:

`cargo +1.90.0 fmt --all`

without installing the `rustfmt` component.

The observed error was:

`error: 'cargo-fmt' is not installed for the toolchain '1.90.0-x86_64-unknown-linux-gnu'`

The workflow exited at that point in both matrix jobs.

## What completed before the block

For both pinned targets:

- ExecSurface source checkout: PASS;
- exact pinned external checkout: PASS;
- frozen environment/command setup: PASS;
- external target build: PASS;
- direct frozen target command: PASS;
- immutable reference ExecSurface release build: PASS.

The failure occurred in `Apply A2 and build candidate binary` before A2 candidate compilation completed.

## Preserved artifacts

### ripgrep

- target: `PTRACE-A2-VALUE-RG-001`
- artifact ID: `10921427982`
- digest: `sha256:325cf6618adb9b023bde1d1401669692ee49b7f97b931002f171c2a76f0bd6ae`

### fzf

- target: `PTRACE-A2-VALUE-FZF-001`
- artifact ID: `10921349558`
- digest: `sha256:2ea8ed926abea0ddc2fa370ad74b613ecb474adb6e0ef7b574036bdc12c7116f`

The failed run and artifacts remain preserved and are not relabeled PASS.

## Prospective correction

The only allowed correction for Attempt 2 is harness/toolchain setup:

- install Rust `1.90.0` with the `rustfmt` component before invoking `cargo fmt`.

No Candidate A2 semantics, target revision, target command, warmup count, measured sample count, sample ordering, semantic prerequisite, statistic, or `10%` acceptance threshold may change because of this failure.

Attempt 2 must remain governed by the existing frozen external-value protocol.

## Authority boundary

- Candidate A2 remains unmerged;
- public runtime remains unchanged;
- public release remains `v0.1.0-alpha.3`;
- no performance-value claim is accepted from Attempt 1;
- eBPF authority remains unchanged.
