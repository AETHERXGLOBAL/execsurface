# ExecSurface — P6-A3 Reproducibility / Privacy / CI-Friction Protocol

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Predecessor: `P6_A2_NO_FALSE_PASS_IN_FROZEN_SCOPE`
Status: **PREREGISTERED — NO A3 RESULT YET**

## 1. Objective

A3 asks whether the bounded A1 observations reproduce across independent hosted-runner executions and records operational facts that matter to adoption:

- setup steps;
- privilege requirements actually observed;
- runner/kernel constraints;
- evidence/artifact handling;
- external data-flow/telemetry context;
- CI integration friction;
- unsupported/not-testable cases.

A3 does **not** measure runtime performance and does not convert friction or privacy facts into points, tiers, rankings, or an overall winner.

## 2. Fixed roles

1. **Innovation Scientist / Systems Architect** — seek structural reproducibility signals, not event-count theater.
2. **Anti-Drift / Scientific Integrity Reviewer** — block score aggregation, unsupported privacy claims, and post-result stability criteria changes.
3. **Independent Falsifier / Red Team** — inspect inconsistent repetitions, missing evidence, hidden external dependencies and failure transparency.
4. **Independent Critical-Milestone Reviewer** — verify identical workload/workflow identity, independent runner executions and retained artifacts.

Dynamic specialists:
- reproducibility / CI engineering;
- Linux runtime observation;
- privacy/data minimization;
- supply-chain/action pinning;
- evidence schema analysis.

## 3. Frozen inputs

Workload:
- path: `experiments/p6-competitive-falsification/workload.sh`
- blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`

Systems:
- ExecSurface `v0.1.0-alpha.4` source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- Harden-Runner `e14015d583714f6e62063499dc959a02595150a1`;
- cicd-sensor-action `6511eb44c91d71b2b93d71193b1bf2cb18352f66`, bundled sensor `v0.0.45`.

Tetragon remains outside the live A3 repetition until its immutable executable identity is frozen. Its status stays `NOT_TESTABLE_YET`, not failed.

## 4. Independent repetition requirement

A3 requires **two independent workflow runs** of the same frozen A3 workflow file on GitHub-hosted `ubuntu-24.04` runners.

The second run may be triggered by a dedicated marker file, but:
- the A3 workflow blob must be identical across both runs;
- the workload blob must be identical across both runs;
- comparator pins must be identical;
- no acceptance criteria may change between runs.

Exact runner image/kernel identities are recorded and may differ naturally. Such platform variation is evidence, not a reason to rewrite the workload.

## 5. Reproducibility invariants

Reproducibility is **structural**, not byte-for-byte artifact equality. Run IDs, timestamps, PIDs, DNS-resolved IPs, event counts and GitHub runner IDs may vary.

### ExecSurface
For each repetition:
- S0 must execute and produce a native classified result;
- S1 process expansion must not native-PASS;
- S2 network scenario must not native-PASS; REVIEW/BLOCK or fail-closed ERROR/INCOMPLETE are both factual non-PASS outcomes;
- S3 file-write expansion must not native-PASS;
- baseline must be learned once per independent runner and not relearned after results;
- baseline/file/report artifacts retained.

A different baseline digest across independent hosts does not automatically mean semantic non-reproducibility; the raw reason must be inspected before classification.

### Harden-Runner
For each repetition:
- pinned action initializes on the hosted runner;
- S0–S3 workload steps execute;
- retained local job output is inspected for the exact S2 logical destination `example.com:443` and process attribution when emitted;
- process/file monitor setup scope is recorded from native logs;
- external security-insights/job-correlation behavior is recorded without importing private dashboard data.

A missing local S1/S3 event line remains a local-output observation only; it is not generalized into product inability.

### cicd-sensor
For each repetition:
- pinned action initializes and finalizes;
- S0–S3 execute;
- `cicd-sensor-attestation` and debug artifacts are retained;
- predicate includes its own GitHub run/job/workflow identity;
- predicate/debug evidence is inspected for `example.com` network evidence, S1 process lineage evidence, and S3 file-write evidence where emitted;
- native `passed` remains native semantics only.

## 6. Privacy/data-flow facts

A3 records observed/configured data flows separately from security quality.

### ExecSurface lane
Record:
- release download source;
- local baseline/report storage;
- whether the A3 lane configures any external ExecSurface telemetry endpoint;
- do not claim universal telemetry absence beyond the tested lane/code contract.

### Harden-Runner lane
Record from the exact pinned action/logs:
- API/telemetry endpoints actually configured or contacted;
- `disable-telemetry` setting;
- security-insights/job correlation;
- deterministic workload contains no secrets/private endpoints.

Do not label external telemetry as inherently good/bad.

### cicd-sensor lane
Record:
- release/action acquisition source;
- local sensor privilege/setup behavior evidenced by logs;
- GitHub artifact uploads for attestation/debug evidence;
- any non-GitHub external endpoint evidenced by the exact run, if present.

Do not infer absence from silence; use `NOT_OBSERVED_IN_RETAINED_EVIDENCE` where appropriate.

## 7. CI-friction record

Per lane, record factual items only:
- number/type of explicit workflow integration steps;
- need for baseline lifecycle;
- need for sudo/elevated capability evidenced by the action/run;
- external service/account dependency evidenced by the run;
- machine-consumable artifacts produced;
- hosted-runner compatibility observed;
- failure mode transparency.

No numeric friction score is allowed.

## 8. A3 acceptance

A3 may close as `P6_A3_REPRODUCIBILITY_CONTEXT_COMPLETE_BOUNDED` only if:
1. two independent runs use identical workload and A3 workflow blobs;
2. all authorized lanes execute S0–S3 in both runs;
3. ExecSurface produces no false PASS under S1–S3 in either repetition;
4. Harden-Runner and cicd-sensor retained evidence is inspected independently in both repetitions;
5. setup/privilege/privacy/artifact facts are recorded without unsupported generalization;
6. discrepancies remain visible rather than normalized away.

If one repetition has infrastructure/action failure, retain it and classify `P6_A3_INCOMPLETE` unless a preregistered fixture-only correction applies.

Allowed decisions:
- `P6_A3_REPRODUCIBILITY_CONTEXT_COMPLETE_BOUNDED`
- `P6_A3_INCOMPLETE`

## 9. Performance boundary

A3 records **no timing/overhead comparison**. P6-A4 may be opened only after A3 and only if an aligned timing methodology can be preregistered. Otherwise P6 must record `NO_VALID_PERFORMANCE_COMPARISON`.