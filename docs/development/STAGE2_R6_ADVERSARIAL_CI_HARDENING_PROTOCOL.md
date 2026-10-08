# ExecSurface — Stage-2 R6 Adversarial CI Trigger Hardening Protocol

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Predecessor: R5 PASS_BOUNDED  
Status: **PREREGISTERED — FAIL-FIRST REQUIRED**

## Objective

R6 closes one bounded CI reliability gap:

> A pull request or main-branch push that changes code or manifests capable of altering ExecSurface runtime-evidence, normalization, baseline, diff, policy, CLI, or workspace dependency semantics must not silently skip the durable Adversarial Regression workflow.

The ordinary CI workflow already runs on every pull request. R6 is specifically about the adversarial regression gate and its trigger contract.

## Fixed review roles

1. **Execution / CI Architecture** — implement the smallest durable trigger contract.
2. **Innovation / Architecture** — prefer a future-proof dependency boundary over a fragile hand-maintained list.
3. **Anti-Drift / Goal Alignment** — do not turn R6 into product semantics, release policy, or observer redesign.
4. **Independent Destruction / Falsification** — own skipped-trigger and push/PR asymmetry attacks.
5. **Independent Critical Reviewer** — require a fail-first replay and executable trigger-contract test before R6 may close.

## Reproduced current gap

Before R6:

- push paths include observer plus selected CLI files;
- pull-request paths are narrower;
- policy, normalize, diff, baseline, model and other crate changes can avoid the adversarial workflow;
- push and pull-request trigger sets are not symmetric.

This can leave a semantics-changing PR with ordinary CI green while the dedicated hostile regression suite is not requested.

## Selected narrow contract

Use a coarse, durable workspace boundary rather than enumerating transitive crate dependencies.

Both `push.paths` and `pull_request.paths` for `.github/workflows/adversarial-regression.yml` must include exactly the same required semantic trigger classes:

- `.github/m12-fixtures/**`
- `.github/workflows/adversarial-regression.yml`
- `crates/**`
- `Cargo.toml`
- `Cargo.lock`
- `README.md`

The two path sets must remain equal.

Why `crates/**`:

- avoids missing a future transitive semantic dependency;
- automatically covers model/observe/normalize/baseline/diff/policy/CLI/report crate changes;
- removes the already falsified push/PR asymmetry caused by manually maintained per-file lists.

R6 does not claim that every crate change is adversarially meaningful. It deliberately accepts some extra CI execution in exchange for eliminating silent semantic under-triggering.

## Frozen fail-first acceptance corpus

Before implementation, a repository test must prove that the current workflow violates the contract.

The test must:

1. parse the workflow text without relying on YAML key coercion;
2. extract `push.paths` and `pull_request.paths`;
3. require the sets to be identical;
4. require every selected trigger class above in both sets;
5. fail on the pre-fix workflow;
6. pass only after the workflow is corrected.

The acceptance test may not be weakened to match the old narrower list.

## Kill conditions

R6 must stop rather than claim success if the change:

- disables path filtering entirely without an explicit cost/intent decision;
- fixes PR paths but leaves push paths semantically different;
- enumerates only today's known policy/CLI files and recreates the maintenance hazard;
- changes product semantics to make CI pass;
- weakens the adversarial test suite;
- uses workflow success on an unrelated SHA as qualification evidence.

## R6 close condition

R6 may become GREEN only when:

- fail-first trigger-contract evidence is preserved;
- the corrected workflow satisfies the durable trigger test;
- ordinary CI is green;
- Adversarial Regression passes on Ubuntu 22.04 and 24.04;
- P9.3 compatibility remains green;
- no prior R0-R5 acceptance test is weakened;
- no merge or release is authorized by R6 alone.
