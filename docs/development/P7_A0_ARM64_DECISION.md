# ExecSurface — P7-A0 Linux arm64 Research Parity Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Protocol: `docs/development/P7_A0_ARM64_PARITY_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P7_A0_ARM64_NOT_PORTABLE**

This is a bounded negative portability result. It does not authorize any public support change and it does not weaken the frozen P7-A0 acceptance criteria.

## Frozen source and execution identity

- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- P7-A0 execution source: `b6c117cff0a1baa007c06f76e9bfd70f977d895b`
- workflow: `.github/workflows/p7-a0-arm64-parity.yml`
- run: `36774237512`
- workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`
- Rust: `1.90.0`

Native runners:
- x86_64: `ubuntu-24.04`
- arm64: `ubuntu-24.04-arm`

## Observed result

The x86_64 job completed the frozen native path successfully, including the exact alpha.4 workspace tests, release build, independent S0 baseline, and the frozen S0-S3 corpus.

The arm64 job ran on a real native ARM64 hosted runner (`RUNNER_ARCH=ARM64`, `uname -m=aarch64`) and compiled the exact alpha.4 workspace, but the frozen alpha.4 test suite failed before the runtime corpus could execute.

The failing test was:

`api_tests::invalid_command_metadata_returns_explicit_error`

Observed assertion:

`matches!(result, Err(ObserveError::InvalidCommand(_)))`

The frozen alpha.4 implementation explicitly uses an architecture guard equivalent to:

`#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]`

and returns:

`ObserveError::UnsupportedPlatform("current observer supports Linux x86_64 only")`

on arm64. Therefore the arm64 path cannot execute the alpha.4 reference observer semantics required by P7-A0 without modifying the frozen public source.

## Classification rationale

The preregistered protocol states that if the exact alpha.4 source cannot build or execute natively on arm64 without source modification, A0 closes as:

`P7_A0_ARM64_NOT_PORTABLE`

The result is not classified as runner/infrastructure insufficiency because:

1. a native GitHub-hosted ARM64 runner was provisioned successfully;
2. the pinned Rust toolchain installed for `aarch64-unknown-linux-gnu`;
3. the exact alpha.4 source compiled far enough to execute its workspace tests;
4. the failure is produced by the frozen source's explicit x86_64-only observer boundary;
5. the corresponding x86_64 job passed the frozen path.

It is also not classified as proposition-level evidence divergence because the arm64 observer path is explicitly unsupported before the S0-S3 proposition corpus can run.

## Retained evidence

### arm64
- job: `110088001396`
- artifact ID: `11125085724`
- artifact name: `p7-a0-arm64-36774237512`
- artifact digest: `sha256:c17fd27b5af92cfd520305f88a68f0c3f8d5d4f69597f9674cfdabc5a9f0014b`
- artifact size: `4588` bytes

### x86_64
- job: `110088001652`
- artifact ID: `11124618384`
- artifact name: `p7-a0-x86_64-36774237512`
- artifact digest: `sha256:7d4c6f41324adad8dd669c54b17f6fb107121ed6b2587fd6ad818acea242b1c8`
- artifact size: `19121` bytes

The failed arm64 run and artifacts are part of the permanent scientific record and must not be deleted or reinterpreted as a positive parity result.

## Immutable boundaries preserved

No change was made to:
- `v0.1.0-alpha.4`;
- stable `v0.1`;
- `main`;
- public v2 semantics;
- the default public observer;
- architecture-specific thresholds or whitelists.

No arm64 release asset or public-support claim is authorized.

## Next P7 phase

A0 is scientifically closed. P7 may proceed to a separately preregistered GitLab CI semantics-preserving adapter experiment. The next phase must reuse the existing ExecSurface behavioral/evidence semantics rather than introduce GitLab-specific authority weakening or platform-specific verdict rules.
