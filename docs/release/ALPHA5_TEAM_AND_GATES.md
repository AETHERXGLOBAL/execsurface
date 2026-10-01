# Alpha.5 Clean Candidate — Team and Gate Constitution

Candidate line: `release/v0.1.0-alpha.5-clean-candidate`

## Dynamic expert roles

- Formal Semantics / PL Lead — proposition binding, checked proof requirements, migration and fail-closed semantics.
- Runtime Evidence Authority Lead — backend/proposition authority, completeness, ambiguity and evidence provenance.
- Rust/API Invariants Engineer — public API integrity, feature-gated exposure, serialization and compatibility.
- Adversarial / Property Testing Lead — mutation, cross-proposition substitution, ambiguity injection, replay, poisoning and downgrade attacks.
- Release / Supply-Chain Engineer — packaging, installability, checksums, artifact reproducibility and GitHub Action compatibility.
- Developer UX Reviewer — zero-assistance install/use path, CLI errors, docs and default behavior.

Fixed roles:

- Innovation Scientist — must seek stronger general invariants instead of case-by-case patches.
- Anti-Drift / Scientific Integrity Reviewer — blocks scope inflation, test weakening, unsupported claims, evidence deletion and silent semantic reinterpretation.
- Independent Falsifier — attempts to break every promoted property with unchanged frozen tests plus new orthogonal attacks.
- Critical-Milestone Reviewer — blocks release unless every required gate has reproducible evidence.

## Non-negotiable rules

1. Alpha.4 and stable `@v0.1` remain untouched during candidate work.
2. No weakened assertions, thresholds, filters or acceptance rules.
3. Failed runs remain evidence.
4. Incomplete/ambiguous/lost/unsupported evidence cannot become PASS.
5. Backend name, frequency, similarity, signature, attestation or platform metadata cannot raise semantic authority by itself.
6. No silent v2→v3 reinterpretation.
7. Native arm64 negative evidence remains non-promoted.
8. P8 external validation cannot be self-certified by this release branch.

## Required execution sequence

A1 — repaired Semantics-v3 admission replay.
A2 — proposition/backend authority replay.
A3 — legitimate-variance anti-poisoning replay.
A4 — attestation/provenance binding replay.
A5 — developer usability/install/package/CLI/GitHub Action compatibility.
A6 — integrated destructive red-team replay including M11/M12 regressions.
A7 — reproducible artifact/checksum/provenance dry-run.
A8 — independent internal release review.
A9 — release decision only after all required evidence is complete.
