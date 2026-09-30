# ExecSurface — P7-A1 GitLab CI Semantics-Preserving Adapter Protocol

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Predecessor: `P7_A0_ARM64_NOT_PORTABLE`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO A1 RESULT YET**

## 1. Question

Can ExecSurface consume GitLab CI job/pipeline identity as bounded provenance context while preserving the existing x86_64 behavioral/evidence semantics exactly, without allowing CI metadata to upgrade observer authority, change verdicts, select baselines, or reinterpret incomplete evidence?

This is a research-only adapter gate. It is not a public GitLab integration or release-promotion gate.

## 2. Hypothesis

A minimal GitLab CI adapter can be semantics-preserving if it performs only deterministic context binding:

GitLab CI environment -> validated context record -> digest/reference

and never participates in:

- runtime proposition authority;
- observer completeness classification;
- baseline selection or mutation;
- policy/verdict computation;
- backend selection;
- unsupported/lost/ambiguous laundering.

## 3. Fixed roles

1. **Innovation Scientist / Systems Architect** — design the smallest useful CI-context boundary; avoid a GitLab-specific product fork.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks authority inflation, CI-specific verdict semantics, baseline auto-selection, and public-support claims.
3. **Independent Falsifier / Red Team** — attacks forged CI variables, replay, newline/control injection, commit substitution, baseline steering, and hidden authority transfer.
4. **Independent Critical-Milestone Reviewer** — verifies exact corpus, deterministic serialization, immutable public boundaries, retained failures, and decision wording.

Dynamic specialists:
- GitLab CI environment/provenance semantics;
- supply-chain / CI identity;
- runtime evidence semantics / PL;
- reproducibility / canonical serialization;
- adversarial input validation.

## 4. Immutable boundaries

A1 MUST NOT:

- modify public `v0.1.0-alpha.4`, stable `v0.1`, `main`, public v2 semantics, or the default observer;
- make GitLab metadata a source of behavioral authority;
- use GitLab project/job/pipeline names to upgrade evidence;
- auto-select or mutate a baseline from CI variables;
- treat missing/malformed CI identity as PASS;
- add GitLab-specific acceptance thresholds;
- claim public GitLab support from this experiment;
- delete negative evidence.

## 5. Frozen input contract

Required context fields for an authoritative context-binding record:

- `CI_PROJECT_PATH`;
- `CI_COMMIT_SHA`;
- `CI_PIPELINE_ID`;
- `CI_JOB_ID`;
- `CI_JOB_NAME`;
- `CI_PIPELINE_SOURCE`.

Optional descriptive fields may be retained only if explicitly versioned and validated. Unknown environment variables are ignored by the adapter and cannot affect the digest.

## 6. Output contract

The research adapter produces a deterministic JSON record with:

- schema: `execsurface.gitlab-ci-context.v1`;
- provider: `gitlab-ci`;
- project path;
- exact lowercase 40/64-hex commit identity;
- pipeline ID;
- job ID;
- job name;
- pipeline source;
- authority: `context_only`;
- `may_select_baseline=false`;
- `may_change_verdict=false`;
- `may_upgrade_observer_authority=false`;
- canonical SHA256 digest over the canonical record.

The adapter exposes no field for ExecSurface PASS/REVIEW/BLOCK/ERROR and no baseline path or baseline digest input.

## 7. Frozen falsification corpus

A1 must execute the same fixed corpus after implementation:

1. complete valid context maps successfully;
2. repeated mapping is byte-deterministic;
3. commit substitution changes the context digest;
4. pipeline/job replay changes the context digest;
5. missing required field fails closed;
6. malformed commit identity fails closed;
7. newline/control injection fails closed;
8. malformed numeric pipeline/job IDs fail closed;
9. project-path injection/traversal-like syntax fails closed;
10. unknown environment variables cannot change canonical output;
11. CI variables cannot select/set a baseline;
12. CI variables cannot supply/override a verdict;
13. provider/backend naming cannot upgrade authority above `context_only`;
14. incomplete/malformed context cannot be represented as an authoritative complete binding.

Acceptance threshold is **14/14**. Do not weaken or delete a test after execution begins.

## 8. Decision classes

A1 may close as:

- `P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED` — 14/14 pass and immutable boundaries/reproofs pass;
- `P7_A1_GITLAB_CONTEXT_ADAPTER_FAIL` — scientifically executable corpus exposes a surviving semantics/authority flaw;
- `P7_A1_INCOMPLETE` — infrastructure or harness failure prevents scientific execution.

## 9. Required reproofs

After the 14-test corpus passes, reprove:

- public alpha.4 tags still resolve to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no A1 change touches public `crates/`;
- P5 standards-composition closeout remains ancestral;
- P6 factual-comparison closeout remains ancestral;
- A0 negative result remains retained and unchanged.

## 10. Promotion boundary

A positive A1 result proves only a bounded semantics-preserving context adapter design. Public GitLab support would require a separate packaging/zero-assistance/live-GitLab promotion gate.
