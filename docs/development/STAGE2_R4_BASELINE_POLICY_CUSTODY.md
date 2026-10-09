# ExecSurface — Stage-2 R4 Baseline/Policy Custody Threat Model

Date: 2026-10-08
Parent remediation: #165 / PR #166
Predecessors: R0–R3
Status: **PREREGISTERED — FAIL-FIRST REQUIRED**

## Objective

R4 closes one narrow semantic gap: a baseline or policy can be structurally valid and self-consistent while still being the wrong artifact for the authority context in which ExecSurface is asked to decide PASS / REVIEW / BLOCK / ERROR.

Self-consistency is not authorization.

The gate asks:

> Can ExecSurface bind the baseline and policy actually consumed by `check` to expected identities supplied by a trust anchor outside the checked-out files, and reject substitution before the target command executes?

## Fixed review roles

1. **Innovation / Architecture** — select the smallest binding that closes the custody gap without inventing a parallel signing ecosystem.
2. **Anti-Drift / Goal Alignment** — prevent P5 research prototypes, GitHub branding, signatures, or backend identity from being promoted into authority without an explicit gate.
3. **Independent Destruction / Falsification** — own substitution, malformed-pin, replay, and execution-order attacks.
4. **Independent Critical Reviewer** — require fail-first evidence, exact source SHA, regression evidence, and explicit residual assumptions before R4 can close.

## Threat model

The adversary may control or modify files in the checked-out workspace, including:

- `execsurface.lock.json`;
- `execsurface-policy.json`;
- other repository content consumed by the target command.

The adversary may construct a *different* baseline that passes `verify_lock()` and a *different* policy that parses successfully. R4 therefore must not equate internal validity with authorization.

R4 assumes that the caller/control plane supplying the expected identities is trusted relative to the checkout. The expected identities MUST NOT derive only from the same mutable baseline/policy files they are intended to authorize.

Out of scope for this gate:

- compromise of the CI control plane or repository administrators;
- an attacker who can rewrite the trusted caller/workflow and its external trust anchors;
- breaking SHA-256 cryptographically;
- promotion of P5 experimental attestation code into the public product;
- proving that a baseline was *wisely* approved rather than merely that it is the specifically authorized baseline.

A malicious or untrusted workflow remains a higher-level custody problem. R4 only establishes a product primitive that a trusted caller can enforce.

## Selected narrow binding

### Baseline identity

Use the existing verified `BaselineLock.baseline_digest` as the semantic identity of the baseline.

Required order:

1. parse baseline;
2. run existing `verify_lock()`;
3. if an expected baseline digest is supplied, validate its syntax and compare it to the verified `baseline_digest`;
4. on mismatch, return ERROR/exit 2 **before observing/executing the target**.

No new baseline schema or automatic migration is introduced.

### Policy identity

Policy v2 has no built-in self-digest. Bind the exact policy bytes with:

`sha256:<64 lowercase hex>`

Required order:

1. read policy bytes;
2. compute exact-byte SHA-256;
3. if an expected policy digest is supplied, validate syntax and compare;
4. only then parse/evaluate the policy;
5. on mismatch, return ERROR/exit 2 **before observing/executing the target**.

Exact-byte identity is deliberate for R4. Canonical policy equivalence is a separate design problem and is not required to close custody.

### CLI/API boundary

Candidate `check` options:

- `--expect-baseline-digest sha256:...`
- `--expect-policy-sha256 sha256:...`

The pins are opt-in at the generic CLI boundary to preserve existing pre-v1 behavior. Their presence creates an explicit custody assertion.

### GitHub Action boundary

Candidate Action inputs:

- `expected-baseline-digest`
- `expected-policy-sha256`
- `require-custody` (default `false` for compatibility)

When `require-custody=true`, both expected identities must be non-empty and must verify before the target runs.

The generated `execsurface init --github-actions` workflow must opt into `require-custody=true` and source the two pins from external GitHub repository/environment variables rather than deriving them from checkout files.

This does **not** prove workflow integrity. A repository that allows an attacker to rewrite the required workflow/control-plane policy must solve that at the GitHub ruleset/CODEOWNERS/reusable-workflow layer.

## P5 reuse boundary

P5 already established the useful separation:

`cryptographic validity != identity != policy authorization != ExecSurface semantic authority`

R4 reuses that separation as an architectural invariant only.

R4 MUST NOT:

- link the experimental P5 crate into the public CLI;
- treat signature presence as baseline authorization;
- infer authority from GitHub/Sigstore/SLSA branding;
- allow a valid attestation to turn incomplete runtime evidence into PASS.

## Frozen fail-first acceptance corpus

Before implementation, tests must encode these invariants:

1. **Self-consistent baseline substitution** — a different valid baseline must be rejected when it does not match the externally expected baseline digest, and the target marker must remain absent.
2. **Valid policy substitution** — a different parseable policy must be rejected when its exact-byte SHA-256 does not match the expected policy digest, and the target marker must remain absent.
3. **Malformed trust pin** — malformed expected digest syntax must fail closed before target execution.
4. **Correct pins** — correct baseline and policy pins must preserve ordinary PASS semantics.
5. **Legacy unpinned path** — absence of pins must preserve the existing compatibility boundary; it carries no new custody claim.
6. **Generated workflow** — generated GitHub workflow must request external pins and set `require-custody=true`.
7. **Action enforcement** — `require-custody=true` with a missing pin must produce ERROR/exit 2 before the target runs.

Tests may be expanded after new counterexamples, but these acceptance conditions may not be weakened to obtain green.

## R4 decision rule

R4 may become GREEN only when:

- fail-first tests are shown to fail for the intended custody reason on the pre-fix source;
- implementation is narrower than this threat model;
- substitution is rejected before target execution;
- correct pins pass;
- unpinned compatibility remains explicit and unchanged;
- P5 remains research-separated;
- full CI, adversarial regression, compatibility and independent destruction gates remain green;
- residual workflow/control-plane assumptions are documented rather than hidden.
