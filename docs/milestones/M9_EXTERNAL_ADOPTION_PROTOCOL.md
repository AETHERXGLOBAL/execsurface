# M9 — Independent Adoption & Real-Workload Evidence Protocol

Date: 2026-09-26
Status: **FROZEN BEFORE TARGET RESULTS**
Tracking: #51
Base: `8f1ec63219e084f10678655ee698fd2a482312d6`

## Objective

M9 moves ExecSurface from AETHER X-controlled compatibility evidence toward independent product evidence while preserving strict claim boundaries.

The program has two independent questions:

1. Does the current ptrace public product produce useful, reproducible runtime-drift evidence on real external developer/CI workloads?
2. Do any real workloads provide an evidence-backed reason to reopen the deferred eBPF path?

A negative result is valid evidence. M9 must not manufacture adoption or an eBPF need.

## Governance team

### Fixed — Innovation Scientist / Product Architect

- choose high-information targets rather than high-volume outreach;
- convert real external friction into minimal product improvements;
- search for workflows where execution-surface drift is genuinely useful;
- separate product value from backend novelty.

### Fixed — Deviation Prevention / Scientific Integrity

- freeze claim levels and benchmark rules before target results;
- prevent AETHER-controlled compatibility from being called adoption;
- preserve negative results, nondeterminism and false positives;
- reject post-hoc sample removal or target switching to improve metrics;
- keep M6.5 eBPF trigger unchanged.

### Independent Adoption / Evidence Red Team

Challenge:

- independence of the evidence source;
- reproducibility from a pinned target commit;
- hidden setup/manual intervention;
- noisy baselines and nondeterministic findings;
- false positive / false negative risk;
- privacy leakage;
- benchmark population and host claims;
- user-facing friction;
- whether an eBPF reopening trigger is genuinely met.

### Dynamic specialists

- developer experience and CLI ergonomics;
- OSS project/CI workflows;
- Linux runtime/process semantics;
- performance measurement/statistics;
- privacy/evidence review;
- release/distribution;
- support/triage.

## Evidence levels

Every target has exactly one current evidence level.

### L0 — AETHER-controlled compatibility

AETHER X runs ExecSurface against a pinned external project/workload.

Permitted claim: compatibility/workload evidence.

Forbidden claims: independent validation, external adoption, maintainer endorsement.

### L1 — External reproduction

An independent user/maintainer runs the prescribed workflow and returns reproducible logs/results.

Permitted claim: independent external reproduction.

Not automatically: retained adoption.

### L2 — Adoption trial

An independent external project/user intentionally installs or integrates ExecSurface in a branch, PR, CI experiment, or sustained evaluation.

Permitted claim: external adoption trial.

### L3 — Adoption

The external project/user merges, retains, or repeatedly uses ExecSurface after the initial trial.

Permitted claim: adoption, scoped to the named project/workflow and evidence date.

### Promotion rule

A target advances only when the required external action actually occurs. Interest, email replies, GitHub stars, issue comments, downloads, or AETHER-authored PRs do not by themselves promote a target to L1/L2/L3.

## M9.0 target-selection rules

A target candidate should satisfy as many as possible:

1. public OSS repository;
2. reproducible Linux workflow from a pinned commit;
3. no secret-dependent setup for the selected command;
4. deterministic or explainably variable execution behavior;
5. meaningful process/file/runtime activity;
6. command executable non-interactively in CI;
7. direct median preferably >=100 ms if used for M6.5 trigger evaluation;
8. practical runtime suitable for at least 11+11 timed samples;
9. no target-source change required for the clean baseline experiment;
10. licensing and repository policy do not block local compatibility testing.

A target may be KILLED before execution for irreproducible setup, secret dependency, excessive runtime, unsafe side effects, or meaningless runtime behavior.

## Target pinning

Every accepted experiment records:

- repository owner/name;
- exact commit SHA;
- selected command;
- host image/OS/kernel/architecture;
- toolchain/runtime versions materially affecting the command;
- ExecSurface version/commit;
- setup commands needed before the timed/observed command.

Never use a moving branch as the evidence identity.

## Reproduction gate

Before ExecSurface observation:

1. checkout the pinned target commit;
2. install only declared prerequisites;
3. execute the selected command directly;
4. require successful, explainable output/exit status;
5. record direct runtime and any expected nondeterminism;
6. if the command cannot reproduce cleanly, record the failure before any workaround.

A target that needs material source modification before direct reproduction is not accepted under the original candidate definition.

## Fresh-install gate

Where practical, the trial must install the public product without using the ExecSurface repository checkout:

```bash
cargo install execsurface --locked
```

If the public registry path cannot be used for a specific host/toolchain reason, the exact alternative must be recorded and the evidence level remains explicit.

## Runtime workflow gate

