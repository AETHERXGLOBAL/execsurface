# M5 — Policy and Verdict Contract

Date: 2026-09-25

Status: **IMPLEMENTATION GATE OPEN**

Issue: #10

## Separation

M5 consumes an M4 diff.

It does not mutate:

- the baseline;
- the candidate surface;
- M4 added/removed/changed evidence.

## Verdicts

- `PASS`: no drift, or every drift finding is explicitly ALLOW.
- `REVIEW`: at least one finding resolves to REVIEW and none resolves to BLOCK.
- `BLOCK`: at least one finding resolves to BLOCK.
- `ERROR`: operational, integrity, normalization, comparability, policy-parse, or policy-validation failure.

Target command exit status remains separate evidence and does not itself become an ExecSurface drift verdict.

## Built-in default

Without `--policy`, unmatched drift resolves to REVIEW.

No drift always resolves to PASS, even if a custom policy has `default_action=block`.

## Rule resolution

Rules are order-independent.

For one finding:

1. collect every matching rule;
2. if no rule matches, use `default_action`;
3. if rules match, choose the most restrictive action:
   `BLOCK > REVIEW > ALLOW`.

Matched rule IDs are sorted in output.

This prevents file-order accidents from changing a verdict.

## Matching v1

Rules may match:

- change kind;
- effect kind;
- path class;
- canonical path prefix;
- executable family;
- network IP;
- network port.

Every rule must contain at least one match field.

Canonical path-prefix matching respects path-component boundaries.

Changed findings are matched against the **after** state because policy evaluates the resulting candidate behavior. The complete before/after evidence remains in the report.

## Exit codes

- PASS: 0
- ERROR: 2
- REVIEW: 10
- BLOCK: 20
- `--diff-only`: 0 after any successful M4 comparison

## M4 preservation

`execsurface check --diff-only ...` retains the policy-free M4 behavior.

## Non-goals

M5 does not implement:

- SARIF;
- GitHub Action packaging;
- PR summaries/artifact upload.

Those belong to M6.
