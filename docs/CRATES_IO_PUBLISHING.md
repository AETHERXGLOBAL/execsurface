# crates.io Publishing

ExecSurface uses GitHub Releases as its checksum-verified, provenance-attested binary distribution channel and crates.io as the Rust-native installation channel.

## Public install

```bash
cargo install execsurface --locked
```

The release workflow for `0.1.0-alpha.4` publishes to crates.io only after the immutable GitHub release, immutable Action consumer, stable `v0.1` promotion, and stable PASS / REVIEW / BLOCK / ERROR gates succeed.

The verified consumer path is:

```text
execsurface --version
execsurface doctor
execsurface learn -- /bin/bash -lc true
execsurface check -- /bin/bash -lc true
```

## Why several crates are published

The CLI is composed from independently tested workspace crates. Registry packages cannot depend on unpublished path-only workspace crates, so publishable runtime crates carry the same exact prerelease version and are published in dependency order:

1. `execsurface-model`
2. `execsurface-observe`
3. `execsurface-normalize`
4. `execsurface-baseline`
5. `execsurface-diff`
6. `execsurface-policy`
7. `execsurface-report`
8. `execsurface`

`execsurface-bench` is internal engineering support and `publish = false`.

## Publication security model

Publishing requires an authenticated crates.io token stored only in the GitHub Environment `crates-io` as `CARGO_REGISTRY_TOKEN`. The token must never be pasted into an issue, chat, commit, log or documentation.

The release workflow checks out the immutable release tag, verifies tag identity against Cargo metadata and `action/release-tag.txt`, runs source/package gates, publishes in dependency order, skips versions already present so interrupted chains can resume safely, and proves a fresh registry install.

## Sequential registry visibility

A prerelease dependency chain cannot truthfully be treated as fully registry-dry-runnable before the new dependency versions exist on crates.io. The publish workflow therefore publishes and verifies visibility in dependency order. This operational constraint is preserved from M12.6 rather than hidden.

## Permanence

crates.io versions are effectively permanent and cannot be overwritten. A release must pass source, package and identity gates before publication.

## Trust boundary

Registry publication proves package availability and source/package identity under the release process. It does not prove that ExecSurface or a traced program is safe.
