# Post-M9 — Ptrace Fast-Path Parity Comparator

Date: 2026-09-27
Status: **FROZEN BEFORE ADVERSARIAL PARITY EXECUTION**
Parent: GitHub issue #84
Protocol: `docs/milestones/POST_M9_PTRACE_PHASE_AWARE_EXIT_FASTPATH_PROTOCOL.md`
Matrix: `docs/milestones/POST_M9_PTRACE_FASTPATH_TEST_MATRIX.md`

## Purpose

The semantic gate requires reference and Candidate A observations to be exactly equivalent in events, warnings, completeness, and command outcome. Reference and candidate are necessarily separate traced executions, so Linux assigns different numeric TIDs/PIDs even when the semantic process tree is identical.

This document freezes the only allowed comparison normalization before the adversarial parity run.

## Allowed normalization

For each observation independently, replace trace-local numeric identifiers in the following fields only:

- `event.tid`;
- `ProcessSpawn.child_tid`;
- `warning.tid` when present.

Identifiers are mapped to canonical integers `T1`, `T2`, ... in deterministic first-appearance order while scanning the serialized observation in event sequence order, then warnings in their existing order. The same identifier must always map to the same canonical token within one observation.

No other value may be normalized.

## Exact equality after TID renaming

After the trace-local TID renaming above, reference and candidate must compare exactly for:

- event count;
- event sequence numbers;
- every event kind and every non-TID field;
- every path;
- every file operation and flags/resolve metadata;
- every process-spawn mechanism;
- every network endpoint;
- warnings, including code, message and order;
- `complete`;
- command exit code and signal;
- backend metadata exposed by the same reference backend.

If an event order differs, a path differs, a warning differs, completeness differs, an outcome differs, or any non-TID field differs, the row fails. The comparator must not sort events, drop duplicates, suppress warnings, rewrite paths, alter sequence numbers, or canonicalize any product semantics.

## Stable fixture inputs

The parity workflow must keep command arguments, working paths, fixture files and network endpoint identical between the reference and candidate executions for a row. Mutable fixture state must be reset to the same bytes/metadata before each mode when the row depends on it.

## Classification

This comparator is a test-only representation for two separate process executions. It does not change ExecSurface raw evidence, baseline/canonicalization semantics, backend ID, privacy rules, or public runtime behavior.

Any comparator ambiguity or unexpected field shape fails closed and blocks Candidate A semantic acceptance.
