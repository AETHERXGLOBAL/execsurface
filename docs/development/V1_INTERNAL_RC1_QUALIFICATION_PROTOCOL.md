# ExecSurface v1.0.0 — Internal RC1 Qualification Protocol

Date: 2026-10-08
Starting source: `main@382f98b1328fdaa12773c9a88dcd02466f050a05`
Governance: `docs/development/V1_INTERNAL_QUALIFICATION_GOVERNANCE.md`
Stable contract: `docs/COMPATIBILITY.md`
Release-control gate: V1-R1 / issue #172

Candidate designation:

**INTERNAL_RC1 — NOT PUBLISHED — NOT TAGGED**

## Safety constraints

During this protocol:

- package/source version may become `1.0.0` on the RC branch;
- `action/release-tag.txt` may identify the intended immutable `v1.0.0` candidate;
- `.release/release-request.json` MUST remain the Alpha.6 request;
- no `v1.0.0` tag may be created;
- no `v1` moving channel may be created;
- no GitHub Release or crates.io v1 publication may occur;
- public README/STATUS continue to identify Alpha.6 as the current published release until actual release authorization.

A v1-looking version string on this private qualification branch is not a public release.

## Internal review board

The IQ0-IQ9 roles from the governance amendment are mandatory.

Final critical reviewers must not author the candidate fixes they review.

## IQ0 — RC source and contract freeze

RC1 must freeze:

- exact branch SHA;
- version `1.0.0`;
- intended immutable tag `v1.0.0`;
- stable moving channel `v1`;
- stable contract from V1-R0;
- no new feature scope.

Any semantic change discovered during qualification invalidates the RC SHA.

## IQ1 — predecessor semantic replay

Required on exact RC source:

- CI;
- Stage-2 Final Internal Gate;
- Adversarial Regression;
- P9.3 Compatibility;
- Alpha.6 Productization/public-consumer regressions where applicable;
- V1-R0 stable contract sentinels;
- V1-R1 release-control contract.

No predecessor assertion may be weakened.

## IQ2 — Destruction Round A

At minimum attack:

1. candidate still silently treats target exit as verdict-bearing;
2. policy-v3 opt-in changes v2 meaning;
3. Alpha.5 profile-3 is silently accepted as profile-4;
4. Alpha.6 baseline is mutated during v1 check;
5. custody pins are ignored on candidate Action path;
6. stable Action installer rejects final v1 tag;
7. candidate changes the public release request;
8. candidate accidentally creates/publishes v1 refs.

Any valid counterexample invalidates the RC SHA.

## IQ3 — Alpha.6 -> v1 upgrade and rollback

Executable rehearsal:

1. download exact public Alpha.6 binary;
2. learn profile-4 baseline with Alpha.6;
3. candidate v1 check against that baseline -> PASS for unchanged command;
4. candidate controlled drift -> REVIEW/10;
5. baseline bytes unchanged;
6. candidate learns a baseline under preserved profile-4 semantics;
7. Alpha.6 checks the candidate-learned baseline -> PASS for unchanged command;
8. policy schema 2 and schema 3 preserved;
9. target outcome separation preserved.

This is semantic rollback/compatibility proof, not a registry/channel rollback yet.

## IQ4 — Environment and installation qualification

Candidate source build must pass on:

- Ubuntu 22.04 x86_64;
- Ubuntu 24.04 x86_64;
- declared Rust 1.82 source floor.

Prebuilt-public-artifact qualification remains Ubuntu-24.04/glibc-2.39-class unless the build floor is explicitly changed and requalified.

RC1 does not expand that boundary.

## IQ5 — Performance characterization

Use fixed representative workloads.

At minimum measure candidate/native wall-clock ratio for:

- trivial shell command;
- a small deterministic filesystem/process workload;
- one developer-oriented command available in the runner.

Requirements:

- repeat count and command fixed before interpretation;
- report median native and observed time;
- retain high-overhead/no-fit outcomes;
- no universal low-overhead threshold;
- no release blocking solely from a number unless it violates a separately frozen claim.

## IQ6 — Release-control and tag governance

Code path must pass V1-R1.

Actual immutable-v1 tag ruleset remains a hard administrator precondition.

Until GitHub reports an active no-bypass update+deletion rule covering `v1.0.0`:

**IQ6_ADMIN_PENDING**

No release authorization is possible.

## IQ7 — Candidate productization

Candidate source must prove:

- `--version` = `execsurface 1.0.0`;
- doctor;
- learn;
- unchanged PASS;
- controlled REVIEW;
- baseline immutability;
- candidate Action local-development path;
- generated new-project workflow selects `@v1` for a v1 binary;
- custody setup remains actionable.

Public Alpha.6 consumer gates remain GREEN in parallel.

## IQ8 — Destruction Round B

After IQ1-IQ7 are otherwise GREEN, add a qualitatively different attack corpus.

Mandatory classes:

- release classifier mismatch;
- moving channel/source mismatch;
- stale release-tag file;
- v1 request accidentally armed;
- unsupported v1 prerelease accepted;
- rollback baseline mutation;
- environment-floor claim drift;
- documentation says external validation exists;
- one novel cross-layer attack selected after the first GREEN candidate.

## IQ9 — Internal Critical Review

Allowed outcomes:

- `INTERNAL_RC1_QUALIFIED_ADMIN_BLOCKED`
- `REWORK_REQUIRED`
- `REMAIN_PRE_V1`

`INTERNAL_RELEASE_AUTHORIZED` is forbidden until IQ6 admin protection is verified and a final release-source review occurs.

External independent validation is not claimed.
