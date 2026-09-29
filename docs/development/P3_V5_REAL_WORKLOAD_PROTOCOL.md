# ExecSurface — P3 V5 Real-Workload Requalification Protocol

Date: 2026-09-29
Tracking: #103
Parent program: #100
Preserved source problem: #62
Branch: `development/post-alpha4-behavioral-integrity`
Status: **V5 PREREGISTERED — STAGED EXECUTION**

## Objective

Requalify the P3 research stack on the exact real workload that exposed the single-run stability defect, without allowing post-hoc thresholds, blanket path normalization, frequency-based authorization, or check-time baseline mutation.

V5 asks two separate questions:

1. Can the bounded GCC ephemeral classifier remove the already-proven producer-specific filename noise without collapsing meaningful path/actor semantics?
2. Can the multi-run variance model describe genuine recurring variability with provenance while keeping unseen behavior distinct and requiring explicit acceptance before any behavior is treated as accepted variance?

A lower finding count alone is not success.

## Team

Fixed roles:
- Innovation Scientist / Systems Architect — maximize useful stability without converting variance into a whitelist;
- Anti-Drift / Scientific Integrity Reviewer — owns preregistration, forbids metric/threshold drift and post-hoc acceptance;
- Independent Falsifier / Red Team — owns unseen-effect and false-PASS challenge cases.

Dynamic specialists for V5:
- Go build/cache/module behavior specialist;
- GCC temporary-file lifecycle specialist;
- reproducibility/statistical experiment engineer;
- provenance/attestation engineer;
- canonicalization/effect-identity specialist;
- Linux ptrace completeness reviewer;
- CI runner/environment reproducibility engineer.

## Frozen workload

Repository:
`junegunn/fzf`

Revision:
`b1be3a8be1b833ce5b92fbbac11637643d60a046`

Host class:
GitHub-hosted `ubuntu-24.04`.

Toolchain:
- ExecSurface development branch source for research harness only;
- public alpha.4 observer/baseline semantics remain unchanged;
- Rust toolchain `1.90.0` for building ExecSurface;
- runner-provided Go toolchain, recorded verbatim in evidence.

Target command identity follows preserved #62 evidence:

```bash
/bin/bash -lc "cd '$WORK_ROOT' && go test ./... >/dev/null 2>&1"
```

Before learning, execute exactly **3 direct priming runs**, matching the preserved discriminator design.

No source modification to fzf is allowed.

## Stage L — trusted learning collection

Collect exactly **6 distinct trusted learning runs** using `execsurface learn` on the frozen command.

For each run retain:
- baseline lock bytes;
- baseline digest;
- SHA-256 of the evidence artifact;
- observer completeness status;
- environment/toolchain record;
- source revision.

Admission requirements before variance analysis:
- all six observations complete under current public v2 rules;
- unique evidence digests;
- valid baseline digests;
- identical comparable profile (tool/command/platform/observer/schema/normalization identity).

If any run is incomplete or incomparable, outcome is `P3_INCOMPLETE_EVIDENCE`; do not substitute or replace the run.

Run V1 analyzer over exactly those six admitted runs and persist the deterministic report.

## Stage A — acceptance freeze before checks

No check run may execute until an explicit acceptance-selection artifact is committed or the gate is deliberately run in `NO_ACCEPTED_VARIANCE` mode.

Frequency (`k/N`) is descriptive only and MUST NOT select an effect.

Any accepted-variable effect must be selected by exact canonical effect identity from the V1 variable-candidate set and preserve its learning-run provenance.

Selection review must reject:
- invariant effects;
- unseen effects;
- broad path-root wildcarding;
- similarity-based acceptance;
- effects justified only by high recurrence;
- incomplete/incomparable evidence.

The first V5 pass will always report results both **before** and **after** any explicit accepted-variance selection so acceptance value is visible separately from observation value.

## Stage C — unchanged check set

After Stage A is frozen, execute exactly **6 unchanged check runs** on the same pinned source and command.

No rerun/replacement is allowed for an admitted failed or noisy sample.

For every check record these counts separately:

1. `raw_single_baseline_findings` — current alpha.4-style single-baseline drift findings;
2. `gcc_ephemeral_eligible_findings` — findings meeting the already accepted bounded V2 GCC producer/role grammar;
3. `invariant_core_matches`;
4. `observed_variable_matches` — seen in learning union but not invariant; **not authorization**;
5. `explicitly_accepted_variable_matches`;
6. `unseen_effects` — absent from the learning union;
7. `residual_after_ephemeral_classification`;
8. `residual_after_explicit_variance`;
9. observer completeness/health state.

Also preserve exact effect identities and provenance for all residual/unseen effects.

## Historical comparator

The preserved #62 comparator is:

- direct priming: 3 runs;
- one learned baseline;
- six unchanged checks;
- finding counts: `11,14,15,13,16,19`;
- four GCC `$TMP/cc<random>.s` assembly-file findings per run;
- Go-cache finding counts: `2,4,5,3,4,3`;
- additional genuine varying Go module/stdlib/source reads.

The historical evidence remains authoritative; V5 does not rewrite or relabel it.

## Meaningful-drift sentinel

Before a positive V5 value claim, run a separately declared sentinel that introduces a canonical effect not present in the learning union and not covered by the bounded GCC grammar.

Required result:
- sentinel effect classified `unseen_effect`;
- sentinel not accepted by lexical similarity, recurrence, or producer grammar;
- no research-mode PASS-equivalent classification if unseen meaningful effects remain.

The sentinel must not be chosen after observing a weakness in check results.

## Fixed success criteria

### Full bounded value

`P3_VARIANCE_MODEL_VALUE_ESTABLISHED_BOUNDED` requires all of:
- six learning runs admitted without completeness/profile failure;
- deterministic V1 report;
- V4 safety properties remain intact;
- bounded GCC noise is removed only under the V2 role contract;
- explicit accepted variance, if used, is exact/provenance-bound and frozen before checks;
- median `residual_after_explicit_variance` is materially below the historical median raw count **and** lower than `residual_after_ephemeral_classification` on at least 4/6 checks;
- sentinel remains unseen/not accepted;
- no false-PASS path observed in the declared scope.

### Ephemeral-only bounded value

`P3_EPHEMERAL_ONLY_VALUE_ESTABLISHED` if:
- GCC bounded classification reliably removes the four known ephemeral assembly identities without collision/false acceptance;
- multi-run/explicit variance does not establish additional safe material reduction under the frozen criteria;
- sentinel remains protected.

### Negative/incomplete outcomes

- `P3_MULTI_RUN_MODEL_TOO_RISKY` if safe selection cannot avoid an authorization leak;
- `P3_NO_MATERIAL_VALUE` if admitted evidence shows no material stability gain under the fixed criteria;
- `P3_INCOMPLETE_EVIDENCE` if required learning/check evidence is inadmissible.

## Prohibited shortcuts

- no broad `$TMP`, Go cache, module, stdlib or workspace whitelist;
- no frequency threshold as permission;
- no majority-vote authorization;
- no changing learning/check counts after execution begins;
- no replacing failed/noisy samples;
- no acceptance artifact generated after check results are visible;
- no changing normalization profile merely to reduce findings;
- no public alpha.4 integration or release promotion from V5 alone.

## Execution order

1. collect Stage L only;
2. seal and inspect learning evidence;
3. freeze exact Stage A selection or `NO_ACCEPTED_VARIANCE` decision;
4. only then execute Stage C and sentinel;
5. classify using one of the preregistered V6-compatible outcomes.
