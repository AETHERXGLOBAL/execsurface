# M6.6.1 — Distribution & Self-Service Architecture

Date: 2026-09-26

Status: **FROZEN FOR IMPLEMENTATION**

Issue: #17

## Goal

Make Linux x86_64 ExecSurface usable without cloning the source repository while preserving all M6.5 evidence, policy and security semantics.

## Product release version

First public alpha:

- Cargo/workspace package version: `0.1.0-alpha.1`
- immutable Git tag: `v0.1.0-alpha.1`
- moving minor channel for the GitHub Action: `v0.1`

Product release versioning is independent from evidence schema versioning.

M6.6 does **not** change:

- raw observation schema v2;
- canonical schema/profile v2;
- baseline/digest v2;
- diff/verdict v2;
- policy v2.

## Primary installation channel

**GitHub Release binary — Linux x86_64 only.**

Initial target:

`x86_64-unknown-linux-gnu`

Release bundle:

`execsurface-v0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz`

Contents:

- `execsurface`
- `LICENSE`
- `NOTICE`
- `BUILD_INFO.txt`

Release also publishes:

- matching `.sha256`;
- release notes;
- GitHub build provenance attestation.

No macOS or Windows artifact/claim is authorized.

## Provenance

GitHub's current documentation recommends `actions/attest@v4` for new artifact-attestation workflows.

The release workflow will grant only:

- `contents: write` for the GitHub Release/tag operations;
- `id-token: write`;
- `attestations: write`.

No long-lived publishing secret is required.

Attestation proves build provenance. It does **not** prove that the binary is safe.

## Cargo fallback

A fresh-runner gate must prove:

`cargo install --git https://github.com/AETHERXGLOBAL/execsurface.git --tag v0.1.0-alpha.1 execsurface-cli --locked`

Before the tag exists, branch CI proves the equivalent immutable-revision install using `--rev $GITHUB_SHA`.

The README will not publish the fallback until this gate is green.

## crates.io decision

**DEFERRED pending packaging evidence.**

The workspace currently uses internal path dependencies without registry versions.

Cargo packaging rules require path dependencies to carry a version for publishable packages.

M6.6 will not distort the workspace or publish a chain of internal crates merely to obtain a crates.io name.

GitHub Releases + the GitHub Action are sufficient for the first public alpha if zero-contact proofs pass.

## GitHub Action channel

The public README must not recommend `@main`.

Promotion order:

1. merge a release-gated commit to `main`;
2. create immutable `v0.1.0-alpha.1`;
3. run the release workflow at that exact tag;
4. verify release binary/checksum/provenance and zero-contact consumer behavior;
5. promote `v0.1` only to that already-gated commit;
6. run a consumer-style `AETHERXGLOBAL/execsurface@v0.1` proof.

The moving `v0.1` ref must never point at an un-gated commit.

## Release orchestration

GitHub suppresses most recursive workflows caused by writes made with `GITHUB_TOKEN`.

Therefore a promotion workflow may create the immutable tag using the repository token, then explicitly dispatch the release workflow at that tag using `workflow_dispatch`.

This avoids personal access tokens and preserves a narrow native-token trust boundary.

## Action dependency pins reviewed for M6.6

Verified current releases on 2026-09-26:

- `actions/checkout v7.0.1` -> `3d3c42e5aac5ba805825da76410c181273ba90b1`
- `actions/upload-artifact v7.0.1` -> `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`
- `actions/attest v4.2.2` -> `1e69f48acb82d1966a394da916b4c1698aa569d6`

Generated public workflows pin checkout to the reviewed commit SHA.

## Doctor

`execsurface doctor` is diagnostic only.

It:

- checks Linux;
- checks x86_64;
- performs a real child ptrace observation of `/bin/true`;
- checks current workspace writeability;
- reports product version;
- gives actionable failure text.

It never changes privileges, sysctls or host security settings.

## Init

`execsurface init --command "..." --github-actions`:

- creates a starter policy;
- optionally creates a GitHub Actions workflow;
- prints the exact explicit learn/check commands;
- does not execute the supplied command;
- refuses overwrite unless `--force` is explicit.

The generated local learn command uses the same `/bin/bash -lc` wrapper as the Action.

## Non-goals

M6.6 does not change observer, canonical, baseline, diff, policy or verdict semantics.
