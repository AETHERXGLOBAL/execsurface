# crates.io Publishing

ExecSurface uses GitHub Releases as its checksum-verifiable, provenance-attested binary distribution channel and crates.io as the Rust-native installation channel.

## Current public install

```bash
cargo install execsurface --version "=0.1.0-alpha.5" --locked
execsurface --version
execsurface doctor
```

Alpha.5 is a prerelease, so consumers should request the exact prerelease version explicitly.

## Verified publication order

The Alpha.5 publication chain publishes only after the immutable GitHub release and public-artifact checks succeed:

`immutable source/tag -> GitHub Release -> public binary proofs -> stable v0.1 proofs -> registry publication -> zero-contact registry install`

Canonical Alpha.5 release workflow run: `36910725515`.

## Publishable crate chain

Registry packages cannot depend on unpublished path-only workspace crates, so publishable runtime crates carry the same exact prerelease version and are published in dependency order:

1. `execsurface-model`
2. `execsurface-observe`
3. `execsurface-normalize`
4. `execsurface-baseline`
5. `execsurface-diff`
6. `execsurface-policy`
7. `execsurface-report`
8. `execsurface`

`execsurface-bench` is internal engineering support and remains `publish = false`.

## Consumer smoke path

```bash
execsurface --version
execsurface doctor
execsurface learn -- /bin/bash -lc true
execsurface check -- /bin/bash -lc true
```

## Publication security model

Publishing requires an authenticated crates.io token stored only in the protected GitHub Environment used for registry publication. A publishing token must never be pasted into an issue, chat, commit, log or documentation.

The release process verifies immutable release identity, Cargo metadata, package topology and source gates; publishes dependencies in order; treats already-visible exact versions idempotently; and proves a fresh registry install after publication.

## Permanence and trust boundary

crates.io versions are effectively permanent and cannot be overwritten. Registry publication proves package availability and bounded source/package identity under the release process. It does not prove that ExecSurface, or software observed by ExecSurface, is safe.
