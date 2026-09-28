# M12.6 — Release Artifact / Provenance Dry Run — Protocol

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`
Status: **PREREGISTERED / NO PUBLICATION AUTHORIZED**

## Team

Fixed roles:

- **Innovative Systems Architect** — design the strongest release rehearsal that exercises the real artifact/metadata boundary without mutating the public release line.
- **Anti-Deviation / Skeptical Reviewer** — block any dry run that silently changes `main`, tags, Marketplace pointers, crate publication, release metadata, or that treats an unsigned local provenance record as a cryptographic GitHub attestation.

Dynamic specialists for this gate:

- Rust workspace/version/package engineering;
- GitHub Actions release engineering;
- supply-chain provenance and artifact integrity;
- crates.io dependency-chain packaging;
- Linux x86_64 binary distribution;
- release-note/claim-boundary review;
- clean-consumer smoke testing.

## Frozen source references

- public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- M12.5 closeout source candidate: `16ff98b205f951cf463b6ae5f811f164f4ba4572`
- current public release pointer: `v0.1.0-alpha.3`
- current stable Action channel: `AETHERXGLOBAL/execsurface@v0.1`
- prospective dry-run version only: `0.1.0-alpha.4`
- prospective dry-run tag only: `v0.1.0-alpha.4`

The prospective version/tag are rehearsal identities only. This protocol does not authorize a committed version bump, tag creation, GitHub Release, crates.io publication, or `v0.1` movement.

## Gate objective

Rehearse the next-alpha release boundary from the frozen portable candidate and attempt to invalidate it before M12.7/M12.8.

The dry run must prove that a staged next-alpha identity can produce internally coherent source/package/binary metadata, checksummed release artifacts, a machine-readable provenance statement, release notes, and a clean consumer install/smoke path without mutating the current public line.

## Failure-first record

Attempt 001 is preserved in `M12_6_FAILURE_001_DOWNSTREAM_PACKAGE_REGISTRY_VISIBILITY.md`.

It established that Cargo can create the prospective `execsurface-model` package, but cannot complete registry-backed packaging of `execsurface-observe` until `execsurface-model = =0.1.0-alpha.4` exists in crates.io. This is a real sequential-publication constraint, not a product-code failure.

The gate therefore does not pretend that all downstream crates can perform a full crates.io package/publish dry run before their exact prospective dependencies exist publicly.

## Mandatory assertions

### A. Public-line immutability

Before and after the rehearsal:

1. `origin/main` must still resolve to the frozen public SHA above;
2. the committed candidate `action/release-tag.txt` must remain `v0.1.0-alpha.3`;
3. `v0.1.0-alpha.4` must not be created by the workflow;
4. no GitHub Release is created/edited;
5. no crates.io publish is invoked;
6. no stable `v0.1` movement is invoked.

Any violation is a gate failure.

### B. Ephemeral staged version simulation

Inside a runner-local staging copy only:

1. workspace package version becomes `0.1.0-alpha.4`;
2. exact internal ExecSurface dependency pins become `=0.1.0-alpha.4`;
3. path-package entries in `Cargo.lock` are updated coherently without refreshing unrelated third-party dependency versions;
4. staged `action/release-tag.txt` becomes `v0.1.0-alpha.4`;
5. staged release notes are materialized as `docs/releases/v0.1.0-alpha.4.md` from the reviewed draft;
6. `cargo metadata --locked` must report the prospective version for every ExecSurface workspace package and no stale internal `0.1.0-alpha.3` exact dependency may remain.

### C. Source/package gates

The staged source must pass:

- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- `cargo metadata --locked`;
- exact prospective internal dependency-topology validation for all publishable crates;
- `cargo package --locked --allow-dirty --no-verify -p execsurface-model`;
- `cargo publish -p execsurface-model --locked --dry-run --allow-dirty` as the registry-front-of-chain dry-run sentinel;
- `cargo package --list --allow-dirty -p <crate>` for every publishable ExecSurface crate;
- an explicit negative sentinel proving `execsurface-observe` registry-backed package preparation fails only because prospective `execsurface-model = =0.1.0-alpha.4` is not yet visible in crates.io.

Any different downstream packaging failure is a gate failure.

Downstream crates are not represented as full crates.io publication dry runs before their prospective dependency versions exist in the registry. The real publish workflow remains responsible for sequential publication and registry visibility waits.

### D. Release binary and bundle

Build the staged Linux x86_64 release binary and require:

- `execsurface --version == execsurface 0.1.0-alpha.4`;
- bundle directory `execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu`;
- binary, `LICENSE`, `NOTICE`, and `BUILD_INFO.txt` included;
- `BUILD_INFO.txt` binds prospective version/tag, frozen source candidate SHA, workflow run, target, evidence schema, and `Cargo.lock` SHA-256;
- deterministic filename contract matching the existing public release workflow;
- archive SHA-256 generated and verified.

The gate does not require bit-for-bit reproducible archives because gzip/tar metadata is not currently normalized for reproducible-build identity. No reproducible-build claim is authorized.

### E. Provenance dry-run record

Generate a machine-readable in-toto/SLSA-shaped provenance statement whose subject is the exact release archive SHA-256 and whose predicate binds at minimum:

- frozen source candidate SHA;
- prospective version/tag;
- target triple;
- Cargo.lock digest;
- release-note digest;
- GitHub workflow/run identity;
- builder platform/toolchain evidence;
- statement that this is a **dry-run local provenance record, not a GitHub-hosted cryptographic attestation**.

Generate and verify a SHA-256 for that provenance record. Do not call GitHub Release publication or a signing/attestation service from this gate.

### F. Clean consumer proof

From a clean extraction directory, not the build tree:

- verify archive checksum;
- extract the archive;
- verify version;
- run `doctor`;
- `learn` and same-command `check` must succeed;
- controlled drift must return REVIEW exit code `10` and the expected verdict text.

### G. Metadata/claim review

Release-note draft must explicitly preserve:

- Linux x86_64 public scope;
- ptrace as the public default/reference path;
- shared-FD change as conservative fail-closed hardening, not exact attribution repair;
- pathname access-attempt metadata is not kernel-object identity;
- hybrid/BPF-LSM remains non-default/research-managed and not baseline-equivalent;
- no production-readiness or universal-completeness claim.

## Evidence package

The workflow must upload an evidence artifact containing at least:

- exact source and public-main SHAs;
- environment/toolchain records;
- staged cargo metadata;
- staged Cargo.lock and its digest;
- publishable-crate package file-set listings;
- actual `execsurface-model` package archive and digest;
- root-crate publish dry-run log;
- expected downstream registry-visibility failure log;
- release binary digest;
- release archive and `.sha256`;
- `BUILD_INFO.txt`;
- release notes and digest;
- provenance JSON and its digest;
- clean-consumer smoke log;
- before/after public-line/tag evidence.

## Gate classifications

Only one classification may close M12.6:

- `M12_6_RELEASE_DRY_RUN_PASS_NO_PUBLIC_MUTATION`
- `M12_6_RELEASE_DRY_RUN_INCOMPLETE`
- `M12_6_RELEASE_DRY_RUN_BLOCKED`

A workflow status of success by itself is insufficient. The result document must inspect the evidence artifact and preserve any failed attempts.

## Next gate

Only after a PASS classification: **M12.7 — Independent release red-team**.
