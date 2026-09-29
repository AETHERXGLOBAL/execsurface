# ExecSurface — P4-A1.4 Public Anti-Drift / Regression Gate

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor A1.2 accepted source: `8ad1fcf1d9ab474fd6a0d007f198844aef4694fb`
Predecessor A1.3 decision: `P4_A1_3_FALSIFICATION_PASS_BOUNDED`
Predecessor A1.3U decision: `P4_A1_3U_ATTEMPT_AUTHORITY_PASS_BOUNDED`
A1.3U correction commit: `7dfdd3958ac7e6c38cb52859d883035fa6e9ace2`
Status: **PREREGISTERED — POST-A1.3U PUBLIC ANTI-DRIFT REPROOF REQUIRED**

## Question

Did A1 research, including the bounded A1.3U attempt-authority correction, introduce any regression or semantic drift into the public/default alpha.4 behavior while building the proposition-scoped ptrace adapter?

## Post-A1.3U reproof requirement

The earlier A1.4 PASS predates the accepted A1.3U authority-model correction and therefore cannot close A1 by itself. A final A1.4 run must execute from a source that contains:

- correction commit `7dfdd3958ac7e6c38cb52859d883035fa6e9ace2`;
- `P4_A1_3_FALSIFICATION_PASS_BOUNDED`;
- `P4_A1_3U_ATTEMPT_AUTHORITY_PASS_BOUNDED`.

This addendum strengthens the anti-drift requirement. It does not alter any public acceptance criterion, regression command, workload, threshold, or expected outcome.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — no feature work in this gate; preserve the smallest research-only boundary.
2. **Anti-Drift / Scientific Integrity Reviewer** — owns public-path invariants and blocks promotion if any existing contract changes.
3. **Independent Falsifier / Red Team** — replays historical shared-FD and PATH-TOCTOU/adversarial regressions.
4. **Independent Milestone Reviewer** — verifies source, tag immutability, exact commands, retained failures, and final decision.

## Immutable public boundary

The gate must not modify public runtime code. The following remain fixed:
- tag `v0.1.0-alpha.4` resolves to release source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable annotated tag `v0.1` continues to target the same release source;
- public raw observation schema remains v2;
- default observer remains legacy conservative ptrace behavior;
- public M11 shared-FD fail-closed behavior remains unchanged;
- public M12 adversarial/PATH-TOCTOU behavior remains unchanged;
- no research authority record is emitted through the public CLI/action path;
- no release/tag/channel movement is authorized.

## Frozen checks

A1.4 executes, without post-result changes:

1. verify the branch and required A1.3/A1.3U result documents;
2. verify the checked-out source contains correction commit `7dfdd3958ac7e6c38cb52859d883035fa6e9ace2`;
3. verify `v0.1.0-alpha.4^{commit}` and `v0.1^{commit}` both equal `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
4. verify changes since accepted A1.2 source did not modify public runtime crates;
5. `cargo +1.90.0 fmt --all -- --check`;
6. `cargo +1.90.0 clippy --locked --workspace --all-targets -- -D warnings`;
7. `cargo +1.90.0 test --locked -p execsurface-observe --all-targets`;
8. explicit M11 replay: `cargo +1.90.0 test --locked -p execsurface-observe --test m11_shared_fd_ambiguity -- --nocapture`;
9. explicit M12 replay: `cargo +1.90.0 test --locked -p execsurface-observe --test m12_adversarial -- --test-threads=1 --nocapture`;
10. full workspace regression: `cargo +1.90.0 test --locked --workspace --all-targets`;
11. seal exact logs and source SHA.

The path audit from A1.2 accepted source permits research/docs/workflow-only A1.3/A1.3U changes. Any change under `crates/execsurface-observe`, public CLI/action packages, or existing raw-v2 public model definitions introduced after the A1.2 accepted source fails the gate.

## Acceptance

A1.4 may close only if all frozen checks pass. Allowed outcomes:
- `P4_A1_PUBLIC_ANTIDRIFT_PASS`
- `P4_A1_PUBLIC_CONTRACT_REGRESSION`
- `P4_A1_INCOMPLETE_EVIDENCE`

Only `P4_A1_PUBLIC_ANTIDRIFT_PASS` together with A1.2, A1.3 and A1.3U permits the bounded A1 closure:

`P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`

That closure authorizes A2 authority-gap analysis only. It does not authorize public integration or a new backend.