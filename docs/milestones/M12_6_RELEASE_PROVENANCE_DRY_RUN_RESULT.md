# M12.6 — Release Artifact / Provenance Dry Run — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`

## Final classification

**`M12_6_RELEASE_DRY_RUN_PASS_NO_PUBLIC_MUTATION`**

M12.6 is closed for the frozen portable source candidate:

`16ff98b205f951cf463b6ae5f811f164f4ba4572`

The prospective release identity exercised by the dry run was:

- version: `0.1.0-alpha.4`
- tag: `v0.1.0-alpha.4`
- target: `x86_64-unknown-linux-gnu`

No public release, tag, crates.io publication, stable Action movement, Marketplace mutation, or `main` mutation was performed.

## Team

Fixed roles:

- **Innovative Systems Architect** — designed an isolated staged-next-alpha rehearsal that exercised the actual source/package/binary/provenance/consumer boundary without mutating the public line.
- **Anti-Deviation / Skeptical Reviewer** — required failure-first retention, blocked false full-chain crates.io dry-run claims, and required artifact-level verification rather than relying on a green workflow badge.

Dynamic specialists used for this gate:

- Rust workspace/version/package engineering;
- crates.io sequential publication semantics;
- GitHub Actions release engineering;
- supply-chain provenance/integrity;
- Linux x86_64 binary distribution;
- release-note/claim-boundary review;
- clean-consumer smoke testing.

## Source and public-line freeze

Frozen public `main`:

`db11761e2d75ebca7d4458dc39094f4aed6a26bb`

Frozen release-code candidate:

`16ff98b205f951cf463b6ae5f811f164f4ba4572`

Successful workflow-driver commit:

`b174500b22ae2c3262af55c07dc50f45ac0f7c26`

Successful workflow run:

`36465228937`

Successful job:

`109073515353`

Evidence artifact:

- artifact ID: `10989662166`
- artifact name: `m12-6-release-provenance-dry-run-36465228937-1`
- GitHub artifact digest: `sha256:719c7270cb91bc16028f47531e5c9904c884f92e7a2b8b1dce029be1b3cfa30e`
- independently downloaded ZIP SHA-256: `719c7270cb91bc16028f47531e5c9904c884f92e7a2b8b1dce029be1b3cfa30e`

The downloaded artifact digest matched GitHub exactly.

## Failure-first history preserved

### Failure 001 — downstream package registry visibility

Run `36464283655` failed after `execsurface-model` packaged successfully because `execsurface-observe` required prospective `execsurface-model = =0.1.0-alpha.4`, which was intentionally not present on crates.io.

This was classified as a real sequential-publication constraint, not product-code failure. The failure is preserved in:

`M12_6_FAILURE_001_DOWNSTREAM_PACKAGE_REGISTRY_VISIBILITY.md`

The repaired harness did **not** weaken this constraint. It instead:

- successfully created the real prospective `execsurface-model` package;
- successfully ran `cargo publish --dry-run` for that dependency-root crate;
- required the next dependent package to fail for the exact expected missing prospective registry version;
- rejected any different failure reason;
- audited package file sets for all publishable crates;
- validated complete prospective internal exact-version topology from Cargo metadata.

### Failure 002 — release-note literal sentinel

Run `36465008842` passed all substantive release/package/binary/provenance/consumer gates, then failed because a literal grep looked for `not kernel-object identity` while the draft said `not represented as kernel-object identity`.

The stronger authority boundary was retained and made additionally explicit. The failure is preserved in:

`M12_6_FAILURE_002_RELEASE_NOTE_SENTINEL_PHRASE.md`

The complete gate was rerun; the failed run was not reclassified as PASS.

## Successful gate evidence

### 1. Staged next-alpha identity coherence — PASS

In an isolated runner-local copy only:

- every publishable ExecSurface workspace package was staged as `0.1.0-alpha.4`;
- every internal publishable ExecSurface dependency requirement was exactly `=0.1.0-alpha.4`;
- staged `Cargo.lock` path-package versions were updated coherently;
- staged `action/release-tag.txt` was `v0.1.0-alpha.4`;
- release-note draft was staged as `docs/releases/v0.1.0-alpha.4.md`.

No staged version change was committed to the public source candidate.

### 2. Full source gates — PASS

The staged source passed:

- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets -- -D warnings`;
- `cargo test --locked --workspace --all-targets`.

This included the portable fail-closed shared-FD regression and M12 adversarial regression suite.

### 3. Packaging boundary — PASS WITH EXPLICIT SEQUENTIAL-REGISTRY CONSTRAINT

All eight publishable package file sets were enumerated with `cargo package --list` and screened against accidental `target/` or `.git/` inclusion.

The dependency-root package was built successfully:

`execsurface-model-0.1.0-alpha.4.crate`

Package SHA-256:

`8316518dc7993fec7de367b9773e718b1c783726005766f98ff99b0e95b77347`

`cargo publish -p execsurface-model --locked --dry-run --allow-dirty` reached the upload stage and then correctly aborted due to dry-run mode.

The next crate, `execsurface-observe`, was required to fail registry-backed package preparation with exit code `101` for the exact expected reason:

`execsurface-model = "=0.1.0-alpha.4"` is not yet visible in crates.io.

Therefore M12.6 makes **no false claim** that the full dependent crate chain can be registry-dry-run before its prospective dependency versions are sequentially published.

### 4. Release binary and bundle — PASS

Built optimized prospective binary:

`execsurface 0.1.0-alpha.4`

Release asset:

`execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu.tar.gz`

The archive SHA-256 check passed after download from the evidence artifact.

The bundle contains:

- `execsurface`;
- `LICENSE`;
- `NOTICE`;
- `BUILD_INFO.txt`.

`BUILD_INFO.txt` binds:

- product version/tag;
- target triple;
- frozen source candidate SHA;
- workflow-driver SHA;
- workflow run URL;
- evidence schema `m12.6-dry-run-v2`;
- staged `Cargo.lock` SHA-256;
- release-note SHA-256;
- `publication=dry-run-only`.

### 5. Release-workflow preparation preview — PASS

The dry run generated non-mutating prospective previews of:

- `.github/workflows/release.yml`;
- `.github/workflows/publish-crates.yml`.

The preview proved the current release plumbing has alpha-specific references that must be atomically updated in M12.8 before tag creation.

No live workflow file, release tag, or stable channel was mutated by this preview.

### 6. Provenance dry-run — PASS WITH BOUNDED AUTHORITY

Generated:

`provenance-dry-run.intoto.json`

The record is in-toto/SLSA-shaped and binds the archive subject, frozen source candidate, prospective version/tag, target, Cargo.lock digest, release-note digest, workflow identity, and dry-run status.

The provenance checksum validated successfully.

The provenance subject is the exact release archive with SHA-256:

`27ac72c944693f1a703f74fdf394f18d484a16b2cba4bddb36580052bb62e833`

Independent artifact inspection recomputed the archive digest and confirmed an exact match to the provenance subject.

Authority boundary remains explicit:

- `dryRun = true`
- `cryptographicGitHubAttestation = false`

This is **not** claimed to be a GitHub-hosted signed cryptographic attestation.

### 7. Clean consumer proof — PASS

From a clean extraction directory outside the build tree:

- archive checksum verification passed;
- extracted binary reported `execsurface 0.1.0-alpha.4`;
- `doctor` reported Linux/x86_64/ptrace/workspace readiness and the prospective version;
- `learn` succeeded;
- same-command `check` returned `ExecSurface: PASS`;
- controlled process drift returned `ExecSurface: REVIEW` with exit code `10`.

### 8. Claim-boundary review — PASS

The prospective release-note draft explicitly preserves:

- Linux x86_64 public scope;
- native ptrace as the public default/reference observer for this portable line;
- shared-FD hardening as conservative fail-closed treatment rather than exact attribution repair;
- pathname access-attempt metadata is not kernel-object identity;
- BPF-LSM/hybrid remains managed/research-only and non-default;
- no automatic hybrid selection;
- no ptrace↔hybrid baseline interchangeability;
- no production-readiness, universal-completeness, or universal performance claim;
- incomplete evidence cannot silently become PASS.

### 9. Public-line immutability — PASS

Artifact evidence `public-refs-before.txt` and `public-refs-after.txt` are byte-identical.

Verified:

- public `main` remained `db11761e2d75ebca7d4458dc39094f4aed6a26bb`;
- `v0.1.0-alpha.4` did not exist before or after the run;
- committed public/candidate release pointer remained `v0.1.0-alpha.3`;
- no release publication occurred;
- no crates.io upload occurred;
- no stable `v0.1` movement occurred.

## Anti-deviation judgment

The strongest attempted falsification found two harness/release-process details and preserved both as failures. Neither establishes a product-code blocker after repair and full rerun.

M12.6 does **not** authorize publication. It proves only that the frozen portable candidate can be staged into a coherent prospective next-alpha release artifact/provenance/consumer path without mutating the public line, subject to the real sequential crates.io publication constraint documented above.

## Decision

**M12.6 PASS.**

Authorized next gate only:

**M12.7 — Independent release red-team.**

Still forbidden until M12.8 closes:

- merge to `main`;
- creation/publication of `v0.1.0-alpha.4`;
- crates.io publication;
- movement of `v0.1`;
- Marketplace/public release mutation;
- hybrid default promotion;
- ptrace↔hybrid baseline equivalence.
