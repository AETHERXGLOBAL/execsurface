# ExecSurface Compatibility & Stability Contract

Status: `PRE-V1 CANDIDATE CONTRACT — EVIDENCE-GATED`
Tracking: P9.3 / issue `#134`
Contract baseline: current public line `v0.1.0-alpha.5`

This document defines the candidate compatibility boundary that must be proved before ExecSurface may publish a stable `v1.0` release.

It does **not** claim that v1.0 is currently qualified or released.

## Compatibility principle

ExecSurface treats compatibility as a semantic contract, not only a parser/API contract.

A future stable release must not silently reinterpret a baseline, policy, verdict, observer-completeness state, or release identity merely to keep an old input syntactically accepted.

When safe compatibility cannot be established, explicit rejection is preferred to silent reinterpretation.

## Candidate stable contract for v1

### 1. Verdict and exit codes

The following meanings are candidate stable v1 behavior:

| Verdict | Exit code | Stable meaning |
|---|---:|---|
| `PASS` | `0` | Evaluation completed and no review/block finding remains under the applicable evidence and policy boundary. |
| `ERROR` | `2` | Required execution/evidence/policy evaluation could not be established as specified. |
| `REVIEW` | `10` | One or more findings require review. |
| `BLOCK` | `20` | One or more findings matched blocking policy. |

Inside the v1 major line these numeric meanings must not be reassigned.

`PASS` remains unavailable when evidence required by the public semantics is incomplete, ambiguous, unsupported, or lost.

### 2. Baseline lock compatibility

Current public baseline facts at contract freeze:

- lock schema: `2`;
- digest format: `2`;
- baseline is separate from policy;
- unsupported schema/version states are rejected rather than silently migrated.

Candidate v1 requirements:

1. A valid baseline-v2 lock produced by the supported Alpha.5/v0.1 public path must remain readable by v1.0 **when the stored semantics remain compatible with the qualified v1 public reference observer**.
2. Unsupported schema, digest, canonical-surface, observer, or semantic states must fail explicitly.
3. Reading an old baseline must not mutate it.
4. No automatic baseline relearning is an upgrade mechanism.
5. Any future migration tool must be explicit, deterministic, preserve the original input, identify source/target versions, and be separately tested.

Compatibility means preserving meaning, not merely accepting JSON.

### 3. Policy and verdict-report compatibility

Current public facts at contract freeze:

- current policy schema: `2`;
- current verdict schema: `2`;
- policy parsing uses explicit schemas and rejects unsupported/invalid input rather than silently weakening policy.

Candidate v1 requirements:

- stable policy-v2 constructs used by the qualified public path retain their documented meanings within v1.x;
- unknown or unsupported fields/versions must not silently become permissive behavior;
- machine-readable verdict/report output remains versioned;
- a breaking schema change requires an explicit version boundary and migration story;
- policy evaluation remains separate from baseline learning/mutation.

### 4. CLI compatibility

The candidate stable v1 CLI surface is intentionally narrow.

#### Stable candidate commands

- `doctor`
- `init`
- `learn`
- `check`
- `--version` / `version`
- help behavior needed for normal user orientation and automation diagnostics

For stable commands, v1.x should preserve documented argument meanings and automation-relevant output/exit semantics unless a change is explicitly backward compatible.

#### Lower-level / non-stable-by-default surfaces

- `observe` is a lower-level evidence surface. Native ptrace behavior may be documented, but existence of the command does not stabilize every backend-specific raw field forever.
- experimental libbpf/eBPF behavior is **not** part of the stable v1 compatibility promise unless a later evidence gate explicitly promotes a bounded proposition/capability.
- `render-error` is an implementation/helper surface and is not automatically a supported public v1 API.
- internal Rust crate APIs are not promised as stable merely because the workspace is public.

### 5. GitHub Action compatibility

A future stable `AETHERXGLOBAL/execsurface@v1` channel must be qualified separately from the current `@v0.1` channel.

