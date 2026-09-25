# GitHub Action

## Current development usage

```yaml
name: ExecSurface

on:
  pull_request:

permissions:
  contents: read

jobs:
  execsurface:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v6

      - name: ExecSurface runtime drift
        id: execsurface
        uses: AETHERXGLOBAL/execsurface@main
        with:
          command: "cargo test --locked"
          baseline: execsurface.lock.json
          policy: execsurface-policy.json
          fail-on-review: "false"
          upload-artifact: "true"

      - name: Show verdict
        run: echo "ExecSurface verdict: ${{ steps.execsurface.outputs.verdict }}"
```

Until a release tag exists, `@main` is for development evaluation only. For stable production use, pin a reviewed tag or commit SHA.

## Learn the Action baseline

The Action executes your command through Bash:

`/bin/bash -lc <command>`

Learn the baseline using the identical wrapper:

```bash
execsurface learn -- /bin/bash -lc 'cargo test --locked'
```

Commit the resulting `execsurface.lock.json` after review.

## Policy behavior

Without an explicit policy, M5's built-in policy returns REVIEW for unmatched drift.

- PASS succeeds.
- REVIEW succeeds by default and emits a GitHub warning.
- `fail-on-review=true` makes REVIEW fail the Action.
- BLOCK fails.
- ERROR fails.

## Outputs

The Action exposes the verdict, stable ExecSurface exit code, JSON/Markdown evidence paths, artifact URL and artifact digest.

## SARIF

M6 intentionally does not emit SARIF findings.

The current runtime evidence does not prove which repository source file/line caused an observed process, file or network effect. Mapping a runtime path to a source location would create false precision.

`sarif-status` is therefore:

`not-generated:no-source-provenance`

## Permissions

The Action does not require PR-comment write access.

Start with:

```yaml
permissions:
  contents: read
```

Add other permissions only for unrelated workflow requirements.
