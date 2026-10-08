# ExecSurface — Five-Minute Start

This path is designed to prove the product workflow without requiring a real project or changing system security settings.

Support: **Linux x86_64 public alpha**.

## 1. Install

No Rust toolchain is required for this path.

```bash
VERSION=v0.1.0-alpha.6
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

If a check fails, follow the action printed by `doctor` and see [Troubleshooting](TROUBLESHOOTING.md). `doctor` never changes ptrace settings or privileges.

## 3. Learn a tiny controlled baseline

```bash
mkdir -p /tmp/execsurface-quickstart
cd /tmp/execsurface-quickstart
execsurface learn -- /bin/bash -lc 'true'
```

## 4. Prove no drift

```bash
execsurface check -- /bin/bash -lc 'true'
```

Expected: `ExecSurface: PASS`.

## 5. Introduce controlled runtime drift

```bash
execsurface check -- /bin/bash -lc 'true; /bin/echo controlled-drift >/dev/null'
```

Expected: `ExecSurface: REVIEW` with exit status 10 under the built-in review policy. REVIEW does not mean malicious; it means the observed execution surface differs and requires review.

## 6. Move to your project

```bash
execsurface init --command "cargo test --locked" --github-actions
execsurface learn -- /bin/bash -lc 'cargo test --locked'
execsurface check --policy execsurface-policy.json -- /bin/bash -lc 'cargo test --locked'
```

## Wrapper consistency

The GitHub Action executes `/bin/bash -lc <command>`. Learn the baseline with the same wrapper. Wrapper mismatch is not silently ignored.

## Boundary

This quickstart proves a workflow, not program safety. Observed behavior is not all possible behavior.
