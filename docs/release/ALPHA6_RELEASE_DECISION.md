# ExecSurface v0.1.0-alpha.6 — Release Decision

Date: 2026-10-08

Decision: **AUTHORIZE_ALPHA6_RELEASE_WITH_BOUNDED_CLAIMS**

## Purpose

Publish Alpha.6 as the recommended public Alpha replacing Alpha.5 for new use, while preserving Alpha.5 as immutable historical evidence and rollback material.

This decision does not rewrite or delete any prior GitHub participation, issue, pull request, comment, review thread, release artifact, or external-engagement record.

## Evidence basis

Stage-2 remediation was merged through PR #166 with retained fail-first history.

Before merge authorization:

- R0-R8 were internally qualified;
- the post-R8 destruction corpus was repeatedly expanded after apparent GREEN states;
- retained semantic RED states were preserved rather than rewritten;
- the final destruction corpus reached **17/17 PASS**;
- all required CI, adversarial, ptrace lifecycle, compatibility, packaging, public-consumer, typed-report/evidence, and consumer-contract workflows passed on the qualified product source.

The release pipeline independently reruns source gates, builds the release artifact, verifies release identity, publishes checksum and provenance, exercises a zero-contact binary consumer, tests immutable Action consumption, and moves `v0.1` only after those gates succeed.

## Compatibility decision

Alpha.6 corrects canonical semantics and therefore uses normalization profile 4.

Alpha.5 profile-3 baselines are not silently migrated. Alpha.6 explicitly rejects incompatible profile-3/profile-4 comparison; users must relearn the baseline under Alpha.6.

Alpha.5 remains available unchanged for rollback and historical reproduction.

## Scope boundary

Public supported boundary remains:

- Linux x86_64;
- native `ptrace` reference observer.

No broader backend/platform authority is claimed.

Research/product scope remains runtime behavioral verification, execution semantics, and semantic-evidence correctness. It is not cybersecurity research.

## External validation boundary

External independent falsification is not claimed by this release decision. Alpha.6 is released on the strength of the documented internal falsification, qualification, compatibility, artifact, and consumer gates.

## Release mechanics

The release request must require:

- version `0.1.0-alpha.6`;
- immutable tag `v0.1.0-alpha.6`;
- stable moving channel `v0.1`.

The existing promotion workflow must create the immutable tag only from the accepted `main` release source, dispatch the release workflow at that tag, and move `v0.1` only after immutable release and consumer proofs succeed.

Registry publication follows the validated stable Action path.

No Alpha.5 tag is moved or modified.
