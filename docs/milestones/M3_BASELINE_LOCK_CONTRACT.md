# M3 — Baseline Lock Contract

Date: 2026-09-25

Status: **IMPLEMENTATION GATE OPEN**

Issue: #6

## Purpose

M3 turns an accepted M2 canonical execution surface into a deterministic, content-addressed lockfile:

`execsurface.lock.json`

The lockfile is a baseline of observed canonical behavior. It is **not policy** and contains no candidate diff or verdict.

## Digest contract v1

Algorithm:

`SHA-256`

Digest input:

> minified UTF-8 JSON serialization of `{"schema_version":1,"payload":<BaselinePayload>}`

Rules:

- `baseline_digest` itself is excluded from digest input;
- struct field order is part of format v1;
- no unordered map exists in the digest payload;
- canonical effects are sorted and deduplicated;
- semantic-root labels are sorted and deduplicated;
- observer capability/limitation lists are sorted and deduplicated;
- no timestamps enter the payload;
- digest output is lowercase hex prefixed with `sha256:`.

Changing these bytes requires a new digest-format version.

A fixed serialization+digest vector is executable test evidence.

## Baseline contents

- lock schema version;
- baseline digest;
- tool name/version;
- privacy-safe command identity:
  - canonical executable,
  - argument count,
  - optional explicit logical label;
- platform OS/architecture;
- observer name/capabilities/limitations;
- M2 canonical execution surface.

Full argv values are not stored.

Physical normalization roots are not stored; only semantic root labels are present.

## Learn semantics

```bash
execsurface learn -- python -m pytest
```

Default output:

`execsurface.lock.json`

M3 requires the target command to exit successfully. A failed/signaled command produces no baseline.

Existing lockfiles are not silently overwritten. Explicit `--overwrite` is required.

Writes use a same-directory temporary file followed by an atomic filesystem publication step.

## Optional normalization context

```text
--workspace PATH
--home PATH
--tmp PATH
--run-tmp PATH
--cache NAME=PATH
```

Without explicit roots, the CLI uses current directory, HOME, TMPDIR and /tmp as local normalization context. Physical roots are used in memory and are not serialized into the lock.

## Non-goals

M3 does not implement:

- candidate-vs-baseline diff;
- policy;
- PASS/REVIEW/BLOCK;
- SARIF;
- GitHub Action;
- performance claims.
