# A8-A7 reproducibility correction — retained first failure

Status: **CORRECTION TO BUILD RECIPE ONLY — ORIGINAL BYTE-IDENTITY CRITERION UNCHANGED**

Original failing run: `36889499922`
Original candidate SHA: `b006f6eab69a4c2629ba9f78d924eaf9e22a068b`

## Retained observation

Two byte-identical `git archive` source snapshots and identical committed `Cargo.lock` inputs built successfully with Rust 1.90.0 and `--locked`, but the two release binaries had different SHA-256 digests when built from different absolute extraction directories:

- snapshot A: `61514804db7ee13eca50f29f5c9b5eb382993392ae3787281ddf042e977b6367`
- snapshot B: `d265902692b152b03b0cccc31768f3537a0b05d5072534ae5c48775b2121d05a`

The frozen gate therefore failed as designed. This failure is retained permanently and is not reclassified as a PASS.

## Minimal correction

The retry keeps every original acceptance criterion, including byte-for-byte binary equality, package equality, deterministic bundle equality, exact source lineage, `--locked`, pinned Rust 1.90.0, smoke checks, immutable alpha.4/v0.1 tags, and all release-claim boundaries.

The only build-recipe correction is to canonicalize snapshot-local absolute source paths with Rust `--remap-path-prefix` and set `SOURCE_DATE_EPOCH` from the candidate commit timestamp. `CARGO_INCREMENTAL=0` remains required.

This does not normalize or alter a produced binary after the fact. Both binaries must independently compile to identical bytes. If they still differ, the gate remains blocked and the new failure must also be retained.

Passing after this correction establishes only bounded reproducibility under the corrected pinned recipe; it does not prove that absolute-path embedding was the sole possible source of nondeterminism.
