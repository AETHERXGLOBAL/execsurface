# ExecSurface — Self-Service Start

No signup, meeting, API key, or AETHER X approval is required.

## Install

```bash
cargo install execsurface --locked
execsurface doctor
```

## Start in your project

```bash
execsurface init --command "cargo test --locked" --github-actions
execsurface learn -- /bin/bash -lc 'cargo test --locked'
execsurface check --policy execsurface-policy.json -- /bin/bash -lc 'cargo test --locked'
```

Replace `cargo test --locked` with your own repeatable project command.

Review the generated policy and baseline before committing them.

## Want to evaluate it independently?

Follow [`INDEPENDENT_EVALUATION.md`](INDEPENDENT_EVALUATION.md). It includes two unchanged checks, a controlled-drift check, and a public GitHub issue form for submitting the result.

Independent evaluators are explicitly invited to report negative, partial, unsupported, or performance-problem results as well as successful ones.

## Product boundary

Public Alpha `v0.1.0-alpha.3` supports Linux x86_64 with native `ptrace` as the correctness-reference backend.

ExecSurface reports observed runtime execution-surface drift. It does not prove that software is safe and is not an antivirus, EDR, malware detector, or sandbox.
