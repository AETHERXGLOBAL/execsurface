# AETHER X ExecSurface

**Code diff shows what changed. ExecSurface shows what started happening.**

Runtime execution-surface drift detection for software, CI pipelines, dependencies and AI tooling.

> **Status:** M0–M6 are accepted. ExecSurface can observe, canonicalize, learn, diff, evaluate independent policy into PASS / REVIEW / BLOCK, and run as a GitHub composite Action with JSON/Markdown evidence artifacts.

ExecSurface is a Linux-first developer tool for learning a content-addressed baseline of externally observable runtime effects, comparing a later run against it, and producing deterministic drift findings plus a policy-aware CI verdict.

## Target workflow

```text
OBSERVE
  ↓
NORMALIZE
  ↓
CANONICALIZE
  ↓
LOCK
  ↓
RUN AGAIN
  ↓
DIFF
  ↓
CLASSIFY EXPANSION
  ↓
VERDICT
  ↓
REPORT / EVIDENCE

SARIF findings are intentionally deferred until ExecSurface can prove causal source-code locations; M6 does not invent file/line annotations from runtime-only evidence.
```

Target CLI:

```bash
execsurface learn -- python -m pytest
execsurface check -- python -m pytest
```

Illustrative future report:

```text
Tests: PASS

ExecSurface: BLOCK

+ EXEC    /usr/bin/curl
+ NETWORK 203.0.113.12:443
+ READ    ~/.ssh/config
```

## Core distinction

A tracer emits observations. ExecSurface starts there and adds a stable, reviewable model:

```text
raw observation
→ canonical execution surface
→ content-addressed baseline
+ independent policy
→ deterministic drift
→ verdict + evidence
```

## Explicit security boundary

ExecSurface is **not** an antivirus, EDR, malware detector, sandbox, or proof of program safety.

- **NO EXECUTION-SURFACE DRIFT ≠ PROGRAM IS SAFE**
- **OBSERVED BEHAVIOR ≠ ALL POSSIBLE BEHAVIOR**
- **NO OBSERVED NETWORK ≠ PROOF THAT NETWORK ACCESS IS IMPOSSIBLE**
- **TRACE COMPLETENESS DEPENDS ON THE OBSERVATION BACKEND**

## M0 decision

M1 uses a **minimal native Linux ptrace observer** after the original strace-ingestion plan failed the privacy gate: decoded strace output can collect string syscall arguments before redaction. eBPF remains deferred until evidence shows a material benefit.

Current CLI:

```bash
cargo run -p execsurface-cli -- observe -- /bin/true
```

`learn` is implemented in M3, policy-free diffing in M4, policy-aware `check` in M5, and GitHub Action packaging in M6. Use `--diff-only` to retain raw M4 behavior.

## GitHub Action

M6 provides a Linux x86_64 composite Action with read-only default permissions, a GitHub job summary, structured JSON evidence and an uploaded evidence artifact.

```yaml
permissions:
  contents: read

steps:
  - uses: actions/checkout@v6

  - name: ExecSurface
    uses: AETHERXGLOBAL/execsurface@main
    with:
      command: "cargo test --locked"
      baseline: execsurface.lock.json
      policy: execsurface-policy.json
```

The Action executes the command as `/bin/bash -lc <command>`. The committed baseline must therefore be learned with the same wrapper, for example:

```bash
execsurface learn -- /bin/bash -lc 'cargo test --locked'
```

For production use, pin the Action to a reviewed release tag or commit SHA rather than a moving branch.

See:
- [M0 Architecture](docs/architecture/M0_ARCHITECTURE.md)
- [ADR-0001: M1 Observation Backend](docs/architecture/ADR-0001-observation-backend.md)
- [Adjacent Systems / Prior-Art Map](docs/architecture/COMPETITOR_MAP.md)
- [M0 Gate](docs/architecture/M0_GATE.md)

## Design invariants

1. Raw observations and canonical execution surfaces are different models.
2. Baseline and policy are different concepts.
3. Observer failure can never become PASS.
4. Normalization may remove irrelevant variability but may not silently collapse distinct security-relevant behavior.
5. Privacy defaults to metadata, not secrets.
6. Evidence and explicit limitations are product outputs.
7. Precision > feature count.

Apache-2.0 licensed.
