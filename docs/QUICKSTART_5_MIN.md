# ExecSurface — Five-Minute Start

This path is designed to prove the product workflow without requiring a real project or changing system security settings.

Support: **Linux x86_64 public alpha**.

## 1. Install

```bash
VERSION=v0.1.0-alpha.1
TARGET=x86_64-unknown-linux-gnu
ASSET="execsurface-${VERSION}-${TARGET}.tar.gz"

curl -fLO "https://github.com/AETHERXGLOBAL/execsurface/releases/download/${VERSION}/${ASSET}"
curl -fLO "https://github.com/AETHERXGLOBAL/execsurface/releases/download/${VERSION}/${ASSET}.sha256"
sha256sum -c "${ASSET}.sha256"
tar -xzf "${ASSET}"

mkdir -p "$HOME/.local/bin"
install -m 0755 "execsurface-${VERSION}-${TARGET}/execsurface" "$HOME/.local/bin/execsurface"
export PATH="$HOME/.local/bin:$PATH"
```

Confirm identity:

```bash
execsurface --version
```

Optional provenance verification:

```bash
gh attestation verify "$ASSET" -R AETHERXGLOBAL/execsurface
```

## 2. Diagnose readiness

```bash
execsurface doctor
```

If a check fails, follow the action printed by `doctor` and see [Troubleshooting](TROUBLESHOOTING.md).

`doctor` never changes ptrace settings or privileges.

## 3. Learn a tiny controlled baseline

Use the exact Bash wrapper used by the GitHub Action:

```bash
mkdir -p /tmp/execsurface-quickstart
cd /tmp/execsurface-quickstart

execsurface learn --   /bin/bash -lc 'true'
```

This creates `execsurface.lock.json`.

## 4. Prove no drift

```bash
execsurface check --   /bin/bash -lc 'true'
```

Expected result:

```text
ExecSurface: PASS
```

## 5. Introduce controlled runtime drift

Keep the same executable and argument count, but change the shell behavior:

```bash
execsurface check --   /bin/bash -lc 'true; /bin/echo controlled-drift >/dev/null'
```

The built-in policy reviews unmatched drift, so the expected result is:

```text
ExecSurface: REVIEW
```

The process exits with status 10 for REVIEW.

Nothing about REVIEW means the command is malicious. It means the observed execution surface differs from the accepted baseline and the default policy requires human review.

## 6. Move to your project

For a Rust project, for example:

```bash
execsurface init --command "cargo test --locked" --github-actions

execsurface learn --   /bin/bash -lc 'cargo test --locked'

execsurface check   --policy execsurface-policy.json   -- /bin/bash -lc 'cargo test --locked'
```

Review the generated policy, workflow and learned baseline before committing them.

## Wrapper consistency

The GitHub Action receives a shell command string and executes:

```text
/bin/bash -lc <command>
```

The local baseline must be learned with the same wrapper. ExecSurface treats command identity/comparability conservatively; wrapper mismatch is not silently ignored.

## Boundary

This quickstart proves a workflow, not program safety.

Observed behavior is not all possible behavior.