For each accepted L0 workload:

### W1 — Learn

Run the selected pinned workload through public ptrace-backed `execsurface learn` and record:

- observation health/completeness;
- baseline digest;
- canonical effect count;
- elapsed time;
- warnings.

### W2 — Repeat no-drift

Run the identical command with `execsurface check` against the learned baseline.

Expected evidence:

- comparable observation;
- no unexplained added/removed/changed effects;
- PASS only if normal product semantics justify it.

Any repeated nondeterministic drift must be recorded before normalization/fixes.

### W3 — Controlled runtime expansion

Introduce a non-destructive runtime expansion without modifying the target source when possible—for example, a wrapper that executes one additional benign program or reads a controlled sentinel file before/after the target command.

The expansion must be visible as an added runtime effect and must produce the policy outcome dictated by the selected policy.

The fixture may test detection, but it must not be presented as target-project behavior.

### W4 — Repeatability

Repeat W2/W3 sufficiently to distinguish deterministic product behavior from one-run scheduling noise.

## Performance protocol

Performance evidence is separate from functional compatibility.

### Eligibility

A workload is eligible for the M6.5 relative-slowdown trigger only if its direct median is at least `100 ms`.

The absolute-overhead trigger may still be evaluated independently.

### Samples

For each eligible target/workload:

- at least 2 untimed warmups per mode;
- at least 11 clean timed direct samples;
- at least 11 clean timed ptrace-observed samples;
- same pinned source, setup, command and host class;
- record raw samples in execution order;
- no post-hoc sample deletion based on result magnitude.

A sample with incomplete/failed observation is excluded from the clean performance distribution only as a health failure, and the exclusion plus reason must remain in the evidence ledger.

### Statistics

Report at minimum:

- sample count;
- median;
- median absolute deviation (MAD);
- median absolute observer overhead = observed median - direct median;
- median slowdown = observed median / direct median.

Optional p90 may be reported but cannot replace the median trigger.

### M6.5 trigger

The eBPF path may be reopened for targeted evaluation if an external workload reproduces either:

1. ptrace median slowdown > `2x` with direct median >= `100 ms`; or
2. ptrace median absolute observer overhead > `500 ms`.

Crossing this trigger means **REOPEN EVALUATION**, not eBPF approval.

## Privacy sentinel

M9 preserves the metadata-only boundary.

Evidence must not persist:

- file contents;
- environment values;
- stdin contents;
- network payloads;
- secret values;
- unrestricted child argv values.

For selected trials, use benign unique sentinel values where practical and assert those literal values do not appear in persisted ExecSurface evidence.

## False-positive / false-negative ledger

Every target gets a ledger entry with:

- unexplained no-drift findings;
- expected effects missing from evidence;
- machine-specific path noise;
- scheduling/interleaving noise;
- transient toolchain/cache effects;
- unsupported semantics;
- usability/setup friction;
- whether the issue is target-specific or product-general;
- evidence label before and after any fix.

A fix cannot erase the pre-fix counterexample.

## Claim rules

Allowed language must match evidence level.

Examples:

- L0: `ExecSurface reproduced a controlled compatibility trial against <repo>@<sha>.`
- L1: `An independent external user reproduced the prescribed trial.`
- L2: `<project/user> is evaluating ExecSurface in a branch/PR/CI trial.`
- L3: `<project/user> retained/merged ExecSurface for the stated workflow.`

Never use `adopted`, `validated by`, or equivalent language for L0.

## External-contact boundary

M9.0 and M9.1 are zero-contact evidence phases. Public repositories may be inspected and reproduced without contacting their maintainers.

Before any external issue/PR/outreach:

- the L0 evidence pack for that target must be clean enough to reproduce;
- the proposed message/integration must be narrow and technically accurate;
- no private AETHER X material is disclosed;
- no unsupported performance/security claims are made;
- the external action is recorded in the M9 ledger.

## M9 phase sequence

1. **M9.0 — Protocol freeze:** this document.
2. **M9.1 — Target qualification:** shortlist and pin high-information external workloads.
3. **M9.2 — L0 compatibility + performance runs:** execute protocol and preserve raw evidence.
4. **M9.3 — Product hardening from real friction:** only evidence-backed fixes.
5. **M9.4 — External reproduction/adoption kit:** prepare smallest reproducible external trial.
6. **M9.5 — Independent evidence:** L1/L2/L3 only if an external party actually performs the required action.
7. **M9.6 — Red Team closure:** final claim/evidence audit and next decision.

## M9.0 acceptance

M9.0 closes when this protocol is committed before target-specific experimental results are interpreted.

After freeze, target-specific changes to thresholds, sample counts, evidence levels, or the M6.5 trigger require an explicit protocol amendment with rationale recorded **before** using the changed rule on new evidence.
