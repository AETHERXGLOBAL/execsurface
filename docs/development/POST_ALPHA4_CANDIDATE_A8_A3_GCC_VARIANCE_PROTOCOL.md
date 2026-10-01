# ExecSurface — A8-A3 Bounded GCC Ephemeral Variance Protocol

Date: 2026-10-01
Branch: `release/post-alpha4-candidate-assembly`
Immutable public base: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Accepted A8-A1 source: `1c65327e99aa62a3b42549764bebe6b5b29516e7`
Accepted A8-A2 source: `4166ff64b8110e27b47954c1f0875cce81ebf898`
P3 promotion evidence source: `96ef1e2f6f316460cc88af4f89403455a910b2b4`
P3 accepted run: `36861859874`
Status: **PREREGISTERED — NO A8-A3 PRODUCT IMPLEMENTATION ACCEPTED YET**

## Question

Can the exact bounded GCC ephemeral-identity projection reduce identity-only false REVIEW in the clean product candidate while preserving all meaningful drift and without turning repetition, similarity, filenames, or frequency into authorization?

## Hypothesis

A derived, opt-in normalization projection can collapse only GCC-owned `$TMP/ccXXXXXX.s` create/write → delete lifecycle pairs when actor, execution-chain tail, temp root, grammar, and role all match. It must leave the raw canonical surface unchanged and must fail closed on conflicts.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — extract only the smallest reusable derived projection and preserve raw evidence as a separate artifact/view.
2. **Anti-Drift / Scientific Integrity Reviewer** — forbid learned acceptance, frequency authorization, broad temp/cache suppression, unrelated normalization changes, or default-profile changes.
3. **Independent Falsifier / Red Team** — attack wrong actor/root/grammar/role, collisions, rename involvement, repeated observation, order variation, and unrelated Go-build normalization.
4. **Independent Critical-Milestone Reviewer** — verify exact ancestry, frozen A1/A2 regressions, public-default invariants, negative evidence retention, and release/P8 boundaries.

Dynamic specialists: Rust normalization/API, compiler toolchain behavior, canonical identity semantics, reproducibility/CI.

## Product delta freeze

A8-A3 may add or modify only:
- `crates/execsurface-normalize/Cargo.toml` — add default-off `gcc-ephemeral-v4` feature only;
- `crates/execsurface-normalize/src/lib.rs` — feature-gated module export only;
- `crates/execsurface-normalize/src/gcc_ephemeral_v4.rs` — bounded derived projection;
- `crates/execsurface-normalize/tests/gcc_ephemeral_v4_candidate.rs` — frozen adversarial corpus;
- `.github/workflows/post-alpha4-candidate-a8-a3-gcc-variance.yml` — gate;
- `docs/development/POST_ALPHA4_CANDIDATE_ASSEMBLY_MANIFEST.md` — cumulative ledger.

Research experiment crates/workflows are evidence-only and MUST NOT be copied into the candidate.

## Frozen product semantics

- feature is default-off;
- existing public normalization profile/default canonicalization remains unchanged;
- candidate derived profile version is 4 only when the explicit projection function is invoked;
- exact canonical target grammar: `$TMP/cc` + 6 ASCII alphanumeric characters + `.s`;
- exact actor path/family: `/usr/bin/gcc`, family `gcc`;
- actor must equal execution-chain tail;
- eligible role requires both create+write open and delete for the same identity;
- any conflicting role/actor or rename involving the identity blocks projection;
- projected target is `$TMP/cc<gcc-ephemeral>.s`;
- input/raw `CanonicalSurface` is never mutated;
- derived effects are sorted/deduplicated deterministically;
- repetition/frequency cannot create eligibility.

## Frozen 12-attack corpus

1. bounded benign pair reduces identity-only false REVIEW;
2. meaningful non-ephemeral drift survives projection;
3. wrong actor never projects;
4. missing create/delete role never projects;
5. conflicting actor collision fails closed;
6. rename collision fails closed;
7. wrong root/grammar near-misses remain distinct;
8. 256-fold repetition cannot authorize missing lifecycle role;
9. 256-fold wrong-actor repetition never authorizes;
10. raw evidence remains byte/structurally unchanged as a distinct view;
11. projection deterministic under effect-order variation;
12. existing Go-build ephemeral normalization remains distinct/untouched.

No assertion, grammar, role, count, or eligibility rule may be weakened after execution.

## Required regressions

- A8-A1 slice guard PASS;
- A8-A2 authority guard PASS;
- default workspace regression PASS;
- default normalize package tests PASS;
- rustfmt PASS;
- Clippy `-D warnings` PASS default and feature-enabled;
- `gcc-ephemeral-v4` default feature state is off;
- public normalization version/default canonicalize behavior remains alpha.4-compatible;
- public alpha.4/stable tags remain immutable.

## Kill criteria

Block A8-A3 if any test shows frequency authorization, wrong actor/root/grammar/lifecycle projection, raw-evidence mutation, meaningful drift erasure, non-deterministic output, unrelated Go-build collapse, or default behavior/version change.

## Allowed decision

Only a complete accepted run may produce:

`POST_ALPHA4_PROMOTION_A8_A3_GCC_VARIANCE_ELIGIBLE_BOUNDED_CLEAN_CANDIDATE`

No release, `main`, tag movement, generalized variance claim, arm64 claim, or P8 closure is authorized.
