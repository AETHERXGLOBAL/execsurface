<p align="center"><strong>AETHER X GLOBAL</strong></p>

# ExecSurface

<p align="center"><strong>Code diff shows what changed. ExecSurface shows what started happening.</strong></p>

<p align="center">
  <a href="https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/AETHERXGLOBAL/execsurface/actions/workflows/ci.yml/badge.svg"></a>
  <a href="https://github.com/AETHERXGLOBAL/execsurface/releases"><img alt="Release" src="https://img.shields.io/github/v/release/AETHERXGLOBAL/execsurface?include_prereleases&label=release"></a>
  <img alt="License Apache-2.0" src="https://img.shields.io/badge/license-Apache--2.0-blue">
  <img alt="Rust 1.82+" src="https://img.shields.io/badge/Rust-1.82%2B-orange">
  <img alt="Linux x86_64" src="https://img.shields.io/badge/platform-Linux%20x86__64-informational">
  <img alt="Public Alpha" src="https://img.shields.io/badge/status-Public%20Alpha-yellow">
</p>

ExecSurface is a Linux-first developer tool that learns an accepted **runtime execution surface**, runs the same command later, and reports what execution behavior appeared, disappeared, or changed.

It is designed for software, CI pipelines, dependencies, developer tools and AI tooling where code review alone does not show every runtime effect.

> **Public Alpha:** Linux x86_64 only. Current product version: **0.1.0-alpha.2**.

### The idea in 10 seconds

Illustrative strict-policy output:

```text
Tests: PASS

ExecSurface: BLOCK

+ EXEC     /usr/bin/curl
+ NETWORK  203.0.113.12:443
+ READ     $HOME/.ssh/config
```

ExecSurface does **not** infer that this is malicious. It reports observed drift and applies the policy you chose.

## Quickstart

### 1. Install from crates.io

```bash
cargo install execsurface --locked
execsurface --version
```

The current public alpha is also available as a signed GitHub Release binary for Linux x86_64.

### 2. Check environment readiness

```bash
execsurface doctor
```

Expected on a supported environment:

```text
ExecSurface Doctor

[PASS] Linux
[PASS] x86_64
[PASS] ptrace observer available
[PASS] workspace writable
[PASS] ExecSurface 0.1.0-alpha.2

Ready.
```

`doctor` is diagnostic only. It does not elevate privileges, change sysctls, or weaken host security settings.

### 3. Create conservative starter files

Replace the command with your real project command:

```bash
execsurface init --command "cargo test --locked" --github-actions
```

This creates a starter policy and GitHub Actions workflow. It **does not run your command** and does not create a baseline automatically.

### 4. Learn an explicit baseline

The GitHub Action executes its input as `/bin/bash -lc <command>`. Learn locally with the same wrapper:

```bash
execsurface learn -- \
  /bin/bash -lc 'cargo test --locked'
```

Review `execsurface.lock.json` before committing it.

### 5. Check the same command

```bash
execsurface check \
  --policy execsurface-policy.json \
  -- /bin/bash -lc 'cargo test --locked'
```

For independent evaluation with machine-readable PASS/REVIEW evidence, use the **[Technical Evaluation Pack](docs/TECHNICAL_EVALUATION.md)**. For the shortest controlled drift demonstration, see **[Five-Minute Start](docs/QUICKSTART_5_MIN.md)**.

`SELF-EVALUATION PASS ≠ INDEPENDENT ADOPTION`

## Alternative installation paths

### GitHub Release binary

```bash
VERSION=v0.1.0-alpha.2
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

Optional provenance verification with GitHub CLI:

```bash
gh attestation verify "$ASSET" -R AETHERXGLOBAL/execsurface
```

A valid attestation links the artifact to its GitHub build provenance. It does **not** prove the binary is safe.

### Immutable Git tag fallback

```bash
cargo install \
  --git https://github.com/AETHERXGLOBAL/execsurface.git \
  --tag v0.1.0-alpha.2 \
  execsurface \
  --locked
```

See [crates.io Publishing](docs/CRATES_IO_PUBLISHING.md).

## GitHub Actions

Use the stable **v0.1** channel, not `@main`:

```yaml
permissions:
  contents: read

