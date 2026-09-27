# Post-M9 — Ptrace Fast-Path Semantic Parity Method

Date: 2026-09-27
Status: **FROZEN BEFORE VALUE MEASUREMENT**
Parent: GitHub issue #84
Candidate protocol: `docs/milestones/POST_M9_PTRACE_PHASE_AWARE_EXIT_FASTPATH_PROTOCOL.md`

## Purpose

Define the semantic-equivalence test before reference-vs-candidate performance measurement. This record does not authorize the candidate and does not change normalization, baseline, diff, policy, verdict, observer authority, or the public release.

## Why raw JSON byte equality is not the acceptance metric

Two independent executions can legitimately use different runtime TIDs and raw event sequence numbers even when they establish the same ExecSurface semantics. Therefore raw `Observation` JSON byte equality across separately launched processes is not the semantic parity criterion.

No candidate-specific projection or new normalization rule is introduced to solve this. Instead the parity gate uses the **existing production canonicalization and baseline machinery already used by `execsurface learn` / `check`**.

## Frozen parity layers

### Layer 1 — observer health and target outcome

For reference and candidate runs:

- observation must complete successfully;
- `complete` must be identical and must be `true` for accepted value samples;
- warnings must be empty for accepted value samples;
- target exit code and signal must match exactly;
- backend public identity/capabilities/limitations must remain unchanged.

Any candidate-only warning, incomplete state, protocol error, crash, or target-outcome difference fails parity.

### Layer 2 — existing canonical surface

For the same pinned command, cwd, semantic-root configuration and host class:

1. obtain an observation with the reference observer;
2. pass it through the current `execsurface-normalize::canonicalize` implementation with the same `NormalizationConfig` used by the public CLI;
3. obtain a candidate observation;
4. canonicalize it with **the same unchanged normalizer and configuration**;
5. require exact equality of the resulting canonical surfaces.

No path family, event type, field, actor, temporary root, or effect may be removed or rewritten specifically for this candidate.

### Layer 3 — public baseline/diff contract

As an end-to-end guard:

- create a reference baseline using the existing public `learn` path;
- run the candidate against the same command and reference baseline using the existing public `check --diff-only` path or an equivalent direct invocation of the existing diff engine;
- require comparable evidence and zero semantic findings for unchanged-workload parity cases;
- reverse the direction on deterministic parity fixtures where practical (candidate baseline -> reference check) to detect asymmetric loss.

A PASS obtained by changing policy is not parity. Policy is downstream of this gate.

## Dynamic-runtime external workloads

The pinned `ripgrep` and `fzf` runtime targets are used for value measurement because their accepted M9.1 runtime cases were complete and stable. The exact command/cwd/revision/toolchain is frozen in the value workflow before measurement.

For these workloads, accepted timing samples require health/outcome parity. Canonical parity is checked in dedicated pre-measurement runs using the same binaries/worktree; timing samples themselves are not post-hoc filtered based on favorable canonical output.

## Adversarial fixture parity

The candidate must also preserve the current observer semantics for the frozen FP-01..FP-16 matrix. Existing tests count only for the semantics they explicitly assert. Candidate-specific adversarial fixtures are required for phase ambiguity around signals/restart, failed exec, and high volumes of irrelevant syscalls.

## Prohibited comparison shortcuts

- no stripping semantic event fields to make outputs match;
- no new normalizer rule after seeing candidate differences;
- no comparing only event counts;
- no policy suppression to turn differences into PASS;
- no ignoring warnings or incomplete states;
- no treating different target outcomes as performance samples;
- no selective deletion of mismatch cases.

## Acceptance rule

Candidate A is **semantic-parity eligible for value measurement** only when:

1. isolated candidate formatting/lint/tests pass;
2. the frozen adversarial matrix is satisfied for the implemented candidate revision;
3. reference/candidate health and target outcomes match;
4. existing canonical surfaces match on the declared parity cases;
5. public baseline/diff unchanged-workload parity produces no semantic findings;
6. all failures and exclusions remain recorded.

Only after these conditions are met may the separately preregistered >=10% per-target value threshold be evaluated.
