# M6 — GitHub Action Contract

Date: 2026-09-25

Status: **ACCEPTED**

Issue: #12

## Purpose

M6 packages the accepted M1–M5 pipeline as a GitHub composite Action without changing observation, canonicalization, baseline, diff or policy semantics.

## Platform

Current Action support:

- Linux;
- x86_64;
- Rust/Cargo available on the runner;
- current native ptrace observer.

Unsupported platforms fail explicitly.

## Command transport

The Action accepts a string `command` and executes:

`/bin/bash -lc "$EXECSURFACE_COMMAND"`

The input is first transported through the environment and then passed as a quoted argv value to Bash.

It is not interpolated into generated shell source.

Because command executable and argument count are part of M4 comparability, a baseline intended for the Action must be learned through the same wrapper:

`execsurface learn -- /bin/bash -lc 'COMMAND'`

## Inputs

- `command` — required;
- `baseline` — default `execsurface.lock.json`;
- `policy` — optional policy path;
- `fail-on-review` — default `false`;
- `upload-artifact` — default `true`;
- `artifact-name` — default `execsurface-evidence`.

## Outputs

- `verdict`;
- `exit-code`;
- `report-json`;
- `summary-markdown`;
- `sarif-status`;
- `artifact-url` when uploaded;
- `artifact-digest` when uploaded.

## Enforcement

- PASS → Action success;
- REVIEW → Action success + warning by default;
- REVIEW + `fail-on-review=true` → Action failure;
- BLOCK → Action failure;
- ERROR → Action failure.

Evidence is generated before the enforcement step where possible.

## Permissions

The Action itself does not request repository write permissions and does not post PR comments.

Recommended caller baseline:

```yaml
permissions:
  contents: read
```

## Evidence surfaces

M6 writes:

1. verdict JSON;
2. Markdown GitHub job summary;
3. evidence artifact bundle when enabled.

The artifact upload exposes GitHub's artifact URL and digest.

## SARIF Red-Team decision

**SARIF FINDING EMISSION: KILLED / DEFERRED FOR M6 v1**

Reason:

Current ExecSurface evidence identifies runtime process, file and network effects. It does not prove the causal repository source file and source line that produced an effect.

A workspace runtime path is not necessarily the source-code location responsible for the effect.

ExecSurface therefore does not invent a repository location solely to satisfy Code Scanning.

The public disposition is:

`not-generated:no-source-provenance`

Revisit SARIF only after a future provenance layer can establish defensible causal source locations.

## Security / privacy boundary

- the observer remains metadata-only;
- full target argv/env values are not part of verdict evidence;
- Markdown fields are escaped against table/HTML injection;
- Action command transport avoids direct expression interpolation into shell source;
- no write-all token is required;
- runtime drift remains evidence, not proof of a vulnerability or program unsafety.
