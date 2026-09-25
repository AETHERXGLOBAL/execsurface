# M4 — Diff Engine Evidence

Date: 2026-09-25

Gate verdict: **ACCEPTED**

Branch: `milestone/m4-diff-engine`

Issue: #8

## Result

M4 adds deterministic, policy-free comparison between a verified M3 baseline and a fresh M2 canonical execution surface.

Implemented:

- `execsurface-diff` crate;
- diff schema version `1`;
- baseline digest verification before comparison;
- comparability gate;
- deterministic exact added/removed set difference;
- conservative changed pairing;
- target exit/signal metadata;
- `execsurface check`;
- text and JSON reports.

## Comparability contract

Comparison is rejected when these differ:

- platform OS;
- platform architecture;
- observer backend name;
- observer capability set;
- canonical schema version;
- normalization profile version;
- semantic-root label set;
- canonical command executable;
- command argument count.

Status: **PROVED BY IMPLEMENTATION + COMPUTATIONAL_EVIDENCE**

## Conservative changed pairing

M4 does not guess pairings.

A changed finding is emitted only when one semantic subject has:

- exactly one unmatched baseline effect; and
- exactly one unmatched candidate effect.

If either side contains multiple unmatched effects for one subject, they remain explicit `added` and `removed` findings.

The adversarial fixture with two old and two new ports for one actor/IP passes and emits no fabricated `changed` pair.

Status: **COMPUTATIONAL_EVIDENCE**

## Real CLI evidence

Successful branch CI:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36176591920

### Real no-drift path

`real_learn_then_check_same_command_has_no_drift`

- learns a real baseline through the ptrace observer;
- reruns the same command;
- emits JSON diff;
- asserts added/removed/changed are empty.

Result: **PASS**

### Real controlled drift

`real_check_reports_controlled_process_drift_without_policy_failure`

- baseline: controlled shell behavior;
- candidate: introduces an additional executable effect;
- report contains drift;
- M4 command still exits successfully because M4 is evidence, not policy.

Result: **PASS**

### Privacy

`argv_secret_is_absent_from_json_diff`

A sentinel supplied only as argv is absent from stdout and stderr diff output.

Result: **PASS**

### Integrity

`corrupted_baseline_is_rejected`

A baseline with a corrupted digest is rejected.

Result: **PASS**

## Diff-library adversarial/unit evidence

8 tests PASS:

- identical surfaces have no drift;
- exact set difference is added/removed;
- unique file-access subject becomes changed;
- ambiguous network pairing stays added/removed;
- unique remote-port change becomes changed;
- normalization-profile mismatch is incomparable;
- observer-capability mismatch is incomparable;
- corrupted baseline is rejected.

## Full workspace evidence

**42 tests PASS, 0 failed**

CI gates:

- `cargo fmt --all -- --check` — PASS
- `cargo clippy --locked --workspace --all-targets -- -D warnings` — PASS
- `cargo test --locked --workspace --all-targets` — PASS
- Cargo.lock integrity — PASS

## Preserved negative evidence

Initial M4 implementation CI:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36176528253

Failure: rustfmt only. Clippy/tests were not reached.

Status: **KILLED / FIXED**

No failed run was hidden or rewritten.

## M4 non-claims

M4 does not assign:

- severity;
- allow/deny semantics;
- PASS / REVIEW / BLOCK;
- SARIF meaning;
- CI enforcement policy.

M4 also does not claim all semantically related runtime effects can always be paired as changed. Ambiguous cases intentionally remain added/removed.

## Gate decision

**M4 ACCEPTED.**

M5 — Policy / Verdict is authorized next, but is not part of this gate.
