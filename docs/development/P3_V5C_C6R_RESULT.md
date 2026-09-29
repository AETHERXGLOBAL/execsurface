# ExecSurface — P3 V5-C C6R Isolated Research Guard Result

Date: 2026-09-29
Parent: #104 / #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — C6R_ISOLATED_RESEARCH_GUARD_PASS_BOUNDED**

## Decision

`C6R_ISOLATED_RESEARCH_GUARD_PASS_BOUNDED`

The certificate-aware shared-FD completion rule is accepted only behind an isolated research-only finalizer. The public/default ptrace path remains the historical alpha.4/raw-v2 contract and continues to use the legacy conservative `shared_fd_table_ambiguity` guard.

No public release integration is authorized.

## Why C6R existed

The preceding direct C6 attempt closed as `C6_COMPATIBILITY_REGRESSION`: applying the new certificate directly to the default ptrace backend made four preserved M11 alpha.4 contract tests complete where raw v2 historically requires fail-closed incompleteness.

C6R therefore separated:
- default/public behavior: legacy conservative guard;
- research-only behavior: certificate-aware finalizer compiled only for tests/research evaluation.

The C6 failure remains retained and is not relabeled.

## Static/adversarial gate

Accepted workflow run:
`36565232413`

Accepted retained implementation commit:
`f175cf40713d02bffb7a611bda4261226165f246`

Passed gates:
- rustfmt PASS;
- clippy `-D warnings` PASS;
- default-policy-with-positive-certificate remains legacy/fail-closed PASS;
- uncertified research mode remains fail-closed PASS;
- positively certified research mode skips only the synthetic clone ambiguity guard PASS;
- independent incompleteness cannot be cleared by the certificate PASS;
- live clone-origin→FORK falsifier PASS;
- live clone-origin→VFORK falsifier PASS;
- live ordinary private/shared clone harness PASS;
- full `execsurface-observe` public-contract regression PASS;
- full workspace regression PASS;
- anti-drift public-path boundary PASS.

### Preserved C6R tooling failure

Run `36565106346` failed at clippy before falsification because the certificate boolean is used only in the `#[cfg(test)]` research branch and was therefore unused in the normal library build. The follow-up only made that internal parameter intentionally underscore-prefixed; no semantic rule, test, threshold, or acceptance condition changed.

## Pinned FZF diagnostic-only gate

Workflow run:
`36565404277`

Execution source:
`cf7fb88d798880a5b19b85445ab79ffbb5bbafc7`

Pinned workload:
`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Environment:
- Ubuntu 24.04 GitHub-hosted runner;
- Linux `6.17.0-1022-azure` x86_64;
- Go `1.24.13`;
- Rust `1.90.0`.

Protocol:
- exactly 3 direct priming runs: PASS;
- exactly 1 C6R diagnostic observation;
- no old V5 sample reused or replaced.

Observed research-finalizer evidence:
- `complete=true`;
- warnings: `0`;
- raw events: `9,152`;
- internal clone/fd certificate: `true`;
- marker: `C6R_FZF_RESEARCH_PASS`.

Immediately afterward, the unchanged historical M11 public-contract suite ran in the same workflow:
- **6/6 PASS**;
- clone-based public-v2 cases remained fail-closed exactly as before.

Artifact:
`11032155788`

Artifact SHA-256:
`07c4148576aadda5b9ed265674513413f213efa376d64a572271993f8c014611`

## Scientific interpretation

The evidence supports a bounded claim:

For the declared Linux x86_64 environment and pinned FZF workload, the internal ptrace collector can positively certify the observed clone/fd-table relation strongly enough for an isolated research finalizer to avoid the conservative synthetic `shared_fd_table_ambiguity` warning, while the existing public/default v2 path remains unchanged and fail-closed.

This does not establish:
- universal ptrace completeness;
- production readiness;
- public-v2 migration correctness;
- whole-surface backend equivalence;
- a release decision.

## Authorized next step

A **new** preregistered P3 V5-R2 real-workload requalification campaign may be created from a new source hash using the isolated research finalizer.

The original V5 Stage L remains permanently frozen as `P3_INCOMPLETE_EVIDENCE`.
