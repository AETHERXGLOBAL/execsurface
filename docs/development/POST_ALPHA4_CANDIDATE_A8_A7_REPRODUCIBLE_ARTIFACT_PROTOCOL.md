# Post-alpha.4 Candidate A8-A7 — Reproducible Artifact Protocol

Status: **PREREGISTERED — NO PUBLIC RELEASE AUTHORIZED**

Candidate branch: `release/post-alpha4-candidate-assembly`

Precondition: the immediately preceding candidate source must have passed both the A8-A6 integrated destructive closeout and the independent final integrated destructive closeout.

## Question

Can the exact clean candidate source produce byte-identical release artifacts from two independent source snapshots under the same pinned toolchain, while preserving the frozen alpha.4 public boundary?

## Fixed roles

- Reproducible-Build Engineer — constructs independent source snapshots and deterministic artifact recipes.
- Supply-Chain / Provenance Reviewer — verifies source, lockfile, toolchain, package, and artifact binding.
- Developer Experience Reviewer — verifies the produced binary is usable and identifies itself consistently.
- Innovation Scientist — searches for hidden nondeterminism or artifact/source substitution paths; may add stricter checks only.
- Anti-Drift / Scientific Integrity Reviewer — prevents changing the reproducibility criterion after seeing a result and prevents release-claim inflation.
- Independent Falsifier — attempts to distinguish the two outputs or substitute a different source/package.
- Independent Critical-Milestone Reviewer — accepts only retained executable evidence from the frozen candidate SHA.

## Frozen inputs

- exact candidate SHA supplied by GitHub Actions checkout;
- Rust toolchain `1.90.0`;
- committed `Cargo.lock` with `--locked`;
- Linux x86_64 hosted runner only;
- `v0.1.0-alpha.4` and `v0.1` must still resolve to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no main merge, tag movement, version bump, or public publication is authorized by this gate.

## Frozen experiment

1. Verify clean candidate lineage and immutable public tags.
2. Produce two independent source snapshots from the exact same Git tree using `git archive` into different directories.
3. Verify each snapshot has the same `Cargo.lock` digest and candidate source-tree identity recorded by the workflow.
4. Build the release `execsurface` binary independently in each snapshot with Rust 1.90.0 and `--locked`.
5. Require byte-identical binary SHA-256 digests.
6. Run `cargo package --locked -p execsurface --allow-dirty` independently in both snapshots and require byte-identical `.crate` package SHA-256 digests.
7. Verify both binaries report the same version and pass `--help`/`doctor` smoke checks.
8. Create a deterministic candidate bundle in each snapshot from the release binary plus a frozen manifest using sorted entries, numeric owner/group, and fixed epoch mtime; require byte-identical bundle SHA-256 digests.
9. Retain both hashes, candidate SHA, Cargo.lock hash, toolchain version, package hashes, and deterministic bundle hash in a machine-readable evidence file.

## Kill criteria

The gate is BLOCKED if:

- either build/package/smoke path fails;
- binary SHA-256 differs across the two independent snapshots;
- `.crate` package SHA-256 differs;
- deterministic bundle SHA-256 differs;
- the two snapshots do not derive from the exact same candidate tree;
- `Cargo.lock` differs or a build succeeds only without `--locked`;
- alpha.4 or `v0.1` moved;
- a mismatch is hidden, normalized away after observation, or the criterion is weakened after seeing it;
- the gate is used to claim P8 closure, arm64 support, production universality, or public-release authorization.

If nondeterminism is observed, retain it as counterevidence and investigate the cause; do not replace byte-identical reproducibility with a weaker criterion post hoc.

## Allowed decision

Only if every frozen check passes:

`POST_ALPHA4_CANDIDATE_A8_A7_REPRODUCIBLE_ARTIFACT_PASS_BOUNDED_INTERNAL`

This decision proves bounded same-environment reproducibility for the tested x86_64 Linux build recipe. It does **not** prove cross-platform reproducibility and does not authorize public release, main merge, stable-tag movement, arm64 support, or P8 closure.
