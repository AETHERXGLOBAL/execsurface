# M7 — Negative Evidence 002: Partial Harness Fix Did Not Reach No-Drift Checks

Date: 2026-09-26

Run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200439290

Result:

**HARNESS FAILURE — NOT AN EXTERNAL DRIFT RESULT**

## Context

After the first M7 harness failure, the workflow was changed to preserve artifacts with `if: always()` and to introduce explicit baseline handling.

However, the repeated no-drift check commands still omitted:

`--baseline m7-evidence/fd.lock.json`

## Proven before the failure

The run successfully:

- checked out exact upstream commit `ce97e473ebaec49697c07daa50a7bc2b32f713d2`;
- verified the supported toolchain;
- downloaded ExecSurface `v0.1.0-alpha.1`;
- verified release SHA-256 and GitHub build provenance;
- reproduced the upstream suite with **118 tests PASS**;
- built `fd 10.5.0` in release mode;
- learned the real `fd` runtime baseline;
- preserved partial M7 evidence as an artifact.

Baseline digest:

`sha256:6d79d51c5bd9203a27005708991a82cf57511c983c14a9dc98b588fad5e658ac`

Canonical effects:

`517`

## Failure

The first repeated no-drift check again fell back to:

`execsurface.lock.json`

instead of using the explicitly learned M7 evidence path.

Observed result:

`ExecSurface: ERROR`

because the default baseline file did not exist.

## Interpretation

This remained a workflow-harness defect.

It was not evidence of:

- upstream `fd` nondeterminism;
- observer incompleteness;
- a false-positive drift;
- a policy error.

The CLI correctly enforced its documented baseline-path behavior.

## Correction

The next commit added the explicit baseline path to the repeated no-drift and controlled-drift CLI checks.

No ExecSurface core, canonicalization, policy or verdict semantics were changed.

Final corrected run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200690279

Result:

**PASS**

## Governance conclusion

**NEGATIVE EVIDENCE PRESERVED.**

A partial harness correction was not treated as success. The exact failing path was fixed and the full proof was rerun.
