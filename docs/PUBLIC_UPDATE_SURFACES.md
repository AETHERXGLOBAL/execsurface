# ExecSurface — Public Update Surfaces

Status: **FROZEN INVENTORY — do not publish the hardening candidate before acceptance**

This file prevents the post-hardening release from updating the repository while leaving older public distribution or announcement surfaces behind.

## Confirmed product/distribution surfaces

1. **GitHub repository / `main`**
   - `AETHERXGLOBAL/execsurface`
   - README currently identifies Public Alpha `0.1.0-alpha.2`.

2. **GitHub Releases**
   - current public prerelease: `v0.1.0-alpha.2`
   - Linux x86_64 archive + SHA-256 + build provenance
   - prior prerelease: `v0.1.0-alpha.1`

3. **crates.io**
   - public package identity: `execsurface`
   - current documented install path: `cargo install execsurface --locked`
   - README currently identifies `0.1.0-alpha.2`.

4. **GitHub Action stable channel**
   - `AETHERXGLOBAL/execsurface@v0.1`
   - Action must continue to resolve to an accepted release artifact and checksum, never a diagnostic hardening branch.

5. **Immutable Git tag/source install path**
   - current documented tag: `v0.1.0-alpha.2`
   - Git source fallback is part of the public installation contract.

6. **GitHub Early Adopters / public adoption surface**
   - existing public early-adopter intake must be updated when the accepted release changes behavior or installation version.

7. **Technical Evaluation Pack / public evaluation documentation**
   - repository evaluation documentation and reproducible evidence must move to the accepted release only after the hardening gate closes.

## Confirmed announcement surface

8. **LinkedIn launch/update post**
   - ExecSurface was publicly announced with the GitHub repository, crates.io install path and stable GitHub Action.
   - A new update post should be written only after the accepted release is actually published; it must distinguish the new evidence from claims of production-grade universality or independent adoption.

## Post-acceptance update order

1. Merge the minimal proved product fix to `main`.
2. Run unchanged CI and historical semantic gates.
3. Rerun canonical M9.1 external evidence against the merged source.
4. Cut the next public prerelease from the verified merge commit.
5. Publish GitHub Release assets/checksum/provenance.
6. Publish the same accepted version to crates.io.
7. Move/update the stable `v0.1` Action channel only after artifact verification.
8. Update README, Quickstart, GitHub Action guide and Technical Evaluation Pack to the exact release version.
9. Update Early Adopters instructions.
10. Publish the LinkedIn update with evidence-linked wording.

## Non-surfaces / exclusions

- hardening branches are not release channels.
- diagnostic workflow artifacts are evidence, not public product releases.
- eBPF research remains excluded from the public default product path.
- zero-contact external reproduction must not be described as independent adoption.

## Release rule

No public channel above is to be updated merely because an experimental candidate passes a few runs. Publication begins only after the M9 hardening acceptance gate, Red Team/ablation, canonical external rerun, and merge to `main` are complete.
