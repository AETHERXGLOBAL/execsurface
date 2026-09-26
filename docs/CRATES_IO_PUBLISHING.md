# crates.io Publishing

ExecSurface uses GitHub Releases as its primary signed binary distribution channel and crates.io as the Rust-native installation channel.

## Public install target

After the first registry publication succeeds, the intended user command is:

~~~bash
cargo install execsurface --locked
~~~

The installed binary remains:

~~~text
execsurface
~~~

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

## First-publication security model

Current Cargo/crates.io documentation requires an authenticated crates.io token for publication. The token is a secret and must never be pasted into an issue, chat, commit, log or documentation.

For the initial publication:

1. sign in to crates.io with the maintainer GitHub account;
2. verify the account email;
3. create a scoped API token suitable for publishing these crates;
4. create a GitHub Environment named `crates-io`;
5. add the token to that environment as the secret `CARGO_REGISTRY_TOKEN`;
6. dispatch `.github/workflows/publish-crates.yml` from the current default branch and set `release_tag` to the immutable version tag to publish;
7. after publication, rotate/revoke the initial token if moving to a stronger supported publisher mechanism.

The workflow checks out the requested immutable release tag and verifies that it exactly matches Cargo metadata and `action/release-tag.txt`. Publication is idempotent: versions already present on crates.io are skipped, so a rate-limited or interrupted dependency chain can be resumed safely.

## Permanence

crates.io versions are effectively permanent and cannot be overwritten. A release must pass source, package and identity gates before publication.

## Trust boundary

Registry publication proves package availability and source/package identity under the release process. It does not prove that ExecSurface or a traced program is safe.
