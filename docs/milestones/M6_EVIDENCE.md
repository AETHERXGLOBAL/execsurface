# M6 — GitHub Action Evidence

Date: 2026-09-25

Gate verdict: **ACCEPTED**

Branch: `milestone/m6-github-action`

Issue: #12

## Result

M6 packages ExecSurface M1–M5 as a Linux x86_64 GitHub composite Action.

Implemented:

- root `action.yml`;
- source-build locked Rust binary;
- environment-transported command input;
- JSON verdict evidence;
- deterministic Markdown job summary;
- evidence artifact upload;
- artifact URL/digest outputs;
- PASS / REVIEW / BLOCK / ERROR enforcement;
- configurable `fail-on-review`;
- explicit SARIF non-generation disposition.

## Rust / model evidence

Final pre-close Rust CI:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36180674644

Result:

- format — PASS;
- strict Clippy — PASS;
- locked tests — PASS;
- Cargo.lock integrity — PASS;
- **60 Rust/integration tests PASS, 0 failed**.

M6 report-specific unit tests include:

- deterministic ERROR summary;
- adversarial Markdown table/HTML escaping;
- explicit SARIF disposition;
- empty PASS summary shape.

## Action smoke evidence

Hardened Action smoke:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36180674711

All six jobs: **PASS**

### PASS

- real baseline learned with the Bash wrapper;
- Action rerun returns PASS;
- artifact upload succeeds;
- artifact digest is non-empty;
- SARIF disposition is explicit.

### REVIEW

- controlled runtime drift under built-in policy;
- verdict REVIEW;
- stable exit code 10;
- Action succeeds by default.

### REVIEW enforcement

- same REVIEW semantics;
- `fail-on-review=true`;
- Action step is correctly marked failure;
- output verdict remains REVIEW and exit code 10.

### BLOCK

- explicit blocking policy;
- controlled added process execution;
- verdict BLOCK;
- stable exit code 20;
- Action step fails after evidence generation.

### ERROR

- missing baseline;
- verdict ERROR;
- stable exit code 2;
- Action step fails.

### Privacy

Sentinel:

`AX_M6_ARGV_SECRET_41e9d3`

The sentinel is present in the target Bash command argv but absent from:

- generated verdict JSON;
- generated Markdown summary.

Result: **COMPUTATIONAL_EVIDENCE**

This does not claim GitHub itself will conceal arbitrary workflow input values in logs. Workflow authors should continue to use GitHub Secrets for secret workflow inputs.

## Command-injection review

The composite Action maps `inputs.command` into an environment variable.

`action/run.sh` passes that variable as a quoted argv value to:

`/bin/bash -lc`

The expression is not inserted into generated shell program text by the Action metadata.

Status: **IMPLEMENTATION EVIDENCE**

The command is intentionally shell syntax because the public input contract is a shell command.

## Artifact evidence

`actions/upload-artifact@v7` runs successfully in the smoke workflow.

The PASS smoke asserts the exported artifact digest is non-empty.

Artifact contents:

- `report.json`;
- `summary.md`.

## SARIF negative result

The original M6 roadmap called for semantically appropriate SARIF.

Red-Team review found no defensible mapping from current runtime effects to causal repository source file/line locations.

Decision:

**SARIF FINDING EMISSION — KILLED / DEFERRED**

M6 emits:

`not-generated:no-source-provenance`

rather than fabricating a location.

This is a deliberate correctness decision, not an unimplemented formatting task.

## Preserved negative evidence

Initial Action smoke:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36178826933

Result: **FAIL**

Three jobs already behaved correctly, but the intended PASS fixture returned REVIEW because the first smoke setup did not establish a fully comparable controlled baseline/execution setup.

The fixture setup was corrected rather than weakening diff/policy semantics.

Successful corrected smoke:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36179060765

Result: **PASS** for PASS / REVIEW / BLOCK / ERROR.

The later hardened smoke added REVIEW-enforcement and privacy evidence:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36180674711

Result: **PASS** for all six jobs.

No failed run was hidden or rewritten.

## M6 non-claims

M6 does not claim:

- runtime drift is a vulnerability;
- current effects identify causal source lines;
- SARIF annotations are safe to synthesize;
- cross-platform support;
- low observer overhead;
- sandbox/EDR/malware detection.

## Gate decision

**M6 ACCEPTED.**

M7 — External Real-World Proof is the next roadmap gate, but is not part of M6.
