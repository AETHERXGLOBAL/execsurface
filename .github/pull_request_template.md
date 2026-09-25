## Change

Describe the problem and the smallest change that addresses it.

## Evidence

- [ ] Tests added/updated where behavior changed
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --locked --workspace --all-targets -- -D warnings`
- [ ] `cargo test --locked --workspace --all-targets`
- [ ] Negative/failure evidence preserved when material

## Semantic boundary

Check all that apply:

- [ ] No observer/canonical/baseline/policy/verdict semantics changed
- [ ] If semantics changed, the PR links the required issue/ADR and migration evidence
- [ ] No new platform/security claim is made without evidence
- [ ] Privacy boundary remains metadata-only
- [ ] `observed behavior ≠ all possible behavior` remains true

## Distribution / DX

If this changes installation, release or Action behavior:

- [ ] fresh-consumer path is tested
- [ ] wrapper/baseline comparability is explicit
- [ ] no long-lived publishing secret was added
