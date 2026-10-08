# ExecSurface Compatibility & Stability Contract

Status: **V1_STABLE_CONTRACT_FROZEN_BOUNDED — RC EVIDENCE PENDING**  
Tracking: P9.3 / issue #134  
Current public baseline: `v0.1.0-alpha.6`  
Stable-support policy: `docs/SUPPORT_POLICY.md`

This document freezes the bounded compatibility and stability contract proposed for the future stable v1 line.

It does **not** claim that v1.0 is released or production-qualified.

## Compatibility principle

ExecSurface treats compatibility as a semantic contract, not merely a parser/API contract.

A stable release must not silently reinterpret a baseline, policy, verdict, observer-completeness state, target outcome or release identity merely to keep an old input syntactically accepted.

When safe compatibility cannot be established, explicit rejection is preferred to silent reinterpretation.

## 1. Verdict and exit-code contract

The stable v1 meanings are:

| Verdict | Exit code | Stable meaning |
|---|---:|---|
| `PASS` | `0` | Evaluation completed and no REVIEW/BLOCK finding remains under the applicable evidence/policy boundary. |
| `ERROR` | `2` | Required execution/evidence/policy evaluation could not be established as specified. |
| `REVIEW` | `10` | One or more findings require review. |
| `BLOCK` | `20` | One or more findings matched blocking policy. |

Inside v1.x these numeric meanings must not be reassigned.

Incomplete, ambiguous, unsupported or lost required evidence cannot silently become PASS.

## 2. Target outcome contract

**Stable v1 decision: target outcome is not verdict-bearing.**

The wrapped target's exit code or terminating signal remains report metadata.

`ExecSurface: PASS` means the qualified observation/diff/policy evaluation left no REVIEW/BLOCK execution-surface finding.

**PASS does not mean the wrapped target command succeeded.**

The generated GitHub Actions onboarding path therefore preserves a separate native target-command correctness gate before the ExecSurface step.

Changing target outcome into a verdict-bearing proposition would require a separately versioned compatibility decision; it is not required for v1.

## 3. Baseline compatibility

Stable facts:

- baseline lock schema: `2`;
- digest format: `2`;
- current corrected normalization profile: `4`;
- baseline remains separate from policy;
- ordinary read/check must not mutate the baseline;
- unsupported semantic states fail explicitly rather than being silently migrated.

### Current upgrade source

**Alpha.6 profile-4 is the current supported upgrade source for the future v1 RC.**

A valid Alpha.6 profile-4 baseline may be consumed directly only when the exact v1 RC proves semantic compatibility.

### Historical Alpha.5 boundary

**Alpha.5 profile-3 remains historical/rollback evidence and is explicitly rejected as semantically incomparable by profile-4 code.**

This preserves the Stage-2 R2 semantic correction:

- Alpha.5 lock schema 2 remains parseable/self-verifiable;
- profile 3 is not silently reinterpreted under profile 4;
- incompatible profile-3/profile-4 comparison fails explicitly with ERROR / exit 2;
- no automatic baseline migration or relearning is permitted;
- the exact Alpha.5 binary remains available for historical reproduction and rollback of its own profile-3 evidence.

Any future migration tool must be explicit, deterministic, preserve the original input and identify the source/target semantic versions.

## 4. Policy compatibility

Current accepted policy versions are explicit:

- schema 1: legacy-read compatibility; deprecated for new authoring;
- **schema 2 remains the default authored policy schema**;
- **schema 3 is a stable opt-in extension** for the already-qualified exact matchers below.

Stable schema-3 opt-in matchers:

- `path_resolution`
- `open_intent`
- `rename_from_class`
- `rename_from_prefix`
- `rename_from_resolution`

Schema 3 does not change schema-1/schema-2 meanings.

Unknown fields, unsupported versions and invalid v3-only matcher use under an older schema must fail explicitly rather than becoming permissive fallback.

Policy evaluation remains separate from baseline learning/mutation.

