# ExecSurface — P3 V5-R2 R4 Fresh Unchanged Checks + Meaningful-Drift Sentinel Protocol

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO R4 CHECK MAY PRECEDE THIS FREEZE**

## Preconditions

R3-C is frozen at:
- result: `P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED`;
- learning run: `36573676797`;
- learning artifact: `11035133723`;
- learning artifact SHA-256: `sha256:a98a1f022e601f0b0653474df8c4500f33fa5670f7e25f105740b2e79bd2fd26`;
- raw learning-set digest: `sha256:437a9f8fca9f0ff4c8573c1cfee06b46cb325d1d1a890649ba536acd8e2e648d`;
- projected learning-set digest: `sha256:16e8519c83ea374359d1e44872d570ccc15be88b7ccf8f9e3b54b5e3d6b7ab30`;
- R3-C freeze commit: `97d759e78a4573f197568e3cf578e06b155a0940`.

Explicit accepted-variable selection is **empty**. Frequency/recurrence grants no authorization.

## Team

Fixed roles:
1. Innovation Scientist / Systems Architect — seek value without broadening authorization.
2. Anti-Drift / Scientific Integrity Reviewer — owns metric definitions and blocks post-hoc selection.
3. Independent Falsifier / Red Team — owns unseen/sentinel and false-PASS attacks.
4. Independent Milestone Reviewer — verifies exact six checks, evidence hashes and allowed decision.

Dynamic specialists:
- Go/GCC build nondeterminism;
- canonical diff / multi-run set semantics;
- Rust research tooling;
- Linux ptrace completeness;
- CI reproducibility;
- provenance/evidence sealing;
- supply-chain false-PASS analysis.

## Frozen workload

- repository: `junegunn/fzf`;
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`;
- host class: GitHub-hosted Ubuntu 24.04 x86_64;
- unchanged command: `/bin/bash -lc "cd \"$1\" && go test ./... >/dev/null 2>&1" bash "$WORK_ROOT"`;
- no FZF source modification;
- no new direct priming stage is added after the R3-C freeze;
- exactly **6 fresh unchanged checks**, no rerun/replacement.

Each check must be exit-zero, `complete=true`, warning-free, certificate-positive, profile-comparable, and produce one verified lockfile. Any inadmissible check closes R4 as incomplete.

## R4 evaluator semantics — frozen before checks

The evaluator consumes the six frozen R2-C learning lockfiles plus one fresh check lockfile.

### Raw single-baseline comparator

Use learning lock #1 as the fixed alpha.4-style single baseline. `raw_single_baseline_findings` is:

`added + removed + changed`

from the existing deterministic `execsurface-diff` engine between learning lock #1 and the fresh raw check surface.

### GCC eligible findings

`gcc_ephemeral_eligible_findings` counts raw diff findings whose before/after target belongs to the exact V2 evidence-backed GCC grammar on the corresponding raw surface. No path-prefix-only classification is allowed.

The historical discriminator predicts four such findings per unchanged check (old random identity open/delete removed + new random identity open/delete added). R4 requires exactly **4/6? No**: to avoid converting historical expectation into an unjustified hidden threshold, the fixed gate is stricter in meaning but not count-specific: every check must have `gcc_ephemeral_eligible_findings > 0`, and after projection `projected_gcc_random_findings == 0`. The exact observed count is retained.

### Ephemeral projection comparator

Apply only the already-frozen V2 candidate projection. Compare projected learning lock #1 with projected current check using the same diff engine.

`residual_after_ephemeral_classification = projected added + removed + changed`.

No non-target effect may be changed by the projection.

### Multi-run/accepted-variance semantics

Build the projected six-run V1 report and the V3 accepted-variance contract with the already-frozen empty selection.

For current projected surface:
- `invariant_core_matches`: projected invariant effects present now;
- `missing_invariant_effects`: projected invariant effects absent now;
- `observed_variable_matches`: current effects in projected learning variable-candidate set; descriptive only, not authorization;
- `explicitly_accepted_variable_matches`: always 0 under this freeze;
- `unseen_effects`: current projected effects absent from projected learning union;
- `residual_after_explicit_variance` = missing invariant effects + observed-variable matches + unseen effects.

Variable-candidate absence is not counted as a finding because the learning model explicitly established presence/absence variability; presence is still residual because the empty selection grants no authorization.

All residual/unseen identities are retained.

## Fixed R4 interpretation

A check is structurally successful only if:
- health/completeness admission passes;
- `gcc_ephemeral_eligible_findings > 0`;
- no targeted randomized GCC identity remains in projected findings;
- non-target effects are not collapsed by V2 projection;
- explicit accepted matches remain 0.

No positive P3 product outcome is issued from R4 alone.

## Meaningful-drift sentinel — frozen before check results

After the six unchanged checks, run exactly one separately marked sentinel observation on the same pinned FZF tree and same command executable/argument-count shape, but with one added behavior inside the shell script before `go test`:

1. create outside the traced command: `/tmp/execsurface-p3-v5-sentinel-meaningful.txt`;
2. traced shell invokes `/usr/bin/cat /tmp/execsurface-p3-v5-sentinel-meaningful.txt >/dev/null`;
3. then runs the unchanged pinned FZF `go test ./...`.

The sentinel path is not a GCC `cc<token>.s` identity and the actor is `cat`, not `gcc`.

Required sentinel outcome:
- observation exit 0, complete, warning-free, certificate-positive;
- sentinel canonical file-read effect absent from projected learning union;
- classified as `unseen_effect`;
- not present in accepted-variable matches;
- not removed by V2 GCC projection;
- `residual_after_explicit_variance > 0`;
- no PASS-equivalent research interpretation while sentinel remains.

If the exact sentinel effect is unexpectedly already in the frozen learning union, outcome is `P3_R4_SENTINEL_COLLISION` and no replacement sentinel may be chosen in the same campaign.

## R4 output metrics per check

1. raw_single_baseline_findings
2. gcc_ephemeral_eligible_findings
3. invariant_core_matches
4. observed_variable_matches
5. explicitly_accepted_variable_matches
6. unseen_effects
7. residual_after_ephemeral_classification
8. residual_after_explicit_variance
9. observer completeness/health state

Also preserve exact effect identities for raw diff, residuals and unseen effects.

## Allowed outcomes

- `P3_R4_CHECK_SET_COMPLETE_SENTINEL_PROTECTED`
- `P3_R4_INCOMPLETE_EVIDENCE`
- `P3_R4_TARGETED_PROJECTION_FAILED`
- `P3_R4_NON_TARGET_COLLAPSE_FOUND`
- `P3_R4_SENTINEL_FALSE_PASS`
- `P3_R4_SENTINEL_COLLISION`

R5 independent falsification remains mandatory before `P3_VARIANCE_MODEL_VALUE_ESTABLISHED_BOUNDED` or `P3_EPHEMERAL_ONLY_VALUE_ESTABLISHED` can be considered.

Public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, and public/default v2 semantics remain unchanged.
