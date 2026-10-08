# ExecSurface v1-R0 — Stable Contract Freeze Protocol

Date: 2026-10-08  
Parent: P9 / issue #129  
Predecessor: `docs/development/V1_PRODUCTION_READINESS_GAP_ANALYSIS.md`  
Starting source: `main@f4c26a204ad139c1920203db96f20fb4fe74c0b9`

## Classification

This gate freezes the stable product contract for runtime behavioral verification, execution semantics and semantic-evidence correctness.

It is **not cybersecurity research**.

No feature is authorized merely to make v1 appear larger.

## Fixed roles

1. **Stable Contract / SemVer Lead** — owns the exact public promises.
2. **Innovation Lead** — seeks the smallest useful stable surface and removes unnecessary promises.
3. **Anti-Drift Reviewer** — blocks accidental stabilization of research/experimental surfaces.
4. **Independent Falsifier / Destruction Team** — attacks contract contradictions, silent migration, support inflation and false compatibility.
5. **Critical Milestone Reviewer** — independently verifies the frozen wording against executable behavior and retained evidence.

## Gate objective

Freeze what a future v1.x line promises **before** v1 release-control implementation or RC construction.

Allowed outcome:

`V1_R0_STABLE_CONTRACT_FROZEN_BOUNDED`

or:

`V1_R0_REWORK_REQUIRED`.

A PASS does not authorize a v1 RC or release.

## Proposed minimal stable contract to falsify

### C1 — Verdict semantics

Stable verdict codes:

- PASS = 0
- ERROR = 2
- REVIEW = 10
- BLOCK = 20

Incomplete, ambiguous, unsupported or lost required evidence cannot silently become PASS.

### C2 — Target outcome remains separate from ExecSurface verdict

The v1 proposal preserves current qualified semantics:

- target exit code/signal remains report metadata;
- target outcome alone does not convert PASS/REVIEW/BLOCK;
- `ExecSurface: PASS` means no remaining policy-relevant execution-surface finding under the qualified evidence boundary;
- it does **not** mean the wrapped command succeeded.

CI/onboarding must preserve a separate native target-command correctness gate.

No `fail-on-target-failure` feature is required for v1-R0.

### C3 — Baseline contract

Stable v1 candidate baseline contract:

- lock schema 2;
- digest format 2;
- normalization profile 4 for the current corrected public semantics;
- Alpha.6 profile-4 baselines are the current upgrade source;
- Alpha.5 profile-3 baselines are historical/rollback evidence and must be explicitly rejected as semantically incomparable by profile-4 v1 code rather than silently migrated;
- ordinary read/check never mutates the baseline;
- no automatic relearn/migration.

### C4 — Policy contract

Stable v1 candidate policy input contract:

- schema 1 remains accepted legacy-read compatibility and is deprecated for new authoring;
- schema 2 remains the default authored/public policy schema;
- schema 3 is a stable **opt-in** extension for the already-qualified exact matchers:
  - `path_resolution`
  - `open_intent`
  - `rename_from_class`
  - `rename_from_prefix`
  - `rename_from_resolution`
- schema 3 does not change v1/v2 meanings;
- unknown fields or unsupported versions fail explicitly;
- policy remains separate from baseline learning.

The destruction team must reject any implementation/documentation that silently makes schema 3 the default or changes v1/v2 meaning.

### C5 — Verdict/report contract

Stable machine-readable verdict schema remains version 2 unless an explicit later version boundary is separately qualified.

Target outcome, findings, policy summary and errors retain their documented meanings.

### C6 — Stable CLI surface

Candidate stable commands:

- `doctor`
- `init`
- `learn`
- `check`
- `--version` / `version`
- `--help` / `help`

Lower-level or non-stable-by-default:

- `observe`
- experimental libbpf/eBPF behavior
- `render-error`
- internal Rust crate APIs

Public documentation of a lower-level command does not automatically make every raw field a SemVer-stable v1 promise.

### C7 — Stable GitHub Action surface

The future `AETHERXGLOBAL/execsurface@v1` candidate contract includes these inputs:

- `command`
- `baseline`
- `policy`
- `expected-baseline-digest`
- `expected-policy-sha256`
- `require-custody`
- `fail-on-review`
- `upload-artifact`
- `artifact-name`

And outputs:

- `verdict`
- `exit-code`
- `report-json`
- `summary-markdown`
- `sarif-status`
- `artifact-url`
- `artifact-digest`

BLOCK and ERROR remain fail-closed. REVIEW succeeds by default and fails only when `fail-on-review=true`.

### C8 — Workload-support boundary

v1 does not promise that every syntactically unchanged command yields zero drift.

The stable contract distinguishes:

- deterministic/stable execution surfaces where unchanged PASS is a qualified normal path; and
- highly nondeterministic build/test graphs where legitimate runtime variation may produce REVIEW under exact baseline semantics.

No broad variance suppression, path whitelist or multi-run auto-authorization is introduced by v1-R0.

Current recommendation to falsify:

**keep exact semantics and narrow the stable promise rather than add a general variance feature.**

### C9 — Platform / environment boundary

Always excluded without separate qualification:

- Windows;
- macOS;
- ARM64;
- public eBPF/BPF-LSM PASS authority;
- backend auto-selection;
- backend baseline interchangeability.

Candidate platform family remains Linux x86_64 + native ptrace.

The exact v1 environment floor must be evidence-backed. R0 must probe:

- exact public Alpha.6 binary on Ubuntu 22.04 and Ubuntu 24.04;
- exact-version crates.io install on those environments where feasible;
- source/MSRV compatibility with declared Rust `1.82`.

If an environment fails, narrow the v1 contract rather than relabel the failure.

### C10 — Performance contract

v1 makes no universal “low overhead” promise.

Stable v1 must disclose that native ptrace overhead is workload-dependent and can be material.

The RC gate must characterize representative declared workload classes under a frozen protocol.

Performance evidence may narrow the stable support/use recommendation or produce NO_FIT; thresholds must not be weakened after measurement.

### C11 — Support/deprecation lifecycle

Before v1 release, one public support policy must freeze:

- immutable `v1.x.y` release identities;
- moving `v1` Action channel semantics;
- backward-compatible v1.x policy;
- deprecation notice/process for stable surfaces;
- bad-release rollback/revocation procedure;
- current supported stable release line.

No exact maintenance duration is invented without an explicit organizational commitment.

### C12 — Upgrade and rollback

Final RC must prove:

- Alpha.6 profile-4 -> v1 candidate compatibility for preserved surfaces;
- explicit Alpha.5 profile-3 rejection under corrected semantics;
- no baseline/policy mutation;
- unsupported schemas fail closed;
- exact previous release remains usable for rollback;
- `@v1` consumer contract is separately qualified.

## Fail-first requirements

Before editing the compatibility contract, executable sentinels must fail if the repository does not explicitly state or preserve:

1. target outcome is not verdict-bearing;
2. policy v3 opt-in stability with v2 default;
3. custody Action inputs in the stable candidate contract;
4. nondeterministic build/test scope boundary;
5. no universal low-overhead claim;
6. experimental backends excluded from stable v1;
7. Alpha.6 is the current upgrade source.

Environment probes must run independently of documentation assertions.

## Anti-drift

This gate may change contract/docs/tests only.

Product semantics, observer authority, baseline format, verdict codes, release tags and public channels remain unchanged unless a failing executable test proves the current implementation contradicts the proposed contract.

Any such contradiction keeps R0 RED and requires an explicit decision rather than silent code repair.
