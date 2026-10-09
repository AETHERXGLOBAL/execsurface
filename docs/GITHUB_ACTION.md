# GitHub Action

## Public channel

The current stable Action channel is:

```text
AETHERXGLOBAL/execsurface@v1
```

`v1` is the moving stable major channel and is promoted only after the immutable version release has passed release-binary, Cargo/tag and Action consumer gates.

For maximum pinning after this release, use:

```text
AETHERXGLOBAL/execsurface@v1.0.0
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
      - name: Run the target command as its own correctness gate
        run: cargo test --locked
      - name: ExecSurface runtime drift
        id: execsurface
        uses: AETHERXGLOBAL/execsurface@v1
        with:
          command: "cargo test --locked"
          baseline: execsurface.lock.json
          policy: execsurface-policy.json
          fail-on-review: "false"
          upload-artifact: "true"
      - name: Show ExecSurface verdict
        run: echo "ExecSurface verdict: ${{ steps.execsurface.outputs.verdict }}"
```

The separate target-command step above is intentional. In stable v1.0.0, ExecSurface evaluates execution-surface drift; it does not turn the wrapped target's native exit code or signal into the ExecSurface verdict.

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

### Stable v1 target-outcome boundary

The stable-v1 target command's native `exit_code` / terminating `signal` is retained in the structured report, but it is **report metadata rather than a verdict input**.

Consequences:

- `ExecSurface: PASS` means there was no policy-relevant execution-surface drift finding;
- it does **not** prove the wrapped target command succeeded;
- a nonzero or signalled target can still receive PASS if its observed surface has no review/block finding;
- run/gate a command such as `cargo test`, `pytest` or a build separately when its success is itself required;
- do not use the Action's `exit-code` output as the target command's native exit status.

This target-outcome behavior was first discovered in the Alpha line and was explicitly frozen into the v1 stable compatibility contract. Any future target-outcome enforcement requires a versioned compatibility decision.

## Outputs

- `verdict`
- `exit-code` — **ExecSurface verdict code** (`0`, `2`, `10`, `20`), not the target command's native exit code
- `report-json`
- `summary-markdown`
- `artifact-url`
- `artifact-digest`
- `sarif-status`

The target command's native exit/signal can be inspected in the structured report where available.

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

A PASS means the recorded comparison and policy did not identify review/block execution-surface drift. It does not prove the target command succeeded, and it does not prove the program is safe.

See [Current Status](STATUS.md) for the current Alpha scope and declared limitations.