Inside v1.x, removing or semantically reassigning the qualified schema-2 or schema-3 behavior is breaking unless an explicit compatible replacement is proved.

## 5. Verdict/report compatibility

Stable machine-readable verdict schema remains `2` at this freeze.

Stable meanings include:

- verdict;
- baseline digest identity where present;
- target outcome as non-verdict-bearing report metadata;
- policy summary;
- findings;
- error state.

A future breaking report/schema change requires an explicit version boundary and migration story.

## 6. Stable CLI surface

Stable v1 candidate commands:

- `doctor`
- `init`
- `learn`
- `check`
- `--version` / `version`
- `--help` / `help`

For these stable commands, documented argument meanings and automation-relevant output/exit semantics must remain compatible inside v1.x unless a separately qualified correctness fix requires an explicit exception.

### Lower-level / non-stable-by-default surfaces

- `observe` remains a lower-level evidence surface;
- experimental libbpf/eBPF behavior is **not part of the stable v1 contract**;
- `render-error` remains an implementation/helper surface;
- internal Rust crate APIs are not stabilized merely because the workspace is public.

Public documentation of a research/lower-level interface does not automatically stabilize every raw field.

## 7. Stable GitHub Action surface

The future `AETHERXGLOBAL/execsurface@v1` stable candidate includes these inputs:

- `command`
- `baseline`
- `policy`
- `expected-baseline-digest`
- `expected-policy-sha256`
- `require-custody`
- `fail-on-review`
- `upload-artifact`
- `artifact-name`

Candidate stable outputs:

- `verdict`
- `exit-code`
- `report-json`
- `summary-markdown`
- `sarif-status`
- `artifact-url`
- `artifact-digest`

Stable enforcement:

- PASS -> Action succeeds;
- REVIEW -> succeeds by default;
- REVIEW + `fail-on-review=true` -> Action fails with the REVIEW contract;
- BLOCK -> fails closed;
- ERROR -> fails closed;
- custody-required mode must reject missing/mismatched externally anchored baseline/policy identities before target execution.

Removal or semantic reassignment of a stable v1 Action input/output is breaking unless an explicit compatible replacement is provided.

## 8. Workload-support boundary

v1 does not promise that every syntactically unchanged command produces zero execution-surface drift.

Historical external workload evidence distinguishes:

- deterministic/stable execution surfaces where unchanged PASS is a qualified normal path; and
- **highly nondeterministic build/test graphs** whose compiler/cache/temp/runtime effects can vary across nominally unchanged runs.

Under exact baseline semantics, such build/test variation **may legitimately produce REVIEW**.

The stable v1 decision is therefore:

- keep exact evidence semantics;
- do not add broad path/cache suppression merely to force PASS;
- do not auto-authorize multi-run variance;
- do not promise unchanged PASS for every build/test graph;
- require users to interpret or policy-handle legitimate variation explicitly when using these workloads.

This is a support-scope decision, not a feature gap.

## 9. Platform, installation route and observer boundary

Stable architecture boundary:

- Linux x86_64;
- native `ptrace` as the bounded public reference observer.

Explicitly not promoted:

- Windows;
- macOS;
- ARM64;
- eBPF/BPF-LSM PASS authority;
- backend auto-selection;
- backend baseline interchangeability.

Experimental eBPF/BPF-LSM and imported/research backends are **not part of the stable v1 contract**.

### Prebuilt GitHub Release binary

Current Alpha.6 evidence proves:

- exact public prebuilt artifact: PASS on Ubuntu 24.04 x86_64;
- exact current prebuilt artifact: FAIL on Ubuntu 22.04 because it requires `GLIBC_2.39`.

Therefore v1 must not claim that the current prebuilt-artifact strategy supports all Linux x86_64 userlands.

Unless V1-R1 deliberately changes the stable build floor and the exact v1 RC reproves older-userland compatibility, the qualified prebuilt-binary support floor is an Ubuntu-24.04 / glibc-2.39-class x86_64 environment with the required ptrace capability.

### crates.io / local-build path

