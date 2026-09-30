# ExecSurface — P5-A4 Signed GitHub / Sigstore Attestation Decision

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Decision: **P5_A4_SIGNED_ATTESTATION_PASS_BOUNDED**
Scope: research/development only; no public integration, release, SLSA-level, or signature-derived semantic-authority claim.

## Decision

The preregistered P5-A4 live gate is accepted for the tested bounded profile.

ExecSurface demonstrated a real GitHub/Sigstore keyless signing and independent verification path for one deterministic research-only subject using the pinned official `actions/attest` implementation and GitHub CLI verifier while preserving the separation between cryptographic validity, signer/workflow identity, source/subject/predicate binding, and ExecSurface runtime semantic authority.

The accepted result establishes only that the bounded attestation can be generated, cryptographically verified, identity-constrained, and falsified under the frozen negative controls. It does not establish that a valid signature upgrades runtime authority, completeness, verdict, or SLSA level.

## Frozen implementation and subject

Pinned signing action:
`actions/attest@1e69f48acb82d1966a394da916b4c1698aa569d6`

Subject:
`experiments/p5-signed-attestation/subject.txt`

Subject name in the generated attestation:
`subject.txt`

Subject SHA-256:
`41d288d8bff5b1088d451a9ce7193192f7ce3204031085a7134e207df4062015`

Predicate type:
`https://slsa.dev/provenance/v1`

Statement type:
`https://in-toto.io/Statement/v1`

No custom ExecSurface predicate was introduced.

## Accepted source and live CI evidence

Accepted source SHA:
`862f0830511fe1421de20717187645d6e0a9a52e`

Full live gate:
- workflow run: `36754398898`
- result: PASS
- evidence artifact: `11115422864`
- artifact name: `p5-a4-36754398898-1`
- artifact size: `39025` bytes
- artifact SHA-256: `96e6813097b8e84f5829d4cd01275ee90eda109677bf7d39af0ba50f72b62843`

Created live GitHub attestation:
- attestation ID: `51561155`
- repository attestation path: `AETHERXGLOBAL/execsurface/attestations/51561155`
- Sigstore transparency log: Rekor
- Rekor log index: `3023113114`
- verified Rekor timestamp: `2026-09-30T17:51:50Z`

Verifier recorded by the gate:
`gh version 2.101.0 (2026-09-15)`

This version is evidence for the executed gate; A4 therefore retains the preregistered verifier-version reproducibility caveat rather than claiming verifier-version independence.

## Frozen A4 checks — 12/12 PASS

1. deterministic subject identity — PASS;
2. pinned live attestation creation — PASS;
3. exact positive cryptographic verification — PASS;
4. mutated subject rejection — PASS;
5. wrong repository rejection — PASS;
6. wrong signer-workflow rejection — PASS;
7. wrong source-digest rejection — PASS;
8. wrong source-ref rejection — PASS;
9. wrong predicate-type rejection — PASS;
10. structure-preserving signed-bundle tamper rejection — PASS;
11. exact verified-result binding inspection — PASS;
12. semantic non-inflation guard — PASS.

The positive verification bound the exact repository, signer workflow, source commit, source ref, subject digest/name, and SLSA provenance predicate. The verified result contained the GitHub workflow certificate identity and a verified Rekor timestamp.

## Same-run predecessor reproofs

The same accepted live run also passed:

- P5-A3 SLSA provenance binding: **12/12 PASS**;
- P5-A2 original SCAI/SVR binding corpus: **14/14 PASS**;
- P5-A2 verifier-identity adequacy addendum: **4/4 PASS**;
- P5-A1 runtime-trace mapping: **12/12 PASS**;
- P5-A0 standards composition: **18/18 PASS**;
- P4 cross-proposition falsification: **12/12 PASS**;
- P4 B1 live FILE.OPEN_OBJECT: **10/10 PASS**;
- P4 B2 live FILE.RENAME_DELETE: **10/10 PASS**;
- P4 B3 live NET.CONNECT_DESTINATION: **10/10 PASS**;
- Semantics v3: **7/7 PASS**;
- public M11 shared-FD fail-closed regression: **6/6 PASS**;
- immutable alpha.4 / stable `v0.1` boundaries: PASS.

## Cryptographic identity observed

The verified certificate/result bound:

- repository: `AETHERXGLOBAL/execsurface`;
- workflow: `.github/workflows/p5-a4-signed-attestation.yml`;
- source SHA: `862f0830511fe1421de20717187645d6e0a9a52e`;
- source ref: `refs/heads/development/post-alpha4-behavioral-integrity`;
- runner environment: `github-hosted`;
- subject: `subject.txt` at the frozen SHA-256;
- predicate: SLSA provenance v1.

The live action signed through the Public Good Sigstore instance because the repository is public and uploaded the signature to the Rekor transparency log.

## Scientific interpretation

A4 establishes a bounded real-world interoperability path from ExecSurface research evidence into existing GitHub/Sigstore/in-toto/SLSA infrastructure without inventing a proprietary signing system.

The evidence does **not** establish that:

- signature validity implies semantic PASS;
- GitHub, Sigstore, SLSA, a certificate, or a signer name grants runtime authority;
- a valid signature repairs ambiguous, lost, incomplete, unsupported, or otherwise inadmissible runtime evidence;
- any SLSA build level has been achieved;
- the GitHub CLI verifier behaves identically across untested versions;
- the research-only prototype is approved for the public product or a release.

The semantic non-inflation guard explicitly re-proved that provenance/attestation presence does not infer SLSA level or raise ExecSurface semantic authority.

## Failure / negative-evidence record

The first frozen A4 live run passed without a scientific or harness failure. No prior P5/P4 failures were removed or rewritten; all historical negative and pre-scientific evidence remains retained in its original records.

## Boundaries preserved

This decision does **not**:

- change public `v0.1.0-alpha.4` or stable `v0.1`;
- change public crates or the default observer;
- reinterpret raw/canonical/baseline v2;
- introduce a proprietary in-toto predicate;
- infer a SLSA level;
- authorize signature-derived authority or verdict inflation;
- authorize public signing, product integration, or release promotion.

## Next step

Perform the formal P5 closeout review against the parent-program deliverables and the accumulated A0-A4 evidence. P5 may close only if that review confirms that runtime-trace mapping, verification-result attestation, provenance binding, signed-attestation interoperability, and the standards-reuse question are all resolved without a remaining semantic or standards gap requiring another preregistered P5 gate.
