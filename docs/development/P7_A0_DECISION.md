# ExecSurface — P7-A0 Linux arm64 Research Parity Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Protocol: `docs/development/P7_A0_ARM64_PARITY_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P7_A0_ARM64_NOT_PORTABLE**

The exact public alpha.4 source is Rust-build-portable to native Linux arm64, but its public/default runtime observer is not arm64-portable. The exact source explicitly gates the ptrace implementation to `target_arch = "x86_64"` and returns `UnsupportedPlatform("current observer supports Linux x86_64 only")` on arm64.

This is a bounded negative result. It is not a claim that ExecSurface can never support arm64 and it does not authorize modifying the immutable alpha.4 release.

## Frozen identities

- alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`;
- Rust: `1.90.0`;
- x86_64 runner: `ubuntu-24.04`;
- arm64 runner: `ubuntu-24.04-arm`;
- observed kernel on both accepted probes: `6.17.0-1022-azure`.

Public alpha.4 release scope remains Linux x86_64 only.

## Historical first probe retained

Run: `36774237512`
Source: `b6c117cff0a1baa007c06f76e9bfd70f977d895b`

x86_64 job: success.

arm64 job stopped at the alpha.4 workspace tests:
- `execsurface-observe` API test `invalid_command_metadata_returns_explicit_error` failed;
- cause: on arm64 the public observer returns `UnsupportedPlatform` before the x86_64 ptrace path can validate the command metadata;
- artifact: `11125085724`;
- artifact digest: `sha256:c17fd27b5af92cfd520305f88a68f0c3f8d5d4f69597f9674cfdabc5a9f0014b`.

The first workflow stopped after the retained test failure, which prevented classification of whether the exact source could still compile a native arm64 binary. No scientific evidence was deleted.

## Harness-only correction

Commit: `2e3c24489d8e630938ab6604c5592c7f5b2cee27`

The workflow was changed only to retain the non-zero test result and continue the already-preregistered build/runtime probe. No alpha.4 source, workload, threshold, proposition, runner, public tag, baseline rule, or acceptance criterion changed.

## Corrected probe

Run: `36774496806`
Source: `2e3c24489d8e630938ab6604c5592c7f5b2cee27`

### x86_64

- workspace tests: `cargo_test_rc=0`;
- release build: `cargo_build_rc=0`;
- S0 baseline learn: `baseline_learn_rc=0`;
- S0/S1/S2/S3 exits: `0 / 10 / 2 / 10`;
- false-PASS file: empty;
- unexpected-exit file: empty;
- baseline digest: `sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1`;
- effects: `113`;
- artifact: `11125605875`;
- artifact digest: `sha256:008f9cdc1f2c011dcc2cb8610be23990d46531821e17a45236afb526671a5a7d`.

This reproduces the accepted P6 alpha.4 behavior vector.

### arm64

Native runner facts:
- runner architecture: ARM64;
- machine: `aarch64`;
- Rust host: `aarch64-unknown-linux-gnu`.

Exact alpha.4 source results:
- workspace tests: `cargo_test_rc=101` due the retained observer API test mismatch;
- release build: `cargo_build_rc=0`;
- native binary produced successfully;
- binary type: `ELF 64-bit LSB pie executable, ARM aarch64`;
- binary SHA256: `532bfcf92c76ba7dd351a0ab4af36a37dfe6079b23e3a8d540d04c2d9d676268`;
- S0 baseline learn: `baseline_learn_rc=2`;
- runtime stderr: `ExecSurface: ERROR` / `execsurface: unsupported platform: current observer supports Linux x86_64 only`;
- S0-S3 runtime corpus therefore could not be entered without modifying the exact alpha.4 observer source;
- artifact: `11125150956`;
- artifact digest: `sha256:925bc3d5112f2913281917d22382434cb9cd0bf5b05f392a69ea27c5c2dbc430`.

## Source-level explanation

At the frozen alpha.4 source, `crates/execsurface-observe/src/lib.rs` declares the ptrace implementation only under:

`#[cfg(all(target_os = "linux", target_arch = "x86_64"))]`

and its non-x86_64 branch returns:

`ObserveError::UnsupportedPlatform("current observer supports Linux x86_64 only")`.

The negative live result therefore matches the declared implementation boundary; it is not a hosted-runner anomaly.

## Scientific classification

A0 cannot close as `P7_A0_ARM64_PARITY_ESTABLISHED_BOUNDED` because the exact alpha.4 observer cannot create the required independent S0 baseline on arm64.

It also is not merely `SOURCE_PORTABLE_EVIDENCE_DIVERGENCE`: the binary builds, but the default runtime observation function explicitly refuses arm64 execution. Under the preregistered vocabulary the correct classification is:

**`P7_A0_ARM64_NOT_PORTABLE`**

where “not portable” is specifically the frozen alpha.4 runtime observer contract, not the Rust build graph.

## Non-actions

This decision does not:
- patch alpha.4;
- add an arm64 release asset;
- change stable `@v0.1`;
- add an architecture-specific whitelist;
- relax a threshold;
- claim baseline interchangeability;
- claim arm64 public support.

## Follow-on research boundary

A future arm64 observer project would require a new preregistered implementation gate covering arm64 syscall/ptrace ABI decoding, proposition authority, completeness/loss, adversarial false-PASS testing, and independent baselines before any public-support discussion.

P7 may continue to the next candidate, GitLab CI semantics-preserving integration, without reopening this negative A0 result.
