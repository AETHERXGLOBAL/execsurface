# GitHub Action

## Public channel

The public-alpha Action channel is:

```text
AETHERXGLOBAL/execsurface@v0.1
```

`v0.1` is a moving minor channel, but it is promoted only after the immutable version release has passed release-binary, Cargo fallback and Action consumer gates.

For maximum pinning after this release, use:

```text
AETHERXGLOBAL/execsurface@v0.1.0-alpha.4
```

Do not use `@main` as the normal consumer path.

## Minimal workflow

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
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
      - name: ExecSurface runtime drift
        id: execsurface
        uses: AETHERXGLOBAL/execsurface@v0.1
        with:
          command: "cargo test --locked"
          baseline: execsurface.lock.json
          policy: execsurface-policy.json
          fail-on-review: "false"
          upload-artifact: "true"
      - name: Show verdict
        run: echo "ExecSurface verdict: ${{ steps.execsurface.outputs.verdict }}"
```

## Binary installation inside the Action

Remote Action consumption does not build ExecSurface from source. The Action reads its pinned immutable release tag, downloads the Linux x86_64 archive and checksum, verifies the checksum and version, then runs the accepted check/verdict path. Repository-local `uses: ./` development uses a locked source build.

## Wrapper consistency

The Action executes `/bin/bash -lc <command>`. Learn with the same wrapper:

```bash
execsurface learn -- /bin/bash -lc 'cargo test --locked'
```

`execsurface init --command "cargo test --locked" --github-actions` generates a starter workflow and matching learn/check commands.

## Policy behavior

Without an explicit policy, unmatched drift is REVIEW. PASS succeeds; REVIEW succeeds by default; `fail-on-review=true` makes REVIEW fail; BLOCK fails; ERROR fails.

## Outputs

- `verdict`
- `exit-code`
- `report-json`
- `summary-markdown`
- `artifact-url`
- `artifact-digest`
- `sarif-status`

## SARIF

Current runtime effects do not prove which repository source file/line caused an effect. `sarif-status` remains `not-generated:no-source-provenance`.

## Permissions

Start with:

```yaml
permissions:
  contents: read
```

ExecSurface does not require PR-comment write permission.

## Security boundary

A PASS means the recorded comparison and policy did not identify review/block drift. It does not prove the program is safe.
