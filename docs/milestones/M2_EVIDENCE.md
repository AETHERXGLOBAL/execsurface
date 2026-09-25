# M2 — Canonicalization Evidence

Date: 2026-09-25

Gate verdict: **ACCEPTED**

Branch: `milestone/m2-canonicalization`

Issue: #4

## Result

M2 converts M1 raw observations into a deterministic canonical execution surface without introducing baseline, diff or policy semantics.

Implemented:

- `execsurface-normalize` Rust crate;
- canonical model in `execsurface-model::canonical`;
- canonical schema version: `1`;
- normalization profile version: `1`;
- explicit semantic roots:
  - `$WORKSPACE`
  - `$HOME`
  - `$TMP`
  - `$RUN_TMP`
  - `$CACHE:<name>`;
- longest-declared-root matching with path-component boundaries;
- rejection of conflicting physical-root semantics;
- rejection of semantic roots that shadow credential-sensitive namespaces;
- deterministic ordered-set effect representation;
- PID/TID/sequence exclusion from canonical identity;
- process executable-state replay across spawn/exec;
- deterministic duplicate collapse;
- destination IP/remote-port preservation;
- path class + resolution status;
- Linux open-intent preservation;
- incomplete-observation rejection.

## Primary invariance result

Synthetic adversarial fixture varies:

- PID/TID,
- raw sequence numbers,
- independent event interleaving,
- workspace physical root,
- home physical root,
- temp physical root,
- run-temp physical root,
- cache physical root.

Expected and observed result:

> exact equality of the canonical surface.

Status: **COMPUTATIONAL_EVIDENCE**

## Real observer → canonical proof

GitHub Actions run #16:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36172118508

The integration test:

`repeated_real_observation_produces_identical_canonical_surface`

runs `/bin/true` twice through the actual M1 ptrace observer, canonicalizes both observations independently, and asserts:

1. structural canonical-surface equality;
2. serialized JSON equality.

Result: **PASS**

This supplies real evidence that PID/trace-instance variance does not enter the canonical identity for this controlled command.

It does not prove all Linux workloads are deterministic.

## Adversarial normalization evidence

Run #16 passes 12 canonicalization unit/adversarial tests:

- `same_logical_behavior_survives_pid_sequence_root_and_interleaving_variance`
- `explicit_run_root_normalization_preserves_security_relevant_suffix`
- `credential_path_remains_specific_and_sensitive`
- `semantic_root_cannot_shadow_credential_namespace`
- `relative_and_parent_traversal_paths_are_not_guessed`
- `root_matching_respects_path_component_boundaries`
- `duplicate_raw_effects_collapse_deterministically`
- `remote_destination_port_remains_significant`
- `linux_open_access_mode_is_not_collapsed`
- `incomplete_observation_is_rejected`
- `duplicate_sequence_is_rejected`
- `same_physical_root_cannot_have_conflicting_semantics`

All: **PASS**

## Red-team findings

### Generic random-token heuristic

Proposal: infer random temp segments from names that resemble UUID/hash/PID/timestamp patterns.

Decision: **KILLED**

Reason: an attacker-controlled or semantically meaningful component can resemble randomness. Removing it can collapse distinct execution targets.

Replacement: explicitly declared `$RUN_TMP` root.

### Credential-root shadowing

Finding: a longest-root rule could have allowed a user-declared cache/workspace root nested under `$HOME/.aws`, `$HOME/.ssh` or `$HOME/.config/gcloud` to erase the credential namespace from path identity.

Decision: **KILLED / FIXED**

M2 now rejects semantic-root declarations that shadow these credential-sensitive namespaces.

### Post-hoc realpath

Proposal: resolve all observed paths with `realpath()` after execution.

Decision: **KILLED**

Reason: the filesystem may have changed and symlink state may differ from trace time. Relative paths and parent traversal are therefore represented as unresolved when M1 evidence is insufficient.

## Preserved failed CI

### Run #13

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36171831155

Failure: rustfmt gate.

Status: **KILLED / FIXED**

### Run #14

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36171888138

Result after formatting fix: **PASS**

### Run #15

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36172019569

Result after credential-shadow hardening: **PASS**

### Run #16

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36172118508

Result with real observer→canonical repeatability test: **PASS**

Checks:

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`
- `cargo test --locked --workspace --all-targets`
- Cargo.lock integrity

## What M2 does not claim

M2 does not establish:

- a baseline lockfile;
- content-addressed baseline digest;
- candidate-vs-baseline diff;
- policy verdict;
- fd-lifecycle-complete read/write attribution;
- trace completeness for all syscalls;
- symlink target truth;
- cross-platform open-flag equivalence;
- low overhead;
- program safety.

## Carry-forward OPEN items

- exact deterministic canonical JSON test vectors for content addressing: **OPEN before M3 digest gate**;
- baseline context/comparability contract: **OPEN M3**;
- fd-lifecycle read/write attribution: **OPEN**;
- richer symlink/dirfd evidence: **OPEN**;
- broader architecture support: **OPEN**;
- performance measurement: **OPEN**.

## Gate decision

**M2 ACCEPTED.**

M3 — Baseline Lock is authorized next, but is not part of this gate.
