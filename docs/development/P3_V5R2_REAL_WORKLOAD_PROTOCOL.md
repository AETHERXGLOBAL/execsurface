# ExecSurface — P3 V5-R2 Certificate-Aware Real-Workload Requalification Protocol

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Predecessor: #104 `C6R_ISOLATED_RESEARCH_GUARD_PASS_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO MEASUREMENT MAY PRECEDE THIS FREEZE**

## Research question

Does the P3 variance model materially reduce the previously preserved GCC ephemeral-identity component of unchanged FZF/Go executions while preserving genuine non-target variability and without introducing a false-PASS or authority-laundering path?

This is a research requalification campaign, not a release or public-schema migration.

## Frozen predecessor facts

- Original V5 Stage L is permanently `P3_INCOMPLETE_EVIDENCE`; its six failed learning samples are immutable negative evidence.
- C6 direct integration is permanently `C6_COMPATIBILITY_REGRESSION`.
- C6R proved only that an isolated certificate-aware research finalizer can produce a complete, warning-free observation on the pinned FZF workload while the public/default v2 path remains unchanged.
- No old V5 or C6R diagnostic sample is admissible as an R2 learning/check sample.

## Team

### Fixed roles

1. **Innovation Scientist / Systems Architect** — owns the minimal research-only architecture and searches for higher-information tests rather than broader permissions.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks post-hoc metric/threshold changes, sample replacement, broad normalization, claim expansion and public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — owns false-PASS, poisoning, mimicry, actor/causal substitution, observer-health and comparability attacks.
4. **Independent Milestone Reviewer** — verifies the frozen source/workload, evidence artifacts, exact run counts and allowed decision before any gate closes.

### Dynamic specialists

- Linux ptrace / clone / clone3 / fd-table lifecycle
- Rust feature gating / compatibility
- Go + GCC build nondeterminism
- canonicalization and baseline integrity
- reproducibility / CI evidence sealing
- set-stability / multi-run analysis
- software-supply-chain attack modeling
- provenance / artifact digesting

## Immutable boundaries

1. Public `v0.1.0-alpha.4`, `main`, stable `@v0.1` and public v2 semantics are out of scope and unchanged.
2. Certificate-aware collection is compile-time opt-in research behavior only.
3. Default `observe_command` remains the historical conservative public path.
4. Missing/incomplete certificate or any independent observer warning remains fail-closed.
5. Exactly three direct priming runs precede learning.
6. Exactly six learning attempts are executed; no failed/noisy sample replacement.
7. Only complete, warning-free, exit-zero research observations may form trusted learning evidence.
8. Frequency is descriptive evidence only; it never grants authorization.
9. No blanket `$TMP`, cache, module, stdlib, workspace or source-tree ignore/normalization.
10. GCC ephemeral classification remains the bounded actor/role grammar accepted by P3 V2.
11. Genuine non-target variability must remain visible through projection.
12. No check run may begin until the learning set and acceptance decision are frozen.
13. All failures and rejected candidates are retained.

## Frozen workload and environment class

- repository: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- command: `/bin/bash -lc "cd <frozen-root> && go test ./... >/dev/null 2>&1"`
- runner: GitHub-hosted `ubuntu-24.04`, Linux x86_64
- Rust: `1.90.0`
- normalization: current declared v2 profile; no new broad normalization is allowed during R2

Exact Go/GCC/kernel versions and ExecSurface source SHA are evidence fields, not silently normalized comparability dimensions.

## R0 — research interface isolation

Add a feature disabled by default that exposes only the already-accepted certificate-aware research finalization path.

Required gates:
- default build has the feature disabled;
- public/default M11 shared-FD suite remains unchanged and green;
- full `execsurface-observe` regression remains green without the feature;
- research mode cannot clear independent warnings;
- uncertified clone/fd semantics remain incomplete;
- no public raw schema change.

## R1 — research baseline collector

Create an isolated experiment collector. For one run it must:

