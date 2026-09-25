# M7 — Negative Evidence 001: Harness Baseline Path

Date: 2026-09-26

Run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200186478

Result:

**HARNESS FAILURE — NOT AN EXTERNAL DRIFT RESULT**

## Proven before the failure

The run successfully:

- checked out `sharkdp/fd` at exact commit `ce97e473ebaec49697c07daa50a7bc2b32f713d2`;
- verified Rust 1.98.1 satisfied fd's Rust 1.90.0 requirement;
- downloaded ExecSurface `v0.1.0-alpha.1`;
- verified the release SHA-256 checksum;
- verified GitHub build provenance;
- reproduced the pinned upstream suite with `cargo test --locked --all-features`;
- passed all 118 upstream tests;
- built `fd 10.5.0` in release mode;
- learned a real fd runtime baseline successfully.

Learned baseline:

`m7-evidence/fd.lock.json`

Baseline digest:

`sha256:6d79d51c5bd9203a27005708991a82cf57511c983c14a9dc98b588fad5e658ac`

Canonical effects:

`517`

## Failure

The first no-drift check omitted:

`--baseline m7-evidence/fd.lock.json`

The CLI correctly fell back to its default:

`execsurface.lock.json`

which did not exist.

Observed result:

`ExecSurface: ERROR`

`cannot read baseline execsurface.lock.json`

Exit code:

`2`

## Interpretation

This is a workflow-harness defect.

It is **not** evidence of:

- fd runtime drift;
- ExecSurface nondeterminism;
- observer incompleteness;
- a policy false positive.

The CLI behaved according to its documented baseline-path semantics.

## Correction

- pass the explicit learned baseline path to every CLI `check`;
- upload M7 evidence with `if: always()` so later failures preserve the partial evidence artifact;
- do not change ExecSurface core semantics or normalization.

Status:

**NEGATIVE EVIDENCE PRESERVED**
