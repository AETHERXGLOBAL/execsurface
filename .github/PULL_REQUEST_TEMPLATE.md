## Change

Describe the problem and the smallest evidence-preserving change that addresses it.

## Product scope / anti-drift

Explain why this remains within ExecSurface's product identity: **runtime behavioral integrity and verification for observed execution-surface drift**.

- [ ] This does not turn ExecSurface into antivirus, EDR, SIEM, malware detection, sandboxing, generic observability or agent orchestration.
- [ ] No unsupported security, performance, portability, adoption or novelty claim is introduced.

## Evidence

- [ ] Tests added/updated where behavior changed.
- [ ] Reproduction details are included.
- [ ] Material negative/rejected results are preserved.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo clippy --locked --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo test --locked --workspace --all-targets` passes.

## Semantic boundary

- [ ] No observer/canonical/baseline/policy/verdict semantics changed; OR the PR links the governing issue/ADR, preregistered test and migration evidence.
- [ ] Incomplete/ambiguous evidence cannot silently become PASS.
- [ ] Backend identity alone does not create authority.
- [ ] Baseline acceptance is not silently converted into policy authorization.
- [ ] `observed behavior != all possible behavior` remains true.

## Architecture impact

Check all that apply: raw observation/backend; authority/completeness; canonical model/normalization; baseline/compatibility; diff; policy/verdict; provenance/attestation; privacy/security; distribution/developer experience; none.

If normalization, authority or comparability changes, include an adversarial counterexample/reproof.

## Distribution / DX

If this changes installation, release or GitHub Action behavior:

- [ ] Fresh-consumer path is tested.
- [ ] Immutable release/source identity is explicit.
- [ ] No long-lived publishing secret was added to source/logs.
- [ ] Stable-channel movement is gated by public-artifact proof.

## Reviewer decision

State the bounded conclusion supported by the evidence. Avoid universal correctness/security claims.
