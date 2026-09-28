# ExecSurface — Semantics v3 S4 Proof-Carrying Prototype Result

Date: 2026-09-29
Tracking: #101
Branch: `development/post-alpha4-behavioral-integrity`
Status: **S4 CLOSED / PASS — RESEARCH-ONLY PROTOTYPE**

## Result

An isolated Semantics v3 prototype now exists in `execsurface-model` and is deliberately disconnected from public alpha.4 `learn`, `check`, baseline, diff, policy and release paths.

The prototype proves the minimum executable properties required to continue P2 design:

1. proposition-scoped evidence is representable independently of raw event names;
2. authority is represented as explicit guarantee sets rather than a scalar backend score;
3. completeness is typed by dimension;
4. proof requirements fail closed when required guarantees/completeness are absent;
5. weak pathname-argument evidence does not satisfy a kernel-object-grounding requirement;
6. an explicit ambiguity does not satisfy required completeness;
7. the same canonical behavioral value with different authority metadata is different evidence;
8. ordered maps/sets provide deterministic serialization for the tested proof profile.

## Prototype code

- `crates/execsurface-model/src/semantics_v3.rs`
- `crates/execsurface-model/src/lib.rs` exposes the research module only.
- `crates/execsurface-model/Cargo.toml` adds `serde_json` as a dev dependency for deterministic serialization tests.
- `Cargo.lock` was updated accordingly.

No public raw/canonical/baseline schema version was changed.

## CI evidence

Dedicated isolated workflow:

`.github/workflows/semantics-v3-prototype-ci.yml`

### Run 1 — retained failure

Run: `36491869554`

Result: failure at `cargo fmt --all -- --check`.

Classification: formatting-only implementation failure. Clippy/tests were not run. The exact rustfmt diff was retained and fixed without semantic changes.

### Run 2 — retained failure

Run: `36492062219`

Result:
- rustfmt PASS;
- clippy did not execute because `cargo --locked` rejected an out-of-date `Cargo.lock` after the prototype test dependency was added.

Classification: dependency-lock discipline correctly blocked the run. `Cargo.lock` was updated; no dependency version/threshold shortcut was used.

### Run 3 — accepted

Run: `36492226283`
Source: `23a1a03453ca95f91fd8f627ffc9d6197f2cba35`

Result:
- Rust formatting: PASS;
- model Clippy with `-D warnings`: PASS;
- model tests: PASS.

The workflow was also corrected to watch `Cargo.lock`, because locked model CI depends on it.

## Authority boundary

S4 does **not** authorize:

- v3 public schema adoption;
- v3 baselines;
- v3 public `learn/check`;
- any alpha.4 change;
- weaker alpha.4 completeness semantics;
- ptrace/eBPF/BPF-LSM equivalence;
- eBPF public authority;
- backend auto-selection.

## Decision

`S4_PROOF_CARRYING_PROTOTYPE_PASS`

The model is executable and sufficiently precise to proceed to S5 — Shared-FD Exactness Experiment.

S5 must test whether retained clone/clone3 flags can distinguish fd-table relationships without reintroducing the known false-completeness class. Current alpha.4 `shared_fd_table_ambiguity` remains authoritative until S5 and later integration/red-team gates prove a replacement.