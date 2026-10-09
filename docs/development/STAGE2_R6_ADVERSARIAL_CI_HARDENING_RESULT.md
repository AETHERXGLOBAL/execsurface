# ExecSurface — Stage-2 R6 Adversarial CI Trigger Hardening Result

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Protocol: `docs/development/STAGE2_R6_ADVERSARIAL_CI_HARDENING_PROTOCOL.md`  
Qualified source: `86ac865e70b60be7ae2306ca18fd33771940f499`  
Decision: **R6_ADVERSARIAL_CI_HARDENING_PASS_BOUNDED**

## Decision

R6 is GREEN for the preregistered bounded proposition:

> Changes within the semantic workspace boundary cannot silently avoid the durable Adversarial Regression workflow merely because they touch a crate or workspace manifest omitted from a hand-maintained path list, and push/pull-request trigger sets remain symmetric.

R6 changes CI trigger coverage only. It does not change observation, normalization, baseline, diff, policy, verdict, custody, or release semantics.

## Fail-first evidence

Fail-first source: `36497d07ede001ce11934aa3cba3cc32a0c96e1a`  
CI run: `37714190947`  
Job: `113106789657`

Formatting and clippy passed. The frozen R6 trigger-contract tests then failed **0/2**:

1. `r6_push_and_pull_request_adversarial_paths_are_symmetric`
   - failed because the push and pull-request path sets differed.
2. `r6_semantic_workspace_changes_trigger_adversarial_regression`
   - failed because the old trigger set omitted the required workspace boundary, including `Cargo.lock`.

The failure was therefore the intended CI-contract falsification, not a formatting or dependency failure.

## Implemented trigger contract

Both `push.paths` and `pull_request.paths` in `.github/workflows/adversarial-regression.yml` now use the same set:

- `.github/m12-fixtures/**`
- `.github/workflows/adversarial-regression.yml`
- `crates/**`
- `Cargo.toml`
- `Cargo.lock`
- `README.md`

The coarse `crates/**` boundary is deliberate: it prefers some additional CI execution over silent under-triggering when semantic dependencies evolve.

## Final executable evidence

At qualified source `86ac865e70b60be7ae2306ca18fd33771940f499`:

- R6 trigger-contract tests: **2/2 PASS**
- R5 hostile policy corpus remains: **14/14 PASS**
- no R0-R5 acceptance test was weakened.

Qualification workflows on the same source:

- CI — `37714285112` — SUCCESS
- Adversarial Regression — `37714285126` — SUCCESS on Ubuntu 22.04 and Ubuntu 24.04
- P9.3 Compatibility Contract — `37714285105` — SUCCESS
- Registry Packaging Gate — `37714285172` — SUCCESS
- Public Consumer Smoke — `37714285158` — SUCCESS
- P8 A3.4 consumer contract red team — `37714285109` — SUCCESS
- P8 A3 typed report prototype — `37714285107` — SUCCESS
- P8 A3 typed evidence output — `37714285128` — SUCCESS
- Ptrace Lifecycle Regression — `37714285153` — SUCCESS

## Anti-drift decision

- Alpha.5 remains immutable historical evidence.
- Product semantics were not changed to satisfy R6.
- Adversarial tests were not weakened.
- Push/PR trigger symmetry is now executable repository policy.
- No merge or release is authorized by R6.
- R7 hostile cross-gate destruction may now open.
