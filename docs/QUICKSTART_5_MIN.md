# ExecSurface — Five-Minute Start

This path is designed to prove the product workflow without requiring a real project or changing system security settings.

Support: **Linux x86_64 public alpha**.

## 1. Install

No Rust toolchain is required for this path.

```bash
VERSION=v1.0.0
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

### Before pushing the generated GitHub workflow

The generated workflow runs the target command as its own correctness gate **and** runs ExecSurface against the same command. This is intentional because a stable-v1 ExecSurface PASS is not the target command's native exit status.

The generated workflow also requires two externally trusted GitHub variables:

- `EXECSURFACE_BASELINE_DIGEST` — the `baseline_digest` from the baseline you just learned;
- `EXECSURFACE_POLICY_SHA256` — `sha256:<SHA-256 of the exact execsurface-policy.json bytes>`.

After `execsurface init --command "cargo test --locked" --github-actions`, the CLI prints copy-paste `gh variable set` commands for both values, including the command that computes the policy SHA-256. Store these values in trusted GitHub repository/environment variables rather than checkout files.

Review the generated workflow, baseline, and policy before committing them.

## Wrapper consistency

The GitHub Action executes `/bin/bash -lc <command>`. Learn the baseline with the same wrapper. Wrapper mismatch is not silently ignored.

## Boundary

This quickstart proves a workflow, not program safety. Observed behavior is not all possible behavior.
