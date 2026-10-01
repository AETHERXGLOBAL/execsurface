# Alpha.5 A6 integrated destructive closeout

Decision: `ALPHA5_A6_INTEGRATED_DESTRUCTIVE_PASS_INTERNAL_ONLY`

Candidate branch: `release/v0.1.0-alpha.5-hardening`

## Evidence retained

### First A6 execution — retained failure
- Run: `36892462480`
- Source: `cb9ba58da68055646d622abb6805f25749115e2f`
- Result: FAIL.
- PASS within the failed run:
  - current frozen 16-invariant integrated corpus;
  - P4/P5 cross-layer authority and attestation attacks;
  - P3 legitimate-variance / poisoning falsifier;
  - M11/M12 PATH-TOCTOU and shared-FD adversarial replay on Ubuntu 22.04 and 24.04;
  - current developer distribution smoke;
  - retained-negative-evidence boundary.
- Failure 1: `semantics-metamorphic` and `full-workspace-regression` stopped at `cargo fmt --check` because the newly added Alpha.5 metamorphic test file required rustfmt line wrapping. Classification: `HARNESS/STATIC_FORMATTING`; no scientific assertion executed or weakened.
- Failure 2: current-candidate execution of frozen historical P6 test 12 rejected product-code changes after its historical A4 floor. Classification: `HARNESS/SCOPE_MISMATCH`; the frozen P6 test was designed to prove that the P6 tooling promotion itself did not modify product runtime code, not to forbid later product-candidate assembly.

### Static correction
- Commit: `1ffcd12b3acac7dd0817d6c97708155269597f8c`
- Scope: rustfmt only for `crates/execsurface-model/tests/alpha5_metamorphic.rs`.
- Metamorphic assertions unchanged.

### Scope-correction supplement
- Run: `36893292360`
- Source: `7d0352061f58227bd27dffc4ab87bd4612e0ea4f`
- Result: SUCCESS.
- Current frozen A7 integrated corpus: 16/16 PASS.
- Frozen P6 falsifier replayed unchanged at exact P6 closeout source `08a5a3775ef1618a50296c2b88f16bb93d332754`: 12/12 PASS.
- Frozen P7 falsifier replayed unchanged at exact P7 closeout source `8b1a5c603e808bbf6fa1aa38e6340a35bc2082ec`: 12/12 PASS.
- Repaired Semantics-v3 corpora: PASS.
- New Alpha.5 metamorphic/property attacks: 8/8 PASS.
- Full workspace rustfmt + Clippy `-D warnings`: PASS.
- Full workspace tests: PASS.
- Release workspace build and `cargo package`: PASS.

## Combined A6 judgment
The first failed run remains negative evidence and is not rewritten. Its successfully executed current-candidate attack families are combined with the scope-correction supplement, which reruns the failed static/metamorphic path and replays the frozen P6/P7 falsifiers at the exact historical scopes for which their product-scope assertions were defined.

No frozen falsifier assertion, threshold, corpus, or acceptance criterion was weakened or deleted.

## Boundaries still in force
- `v0.1.0-alpha.4` remains at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.
- stable `v0.1` remains at the alpha.4 source.
- Native arm64 negative evidence remains non-promoted.
- P8 external validation is not closed by this internal result.
- Public Alpha.5 release is NOT authorized by A6 alone.

Next gate: A7 reproducible artifact / checksum / exact-source verification.
