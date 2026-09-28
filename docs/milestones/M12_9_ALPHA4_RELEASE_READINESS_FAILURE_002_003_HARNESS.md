# M12.9 — Alpha.4 Release Readiness — Failures 002 / 003

Date: 2026-09-28
Release branch: `release/v0.1.0-alpha.4`
PR: #93

## Classification

`HARNESS_STALE_VERSION_AND_LITERAL_SENTINEL_FAILURES`

These failures are retained and are not product/runtime failures.

## Failure 002 — registry packaging stale release literal

Run `36470405451` failed after lockfile generation and the full fmt/Clippy/workspace-test source gates passed.

The failure occurred in `Registry Packaging Gate` because the reusable manifest validator still hard-coded:

`version = "0.1.0-alpha.3"`

while the exact release-preparation source correctly pins internal dependencies to `=0.1.0-alpha.4`.

Remediation: derive the expected version from `[workspace.package].version` in the checked-out source. This strengthens the reusable gate and does not relax package metadata or dependency requirements.

## Failure 003 — adversarial claim-boundary punctuation sentinel

Run `36470405355` failed on both Ubuntu 22.04 and Ubuntu 24.04 before semantic adversarial execution because a README sentinel required the obsolete literal:

`pathname access attempts;`

The alpha.4 README retains the same bounded claim as:

`pathname access attempts, successful-open ...`

and additionally preserves the explicit ptrace / BPF-LSM authority boundary.

Remediation: make the sentinel assert the current bounded phrase plus the explicit research-only/non-default BPF-LSM statement. No product semantics, claims, or adversarial acceptance criteria are weakened.

## Additional anti-deviation correction

The second alpha.4 readiness harness had introduced `cargo package --no-verify` while trying to avoid the known unpublished-dependency registry constraint. That bypass is not accepted as release evidence and contradicts the Failure 001 remediation boundary.

The corrected readiness gate now:

1. packages `execsurface-model` normally with verification;
2. validates the complete prospective internal dependency graph structurally;
3. executes `cargo package` for `execsurface-observe` without `--no-verify` and requires the exact expected crates.io visibility failure while alpha.4 remains unpublished;
4. treats unexpected success or any different failure reason as a blocker.

No exact pin is loosened, no local registry is substituted, no package is prepublished, and no failed run is relabeled PASS.
