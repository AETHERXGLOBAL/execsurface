# ExecSurface — Self-Service Start

No signup, meeting, API key, or AETHER X approval is required.

Stable `v1.0.0` support: **Linux x86_64**.

## Install — recommended path (no Rust required)

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
execsurface --version
execsurface doctor
```

Do not run the binary if checksum verification fails.

## Alternative install — crates.io

```bash
cargo install execsurface --version "=1.0.0" --locked
execsurface --version
execsurface doctor
```

The current public release is stable `v1.0.0`. Exact-version installation remains recommended for reproducible evaluation.

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

Stable v1.0.0 supports the documented Linux x86_64 boundary with native `ptrace` as the correctness-reference backend. ExecSurface reports observed runtime execution-surface drift; it does not prove software is safe and is not antivirus, EDR, malware detection, or a sandbox.
