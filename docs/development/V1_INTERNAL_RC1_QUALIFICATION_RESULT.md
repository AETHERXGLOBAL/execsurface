# ExecSurface v1.0.0 — Internal RC1 Qualification Result

Date: 2026-10-08  
Protocol: `docs/development/V1_INTERNAL_RC1_QUALIFICATION_PROTOCOL.md`  
Governance: `docs/development/V1_INTERNAL_QUALIFICATION_GOVERNANCE.md`  
Qualified candidate source: `1c8d7a8204b38c0c0992d4144fecd325188e144e`

Decision:

**INTERNAL_RC1_QUALIFIED_ADMIN_BLOCKED**

## Meaning

The v1.0.0 internal RC1 candidate has passed the strengthened internal IQ0-IQ8 qualification chain for the frozen bounded product contract.

It is **not release-authorized**.

The remaining hard blocker is IQ6 repository administration:

**IQ6_ADMIN_PENDING**

GitHub does not currently report an active no-bypass deletion+update tag ruleset protecting the immutable `v1.0.0` release tag.

No `v1` or `v1.0.0` public tag was created during qualification.

No v1 GitHub Release or crates.io v1 publication occurred.

Alpha.6 remains the current public release.

Independent external validation is not claimed.

## Fail-first chronology

### RC identity fail-first

The first RC tests were committed before changing the candidate package identity.

Initial semantic result:

**1/3 PASS — 2/3 FAIL**

Passed:
- public README/STATUS still identified Alpha.6 as the published release.

Failed:
- internal RC source was still `0.1.0-alpha.6`, not `1.0.0`;
- generated workflow still selected `@v0.1`, not future stable `@v1`.

This was retained as genuine fail-first evidence.

The RC branch was then changed to internal candidate identity only:

- workspace/package version -> `1.0.0`;
- internal dependency pins -> `=1.0.0`;
- lockfile workspace packages -> `1.0.0`;
- `action/release-tag.txt` -> `v1.0.0`.

The active release request deliberately remained Alpha.6 and public status/docs remained Alpha.6.

### Version-aware onboarding correction

The self-service workflow generator was made maturity-aware:

- `0.1.*` -> `@v0.1`;
- `1.*` -> `@v1`;
- unqualified future majors -> explicit error.

Historical self-service integration coverage was converted from a hard-coded `@v0.1` assumption into a version-aware assertion while retaining explicit unit coverage of both Alpha and v1 mappings.

### Registry bootstrap counterexample

The Alpha.6 Productization source packaging step attempted:

`cargo publish -p execsurface --locked --dry-run`

against the unpublished v1 candidate.

This failed correctly because the final CLI package depends on internal `=1.0.0` crates that do not yet exist on crates.io.

The gate was not bypassed with `--no-verify`.

The corrected unpublished-candidate proof now requires:

1. real dry-run of the first publishable internal crate;
2. executable proof that the publication workflow contains every internal crate exactly once in dependency-safe order;
3. retained zero-contact exact-version registry install after the publish chain.

The public Alpha path continues to execute the full CLI dry-run.

### Harness-only failures retained

Several commits failed only because `rustfmt` required formatting changes in newly added tests.

A publish-chain test also initially used substring matching that confused `publish_one execsurface` with `publish_one execsurface-model`. The test harness was corrected to compare exact publish-step tokens; the release workflow itself was not changed to satisfy that harness defect.

No acceptance assertion was weakened.

## First GREEN identity candidate

The first candidate for which the existing complete predecessor/productization/packaging workflow set was GREEN was:

`f528bba6b10dfd42fb014bb9ade3d533919c4430`

This source is preserved as the first GREEN v1.0.0 identity candidate before the strengthened RC-specific destruction chain was added.

## IQ0 — Source and contract freeze

PASS.

Candidate identity:

- version: `1.0.0`;
- intended immutable tag file: `v1.0.0`;
- future stable channel selected by generated v1 workflow: `v1`.

