# ExecSurface V1-R1 — Stable Release Control Plane Result

Date: 2026-10-08  
Protocol: `docs/development/V1_R1_RELEASE_CONTROL_PLANE_PROTOCOL.md`  
Qualified code source: `457149755369c1a355eb0957c22f0689bca3d10c`

Decision:

**V1_R1_CONTROL_PLANE_CODE_PASS_BOUNDED — V1_TAG_GOVERNANCE_PENDING**

## Meaning

The repository release-control code is now capable of distinguishing the existing Alpha line from a future stable v1 line without arming or publishing v1 during this gate.

V1-R1 is **not fully closed** because GitHub repository administration does not yet contain an active immutable-v1 tag ruleset.

The promotion workflow now fails closed for stable v1 until that rule exists and is verified.

No v1 release, tag, moving channel or registry version was created by this gate.

## Fail-first evidence

After formatting-only setup correction, the preregistered release-control test reached semantic execution with:

**2/5 PASS, 3/5 FAIL**

Passed initially:

- unsupported/mismatched release contracts failed closed;
- V1-R1 did not arm a real v1 release request.

Failed:

1. no testable release classifier existed;
2. promotion was hard-coded to `stable_channel == v0.1`;
3. release publication, stable-channel promotion and consumer proof were hard-coded to the Alpha line.

The failures propagated through CI, Stage-2 and Adversarial workflows and are retained as genuine release-control RED evidence.

## Corrections

### Testable release classifier

Added:

`.github/scripts/release_contract.py`

Bounded accepted classes:

- `0.1.0-alpha.N / v0.1.0-alpha.N / v0.1` -> prerelease;
- final `1.X.Y / v1.X.Y / v1` -> stable.

Rejected by design:

- version/tag mismatch;
- wrong stable channel;
- v1 RC/beta prereleases;
- v2+;
- malformed versions.

Current Alpha.6 request remains valid.

### Promotion workflow

`promote-release.yml` now uses the classifier rather than a hard-coded `v0.1` test.

The active release request remains Alpha.6 and was not changed to v1.

### Release publication maturity

`release.yml` now exports:

- release kind;
- stable channel;
- prerelease boolean.

GitHub Release publication is conditional:

- Alpha line -> prerelease;
- stable v1 line -> non-prerelease / latest stable release.

### Moving channel

Stable-channel promotion now uses the classified channel rather than hard-coded `v0.1`.

The channel is moved only after:

- immutable release build;
- binary consumer;
- tag consumer;
- immutable Action consumer.

### Stable Action consumer

The stable consumer now checks out the classified moving channel, verifies:

- channel checkout HEAD == immutable release source SHA;
- channel's `action/release-tag.txt` == immutable release tag;

then removes checkout Git metadata and runs the local Action consumer path.

This supports both current `v0.1` and future `v1` without dynamic unverified Action indirection.

## Immutable v1 tag governance guard

Added:

`.github/scripts/verify_tag_ruleset.py`

A stable v1 promotion now queries repository rulesets before immutable tag creation and requires an active tag ruleset that:

- matches the exact candidate v1 release tag;
- blocks deletion;
- blocks update;
- has no bypass actors;
- does not allow the current actor to bypass when that field is reported.

Synthetic tests prove:

- a correct no-bypass `refs/tags/v1.*` rule is accepted;
- Alpha-only protection is rejected;
- bypass-enabled protection is rejected;
- deletion-only protection is rejected.

## Current GitHub administration evidence

Current repository rulesets at closeout:

- `Protect main` — active branch ruleset;
- `Protect release v0.1.0-alpha.5` — active tag ruleset.

No immutable v1 release-tag ruleset is currently configured.

Current Alpha.6 GitHub Release reports:

`immutable: false`

Therefore the remaining V1-R1 admin state is:

**V1_TAG_GOVERNANCE_PENDING**

The connected GitHub automation available to this work can read rulesets but cannot create/update repository rulesets.

This is a real administration precondition, not a documentation task.

## No side effects

V1-R1 read-only qualification explicitly proved:

- no `v1` tag exists;
- no `v1.0.0` tag exists;
- active `.release/release-request.json` remains Alpha.6;
- synthetic stable-v1 classification does not publish anything.

## Exact-source qualification

All required top-level workflows completed SUCCESS on `457149755369c1a355eb0957c22f0689bca3d10c`:

- V1-R1 Release Control Plane — `37756429326`
- CI — `37756429331`
- Alpha.6 Productization Gate — `37756429072`
- P9.3 Compatibility Contract — `37756429148`
- Stage-2 Final Internal Gate — `37756429152`
- Adversarial Regression — `37756429236`

V1-R1 jobs:

- Release classifier and workflow contract — SUCCESS
- Synthetic stable-v1 classification / no publication — SUCCESS
- No v1 tag/channel side effect during V1-R1 — SUCCESS

## Alpha-line anti-regression

Alpha.6 Productization remains GREEN.

The Alpha release contract remains accepted as prerelease with moving channel `v0.1`.

No Alpha tag/release was rewritten.

## Late destruction expansion — stable Action installer

After the initial V1-R1 control-plane code was GREEN, the destruction team challenged the published Action install path itself.

The existing `action/install.sh` accepted prerelease-shaped tags only. A future moving `v1` channel could therefore resolve to valid stable source while the Action rejected its pinned `v1.0.0` release tag before downloading the binary.

A fail-first test was added before the fix.

After formatting-only setup correction, the V1-R1 contract suite produced:

**6/7 PASS, 1/7 FAIL**

The only failing proposition was:

`published_action_installer_accepts_bounded_alpha_and_final_v1_tags`

### Bounded correction

The published Action installer now accepts exactly these qualified release-tag families:

- `v0.1.0-alpha.N`
- final `v1.X.Y`

It continues to reject unsupported tag forms through the existing fail-closed `invalid pinned release tag` path.

The platform error wording was also made maturity-neutral: it now says the public Action supports Linux x86_64 rather than calling every future release an Alpha.

Final corrected source:

`a2fd697252ef2b8f4db457776f30d507d15efd5a`

Exact-source qualification:

- V1-R1 Release Control Plane — `37757518687` — SUCCESS
- CI — `37757518616` — SUCCESS
- Alpha.6 Productization Gate — `37757518585` — SUCCESS
- Public Consumer Smoke — `37757518636` — SUCCESS
- P9.3 Compatibility Contract — `37757518668` — SUCCESS
- Stage-2 Final Internal Gate — `37757518655` — SUCCESS
- Adversarial Regression — `37757518667` — SUCCESS

This expansion does not change the remaining immutable-v1 tag administration precondition.

## Remaining action before actual stable v1 tag creation

An administrator must configure an active tag ruleset protecting immutable v1 release tags, for example a bounded ref include matching exact stable tags such as:

`refs/tags/v1.*`

while leaving the deliberately movable `refs/tags/v1` channel outside that immutable rule.

Required rules:

- deletion protection;
- update protection;
- no bypass actors.

After configuration, the ruleset must be re-read from GitHub and passed through `verify_tag_ruleset.py`.

Only then may V1-R1 change to:

`V1_R1_RELEASE_CONTROL_PLANE_PASS_BOUNDED`

This result does not authorize a v1 RC or release.

External independent validation is not claimed under the current governance amendment.
