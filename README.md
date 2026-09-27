<p align="center"><strong>AETHER X GLOBAL</strong></p>

# ExecSurface

<p align="center"><strong>Code diff shows what changed. ExecSurface shows what started happening.</strong></p>

<p align="center">
  <a href="https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/ci.yml/badge.svg"></a>
  <a href="https://github.com/AETHERXGLOBAL/execsurface/releases"><img alt="Release" src="https://img.shields.io/github/v/release/AETHERXGLOBAL/execsurface?include_prereleases&label=release"></a>
  <img alt="License Apache-2.0" src="https://img.shields.io/badge/license-Apache--2.0-blue">
  <img alt="Linux x86_64" src="https://img.shields.io/badge/platform-Linux%20x86__64-informational">
  <img alt="Public Alpha" src="https://img.shields.io/badge/status-Public%20Alpha-yellow">
</p>

ExecSurface learns an accepted **runtime execution surface**, runs the same command later, and reports execution behavior that appeared, disappeared, or changed.

It is intended for CI pipelines, dependencies, developer tools and AI-assisted workflows where source review alone does not show every runtime effect.

> **Public Alpha:** Linux x86_64 only. Current product version: **0.1.0-alpha.3**.
>
> **Self-service:** no signup, API key, meeting, or AETHER X approval is required.

## Start here

Choose the path that matches your environment.

### A. Linux x86_64 — no Rust required (recommended first run)

Download the published release, verify its checksum, and install it in your user path:

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

Then run the controlled **PASS → REVIEW** walkthrough in **[Five-Minute Start](docs/QUICKSTART_5_MIN.md)**.

### B. Rust already installed

```bash
cargo install execsurface --locked
execsurface --version
execsurface doctor
```

The current verified registry release is `0.1.0-alpha.3`. See [crates.io Publishing](docs/CRATES_IO_PUBLISHING.md).

### C. Add it to a GitHub Actions project

First generate conservative starter files from your project directory:

```bash
execsurface init --command "cargo test --locked" --github-actions
```

`init` creates a starter policy and workflow. It **does not run your target command** and does not create a baseline automatically.

The generated workflow uses the stable public-alpha Action channel:

```text
AETHERXGLOBAL/execsurface@v0.1
```

Do not use `@main` as the normal consumer path. See the **[GitHub Action guide](docs/GITHUB_ACTION.md)**.

## First real project

Replace the example command with the command you actually want to monitor.

```bash
# 1. Diagnose this host first
execsurface doctor

# 2. Generate a policy and optional GitHub Actions workflow
execsurface init --command "cargo test --locked" --github-actions

# 3. Learn the accepted baseline using the same wrapper as the Action
execsurface learn -- /bin/bash -lc 'cargo test --locked'

# 4. Run the same command later and compare it
execsurface check \
  --policy execsurface-policy.json \
  -- /bin/bash -lc 'cargo test --locked'
```

Review `execsurface-policy.json`, `.github/workflows/execsurface.yml`, and `execsurface.lock.json` before committing them.

If `doctor` fails, follow the action it prints and see **[Troubleshooting](docs/TROUBLESHOOTING.md)**. `doctor` never elevates privileges, changes ptrace settings, or weakens host security settings.

## What a result means

Example of drift under a strict policy:

```text
Tests: PASS

ExecSurface: BLOCK

+ EXEC     /usr/bin/curl
+ NETWORK  203.0.113.12:443
+ READ     $HOME/.ssh/config
```

ExecSurface does **not** infer that the behavior is malicious. It reports observed drift and evaluates the explicit policy you selected.

| Result | Exit code | Meaning |
|---|---:|---|
| PASS | 0 | comparison/evaluation completed with no review/block finding |
| ERROR | 2 | evidence/comparison/policy could not be established |
| REVIEW | 10 | one or more findings require review |
| BLOCK | 20 | one or more findings matched blocking policy |

## What is observed

The current Linux x86_64 native `ptrace` reference backend can produce evidence for:

