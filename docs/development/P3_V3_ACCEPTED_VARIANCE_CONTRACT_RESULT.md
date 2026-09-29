# ExecSurface — P3 V3 Accepted-Variance Contract Result

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`

## Formal decision

`P3_V3_ACCEPTED_VARIANCE_CONTRACT_PASS_RESEARCH_ONLY`

This closes the V3 research prototype gate only. It does not change public `v0.1.0-alpha.4`, baseline/check semantics, normalization, policy or stable `@v0.1`.

## Protocol

`docs/development/P3_V3_ACCEPTED_VARIANCE_CONTRACT_PROTOCOL.md`

Protocol commit:
`dd7711bde90ad7261e577b8499d3490dff4d4332`

## Accepted source and CI

Accepted source:
`38a103d90dc1dd51583fd2c29bccf1b252addcfe`

Accepted CI run:
`36558724890`

Accepted job:
`109374114300`

Environment:
GitHub-hosted Ubuntu 24.04.5.

Accepted steps:
- isolated dependency lock generation — PASS;
- `cargo fmt ... --check` — PASS;
- `cargo clippy --locked ... -D warnings` — PASS;
- `cargo test --locked ...` — PASS;
- unit tests — **9 passed / 0 failed**.

## Contract properties established in the declared scope

The research contract keeps these categories distinct:
- invariant effects;
- observed variable candidates;
- explicitly accepted variable effects;
- unseen effects.

It binds output to:
- exact learning-set digest;
- comparable V1 profile;
- deterministic invariant-core digest;
- every variable candidate's support count;
- every variable candidate's exact source evidence digests;
- an explicit accepted subset only.

The contract contains no policy verdict.

## Falsification results

The executable suite established:
1. a rare 1/N candidate is not auto-accepted;
2. a frequent N-1/N candidate is not auto-accepted;
3. explicit exact candidate acceptance preserves support/provenance;
4. invariant-as-variance selection is rejected;
5. unseen effect selection is rejected;
6. duplicate acceptance selection is rejected;
7. learning-set digest mismatch is rejected;
8. unsupported selection schema is rejected;
9. selection-order permutation produces byte-identical contract output;
10. changing recurrence/support frequency without changing explicit selection still does not create acceptance.

The key safety result is therefore bounded but material: **frequency is data, not permission**.

## Preserved failure

Initial V3 run `36558607813` failed at rustfmt before clippy/tests. The failure remains part of history. Commit `38a103d90dc1dd51583fd2c29bccf1b252addcfe` applied only rustfmt-prescribed formatting changes; no contract rule, test, threshold or acceptance condition was weakened.

## Remaining limits

V3 does not prove that a human/automation acceptance decision itself is trustworthy. It also does not yet prove resistance to a malicious learning set, actor/causal-chain substitution, normalization mimicry or mixed-profile poisoning beyond the inherited V1/V2 checks.

Those are V4 obligations.

## Next gate

V4 — poisoning / consolidated falsification gate.

No V5 real-workload value claim is admissible until V4 closes without an unclassified false-PASS/authorization path in the declared suite.
