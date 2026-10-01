# Changelog

All notable public-product changes are recorded here. The detailed pre-Alpha.5 engineering chronology is retained at `docs/archive/CHANGELOG_PRE_ALPHA5.md`.

## Unreleased

- Independent P8 validation continues as a post-release evidence program.
- Historical one-shot release workflows are retired from the active GitHub Actions surface without deleting their source/history.

## v0.1.0-alpha.5 — 2026-10-01

### Product / semantics

- Strengthened semantics-preserving evidence handling; ambiguous or incomplete evidence remains non-PASS-eligible.
- Strengthened proposition-scoped backend-authority boundaries.
- Strengthened legitimate-variance handling while retaining anti-poisoning rules.
- Strengthened attestation, provenance and executable/artifact binding.
- Retained native ptrace as the bounded Linux x86_64 public reference observer.
- Did not promote ARM64 support or research-only BPF-LSM/hybrid backends.

### Release engineering

- Repaired deterministic release reproducibility after the first A7 attempt exposed a byte mismatch.
- Repaired Ubuntu 22.04 portability after the first A7 attempt exposed a GLIBC incompatibility.
- Proved deterministic dual builds, compatibility, checksums, provenance, public source/artifact equivalence, public binary consumption/tamper rejection, immutable tag/Action behavior, stable `v0.1` behavior and zero-contact registry installation.

Canonical release run: `36910725515`.

### Governance / evidence

- Retained failed and negative evidence rather than rewriting history.
- Owner explicitly waived P8 only as a pre-publication gate.
- Alpha.5 publication does not claim that P8 passed.
- P8 independent external validation remains open post-release through issues #114 and #118.

## Earlier public alphas

Detailed engineering history through Alpha.4 is retained intact in `docs/archive/CHANGELOG_PRE_ALPHA5.md` and release-specific documents under `docs/releases/`.