Safety controls remained intact:

- release request remained `0.1.0-alpha.6 / v0.1.0-alpha.6 / v0.1`;
- README/STATUS continued to identify Alpha.6 as published;
- no public v1 tag/channel/release was created.

## IQ1 — Predecessor semantic replay

PASS on the final qualified candidate source.

Top-level workflow results:

- CI — run `37762428166` — SUCCESS
- Stage-2 Final Internal Gate — run `37762428118` — SUCCESS
- Adversarial Regression — run `37762428128` — SUCCESS
- P9.3 Compatibility Contract — run `37762428231` — SUCCESS
- V1-R0 Stable Contract Freeze — run `37762428160` — SUCCESS
- Alpha.6 Productization Gate — run `37762428181` — SUCCESS
- Public Consumer Smoke — run `37762428175` — SUCCESS
- Registry Packaging Gate — run `37762428089` — SUCCESS
- Technical Evaluation Pack — run `37762428127` — SUCCESS
- V1 RC1 Internal Qualification — run `37762428213` — SUCCESS

No predecessor acceptance assertion was weakened.

## IQ2 — Destruction Round A

PASS.

The RC-specific first destruction round re-attacked:

- internal-only release identity;
- accidental v1 release-request arming;
- accidental public v1 refs;
- generated stable Action channel;
- stable contract sentinels;
- custody fail-closed behavior;
- policy v1/v2/v3 semantic boundaries;
- registry publication-chain completeness.

No valid counterexample survived.

## IQ3 — Alpha.6 <-> v1 upgrade and rollback

PASS.

Executable evidence proved:

1. exact public Alpha.6 artifact was checksum-verified and attestation-verified;
2. Alpha.6 learned a profile-4 baseline;
3. v1 RC1 consumed the Alpha.6 baseline and returned PASS for unchanged execution;
4. controlled drift returned REVIEW / exit 10;
5. Alpha.6 baseline bytes remained unchanged;
6. v1 RC1 learned its own profile-4 baseline;
7. Alpha.6 consumed the v1-learned profile-4 baseline and returned PASS;
8. v1 baseline bytes remained unchanged.

This is semantic cross-version rollback evidence. It does not substitute for an actual registry/channel rollback after a stable release.

## IQ4 — Environment and source floor

PASS for the frozen source contract.

Candidate source build + `doctor -> learn -> check` passed on:

- Ubuntu 22.04 x86_64;
- Ubuntu 24.04 x86_64.

Declared Rust source floor:

- Rust `1.82.0` — PASS.

The frozen install-route boundary remains unchanged:

- current published prebuilt strategy is qualified for Ubuntu-24.04/glibc-2.39-class x86_64;
- the historical Alpha.6 prebuilt Ubuntu-22.04 `GLIBC_2.39` incompatibility remains retained negative evidence;
- local-build/source evidence must not be relabeled as universal prebuilt compatibility.

## IQ5 — Performance characterization

PASS as characterization evidence, not as a low-overhead claim.

Protocol:

- 3 fixed workloads;
- 7 repetitions each;
- medians reported;
- no post-result pass/fail threshold.

Final qualified-source results:

### trivial-shell

- native median: `0.09498375500000122 s`
- ExecSurface check median: `0.433528208999995 s`
- ratio: `4.56423531581784x`

### deterministic-python

- native median: `0.11326759900000383 s`
- ExecSurface check median: `0.48449059900000435 s`
- ratio: `4.277397978569211x`

### small-file

- native median: `0.09896255999999681 s`
- ExecSurface check median: `0.45572639800000303 s`
- ratio: `4.605038491324574x`

Artifact digest:

`365e47a463c00832f83a40bf9e7ead4514aaf27ce57ed372930fec412d4bdf92`

The overhead is material and is retained honestly.

No universal low-overhead claim is made.

No threshold was weakened.

## IQ6 — Release-control and immutable-tag governance

