# M12.7 Failure 001 — Red-Team Harness Package ID

Date: 2026-09-28
Tracking: #90
Branch: `integration/m12-portable-clean`

## Classification

`HARNESS_FAILURE_NOT_PRODUCT_BLOCKER`

## Failed run

- Workflow: `M12.7 Independent Internal Release Red-Team`
- Run: `36466685070`
- Job: `109078411741`
- Candidate SHA: `6a7e7ec7aed7e8692124e7803efe8d161ff179e3`
- Conclusion: **failure**

## What passed before the failure

The red-team had already passed:

- public-line freeze and alpha.4-tag absence;
- proof that runtime/product source remained frozen after the M12.6 semantic candidate;
- diff-scope kill check;
- adversarial fixture build;
- rustfmt and full-workspace Clippy `-D warnings`;
- shared-FD fail-closed tests;
- full M12 adversarial replay;
- existing ptrace Linux regressions;
- frozen baseline digest test.

## Failure

The CLI compatibility sub-step invoked:

`cargo test --locked -p execsurface-cli ...`

but the package at `crates/execsurface-cli` is named:

`execsurface`

Cargo therefore stopped with:

`package ID specification 'execsurface-cli' did not match any packages`

exit code `101`.

## Interpretation

This is a red-team harness error, not evidence that the product or CLI compatibility failed. The failed run is retained and is not reclassified as PASS.

The failure also means all later red-team sentinels in that run were skipped, including the conservative clone-guard disclosure check. Therefore the gate remains OPEN.

## Authorized repair

Change only the CLI package selector in the M12.7 workflow from `execsurface-cli` to `execsurface` and rerun the complete red-team gate. No assertion may be weakened or removed.