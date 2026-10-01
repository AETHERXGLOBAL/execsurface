# ExecSurface — Promotion A6 P7 Platform/CI Contract Eligibility Decision

Date: 2026-10-01
Tracking: #115
Protocol: `POST_ALPHA4_PROMOTION_A6_P7_PLATFORM_PROTOCOL.md`
Candidate branch: `integration/post-alpha4-promotion-candidate`

## Decision

**`POST_ALPHA4_PROMOTION_A6_P7_PLATFORM_CONTRACTS_ELIGIBLE_BOUNDED_CANDIDATE`**

The bounded P7 GitLab-context and self-hosted evidence/package contracts are eligible to continue as internal candidate integration contracts only. The native Linux arm64 result remains negative and is not promoted.

## Accepted evidence

- A6 execution source: `44b4deba738ed0238b4c79984cdccbb52a99fa80`;
- workflow run: `36864669697` — SUCCESS;
- exact frozen A6 falsifier: 12/12 PASS;
- closeout artifact: `11162889486`, digest `sha256:8cfd9c0a9f943ccf1c955b7279f981378ed191de08e41e627a0520f0a0cfc372`;
- P7 contract replay artifact: `11163098918`, digest `sha256:ade49f163c5c30d4c6d804a257053d6f946715788eec1957686918bbbbadeebc`;
- A6 falsifier artifact: `11162179066`, digest `sha256:995a864390729f5daeeb0cca1579b5af12b0b91352934a27e50c1cc94273d4d2`;
- repaired Semantics-v3 reproof: PASS;
- GitLab context-adapter corpus replay: PASS;
- self-hosted evidence-contract corpus replay: PASS;
- self-hosted package-contract corpus replay: PASS;
- immutable alpha.4 / stable `v0.1` anchors: PASS.

## Negative result retained

The following remains unchanged and binding:

**`P7_A0_ARM64_NOT_PORTABLE`**

- native parity run `36774237512` remains failed historical evidence;
- no Linux arm64 support or parity claim is authorized;
- no arm64 release asset is authorized;
- no cross-platform baseline interchangeability is established.

A6 did not rerun arm64 in an easier environment or change any threshold to convert this negative result into success.

## Eligible bounded contracts

### GitLab context

The bounded GitLab context adapter remains context-only. Project/ref/job/pipeline metadata cannot:
- select a baseline;
- change a verdict;
- upgrade evidence completeness;
- upgrade observer or proposition authority.

No live/public GitLab support is claimed.

### Self-hosted CI

The bounded self-hosted evidence/package contracts remain eligible only with explicit source/package integrity and evidence semantics. Runner ownership, provider identity, labels, root/admin state and environment metadata remain non-authoritative.

CI environment cannot steer baseline identity or override verdicts.

No public self-hosted or zero-assistance deployment support is claimed.

## Public state

Unchanged:
- public release `v0.1.0-alpha.4`;
- public source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable Action `AETHERXGLOBAL/execsurface@v0.1`;
- public raw/canonical/baseline semantics remain v2.

## Scope

A6 authorizes only the next internal promotion/integration gate. It does not authorize:
- arm64 support;
- public GitLab support;
- public self-hosted support;
- `main` merge;
- public release or tag movement;
- adoption/external-validation claims;
- P8 closure.

## Next gate

**A7 — integrated candidate destructive closeout.**

A7 must attack the combined promoted candidate across Semantics v3, proposition authority, provenance/attestation, bounded variance, comparison-tooling boundaries and platform-context boundaries as one composed system. It must preserve every retained negative result and fail closed on cross-layer authority laundering, downgrade, substitution, replay, schema migration and unsupported-platform claims.