**ADMIN PENDING / RELEASE BLOCKING**

Release-control code is qualified.

Repository administration is not.

The exact qualified run re-read GitHub rulesets and produced:

`tag-governance error: no active no-bypass deletion+update tag ruleset protects v1.0.0`

Classification:

**IQ6_ADMIN_PENDING**

The same run re-proved that no public `v1` or `v1.0.0` tag exists.

Before release authorization, GitHub must report an active tag ruleset covering the immutable stable-v1 release identity with:

- deletion protection;
- update protection;
- no bypass actors;
- no current-user bypass where GitHub reports that field.

The deliberately movable `v1` channel must remain outside the immutable exact-release rule.

## IQ7 — Candidate productization

PASS within the unpublished-candidate boundary.

Evidence includes:

- `execsurface --version == execsurface 1.0.0`;
- source packaging chain proof;
- candidate `doctor`;
- learn/check;
- generated v1 project workflow selects `@v1`;
- custody inputs remain actionable;
- Alpha.6 public productization/consumer gates remain GREEN in parallel;
- public README/STATUS remain Alpha.6 until actual release authorization.

No public v1 artifact was manufactured merely to satisfy candidate qualification.

## IQ8 — Destruction Round B

PASS.

Round B was added **after** a GREEN candidate, as required by governance.

It was qualitatively different from Round A and attacked release lifecycle and cross-layer boundaries.

### Release-lifecycle corpus

PASS:

- version/tag mismatch rejected;
- unsupported v1 prerelease identity rejected;
- final v1 classification is stable, not prerelease;
- wrong v1 stable channel rejected;
- stale Alpha action release tag rejected for v1 request;
- release request remains Alpha.6 on RC branch;
- public docs do not claim v1 is already published;
- forbidden external-validation/maturity claims are absent;
- immutable-v1 tag governance guard appears before tag creation;
- immutable Action consumer precedes moving-channel promotion;
- stable-channel consumer precedes registry publication;
- environment-floor negative evidence remains explicit.

### Novel cross-layer attack

Attack composition:

- baseline created by exact public Alpha.6;
- candidate v1 RC1 consumes it;
- valid policy supplied;
- policy SHA pin correct;
- baseline digest pin deliberately wrong;
- target would create `MUST_NOT_EXIST`.

Required result:

- ERROR / exit 2;
- target must not execute;
- marker must not exist;
- Alpha.6 baseline bytes must not change.

Result:

**PASS — fail-closed before target execution.**

No valid Round-B counterexample survived.

## IQ9 — Independent Internal Critical Review disposition

Based on IQ0-IQ8 evidence, the permitted bounded disposition is:

**INTERNAL_RC1_QUALIFIED_ADMIN_BLOCKED**

What is proven internally:

- frozen v1 semantic contract survives predecessor replay;
- v1 identity/channel selection is coherent;
- Alpha.6 <-> v1 profile-4 semantic compatibility is executable;
- source environment/MSRV path is qualified within the frozen boundary;
- release lifecycle ordering and classification survive two separated destruction rounds;
- candidate packaging chain is explicit and fail-closed;
- current public Alpha path remains operational;
- no v1 public refs/releases were created during qualification.

What remains unproven / blocked:

- actual immutable-v1 tag governance is not configured;
- no exact public v1 release artifact exists yet;
- no actual `@v1` channel exists yet;
- no actual crates.io `1.0.0` publication exists yet;
- no independent external validation is claimed;
- measured ptrace overhead remains material.

Therefore:

- do not create or move `v1.0.0` / `v1`;
- do not publish v1;
- keep the RC PR in Draft/admin-blocked state;
- after administrator tag-rule configuration, re-read rulesets and perform final release-source review before any release request is armed.

## Final decision

**INTERNAL_RC1_QUALIFIED_ADMIN_BLOCKED**

This is an internal qualification result, not a release authorization and not an external-validation claim.
