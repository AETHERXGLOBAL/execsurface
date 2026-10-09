# ExecSurface V1-R1 — Stable Release Control Plane Protocol

Date: 2026-10-08  
Parent: issue #172  
Starting source: `main@14536e58692157fdb45186a266eb2c5487bae523`

Decision target:

`V1_R1_RELEASE_CONTROL_PLANE_PASS_BOUNDED`

This gate makes the existing durable Alpha release path capable of a future stable v1 release **without publishing v1 during this gate**.

## Classification

Release engineering, artifact provenance, compatibility and product governance.

Not cybersecurity research.

## Fixed review roles

- Release Architecture Lead
- SemVer / Compatibility Reviewer
- Supply-Chain & Artifact Reviewer
- Destruction / Falsification Team
- Anti-Drift Reviewer
- Independent Internal Critical Review Board

The critical-review board does not author the candidate fix.

## Non-negotiable safety rule

During V1-R1:

- do not change `.release/release-request.json` to a v1 request;
- do not create `v1.0.0`;
- do not create or move `v1`;
- do not publish a stable GitHub Release;
- do not publish v1 to crates.io.

This gate qualifies the control plane only.

## Existing Alpha behavior that must remain valid

Current Alpha.6 path remains supported:

- version `0.1.0-alpha.6`;
- immutable tag `v0.1.0-alpha.6`;
- moving channel `v0.1`;
- GitHub Release classified as prerelease;
- artifact/checksum/attestation;
- binary consumer;
- immutable Action consumer;
- stable-channel Action consumer;
- registry publication only after stable-channel proof.

No Alpha tag is rewritten.

## Proposed bounded v1 behavior to falsify

A future stable request:

- version `1.x.y`;
- immutable tag `v1.x.y`;
- moving channel `v1`;
- GitHub Release classified as stable, not prerelease.

Only stable v1 final versions are supported by this gate.

`v1.0.0-rc.1`, beta/dev versions, v2 and arbitrary channels are rejected until separately designed.

## Release-contract authority

Introduce one testable validator/classifier under `.github/scripts/`.

It must be the single source for release-line classification used by promotion/release workflows.

Accepted classes:

### Alpha line

Pattern:

`0.1.0-alpha.N`

Required stable channel:

`v0.1`

Release classification:

prerelease.

### Stable v1 line

Pattern:

`1.MAJOR_MINOR_PATCH` in ordinary SemVer numeric form `1.x.y`.

Required stable channel:

`v1`

Release classification:

stable.

The validator must reject:

- tag/version mismatch;
- action release-tag mismatch;
- wrong stable channel;
- unsupported prerelease forms;
- v2+;
- malformed versions.

## Fail-first corpus

Before implementation, executable tests must prove the current control plane fails these propositions:

1. synthetic `1.0.0 / v1.0.0 / v1` request is accepted by the validator;
2. current Alpha.6 request remains accepted as prerelease;
3. stable v1 with `v0.1` is rejected;
4. `1.0.0-rc.1` is rejected by this bounded gate;
5. promotion workflow uses the testable release classifier rather than hard-coded `stable_channel == v0.1`;
6. release workflow derives stable channel and prerelease/stable classification from the classifier;
7. GitHub Release publication is conditional: prerelease only for Alpha, stable for v1;
8. stable-channel promotion moves the classified channel rather than hard-coded `v0.1`;
9. zero-contact stable Action consumer verifies the classified moving channel;
10. Alpha-line behavior remains encoded and tested.

Any valid counterexample keeps the gate RED.

## Immutable tag governance

Repository rules remain a separate administrator boundary.

Before actual v1 release, GitHub must have a verified rule preventing unauthorized update/deletion of immutable v1 release tags.

V1-R1 code may become GREEN while this admin precondition remains OPEN; that state is:

`CONTROL_PLANE_CODE_PASS / V1_TAG_GOVERNANCE_PENDING`

and cannot authorize v1 release.

## Build floor

V1-R0 found:

- current Alpha.6 prebuilt PASS on Ubuntu 24.04;
- current prebuilt FAIL on Ubuntu 22.04 due GLIBC_2.39;
- local build PASS on both.

V1-R1 does not expand this support boundary.

Changing the v1 build floor requires a separate evidence-backed contract update.

## Closeout

V1-R1 closes only when:

- fail-first RED is retained;
- release classifier tests pass;
- Alpha regression passes;
- synthetic stable-v1 dry contract passes without creating a tag;
- CI / Stage-2 / Adversarial / P9.3 / Productization remain GREEN;
- no release request is changed;
- no v1 tag/channel exists as a side effect;
- critical review records the remaining immutable-v1-tag admin precondition.

No actual stable release is authorized by this gate.
