# M9.1a — External Workload Target Qualification

Date: 2026-09-26
Status: **CLOSED / INITIAL COHORT QUALIFIED — M9.1b EXECUTION PENDING**
Tracking: #51
Canonical protocol: `docs/milestones/M9_PROTOCOL.md`
Supplementary progression protocol: `docs/milestones/M9_EXTERNAL_ADOPTION_PROTOCOL.md`

## Claim boundary

All targets in this document begin at **L0 / `ZERO_CONTACT_EXTERNAL_REPRO`**.

Selection, cloning, CI execution, performance measurement, or preparation of an integration kit by AETHER X does **not** constitute independent validation or adoption.

No maintainer contact is part of this qualification gate.

## Selection freeze

The cohort was pinned before target execution results were interpreted. It is intentionally small and heterogeneous:

- one process/shell-orchestration-heavy Rust project;
- one file/search-heavy Rust project;
- one non-Rust/Python ecosystem project with a documented focused test workflow.

A fourth project previously used in M7 is retained only as a calibration control and is not counted as new M9 external-workload expansion.

All three primary pinned revisions were resolved successfully from their upstream GitHub repositories during qualification. That verifies target identity only; it is not ExecSurface compatibility evidence.

## T1 — casey/just

**Qualification: ACCEPT / PRIMARY FIRST TARGET**

- repository: `casey/just`
- pinned commit: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- canonical evidence class: `ZERO_CONTACT_EXTERNAL_REPRO`
- progression level before execution: `L0`
- initial workload: `cargo test --all`
- performance eligibility: **TBD by frozen direct timing protocol**

### Why this target

`just` is a command runner. Its test workload exercises process creation, shell execution, temporary files, command outcomes and runtime orchestration close to ExecSurface's product domain.

The selected command is an upstream project test command rather than an AETHER-designed synthetic benchmark.

### Expected information value

- high process-lineage diversity;
- realistic shell/child-exec activity;
- temporary path and scheduling pressure that may expose canonicalization noise;
- potential direct runtime suitable for M6.5 trigger evaluation, but this is not assumed before measurement.

### Initial risk

The suite may contain environment-sensitive or temporary-path behavior. Any repeated unexplained drift is evidence to preserve and investigate, not a reason to silently change normalization.

## T2 — BurntSushi/ripgrep

**Qualification: ACCEPT / SECOND TARGET**

- repository: `BurntSushi/ripgrep`
- pinned commit: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- canonical evidence class: `ZERO_CONTACT_EXTERNAL_REPRO`
- progression level before execution: `L0`
- initial workload: `cargo test --all`
- performance eligibility: **TBD by frozen direct timing protocol**

### Why this target

The workload is complementary to `just`: it is filesystem/search-heavy and exercises temporary test trees, pattern processing and repeated binary execution rather than primarily command-runner orchestration.

### Expected information value

- heavy filesystem/path activity;
- temporary-directory and deterministic-path-normalization pressure;
- execution of built project binaries from integration tests;
- broad Rust workspace behavior under one pinned command.

### Initial risk

The full suite may be expensive for the canonical **3 warmups + 15 measured direct + 15 measured ptrace** performance protocol. M9.1b must first reproduce and measure direct runtime. If the suite is operationally unsuitable for repeated performance sampling, preserve that result and keep the full suite as functional L0 evidence only. Any replacement performance workload requires a pre-result protocol amendment before accepted measurements.

## T3 — pytest-dev/pytest

**Qualification: ACCEPT / CROSS-ECOSYSTEM TARGET, SETUP GATE REQUIRED**

- repository: `pytest-dev/pytest`
- pinned commit: `8721173580390a9d297e5af06cac3f0b6841f425`
- canonical evidence class: `ZERO_CONTACT_EXTERNAL_REPRO`
- progression level before execution: `L0`
- initial focused workload: `pytest testing/test_config.py`
- performance eligibility: **TBD by frozen direct timing protocol**

### Why this target

This target prevents M9 from becoming a Rust-only compatibility exercise. A focused pytest development workflow exercises Python processes, plugins, temporary directories, configuration lookup and subprocess-oriented test infrastructure.

### Expected information value

- different runtime/toolchain ecosystem;
- path/configuration discovery behavior;
- Python subprocess and temporary-file semantics;
- baseline stability under a dynamic-language workflow.

### Initial risk / setup gate

Development-environment setup is more complex than the Rust targets. M9.1b must reproduce the pinned target's documented development setup without secret-dependent services before accepted timing or observation evidence.

If setup cannot be made reproducible and bounded on the standard Linux runner without material target changes, T3 is **KILLED AS A CURRENT M9 TARGET** and the failure is preserved.

## C0 — sharkdp/fd calibration control

**Qualification: CONTROL ONLY / NOT NEW M9 EXPANSION**

- repository: `sharkdp/fd`
- pinned commit: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`
- prior evidence: M7 compatibility proof

M7 already proved an AETHER-controlled compatibility workflow against this revision, including repeated no-drift PASS and controlled runtime expansion detection. Re-running it may validate the M9 harness, but it cannot be counted as a new M9 target, independent evidence, or adoption.

No claim about the target's current moving branch is required for this calibration identity.

## Execution order

1. **T1 / just first** — highest process-orchestration information value.
2. **T2 / ripgrep second** — complementary filesystem-heavy workload.
3. **T3 / pytest third** — cross-ecosystem after setup reproducibility is proved.
4. C0 / fd only if needed to distinguish harness regression from target-specific behavior.

## Qualification decision

**PROVED — INITIAL COHORT PINNED BEFORE EXECUTION RESULTS.**

This gate proves only that the cohort identity and execution order were frozen in advance and that the pinned upstream commit identities resolve. It does **not** prove target setup, compatibility, baseline stability, performance, drift detection, or adoption.

The next step is **M9.1b**: execute the frozen zero-contact protocol against T1 (`casey/just`) first, without contacting or modifying the upstream project, and preserve every success/failure under the machine-readable M9 schema.
