# M12.9 — Alpha.4 Release Readiness — Failure 001

Date: 2026-09-28
Release branch: `release/v0.1.0-alpha.4`
PR: #93

## Classification

`EXPECTED_SEQUENTIAL_REGISTRY_VISIBILITY_FAILURE`

This is not a product-code failure and is not evidence that the alpha.4 package manifests are invalid.

## Failed run

- workflow: `Alpha.4 Release Readiness`
- run: `36470175704`
- job: `109090152904`
- release candidate SHA: `caae35b008456c4296e6b73097b0d7b6cdbf923a`
- conclusion: **failure**

## What passed before the failure

- release identity and alpha.4 tag absence;
- no stale alpha.3 reference in active release surfaces;
- exact internal dependency pin checks;
- alpha.4 release-workflow bindings;
- full `cargo fmt`;
- full-workspace Clippy with `-D warnings`;
- full-workspace tests;
- Cargo.lock stability;
- release build;
- `execsurface --version == 0.1.0-alpha.4`;
- `execsurface doctor` on Ubuntu 24.04;
- `cargo package` for the root dependency `execsurface-model 0.1.0-alpha.4`.

## Failure

The next packaging command attempted `execsurface-observe 0.1.0-alpha.4` before `execsurface-model 0.1.0-alpha.4` existed on crates.io.

Cargo rejected the package preparation because the exact registry requirement:

`execsurface-model = "=0.1.0-alpha.4"`

could only see previously published alpha.3 and alpha.2 versions in the registry index.

## Interpretation

This reproduces the M12.6 constraint: a dependent prerelease crate cannot truthfully be registry-package/dry-run validated against a new exact dependency version before that dependency becomes visible in crates.io.

The production publish workflow already handles this correctly by publishing in dependency order and checking registry visibility between crates.

The failed run is retained as negative evidence. It must not be relabeled PASS.

## Authorized remediation

The pre-tag readiness gate will:

1. package the root `execsurface-model` crate;
2. verify all workspace package versions and exact internal dependency requirements structurally from Cargo metadata/manifests;
3. rely on full-workspace build/test for local path dependency coherence;
4. preserve sequential registry publication as the only valid end-to-end registry package proof for dependent crates.

No product code, semantic behavior, dependency requirement, or publish order is relaxed.