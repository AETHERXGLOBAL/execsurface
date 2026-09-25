# M6.6 — Release Candidate Evidence

Date: 2026-09-26

Status: **RELEASE CANDIDATE — PRE-PROMOTION**

Issue: #17

Branch: `milestone/m6.6-public-self-service`

Candidate commit before release-request marker:

`a491b0eb6daf49d2754c9fffb55adb4ac911cdac`

## Product version decision

- product version: `0.1.0-alpha.1`
- immutable release tag: `v0.1.0-alpha.1`
- promoted Action channel after consumer proof: `v0.1`
- evidence schemas remain v2

Product SemVer is deliberately independent from evidence-schema versioning.

## Proven branch gates

### Core CI

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36193151532

Result: **PASS**

- format
- strict Clippy
- all workspace tests
- Cargo.lock integrity

M6.5 semantics remain green.

### M6 Action regression

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36193151190

Result: **PASS**

All six cases:
- PASS
- REVIEW
- REVIEW-as-failure
- BLOCK
- ERROR
- privacy sentinel

### Distribution probe

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36193151226

Result: **PASS**

#### Fresh cargo git install

No source checkout.

The runner installs the CLI from the immutable candidate commit using Cargo Git install and proves:

- product version;
- `doctor`;
- `learn`;
- no-drift `check`;
- controlled drift -> REVIEW.

#### Release bundle dry run

Builds the exact intended Linux x86_64 bundle shape:

- `execsurface`;
- `LICENSE`;
- `NOTICE`;
- `BUILD_INFO.txt`;
- compressed archive;
- SHA-256 checksum.

The gate verifies checksum, extracts into a fresh consumer directory, and proves version/doctor/learn/check.

#### crates.io feasibility

The current CLI package is intentionally **not** registry-ready because internal workspace path dependencies are not being given registry publication metadata merely for M6.6.

Decision: **DEFERRED**.

GitHub Release binaries + Git install fallback + GitHub Action are the public-alpha distribution channels.

## New self-service CLI

### doctor

`execsurface doctor` proves supported-environment readiness through:

- Linux check;
- x86_64 check;
- real child ptrace observation;
- workspace writeability;
- product version.

It never modifies privileges, ptrace sysctls or host security configuration.

### init

`execsurface init --command "..." --github-actions`:

- creates a conservative policy;
- optionally creates an Action workflow;
- prints exact wrapper-consistent learn/check commands;
- does not execute the target command;
- refuses overwrites unless `--force` is explicit.

## Public developer UX

Added/reworked:

- product-first README;
- five-minute start;
- troubleshooting;
- minimal Rust/Python/Node command examples;
- GitHub Action public guide;
- SUPPORT;
- CODE_OF_CONDUCT;
- PR template;
- bug template;
- Adoption / Integration template;
- repository metadata recommendations.

Python/Node examples are explicitly illustrative command recipes, not ecosystem-certification claims.

## Release security architecture

The release pipeline is designed to:

1. validate tag ↔ Cargo version;
2. run full Rust gates;
3. build/smoke the release binary;
4. create checksum;
5. generate GitHub build-provenance attestation;
6. publish the GitHub prerelease;
7. run release-binary zero-contact proof;
8. run Cargo immutable-tag proof;
9. run immutable Action proof;
10. only then promote `v0.1`;
11. run stable-channel PASS/REVIEW/BLOCK/ERROR proof.

Current reviewed action pins:

- checkout v7.0.1: `3d3c42e5aac5ba805825da76410c181273ba90b1`
- upload-artifact v7.0.1: `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`
- attest v4.2.2: `1e69f48acb82d1966a394da916b4c1698aa569d6`

No long-lived publishing credential is introduced.

## Preserved negative evidence

Early M6.6 iterations remain visible:

- CI #76 failed on a Rust quoting/formatting error;
- Distribution Probe #1 failed because the same invalid source could not compile;
- CI #77/#78 exposed remaining rustfmt/strict-Clippy issues;
- later fixes did not weaken semantics or test gates.

## Promotion boundary

This document does **not** declare M6.6 closed.

Closure requires the actual immutable GitHub Release, checksum/provenance verification, zero-contact public release proof, stable `@v0.1` proof, and final main evidence.
