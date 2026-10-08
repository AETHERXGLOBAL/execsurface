# Changelog

All notable public-product changes are recorded here. The detailed pre-Alpha.5 engineering chronology is retained at `docs/archive/CHANGELOG_PRE_ALPHA5.md`.

## Unreleased

- No unreleased public-product changes after the v1.0.0 stable release.

## v1.0.0 — 2026-10-08

### Stable contract

- Published the first stable ExecSurface release for the documented Linux x86_64 + native ptrace boundary.
- Froze PASS=0, ERROR=2, REVIEW=10, BLOCK=20 semantics and retained fail-closed incomplete-evidence behavior.
- Froze the target-outcome separation: ExecSurface PASS does not prove the wrapped target command succeeded.
- Preserved Alpha.6 profile-4 upgrade/rollback compatibility and explicit Alpha.5 profile-3 incompatibility.
- Preserved policy schemas v1/v2 and opt-in v3 matcher semantics.

### Release and distribution

- Published stable GitHub Release `v1.0.0` with checksum and GitHub provenance attestation.
- Promoted stable Action channel `AETHERXGLOBAL/execsurface@v1` only after immutable artifact and Action consumer proofs.
- Published the full crates.io dependency chain and proved exact zero-contact installation of `execsurface = 1.0.0`.
- Protected immutable `v1.*` release tags against update/deletion with no bypass actors.

Release workflow: `37771109825`.  
Registry publication workflow: `37771295572`.

Independent external validation is not claimed.

## v0.1.0-alpha.6 — 2026-10-08

### Semantic correctness

- Corrected stale file-descriptor identity after successful dup2/dup3-style replacement from untracked sources.
- Fail closed when FD I/O attribution cannot be established instead of silently omitting evidence under complete=true.
- Added bounded kernel-resolved object/path authority checks for side-effectful opens and hardlink ambiguity.
- Advanced normalization to profile 4; Alpha.5 profile-3 baselines remain immutable history and are explicitly incomparable rather than silently migrated.
- Hardened verdict materialization against workload-mutated outputs, rename lineage, create/truncate/ftruncate/delete final-state changes, symlink/hardlink aliases, non-regular outputs, and output-parent rebinding.

### Policy and custody

- Added externally anchored baseline semantic-digest and policy-byte custody checks.
- Added opt-in policy schema v3 matchers for open intent, path resolution and rename source while preserving v1/v2 meanings.
- Preserved fail-closed verdict precedence and explicit incompatibility behavior.

### Qualification

- Replayed R1-R8 plus the post-R8 destruction corpus on Ubuntu 22.04 and 24.04.
- Expanded post-R8 merge-authorization destruction to 17/17 passing cases after retained fail-first counterexamples.
- Required CI, adversarial regression, ptrace lifecycle, compatibility, packaging, public consumer, typed report/evidence and consumer-contract workflows all passed on the qualified source.

### Compatibility and history

- Alpha.5 remains available unchanged for rollback and historical reproduction.
- No Alpha.5 release artifact, tag, issue, PR, comment, review thread, or external-engagement record is rewritten by this release.
- The stable `v0.1` Action channel moves to Alpha.6 only after immutable Alpha.6 release and consumer gates succeed.
- Research scope remains runtime behavioral verification / execution semantics / semantic-evidence correctness, not cybersecurity research.

### Repository and documentation

- Independent P8 validation continues as a post-release evidence program.
- Historical one-shot release workflows are retired from the active GitHub Actions surface without deleting their source/history.
- Added a documentation index that separates current product guidance from historical engineering evidence.
- Consolidated current OpenSSF engagement state under `docs/external/` and removed the engagement-specific baseline from the repository root.
- Strengthened contribution and governance documentation around evidence preservation, release boundaries, and external-review claims.

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
