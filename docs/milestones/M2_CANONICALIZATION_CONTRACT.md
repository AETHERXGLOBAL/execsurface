# M2 — Canonicalization Contract

Date: 2026-09-25

Status: **IMPLEMENTATION GATE OPEN**

Issue: #4

## Goal

Convert backend-specific M1 raw evidence into a deterministic canonical execution surface without inventing behavior, hiding security-relevant distinctions, or leaking machine-specific identity into the surface.

## Dynamic team

Task-specific:
- Rust systems engineer
- dynamic-analysis / trace-model engineer
- reproducibility engineer
- filesystem semantics reviewer
- software supply-chain security reviewer
- deterministic serialization reviewer
- independent adversarial normalization tester

Fixed:
- Innovation Scientist / Architect
- Deviation Prevention / Scientific Integrity

## Key decisions

### Explicit semantic roots, not random-looking heuristics

M2 accepts concrete roots from the invocation context and maps them to semantic tokens:

- `$WORKSPACE`
- `$HOME`
- `$TMP`
- `$RUN_TMP`
- `$CACHE:<name>`

The concrete physical root is not emitted into the canonical surface.

A run-specific temp directory may therefore differ between executions while a suffix such as `plugin/curl` remains visible.

**KILLED:** generic regex rules that remove path segments merely because they resemble UUIDs, hashes, PIDs, timestamps or random tokens.

Reason: such rules can collapse attacker-controlled or semantically meaningful names.

### Longest declared root wins

This preserves nested semantics such as:

`$RUN_TMP` inside `$TMP`, or `$WORKSPACE` inside `$HOME`.

A physical path assigned to two different semantic root labels is rejected as ambiguous.

### Relative paths are unresolved

M1 does not record dirfd/CWD semantics sufficiently to prove what a relative `openat` path resolves to.

M2 therefore records relative paths as `relative_unresolved`.

### Parent traversal is unresolved

M2 does not perform post-hoc `realpath()`. Paths containing `..` are retained lexically and marked `contains_parent_traversal`.

This avoids pretending that a later filesystem view proves the trace-time target.

### PID, TID and sequence are not canonical identity

Raw sequence is used only to replay causal state. It is excluded from canonical effects.

Duplicate sequence identifiers are rejected rather than tie-broken arbitrarily.

### Process state

M2 replays raw spawn/exec events to associate file/network effects with the current executable image.

On spawn, a child inherits the parent's current executable image until an exec event changes it.

Canonical process effects contain executable identity, not PID.

### Repetition collapses

The execution surface represents presence of an effect, not event count.

Identical canonical effects deduplicate through deterministic ordered-set semantics.

### File open flags are not silently dropped

For the current Linux source backend, M2 maps access mode and key flags to semantic fields and preserves all remaining flag bits in `other_flags`.

This prevents read-only and write-capable opens of the same path from collapsing.

Cross-platform flag equivalence remains OPEN because M1 is Linux x86_64 only.

### Network identity

Destination IP and remote port remain canonical identity.

Hostname is not invented.

## Adversarial counterexamples

1. `/tmp/run-A/plugin/curl` and `/tmp/run-B/plugin/ssh` become `$RUN_TMP/plugin/curl` and `$RUN_TMP/plugin/ssh`, not one wildcard path.
2. `$HOME/.ssh/config` remains credential-sensitive and keeps the `.ssh/config` suffix.
3. `/workspace` must not match `/workspace2`.
4. read-only and write-only opens of the same path remain distinct.
5. remote `:443` and `:8443` remain distinct.
6. relative and `..` paths are unresolved, not guessed.

## M2 non-goals

No:
- lockfile,
- baseline digest,
- diff,
- policy/verdict,
- SARIF,
- performance claim,
- eBPF,
- filesystem content capture.

## Gate

M2 closes only after independent Linux CI proves the invariance and adversarial fixtures.
