# ExecSurface — P6-A4 Performance Gate Decision

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Predecessor: `P6_A3_REPRODUCIBILITY_CONTEXT_COMPLETE_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**NO_VALID_PERFORMANCE_COMPARISON**

P6-A4 is not executed as a timing benchmark because the current frozen systems do not expose a sufficiently aligned measured boundary for a scientifically defensible cross-system overhead comparison.

This is not a claim that performance is unimportant and not a claim that any system is faster or slower.

## Why timing is not valid under the current frozen architecture

The P6 protocol permits A4 only if A1-A3 establish comparable correctness evidence and the runtime environments are sufficiently aligned. The runner environment is aligned, but the measured product boundaries are not.

### ExecSurface alpha.4

The tested public lane downloads a fixed release and then executes the target through the ExecSurface CLI. The measured path includes direct target observation plus ExecSurface evidence construction, canonicalization and baseline comparison. A separate baseline-learning lifecycle is also required.

### Harden-Runner

The tested lane installs a pinned GitHub Action that initializes job-level monitoring and operates across the surrounding workflow. Its setup, Security Insights/job-correlation behavior and telemetry context are not the same execution boundary as a single wrapped command.

### cicd-sensor

The tested lane initializes a pinned action plus runtime sensor/agent/proxy services and later emits Runtime Trace/debug artifacts. This is a broader job/runtime collection boundary than the ExecSurface alpha.4 CLI invocation.

## Confounders that would make one-number timing misleading

1. **Different lifecycle boundaries** — per-command CLI execution versus job-level monitor initialization/finalization.
2. **Different output work** — ExecSurface canonical/diff reports, Harden-Runner job/Security Insights context, and cicd-sensor attestation/debug artifacts.
3. **Different external-service behavior** — the exact A3 lanes do not have identical telemetry or remote-service dependencies.
4. **Different correctness states on S2** — ExecSurface alpha.4 fails closed as `ERROR/INCOMPLETE`, while the other lanes expose network evidence. Timing those outcomes as though they were equivalent successful observations would mix semantic completeness with speed.
5. **Baseline lifecycle asymmetry** — ExecSurface requires a learned baseline for drift comparison; the comparator lanes do not implement the same lifecycle.
6. **Hosted-runner variance** — GitHub-hosted runner scheduling, image state, network/DNS and action acquisition add noise that cannot be attributed equally to the compared systems without changing the frozen integration contracts.

## Rejected benchmark shortcuts

The following are explicitly rejected because they would create an apples-to-oranges metric:

- total workflow duration as a product-overhead score;
- action setup time versus ExecSurface command time;
- event count per second without equivalent event semantics;
- timing only S2 despite different completeness outcomes;
- subtracting a direct-control run while leaving unequal setup/telemetry/finalization work in the measured lanes;
- combining correctness, friction and timing into a composite score.

## What would be required for a future valid performance study

A future performance track may be opened only if it first freezes a proposition-equivalent measurement boundary, for example:

- exact same target process lifetime;
- collectors already initialized before the measured interval;
- equivalent proposition set and completeness requirement;
- output serialization/finalization measured separately;
- identical warm-up and repetition policy;
- direct unobserved control on the same runner class;
- enough repetitions for distributional reporting rather than a single mean;
- CPU, wall-clock and memory reported separately;
- no correctness failure included as a comparable successful timing sample.

That would be a new preregistered performance experiment, not a reinterpretation of P6-A1/A3.

## Scientific boundary

This decision establishes only that **P6's current frozen comparison corpus does not support a valid cross-system performance claim**.

It does not establish:

- which system is faster;
- which system has lower CPU or memory overhead;
- whether one system scales better;
- whether a future aligned performance experiment is impossible.

P6 may proceed to bounded factual closeout without a performance ranking.
