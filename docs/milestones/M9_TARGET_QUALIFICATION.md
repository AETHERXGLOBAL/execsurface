# M9.1 — External Workload Target Qualification

Date: 2026-09-26
Status: **CLOSED / INITIAL COHORT QUALIFIED — EXECUTION PENDING**
Tracking: #51
Protocol: `docs/milestones/M9_EXTERNAL_ADOPTION_PROTOCOL.md`

## Claim boundary

All targets in this document begin at **L0 / AETHER-controlled compatibility**.

Selection, cloning, CI execution, performance measurement, or preparation of an integration kit by AETHER X does **not** constitute independent validation or adoption.

No maintainer has been contacted as part of M9.1.

## Selection strategy

The initial cohort is intentionally small and heterogeneous:

- one process/shell-orchestration-heavy Rust project;
- one file/search-heavy Rust project;
- one non-Rust/Python ecosystem project with a documented focused test workflow.

This gives higher information value than testing many near-identical Rust CLIs.

A fourth project previously used in M7 is retained only as a calibration control and is not counted as new M9 external-workload expansion.

## T1 — casey/just

**Qualification: ACCEPT / PRIMARY FIRST TARGET**

- repository: `casey/just`
- default branch: `master`
- pinned commit: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- evidence level at selection: `L0`
- initial workload: `cargo test --all`
- performance eligibility: **TBD by frozen direct timing protocol**

### Why this target

`just` is a command runner. Its integration tests exercise process creation, shell execution, temporary files, command outcomes and cross-platform runtime behavior, which are close to ExecSurface's actual product domain.

The upstream CI at the pinned revision runs `cargo test --all` on Ubuntu, macOS and Windows, making the selected workload an upstream-defined test command rather than an AETHER-designed synthetic benchmark.

The protected branch also lists Ubuntu test CI among required checks at qualification time.

### Expected information value

- high process-lineage diversity;
- realistic shell/child-exec activity;
- temporary path and scheduling pressure that may expose canonicalization noise;
- likely direct runtime long enough to evaluate the M6.5 trigger, but this is not assumed before measurement.

### Initial risk

The test suite may contain environment-sensitive or temporary-path behavior. Any repeated no-drift finding is evidence to investigate, not a reason to silently change normalization.

## T2 — BurntSushi/ripgrep

**Qualification: ACCEPT / SECOND TARGET**

- repository: `BurntSushi/ripgrep`
- default branch: `master`
- pinned commit: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- evidence level at selection: `L0`
- initial workload: `cargo test --all`
- performance eligibility: **TBD by frozen direct timing protocol**

### Why this target

The upstream README explicitly documents `cargo test --all` from the repository root as the full test suite.

The workload is complementary to `just`: it is dominated by file/search semantics, temporary test trees, pattern processing and repeated binary execution rather than command-runner orchestration.

### Expected information value

- heavy filesystem/path activity;
- temporary test directories and deterministic path normalization pressure;
- process execution of the built `rg` binary from integration tests;
- broad Rust workspace behavior under a single documented command.

### Initial risk

The full suite may be larger than needed for repeated 11+11 performance runs. M9.2 may first measure direct runtime and, if operational cost is excessive, mark the full suite unsuitable for performance sampling while retaining it for functional L0 evidence. A replacement workload requires a protocol-compliant predeclared amendment before accepted measurements.

## T3 — pytest-dev/pytest

**Qualification: ACCEPT / CROSS-ECOSYSTEM TARGET, SETUP GATE REQUIRED**

- repository: `pytest-dev/pytest`
- default branch: `main`
- pinned commit: `8721173580390a9d297e5af06cac3f0b6841f425`
- evidence level at selection: `L0`
- initial focused workload: `pytest testing/test_config.py`
- performance eligibility: **TBD by frozen direct timing protocol**

### Why this target

The upstream contributing documentation explicitly gives `pytest testing/test_config.py` as a normal focused development test command.

This target prevents M9 from becoming a Rust-only compatibility exercise. Pytest's tests exercise Python processes, plugins, temporary directories, configuration lookup and subprocess-oriented testing infrastructure.

### Expected information value

- different runtime/toolchain ecosystem;
- path/configuration discovery behavior;
- Python subprocess and temporary-file semantics;
- useful stress on baseline stability across a dynamic-language test workflow.

### Initial risk / gate

Development-environment setup is more complex than the Rust targets. M9.2 must reproduce the pinned target's documented development setup without secret-dependent services before this target enters accepted timing or observation evidence.

If setup cannot be made reproducible and bounded on the standard Linux runner without material target changes, T3 is **KILLED AS A CURRENT M9 TARGET** and the failure is preserved.

## C0 — sharkdp/fd calibration control

**Qualification: CONTROL ONLY / NOT NEW M9 EXPANSION**

- repository: `sharkdp/fd`
- pinned commit: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`
- prior evidence: M7 compatibility proof

The pinned commit is still the current upstream `master` revision at M9.1 qualification time.

M7 already proved an AETHER-controlled compatibility workflow against this revision, including repeated no-drift PASS and controlled runtime expansion detection. Re-running it may validate the M9 harness, but it cannot be counted as a new independent target or as adoption.

## Execution order

1. **T1 / just first** — highest product-semantic value and upstream-defined Linux CI command.
2. **T2 / ripgrep second** — complementary filesystem-heavy workload.
3. **T3 / pytest third** — cross-ecosystem after setup reproducibility is proved.
4. C0 / fd only if needed to distinguish M9 harness regression from target-specific behavior.

## M9.1 acceptance decision

**ACCEPT.**

The cohort is pinned before execution results are interpreted. The next step is M9.2a: execute the frozen L0 protocol against T1 (`casey/just`) without contacting or modifying the upstream project.
