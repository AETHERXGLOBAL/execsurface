# ExecSurface — P8-A4 External Real-Workload Report Protocol

Date: 2026-09-30
Parent program: #100
P8 issue: #114
Status: **PREREGISTERED — EXTERNAL WORKLOAD REPORT REQUIRED**

## Question
Can an external user/evaluator apply the public ExecSurface path to a workload not authored by AETHER X and produce decision-quality evidence about fit, evidence completeness, false PASS/REVIEW, privacy and setup friction?

A4 is about external workload evidence. AETHER X running against third-party repositories remains compatibility evidence, not A4 independence.

## Fixed roles
1. Innovation Scientist / Product Architect — extract high-information fit/friction signals without feature sprawl.
2. Anti-Drift / Scientific Integrity Reviewer — blocks workload cherry-picking, hidden failures and adoption inflation.
3. Independent Falsifier / Red Team — attacks baseline approval, nondeterminism, completeness and false-PASS interpretations.
4. Independent Critical-Milestone Reviewer — verifies workload origin, evaluator relationship, initial result and raw evidence locator.

Dynamic specialists:
- developer experience / CI;
- Linux runtime semantics;
- build-system/nondeterminism;
- privacy/data minimization;
- performance/reproducibility when timing claims are made.

## Minimum qualifying workload
- workload/project is not authored by AETHER X;
- external evaluator controls the target/command or independently chooses a public workload;
- public alpha.4 Linux x86_64 scope is respected unless the result is explicitly `UNSUPPORTED`;
- baseline approval is an explicit evaluator decision; automatic baseline mutation is forbidden;
- result is recorded before AETHER X troubleshooting alters the attempt.

## Required report fields
- evaluator relationship;
- workload/repository identity if shareable;
- pinned revision where practical;
- command/workflow;
- ExecSurface version/install route;
- host OS/architecture and relevant CI context;
- how baseline was created and approved;
- unchanged/check result;
- observed or controlled drift result if attempted;
- completeness/error state;
- suspected false PASS / false REVIEW / false incompleteness;
- setup/privilege/documentation friction;
- privacy/data captured observations;
- whether the intended layer fits this workload;
- evidence/log locator with secrets redacted.

## No performance shortcut
If the evaluator makes a performance claim, preserve raw methodology and samples where available. Do not generalize one host/workload to universal overhead.

## Fit outcomes are symmetric
Valid outcomes include:
- useful fit;
- useful but noisy;
- incomplete for the workload;
- unsupported;
- false-review issue;
- false-pass/counterexample;
- operationally too costly;
- `NO_FIT` / redundant with the evaluator's existing layer.

No-fit is not a failed P8 program; it is decision-quality evidence.

## Prohibited
- AETHER X selecting only workloads likely to pass and calling them external;
- editing the baseline/policy to force PASS after the initial result;
- treating a branch/PR experiment as long-term adoption;
- collecting secrets, file contents, env values, stdin, network payloads or unrestricted argv as a condition of evaluation;
- deleting a negative report after a later fix.

## A4 outcomes
- `P8_A4_EXTERNAL_REAL_WORKLOAD_USEFUL_FIT_BOUNDED`
- `P8_A4_EXTERNAL_REAL_WORKLOAD_FRICTION_BOUNDED`
- `P8_A4_EXTERNAL_REAL_WORKLOAD_COUNTEREXAMPLE_BOUNDED`
- `P8_A4_EXTERNAL_REAL_WORKLOAD_NO_FIT_BOUNDED`
- `P8_A4_EXTERNAL_REAL_WORKLOAD_PARTIAL_OR_UNSUPPORTED_BOUNDED`
- `P8_A4_EXTERNAL_REAL_WORKLOAD_INCONCLUSIVE`

Current state: **`P8_A4_EXTERNAL_REAL_WORKLOAD_PENDING`**.
