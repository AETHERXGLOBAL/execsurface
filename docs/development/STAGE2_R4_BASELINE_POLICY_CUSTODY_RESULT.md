# ExecSurface — Stage-2 R4 Baseline/Policy Custody Result

Date: 2026-10-08
Parent remediation: #165 / PR #166
Threat model: `docs/development/STAGE2_R4_BASELINE_POLICY_CUSTODY.md`
Qualified source: `a4a11fdb4b44e741e5770586311992b44f8815a9`
Decision: **R4_BASELINE_POLICY_CUSTODY_PASS_BOUNDED**

## Decision

R4 is GREEN for the bounded proposition defined by the preregistered threat model:

> When an expected baseline identity and/or policy identity is supplied by a trust anchor outside the checked-out artifacts, ExecSurface binds the artifacts actually consumed by `check` to those expected identities and rejects a mismatch before the target command executes. The generated GitHub Actions workflow opts into requiring both external custody pins.

This is a custody/authorization binding result. It is not a claim that a baseline was wisely approved, that a workflow is immutable, or that a signature/standard/backend name grants ExecSurface semantic authority.

## Implemented binding

### Baseline

- the baseline is parsed and verified using the existing lock self-consistency rules;
- `--expect-baseline-digest sha256:...` binds the verified `baseline_digest` to an external expected identity;
- malformed, mismatched, or duplicated expected baseline digests fail before target execution.

### Policy

- `--expect-policy-sha256 sha256:...` binds the exact policy bytes to an external expected identity;
- the policy digest is checked before policy parsing/evaluation and before target execution;
- an expected policy digest without an explicit policy is rejected;
- malformed, mismatched, or duplicated expected policy digests fail closed.

### GitHub Action / generated workflow

The Action exposes:

- `expected-baseline-digest`;
- `expected-policy-sha256`;
- `require-custody`.

When `require-custody=true`, both external pins are mandatory. Missing/partial pins and invalid custody-mode values produce ERROR/exit 2 before target execution.

`execsurface init --github-actions` now generates a workflow that:

- sets `require-custody: "true"`;
- sources baseline identity from `${{ vars.EXECSURFACE_BASELINE_DIGEST }}`;
- sources policy identity from `${{ vars.EXECSURFACE_POLICY_SHA256 }}`;
- does not derive either expected identity from the checked-out baseline/policy files.

## Final hostile corpus

At qualified source `a4a11fdb4b44e741e5770586311992b44f8815a9`:

- `r4_custody.rs`: **7/7 PASS**
  - self-consistent baseline substitution;
  - parseable policy substitution;
  - malformed expected digest;
  - correct baseline + policy pins;
  - legacy unpinned compatibility;
  - duplicate baseline trust assertion;
  - duplicate policy trust assertion.

- `r4_action_custody.rs`: **6/6 PASS**
  - missing both required pins;
  - partial pin;
  - invalid `require-custody` value;
  - correct external pins;
  - well-formed but unauthorized baseline pin;
  - well-formed but unauthorized policy pin.

Every custody-rejection test that uses a marker requires the marker to remain absent, proving the failure occurs before target execution rather than after the side effect.

## Retained failure history

Failures were preserved and not rewritten into success:

1. Initial R4 test addition failed rustfmt before scientific execution. Classified formatting-only; assertions unchanged.
2. After formatting correction, the preregistered CLI corpus produced **1 PASS / 4 FAIL** because the custody options did not exist. This is the fail-first implementation baseline.
3. Initial CLI implementation exposed a rustfmt-only failure and a Cargo.lock regeneration failure. Both were corrected mechanically; semantics/tests were not weakened.
4. Action/workflow fail-first tests then failed exactly because the Action did not expose custody inputs and the generated workflow did not require external pins.
5. The first real Action E2E destruction found that the preflight blocked execution but did not surface its rejection reason in stderr. The implementation was hardened to emit the explicit preflight reason; the test was retained.
6. A later destruction pass found that duplicate trust pins used ordinary last-value-wins parsing. Both duplicate-baseline and duplicate-policy attacks reached target execution and returned REVIEW (10) rather than the required pre-execution ERROR (2). The parser was hardened to reject duplicate trust assertions; the tests were retained.
7. Final well-formed-but-wrong Action pin attacks passed fail-closed after a formatting-only correction.

## Qualification workflows

All required workflows passed against the same qualified source:

- CI — run `37711694994` — SUCCESS
- Public Consumer Smoke — run `37711695108` — SUCCESS
- Adversarial Regression — run `37711695060` — SUCCESS
- Registry Packaging Gate — run `37711695040` — SUCCESS
- P8 A3.4 consumer contract red team — run `37711695055` — SUCCESS
- P8 A3 typed report prototype — run `37711695061` — SUCCESS
- P8 A3 typed evidence output — run `37711694983` — SUCCESS
- P9.3 Compatibility Contract — run `37711695031` — SUCCESS
- Ptrace Lifecycle Regression — run `37711694986` — SUCCESS

## P5 boundary

R4 reuses one architectural invariant established during P5 research:

`cryptographic validity != identity != policy authorization != ExecSurface semantic authority`

No P5 experimental crate was linked into the public CLI. R4 does not grant authority based on GitHub, Sigstore, SLSA, signer identity, attestation presence, or backend naming.

## Residual assumptions

R4 deliberately assumes the expected pins are supplied by a trust anchor outside the mutable checked-out artifacts.

Therefore R4 does **not** close:

- malicious modification of the trusted workflow/control plane itself;
- administrator compromise;
- authorization quality of the human/process that chose the expected baseline or policy;
- cryptographic compromise of SHA-256.

Repository/environment variable governance, protected workflows/rulesets, CODEOWNERS or reusable trusted workflows remain deployment/control-plane responsibilities rather than properties proven by this product gate.

## Anti-drift decision

- Alpha.5 remains immutable historical evidence.
- No old baseline is rewritten or silently relearned.
- No release is authorized.
- No PR merge is authorized by R4 alone.
- R5 may now open.
