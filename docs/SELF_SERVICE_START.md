# ExecSurface — Self-Service Start

No signup, meeting, API key, or AETHER X approval is required.

Public Alpha support: **Linux x86_64**.

## Install — recommended path (no Rust required)

Use the published Linux x86_64 release binary:

```bash
VERSION=v0.1.0-alpha.3
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

This path does not require a Rust toolchain. Do not run the binary if checksum verification fails.

## Alternative install — crates.io

If Rust/Cargo is already installed:

```bash
cargo install execsurface --locked
execsurface --version
execsurface doctor
```

## Start in your project

```bash
execsurface init --command "cargo test --locked" --github-actions
execsurface learn -- /bin/bash -lc 'cargo test --locked'
execsurface check --policy execsurface-policy.json -- /bin/bash -lc 'cargo test --locked'
```

Replace `cargo test --locked` with your own repeatable project command.

`init` creates the starter policy and optional GitHub Actions workflow, but it does not execute your command or silently create a baseline. Review the generated policy and learned baseline before committing them.

## Fast controlled proof

If you want to verify the product workflow before using a real project, follow [`QUICKSTART_5_MIN.md`](QUICKSTART_5_MIN.md). It demonstrates an unchanged PASS followed by controlled REVIEW drift.

## Want to evaluate it independently?

Follow [`INDEPENDENT_EVALUATION.md`](INDEPENDENT_EVALUATION.md). It includes two unchanged checks, a controlled-drift check, and a public GitHub issue form for submitting the result.

Independent evaluators are explicitly invited to report negative, partial, unsupported, or performance-problem results as well as successful ones.

## If something fails

Run:

```bash
execsurface doctor
```

Then use [`TROUBLESHOOTING.md`](TROUBLESHOOTING.md). ExecSurface does not automatically elevate privileges, change ptrace sysctls, or weaken host security settings.

## Product boundary

Public Alpha `v0.1.0-alpha.3` supports Linux x86_64 with native `ptrace` as the correctness-reference backend.

ExecSurface reports observed runtime execution-surface drift. It does not prove that software is safe and is not an antivirus, EDR, malware detector, or sandbox.
