# ExecSurface — Self-Service Start

No signup, meeting, API key, or AETHER X approval is required.

Public Alpha support: **Linux x86_64**.

## Install — recommended path (no Rust required)

```bash
VERSION=v0.1.0-alpha.5
TARGET=x86_64-unknown-linux-gnu
ASSET="execsurface-${VERSION}-${TARGET}.tar.gz"

curl -fLO "https://github.com/AETHERXGLOBAL/execsurface/releases/download/${VERSION}/${ASSET}"
curl -fLO "https://github.com/AETHERXGLOBAL/execsurface/releases/download/${VERSION}/${ASSET}.sha256"
sha256sum -c "${ASSET}.sha256"
tar -xzf "${ASSET}"

mkdir -p "$HOME/.local/bin"
install -m 0755 "execsurface-${VERSION}-${TARGET}/execsurface" "$HOME/.local/bin/execsurface"
export PATH="$HOME/.local/bin:$PATH"
execsurface --version
execsurface doctor
```

Do not run the binary if checksum verification fails.

## Alternative install — crates.io

```bash
cargo install execsurface --locked
execsurface --version
execsurface doctor
```

The registry publication follows the immutable GitHub release and stable Action validation.

## Start in your project

```bash
execsurface init --command "cargo test --locked" --github-actions
execsurface learn -- /bin/bash -lc 'cargo test --locked'
execsurface check --policy execsurface-policy.json -- /bin/bash -lc 'cargo test --locked'
```

`init` creates starter files but does not run your command or silently create a baseline. Review the generated policy and learned baseline before committing them.

## Fast controlled proof

Follow [`QUICKSTART_5_MIN.md`](QUICKSTART_5_MIN.md) for PASS followed by controlled REVIEW drift.

## Independent evaluation

Follow [`INDEPENDENT_EVALUATION.md`](INDEPENDENT_EVALUATION.md). Negative, partial, unsupported, usability, and performance-problem results are welcome.

## If something fails

Run `execsurface doctor`, then use [`TROUBLESHOOTING.md`](TROUBLESHOOTING.md). ExecSurface does not automatically elevate privileges, change ptrace sysctls, or weaken host security settings.

## Product boundary

Public Alpha `v0.1.0-alpha.5` supports Linux x86_64 with native `ptrace` as the correctness-reference backend. ExecSurface reports observed runtime execution-surface drift; it does not prove software is safe and is not antivirus, EDR, malware detection, or a sandbox.
