# ExecSurface v1.0.0 — Stable Release Decision

Date: 2026-10-08

Decision: **RELEASE_V1_0_AUTHORIZED_BOUNDED**

## Authorization

Explicit owner authorization has been received to:

- merge the internally qualified RC1;
- arm the stable v1.0.0 release transaction;
- create the immutable `v1.0.0` release tag only through the qualified release-control plane;
- promote the moving `v1` Action channel only after immutable artifact and consumer proofs pass;
- publish the stable GitHub Release and crates.io version only through the qualified workflow chain.

## Qualified source lineage

RC1 qualified product source:

`1c8d7a8204b38c0c0992d4144fecd325188e144e`

Final reviewed RC PR head:

`f18d6e952e3215ffd07fede8ad882159d93bec05`

Merged RC commit on main:

`16df948860cd6c9b8a73f1ceef46cff4341513a4`

The RC PR was merged only after:

- final exact-source release review PASS_BOUNDED;
- all required workflows were GREEN;
- immutable-v1 tag governance was verified.

## Immutable tag governance

Verified GitHub ruleset:

- id: `24717518`;
- name: `Protect immutable v1 releases`;
- target: tags;
- enforcement: active;
- include: `refs/tags/v1.*`;
- update protection: enabled;
- deletion protection: enabled;
- bypass actors: none;
- current user bypass: never.

The deliberately movable exact `v1` channel is outside the immutable `v1.*` release-tag pattern.

## Release request

The authorized stable request is:

- version: `1.0.0`;
- immutable tag: `v1.0.0`;
- moving stable Action channel: `v1`.

## Required transaction ordering

The release must proceed only through the existing qualified workflows:

`release request merge -> promote-release validation -> immutable tag -> release.yml -> public artifact proofs -> v1 promotion -> stable Action proof -> crates.io publication -> exact registry install proof`

Any failed step keeps the release transaction RED and must not be manually bypassed.

## Claims boundary

Allowed bounded release claim:

> ExecSurface v1.0 is internally qualified under AETHER X GLOBAL's published compatibility, adversarial, artifact, rollback and productization gates for the documented support boundary. Independent external validation is not claimed.

Not authorized:

- independent/external validation claims;
- universal Linux compatibility;
- ARM64 support;
- eBPF/libbpf public authority;
- backend equivalence;
- universal low-overhead claims;
- interpreting ExecSurface PASS as target-command success.

## Project classification

Runtime behavioral verification / execution semantics / semantic-evidence correctness.

Not cybersecurity research.

## Final decision

**RELEASE_V1_0_AUTHORIZED_BOUNDED**

This decision authorizes the controlled release transaction. It does not authorize bypassing a failed workflow, rewriting negative evidence, or weakening a release gate.