Candidate v1 public Action contract includes the documented meanings of these current public inputs where retained for v1:

- `command`
- `baseline`
- `policy`
- `fail-on-review`
- `upload-artifact`
- `artifact-name`

Candidate public outputs where retained for v1:

- `verdict`
- `exit-code`
- `report-json`
- `summary-markdown`
- `sarif-status`
- `artifact-url`
- `artifact-digest`

The v1 Action must preserve fail-closed verdict enforcement and must not silently change an ERROR/BLOCK into a successful workflow outcome. REVIEW behavior remains controlled only by the documented `fail-on-review` contract.

Removal or semantic reassignment of a stable v1 Action input/output is a breaking change unless an explicit compatible replacement is provided.

### 6. Platform and observer boundary

At this freeze, the public support boundary remains:

- Linux x86_64;
- native `ptrace` as the bounded public reference observer.

This document does not promote ARM64, another operating system, eBPF/BPF-LSM, imported traces, or any research backend into v1 support.

Platform/back-end expansion requires its own reproducible compatibility and semantic-authority evidence.

### 7. Upgrade contract

Before v1.0 release, the release candidate must prove at minimum:

- current Alpha.5/v0.1 baseline-v2 compatibility for the explicitly preserved path;
- current stable verdict/exit-code behavior;
- current supported policy behavior;
- current public Action semantics or an explicitly documented compatible transition to `@v1`;
- explicit rejection of unsupported old inputs;
- no irreversible mutation of user baselines/policies during ordinary upgrade validation.

### 8. Rollback contract

A failed/bad v1 release must be recoverable without rewriting user baseline/policy history.

Required properties:

- immutable release tags remain immutable;
- stable moving channel changes are auditable;
- rollback identifies the exact previously qualified release source;
- rollback does not silently rewrite baseline or policy files;
- release documentation records the reason and affected compatibility boundary.

## Breaking-change classification inside v1

The following are breaking unless separately demonstrated to preserve the stable semantics:

- reassigning verdict exit codes;
- silently accepting a previously unsupported schema with different meaning;
- changing baseline digest/canonical meaning so old valid baselines are reinterpreted;
- relaxing incomplete-evidence behavior into PASS eligibility;
- changing a stable policy matcher/action meaning;
- removing or semantically reassigning a stable CLI command/argument;
- removing or semantically reassigning a stable GitHub Action input/output;
- broadening backend authority based only on backend identity or implementation availability.

Bug fixes are not automatically breaking merely because alpha behavior differed. If a bug affected a documented stable v1 semantic promise, the correction requires an explicit compatibility/security decision.

## Required executable proof before closeout

P9.3 remains OPEN until executable evidence covers this contract. Required proof set:

1. **Golden Alpha.5 baseline-v2 fixture** consumed successfully by candidate v1 code under the preserved reference semantics.
2. **Unsupported-schema rejection fixtures** proving no silent migration/reinterpretation.
3. **Verdict/exit regression** for PASS `0`, ERROR `2`, REVIEW `10`, BLOCK `20`.
4. **Policy/report version regression** including unknown/unsupported input failure behavior.
5. **GitHub Action contract test** for stable candidate inputs/outputs and verdict enforcement.
6. **Upgrade rehearsal** from the frozen current public line to a frozen v1 release candidate.
7. **Rollback rehearsal** back to the previously qualified release path without mutating user evidence.
8. **Experimental-surface anti-drift review** proving experimental libbpf/research capabilities were not accidentally promoted.

Historical M12.2 evidence is relevant starting evidence, but it does not by itself qualify an unreleased v1 candidate.

## Change control

Changes to this candidate contract before v1.0 must be recorded with:

- the exact changed promise;
- rationale;
- compatibility impact;
- required new evidence;
- anti-drift review.

After v1.0, changes to a stable promise follow normal SemVer-breaking-change discipline and the evidence-gated release process.
