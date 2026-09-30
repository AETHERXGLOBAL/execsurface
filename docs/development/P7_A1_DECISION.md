# ExecSurface — P7-A1 GitLab CI Context Adapter Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Protocol: `docs/development/P7_A1_GITLAB_CI_ADAPTER_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED**

The research adapter can bind a validated GitLab CI job/pipeline identity as deterministic `context_only` provenance without gaining behavioral authority, selecting a baseline, changing a verdict, or upgrading observer authority.

This is a bounded adapter-contract result only. It is not evidence of successful execution on a live GitLab SaaS runner and does not authorize public GitLab support.

## Frozen contract

Required context:
- `CI_PROJECT_PATH`;
- `CI_COMMIT_SHA`;
- `CI_PIPELINE_ID`;
- `CI_JOB_ID`;
- `CI_JOB_NAME`;
- `CI_PIPELINE_SOURCE`.

Frozen output properties:
- schema `execsurface.gitlab-ci-context.v1`;
- provider `gitlab-ci`;
- authority `context_only`;
- `may_select_baseline=false`;
- `may_change_verdict=false`;
- `may_upgrade_observer_authority=false`;
- deterministic canonical JSON + SHA256 context digest.

## Historical first scientific run retained

Source: `02ae988a8c896d4394010a3036c840754fe2acf0`
Run: `36774938918`
Artifact: `11125496504`
Artifact digest: `sha256:7889c7b573fbb357a8d03e08b7ffe3dea6771ae888adef9a6693081c7a1f7345`

Result: **13/14 PASS**.

The surviving failure was test 11. The adapter itself produced the expected safe record with `may_select_baseline=false` and ignored the injected `EXECSURFACE_BASELINE=/tmp/evil.json` and `BASELINE_DIGEST=deadbeef`. The test nevertheless searched for the raw substring `baseline` in the whole canonical JSON, so it matched the legitimate contract field name `may_select_baseline`.

Classification: **test assertion defect**, not a surviving baseline-steering path.

The failure is retained and was not relabeled as a PASS.

## Smallest correction

Commit: `15685df095d04d04bbd415050cb5ad14f17e458c`

Only test 11 was corrected to assert the actual security property:
- malicious baseline path `/tmp/evil.json` is absent from canonical output;
- malicious digest `deadbeef` is absent from canonical output;
- context digest remains unchanged;
- `may_select_baseline` remains false.

No adapter behavior, required field, acceptance threshold, public semantic, baseline rule, authority rule, or test count changed.

Corrected frozen test blob:
`51d84d2795948775e43e5b87735feba1ecccb884`

Workflow pin commit:
`67ed3664a130b829ddc2f3ad3a894fc382e3f21e`

## Corrected accepted run

Run: `36775178743`
Source: `67ed3664a130b829ddc2f3ad3a894fc382e3f21e`
Conclusion: `success`
Artifact: `11125646709`
Artifact digest: `sha256:b86b68056751301db72da23b2fab52939a4212192913e004bafce3254a7fd1fe`

All workflow gates passed:
1. frozen predecessor/public boundaries;
2. frozen protocol/adapter/test blobs;
3. Python static syntax gate;
4. exact **14/14** falsification corpus;
5. deterministic reference binding;
6. candidate decision emission;
7. evidence sealing/upload.

## What A1 establishes

For the tested adapter contract, GitLab CI metadata can be represented as bounded provenance context while preserving separation from ExecSurface behavioral semantics.

The corpus proved, within this scope:
- deterministic mapping;
- commit substitution changes binding identity;
- pipeline/job replay changes binding identity;
- missing/malformed identity fails closed;
- newline/control injection fails closed;
- malformed IDs fail closed;
- project-path traversal-like syntax fails closed;
- unknown environment variables do not affect the binding;
- CI variables cannot steer baseline selection;
- CI variables cannot override a verdict;
- provider/backend naming cannot upgrade authority;
- incomplete context cannot become authoritative complete context.

## Promotion boundary

A1 does **not** establish:
- live GitLab.com runner compatibility;
- ptrace availability inside GitLab hosted executor/container boundaries;
- zero-assistance GitLab installation;
- public GitLab support;
- GitLab-specific behavioral authority;
- automatic baseline learning/approval in a pipeline.

A live GitLab promotion/reproduction gate therefore remains separate and would require an actual GitLab project/runner execution environment.

## Next P7 gate

Proceed to P7-A2: **self-hosted CI packaging and evidence contract**, reusing the same public alpha.4 semantics and fail-closed observer boundary. A2 must not use self-hosted runner trust or machine ownership to upgrade evidence authority.