- descendant process spawn/exec;
- pathname access attempts;
- successful-open file descriptor identity;
- actual fd-attributed read/write effects for the covered syscalls;
- rename/delete operations in the covered syscall set;
- network connect destinations;
- trace-time relative/openat/openat2 path semantics;
- causal executable chains;
- explicit observer incompleteness.

Incomplete evidence cannot silently become PASS.

## Security boundary

ExecSurface detects **observed execution-surface drift under its recorded observer and policy**.

It is **not**:

- antivirus;
- EDR;
- malware detection;
- a sandbox;
- a proof that a program is safe.

The governing boundaries are:

- **NO EXECUTION-SURFACE DRIFT ≠ PROGRAM IS SAFE**
- **OBSERVED BEHAVIOR ≠ ALL POSSIBLE BEHAVIOR**
- **NO OBSERVED NETWORK ≠ NETWORK ACCESS IS IMPOSSIBLE**
- **TRACE COMPLETENESS DEPENDS ON THE OBSERVATION BACKEND**

The default evidence boundary excludes file contents, environment values, stdin, network payloads and full child argv values.

The public correctness-reference backend is native `ptrace`. eBPF work remains research-only and is not the public PASS/learn/check backend.

See **[Security Policy](SECURITY.md)** and **[Troubleshooting](docs/TROUBLESHOOTING.md)**.

## Baseline is not policy

ExecSurface keeps these separate deliberately.

The baseline answers:

> What canonical execution surface was accepted?

The policy answers:

> What drift should be allowed, reviewed or blocked?

A new baseline is not automatically an approval decision.

## Independent evaluation

You can evaluate the public alpha without contacting AETHER X.

Use:

- **[Self-Service Start](docs/SELF_SERVICE_START.md)** — installation and project setup;
- **[Five-Minute Start](docs/QUICKSTART_5_MIN.md)** — controlled PASS → REVIEW demonstration;
- **[Independent Evaluation](docs/INDEPENDENT_EVALUATION.md)** — third-party evaluation protocol;
- **[Technical Evaluation Pack](docs/TECHNICAL_EVALUATION.md)** — machine-readable evidence workflow.

Negative, partial, unsupported-environment, usability and performance-problem results are welcome. A self-evaluation PASS is not evidence of independent adoption.

## Distribution and verification

The public alpha is available through:

- checksum-verified GitHub Release binary for Linux x86_64;
- `cargo install execsurface --locked` for Rust users;
- GitHub Action `AETHERXGLOBAL/execsurface@v0.1`.

The release process gates the immutable binary and Action consumer paths before promoting the stable `v0.1` channel. For maximum Action pinning, use `AETHERXGLOBAL/execsurface@v0.1.0-alpha.3`.

Optional GitHub build provenance verification for the downloaded release archive:

```bash
gh attestation verify "$ASSET" -R AETHERXGLOBAL/execsurface
```

A valid attestation links the artifact to its build source/workflow. It does **not** prove the binary is safe.

## Documentation

- [Self-Service Start](docs/SELF_SERVICE_START.md)
- [Five-Minute Start](docs/QUICKSTART_5_MIN.md)
- [GitHub Action](docs/GITHUB_ACTION.md)
- [Troubleshooting](docs/TROUBLESHOOTING.md)
- [Independent Evaluation](docs/INDEPENDENT_EVALUATION.md)
- [Technical Evaluation Pack](docs/TECHNICAL_EVALUATION.md)
- [Command examples](docs/EXAMPLES.md)
- [crates.io Publishing](docs/CRATES_IO_PUBLISHING.md)
- [Roadmap](ROADMAP.md)
- [Contributing](CONTRIBUTING.md)
- [Support](SUPPORT.md)

## Developing ExecSurface

Source-build commands are for contributors, not the normal installation path.

```bash
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace --all-targets

cargo run -p execsurface -- --version
cargo run -p execsurface -- doctor
```

Architecture-affecting changes remain evidence-gated. See [CONTRIBUTING.md](CONTRIBUTING.md) and [GOVERNANCE.md](GOVERNANCE.md).

## License

Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
