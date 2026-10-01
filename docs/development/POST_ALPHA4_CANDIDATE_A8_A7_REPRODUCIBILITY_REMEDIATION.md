# Post-alpha.4 Candidate A8-A7 — Reproducibility Remediation Record

Status: **ACTIVE REMEDIATION — FROZEN BYTE-IDENTITY CRITERION UNCHANGED**

Candidate branch: `release/post-alpha4-candidate-assembly`

## Retained counterevidence

Initial A8-A7 run: `36889499922`

Source: `b006f6eab69a4c2629ba9f78d924eaf9e22a068b`

The two exact source snapshots were byte-identical and had the same committed `Cargo.lock`, but their independently built release binaries differed:

- snapshot A SHA-256: `61514804db7ee13eca50f29f5c9b5eb382993392ae3787281ddf042e977b6367`
- snapshot B SHA-256: `d265902692b152b03b0cccc31768f3537a0b05d5072534ae5c48775b2121d05a`

Classification: **REPRODUCIBILITY FAILURE / RELEASE BLOCKER**.

This is retained as counterevidence. It is not reclassified as PASS and the preregistered byte-identical criterion is not weakened.

## Remediation hypothesis

The two builds compile the same Rust source from different absolute snapshot directories (`.../a/...` versus `.../b/...`). Rust artifacts may retain source-path material even in optimized release builds. If absolute workspace paths are the differentiator, using the stable compiler path-remapping facility to map each snapshot root to the same canonical virtual source root should remove that environmental distinction without changing program semantics or source inputs.

This is a remediation hypothesis, not a post-hoc success claim.

## Frozen remediation

For both independent snapshots:

- keep Rust `1.90.0`;
- keep the exact committed `Cargo.lock` and `--locked`;
- keep `CARGO_INCREMENTAL=0`;
- set the same `SOURCE_DATE_EPOCH=0`;
- pass `--remap-path-prefix=<actual snapshot root>=/usr/src/execsurface` through `RUSTFLAGS`;
- keep every original A8-A7 byte-identity assertion for source archive, release binary, `.crate`, and deterministic bundle;
- record the deterministic build recipe in the emitted evidence.

## Kill rule remains unchanged

If binary, package, or deterministic bundle SHA-256 still differs, A8-A7 remains BLOCKED. No fallback to semantic equivalence, same-version equivalence, or a weaker reproducibility claim is allowed.

No public release, main merge, stable-tag movement, arm64 claim, or P8 closure is authorized by this remediation.
