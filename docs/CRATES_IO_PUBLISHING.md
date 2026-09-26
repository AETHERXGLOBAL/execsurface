# crates.io Publishing

ExecSurface uses GitHub Releases as its signed binary distribution channel and crates.io as the primary Rust-native installation channel.

## Public install

The verified public install path is:

```bash
cargo install execsurface --locked
```

The installed binary is:

```text
execsurface
```

The first verified registry release is `0.1.0-alpha.2`.

A fresh Ubuntu 24.04 runner successfully installed the package from crates.io and then passed:

```text
execsurface --version
execsurface doctor
execsurface learn -- /bin/bash -lc true
execsurface check -- /bin/bash -lc true
```

The final no-drift check returned `PASS` with zero findings.

## Why several crates are published

The CLI is intentionally composed from independently tested workspace crates. Cargo registry packages cannot depend on unpublished path-only workspace crates, so the publishable runtime crates carry the same exact prerelease version and are published in dependency order.

The benchmark probe is internal engineering support and is explicitly `publish = false`.

Publish order:

1. `execsurface-model`
2. `execsurface-observe`
3. `execsurface-normalize`
4. `execsurface-baseline`
5. `execsurface-diff`
6. `execsurface-policy`
7. `execsurface-report`
8. `execsurface`

## Publication security model

Publishing requires an authenticated crates.io token stored only in the GitHub Environment named `crates-io` as `CARGO_REGISTRY_TOKEN`.

The token must never be pasted into an issue, chat, commit, log or documentation.

The normal release workflow:

1. checks out the requested immutable release tag;
2. verifies tag identity against Cargo metadata and `action/release-tag.txt`;
3. runs source/package gates;
4. publishes the workspace crates in dependency order;
5. skips versions already present in crates.io so interrupted chains can resume safely;
6. proves a fresh `cargo install execsurface` consumer from the registry.

## Preserved first-publication failures

The initial `0.1.0-alpha.2` publication retained two operational failures rather than hiding them:

- the first authenticated upload was blocked because the crates.io account email was not yet verified;
- after several new crate names were published, crates.io enforced its new-crate rate limit.

The recovery workflow was idempotent, kept the immutable release source fixed, skipped packages that were already present, resumed after the server-provided rate-limit window, and completed the remaining publications.

## Permanence

crates.io versions are effectively permanent and cannot be overwritten. A release must pass source, package and identity gates before publication.

## Trust boundary

Registry publication proves package availability and source/package identity under the release process. It does not prove that ExecSurface or a traced program is safe.
