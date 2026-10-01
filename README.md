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

> **Public Alpha:** Linux x86_64 only. Release candidate version: **0.1.0-alpha.5**.
>
> **Self-service:** no signup, API key, meeting, or AETHER X approval is required.

## Start here

Choose the path that matches your environment.

### A. Linux x86_64 — no Rust required (recommended first run)

Download the published release, verify its checksum, and install it in your user path:

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

Then run the controlled **PASS → REVIEW** walkthrough in **[Five-Minute Start](docs/QUICKSTART_5_MIN.md)**.

### B. Rust already installed

```bash
cargo install execsurface --locked
execsurface --version
execsurface doctor
```

The crates.io channel is published only after the immutable GitHub release and stable Action gates succeed. See [crates.io Publishing](docs/CRATES_IO_PUBLISHING.md).

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
execsurface doctor
execsurface init --command "cargo test --locked" --github-actions
execsurface learn -- /bin/bash -lc 'cargo test --locked'
execsurface check --policy execsurface-policy.json -- /bin/bash -lc 'cargo test --locked'
```

Review `execsurface-policy.json`, `.github/workflows/execsurface.yml`, and `execsurface.lock.json` before committing them.

If `doctor` fails, follow the action it prints and see **[Troubleshooting](docs/TROUBLESHOOTING.md)**. `doctor` never elevates privileges, changes ptrace settings, or weakens host security settings.

## What a result means

| Result | Exit code | Meaning |
|---|---:|---|
| PASS | 0 | comparison/evaluation completed with no review/block finding |
| ERROR | 2 | evidence/comparison/policy could not be established |
| REVIEW | 10 | one or more findings require review |
| BLOCK | 20 | one or more findings matched blocking policy |

ExecSurface does **not** infer that drift is malicious. It reports observed drift and evaluates the explicit policy you selected.

## What is observed

The current Linux x86_64 native `ptrace` reference backend can produce evidence for descendant process spawn/exec, pathname access attempts, successful-open file descriptor identity, covered fd-attributed read/write effects, rename/delete operations in the covered syscall set, network connect destinations, trace-time relative/openat/openat2 path semantics, causal executable chains, and explicit observer incompleteness.

Incomplete evidence cannot silently become PASS.

The portable ptrace guard may conservatively mark some clone/thread concurrency incomplete even when exact fd-table sharing is not proven. Raw observation v2 does not retain enough `CLONE_FILES` detail to certify exact sharing, so this guard intentionally trades possible false incompleteness for preventing the known false-completeness class. This is not an exact shared-FD attribution repair.

## Security boundary

ExecSurface detects **observed execution-surface drift under its recorded observer and policy**.

It is **not** antivirus, EDR, malware detection, a sandbox, or a proof that a program is safe.

The governing boundaries are:

- **NO EXECUTION-SURFACE DRIFT ≠ PROGRAM IS SAFE**
- **OBSERVED BEHAVIOR ≠ ALL POSSIBLE BEHAVIOR**
- **NO OBSERVED NETWORK ≠ NETWORK ACCESS IS IMPOSSIBLE**
- **TRACE COMPLETENESS DEPENDS ON THE OBSERVATION BACKEND**

The default evidence boundary excludes file contents, environment values, stdin, network payloads and full child argv values.

The public correctness-reference backend is native `ptrace`. eBPF/BPF-LSM work remains research-only/non-default and is not the public PASS/learn/check backend.

See **[Security Policy](SECURITY.md)** and **[Troubleshooting](docs/TROUBLESHOOTING.md)**.

## Baseline is not policy

The baseline answers what canonical execution surface was accepted. The policy answers what drift should be allowed, reviewed or blocked. A new baseline is not automatically an approval decision.

## Independent evaluation

Use [Self-Service Start](docs/SELF_SERVICE_START.md), [Five-Minute Start](docs/QUICKSTART_5_MIN.md), [Independent Evaluation](docs/INDEPENDENT_EVALUATION.md), and [Technical Evaluation Pack](docs/TECHNICAL_EVALUATION.md). Negative, partial, unsupported-environment, usability and performance-problem results are welcome. A self-evaluation PASS is not evidence of independent adoption.

## Distribution and verification

The public alpha distribution surfaces are:

- checksum-verified GitHub Release binary for Linux x86_64;
- `cargo install execsurface --locked` for Rust users after registry publication;
- GitHub Action `AETHERXGLOBAL/execsurface@v0.1` after stable-channel promotion.

For maximum Action pinning after release, use `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`.

Optional GitHub build provenance verification:

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