1. invoke the certificate-aware research observer;
2. require target exit code 0 and no signal;
3. require `complete=true` and `warnings=[]`;
4. canonicalize using the frozen normalization config;
5. require a confirmed root exec;
6. build the existing v2 `BaselinePayload` and digest-verified `BaselineLock`;
7. serialize deterministically;
8. refuse to write a trusted lockfile if any admissibility condition fails.

The research collector is not installed or exposed by the public CLI.

## R2 — Stage L2 learning collection

Execution sequence:

1. checkout the pinned FZF revision into a frozen work root;
2. run exactly 3 direct priming executions, all must exit 0;
3. execute exactly 6 research learning attempts, with no replacement;
4. seal every stdout/stderr/status/lockfile and environment record;
5. verify all six lockfiles independently before variance analysis.

For each learning run record at minimum:
- source SHA;
- workload SHA;
- environment identity;
- exit state;
- observation completeness;
- warning count;
- raw event count;
- canonical effect count;
- baseline digest;
- lockfile SHA-256.

Admission rule: **6/6 or stop**. Any inadmissible run => `P3_R2_INCOMPLETE_EVIDENCE` and R3/R4 are forbidden.

## R3 — variance analysis and acceptance freeze

Run the existing V1/V2/V3 research chain over exactly the six admitted lockfiles.

Freeze before checks:
- learning-set digest;
- invariant effects;
- raw variable candidates;
- bounded GCC-targeted variable candidates;
- non-target genuine variable candidates;
- projected recurrence;
- explicit accepted-variable set.

No effect is accepted because of `k/N`, majority, high recurrence or union membership.

### R3 value condition

The bounded GCC projection is valuable only if:
- at least one targeted GCC ephemeral variable is reproduced in raw evidence;
- targeted random identities are eliminated only through the accepted bounded grammar;
- non-target variable effects preserve their support/provenance semantics;
- no accepted-variable effect is silently created;
- no broad path class is collapsed.

## R4 — unchanged checks

Only after R3 is committed/frozen, execute exactly 6 unchanged check observations from the same pinned workload/environment class.

Record separately:
- raw drift findings;
- targeted ephemeral findings;
- non-target genuine variability;
- residual findings after the explicit accepted-variance contract;
- unseen effects;
- observer incompleteness.

No check sample replacement.

## R5 — independent falsification

Mandatory attacks:

1. one-run poisoning;
2. repeated poisoning;
3. wrong-actor GCC grammar mimicry;
4. missing create/write/delete role;
5. unseen-but-similar path;
6. accepted effect under a different actor/causal chain;
7. duplicate learning artifact;
8. run-order manipulation;
9. observer/profile/source mismatch;
10. incomplete observation presented as trusted evidence;
11. non-target variable hidden by projection;
12. certificate-positive session with an independent warning.

Any unclassified permission path => `P3_R2_FALSE_PASS_FOUND`.

## Preregistered metrics

No composite score.

Report:
- `learning_admitted / 6`;
- raw variable candidate count;
- targeted GCC variable candidate count;
- non-target variable candidate count;
- projected variable candidate count;
- targeted residual random-path count;
- non-target support/provenance mismatch count;
- accepted variable effect count;
- per-check raw/residual findings;
- unseen-effect count;
- incomplete-check count;
- falsifier blocked / total.

No threshold may be invented after results are visible.

## Allowed decisions

- `P3_VARIANCE_MODEL_VALUE_ESTABLISHED_BOUNDED`
- `P3_EPHEMERAL_ONLY_VALUE_ESTABLISHED`
- `P3_MULTI_RUN_MODEL_TOO_RISKY`
- `P3_NO_MATERIAL_VALUE`
- `P3_R2_INCOMPLETE_EVIDENCE`
- `P3_R2_FALSE_PASS_FOUND`

A positive result is bounded research evidence only. Public integration requires a later, separate compatibility/release gate.