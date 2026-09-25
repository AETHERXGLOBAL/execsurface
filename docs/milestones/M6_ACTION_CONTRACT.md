# M6 — GitHub Action Contract

Date: 2026-09-25

Status: **IMPLEMENTATION GATE OPEN**

Issue: #12

## Packaging

M6 exposes a root composite Action through `action.yml`.

Current platform:

- Linux;
- x86_64;
- Rust/Cargo available on the runner;
- current observer remains ptrace metadata-only.

The M6 source-build Action uses `cargo build --locked --release`.

Prebuilt/release-distribution optimization is future work and is not a correctness claim.

## Command boundary

The Action input `command` is passed to the runner script through an environment variable.

It is never interpolated into generated shell source.

Execution is:

`/bin/bash -lc "$EXECSURFACE_COMMAND"`

The command is intentionally a shell command supplied by the workflow author.

For M3/M4 comparability, a baseline intended for this Action must be learned with the same wrapper:

```bash
execsurface learn -- /bin/bash -lc 'your command'
```

## stdout/stderr separation correction

Red-Team review found that the ptrace tracee inherits the CLI process stdout/stderr.

Therefore redirecting `execsurface check --json > report.json` would allow normal target output to corrupt structured JSON.

M6 rejects that design.

Instead, the CLI adds:

- `--json-output PATH`;
- `--markdown-output PATH`.

Structured evidence is written directly to dedicated files while target stdout/stderr remain normal CI logs.

## GitHub summary and artifact

The Action appends the Markdown report to `GITHUB_STEP_SUMMARY`.

It does not create or mutate PR comments and therefore does not require `pull-requests: write`.

When enabled, the evidence directory is uploaded with `actions/upload-artifact@v7`. The Action surfaces the artifact URL and SHA-256 digest reported by the upload action.

References:

- https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax
- https://docs.github.com/en/actions/reference/workflows-and-actions/variables
- https://github.com/actions/upload-artifact

## Verdict enforcement

- PASS → Action success.
- REVIEW → Action success by default plus GitHub warning.
- REVIEW + `fail-on-review=true` → Action failure.
- BLOCK → reports/artifact first, then Action failure.
- ERROR → error evidence where possible, then Action failure.

## SARIF decision

**SARIF findings are not generated in M6 v1.**

GitHub code scanning requires at least one location for a result to be displayed and uses the first location to determine the file to annotate.

Current ExecSurface findings prove observed runtime effects but do not prove the causal repository file/source line.

Inventing a path or line would create false provenance.

Reference:

https://docs.github.com/en/code-security/reference/code-scanning/sarif-files/sarif-support

Disposition output:

`not-generated:no-source-provenance`

Revisit after a future provenance layer establishes defensible source locations.

## Evidence files

Successful or policy-result runs produce:

- `report.json` — full M5 verdict evidence;
- `summary.md` — human-readable summary.

ERROR runs generate an explicit ERROR report with a generic non-secret message and leave detailed execution diagnostics in workflow logs.

## Summary safety

The Markdown renderer intentionally does not echo raw runtime paths into its findings table.

Policy source/rule identifiers are Markdown-escaped to prevent table/HTML formatting injection.

Full canonical evidence remains in JSON.

## Non-goals

No:

- invented source provenance;
- PR-comment mutation;
- EDR/sandbox behavior;
- claim that runtime drift equals a vulnerability.