Current Alpha.6 evidence proves exact-version local build/install plus `doctor -> learn -> check` on both:

- Ubuntu 22.04 x86_64;
- Ubuntu 24.04 x86_64.

The source manifest declares Rust `1.82`, and the V1-R0 MSRV probe successfully checks the CLI package with Rust `1.82.0`.

These facts do not imply universal Linux support. Kernel/ptrace/container restrictions still apply.

## 10. Performance contract

There is **no universal low-overhead promise** in stable v1.

Measured historical evidence shows that **native ptrace overhead is workload-dependent** and can be material.

A stable RC must characterize representative declared workload classes under a frozen protocol.

Performance evidence may result in:

- a bounded fit;
- a noisy/high-overhead fit;
- a narrower operational recommendation;
- NO_FIT for a workload.

No performance threshold may be weakened after observing results merely to obtain a stable-release label.

A faster backend is not required for v1 unless the declared support/use contract cannot be met without it.

## 11. Stable support and deprecation lifecycle

The operating policy is frozen in `docs/SUPPORT_POLICY.md`.

Core rules:

- exact `v1.x.y` releases are immutable identities;
- `@v1` is a deliberately movable stable Action channel;
- the newest fully qualified stable release is the current supported stable release;
- no fixed support duration or response-time SLA is invented by the project;
- stable surfaces are normally deprecated before removal and are not removed inside v1 merely for cleanup;
- bad-release rollback moves the stable channel to a previously qualified immutable release without rewriting user evidence;
- historical failed/bad release evidence is retained.

## 12. Upgrade and rollback contract

Before v1.0 release, the exact frozen RC must prove:

1. Alpha.6 profile-4 compatibility for the preserved stable path;
2. explicit Alpha.5 profile-3 incompatibility handling where semantics differ;
3. unsupported schema/semantic states fail closed;
4. stable verdict/exit behavior;
5. stable policy schema-2 and schema-3 behavior;
6. stable GitHub Action inputs/outputs and custody enforcement;
7. no baseline/policy mutation during upgrade validation;
8. rollback to the previously qualified release path without rewriting user evidence;
9. experimental backend/research surfaces were not accidentally promoted.

## 13. Breaking-change classification inside v1

The following are breaking unless separately proved compatible:

- reassigning verdict exit codes;
- making target outcome verdict-bearing without an explicit versioned contract;
- silently accepting a previously unsupported schema with different meaning;
- changing baseline digest/canonical meaning so old valid baselines are reinterpreted;
- relaxing incomplete-evidence behavior into PASS eligibility;
- changing a stable policy matcher/action meaning;
- silently changing policy-v2 default or policy-v3 opt-in status;
- removing or semantically reassigning a stable CLI command/argument;
- removing or semantically reassigning a stable GitHub Action input/output;
- broadening backend authority based only on backend identity/availability.

Bug fixes are not automatically breaking merely because alpha behavior differed. If a correction changes a stable v1 promise, it requires an explicit compatibility decision and evidence.

## 14. Required executable proof before v1 closeout

P9.3 remains OPEN until the exact v1 RC passes:

1. V1-R0 stable-contract sentinels;
2. Alpha.6 profile-4 upgrade rehearsal;
3. retained Alpha.5 profile-3 cross-version boundary proof;
4. unsupported-schema rejection;
5. verdict/exit regression;
6. policy schema-2/schema-3 regression;
7. GitHub Action contract/custody test;
8. environment/install-route qualification;
9. upgrade and rollback rehearsal;
10. experimental-surface anti-drift review.

Historical Alpha.5/Alpha.6 evidence is starting evidence; it does not by itself qualify an unreleased v1 candidate.

## 15. Change control

The frozen contract is:

**V1_STABLE_CONTRACT_FROZEN_BOUNDED**

Any change before v1.0 must record:

- exact changed promise;
- rationale;
- compatibility impact;
- required new evidence;
- anti-drift review.

After v1.0, changes to a stable promise follow SemVer-breaking-change discipline plus the evidence-gated release process.