steps:
  - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1

  - name: ExecSurface
    uses: AETHERXGLOBAL/execsurface@v0.1
    with:
      command: cargo test --locked
      baseline: execsurface.lock.json
      policy: execsurface-policy.json
```

The public Action downloads the pinned ExecSurface release binary and verifies its SHA-256 checksum. Local `uses: ./` development continues to build from source.

See **[GitHub Action guide](docs/GITHUB_ACTION.md)**.

## Why runtime drift?

A source diff can tell you that a dependency version changed. It cannot by itself tell you whether the new runtime now:

- spawns another executable;
- reads another file;
- writes to a new location;
- connects to another destination;
- changes the process chain that produced an effect.

ExecSurface records selected metadata-only runtime effects, canonicalizes unstable machine details, locks an accepted baseline, compares a later run, and evaluates an independent policy.

```text
OBSERVE
  ↓
CANONICALIZE
  ↓
LOCK
  ↓
RUN AGAIN
  ↓
DIFF
  ↓
POLICY
  ↓
PASS / REVIEW / BLOCK / ERROR
  ↓
JSON + Markdown evidence
```

## What is observed

The current Linux x86_64 ptrace reference backend can produce evidence for:

- descendant process spawn/exec;
- pathname access attempts;
- successful-open file descriptor identity;
- actual fd-attributed read/write effects for the covered syscalls;
- rename/delete operations in the covered syscall set;
- network connect destinations;
- trace-time relative/openat/openat2 path semantics;
- causal executable chains;
- explicit observer incompleteness.

M6.5 added fail-closed event-budget truncation and fault-injection coverage. Incomplete evidence cannot silently become PASS.

## Security boundary

ExecSurface is **not**:

- an antivirus;
- an EDR;
- a malware detector;
- a sandbox;
- a proof of program safety.

The governing statements remain:

- **NO EXECUTION-SURFACE DRIFT ≠ PROGRAM IS SAFE**
- **OBSERVED BEHAVIOR ≠ ALL POSSIBLE BEHAVIOR**
- **NO OBSERVED NETWORK ≠ NETWORK ACCESS IS IMPOSSIBLE**
- **TRACE COMPLETENESS DEPENDS ON THE OBSERVATION BACKEND**

The default evidence boundary excludes file contents, environment values, stdin, network payloads and full child argv values.

See **[Security Policy](SECURITY.md)** and **[Troubleshooting](docs/TROUBLESHOOTING.md)**.

## Baseline is not policy

ExecSurface deliberately keeps these separate.

The baseline answers:

> What canonical execution surface was accepted?

The policy answers:

> What drift should be allowed, reviewed or blocked?

A new baseline is not automatically an approval decision.

## Exit codes

| Result | Exit code | Meaning |
|---|---:|---|
| PASS | 0 | comparison/evaluation completed with no review/block finding |
| ERROR | 2 | evidence/comparison/policy could not be established |
| REVIEW | 10 | one or more findings require review |
| BLOCK | 20 | one or more findings matched blocking policy |

## Looking for early adopters

ExecSurface public alpha is most relevant to teams experimenting with:

- dependency-update CI;
- build and test pipelines;
- developer tools;
- AI tooling that launches subprocesses;
- security-sensitive automation.

We are looking for compatibility evidence and workflow feedback, not testimonials.

ExecSurface detects **observed execution-surface drift under its recorded observer and policy**. It does not prove that a program is safe.

Use the **[Adoption / Integration issue template](https://github.com/AETHERXGLOBAL/execsurface/issues/new/choose)** and do not post secrets.

You can also read the public adopter call in [Issue #28](https://github.com/AETHERXGLOBAL/execsurface/issues/28).

## Documentation

- [Five-Minute Start](docs/QUICKSTART_5_MIN.md)
- [Technical Evaluation Pack](docs/TECHNICAL_EVALUATION.md)
- [Troubleshooting](docs/TROUBLESHOOTING.md)
- [Command examples](docs/EXAMPLES.md)
- [GitHub Action](docs/GITHUB_ACTION.md)
- [crates.io Publishing](docs/CRATES_IO_PUBLISHING.md)
- [M6.6 distribution architecture](docs/milestones/M6_6_DISTRIBUTION_ARCHITECTURE.md)
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
