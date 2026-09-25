# M7 Target Selection — sharkdp/fd

Date: 2026-09-26

Status: **PINNED FOR COMPATIBILITY PROOF**

Issue: #23

## External target

Repository:

`sharkdp/fd`

Pinned upstream commit:

`ce97e473ebaec49697c07daa50a7bc2b32f713d2`

Observed upstream package metadata at selection:

- package: `fd-find`
- version: `10.5.0`
- Rust edition: 2024
- declared Rust version: `1.90.0`
- Cargo.lock: present
- upstream Linux CI: present
- upstream locked test command includes `cargo test --locked`

The pin is immutable for this M7 evidence round. Later upstream commits are not silently substituted.

## Why this target

`fd` is a real developer-facing Rust CLI rather than a synthetic fixture.

It is useful for M7 because:

1. the upstream repository is mature and actively maintained;
2. Linux execution is a real supported upstream environment;
3. the repository carries a lockfile and real CI;
4. it can be reproduced without requiring proprietary services;
5. the built tool can be exercised on a deterministic local fixture;
6. process/file runtime expansion can be introduced locally without changing upstream history.

## Proof structure

The M7 workflow will:

1. checkout the exact upstream commit with credentials disabled;
2. verify the checkout SHA;
3. reproduce upstream tests with `cargo test --locked --all-features`;
4. build the real release binary;
5. install ExecSurface from the published public-alpha release;
6. verify its release checksum and GitHub attestation;
7. create an untracked local fixture;
8. learn a baseline for a real `fd` invocation;
9. require two repeated no-drift PASS checks;
10. add `/usr/bin/sha256sum` in the same Bash wrapper as a controlled runtime expansion;
11. require REVIEW and an added `sha256sum` process-exec finding;
12. run the same baseline through public `AETHERXGLOBAL/execsurface@v0.1`;
13. preserve JSON/Markdown/baseline/environment evidence as an artifact.

## Why the controlled change is relevant

The controlled change does not assert a vulnerability in `fd`.

It simulates the class of runtime change ExecSurface is intended to surface:

> a previously accepted developer-tool invocation begins launching an additional executable and reading an additional input.

The expected result is REVIEW under the built-in unmatched-drift policy, not a malware or safety verdict.

## Upstream boundary

No modification is pushed to `sharkdp/fd`.

The workflow may create local untracked fixture files only.

No issue, pull request, email or other maintainer contact is authorized until this compatibility proof passes.

## ExecSurface boundary

Failure on the external workload must not be fixed by:

- weakening M6.5 completeness;
- broad random-path wildcards;
- changing verdict semantics;
- collapsing baseline and policy;
- claiming unsupported platforms.

Any failure is evidence first.
