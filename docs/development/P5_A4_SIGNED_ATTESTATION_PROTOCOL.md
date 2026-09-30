# ExecSurface — P5-A4 Signed GitHub / Sigstore Attestation Gate

Status: **PREREGISTERED — EXACTLY 12 LIVE CHECKS / RESEARCH-ONLY**
Date: 2026-09-30
Parent program: #100
Predecessor: `P5_A3_SLSA_PROVENANCE_BINDING_PASS_BOUNDED`
Development branch: `development/post-alpha4-behavioral-integrity`

## Question

Can ExecSurface produce and independently verify a real GitHub/Sigstore-signed SLSA provenance attestation for a deterministic research-only subject while preserving the separation between cryptographic validity, signer/workflow identity, policy authorization, and ExecSurface runtime semantic authority?

## Hypothesis

A minimal live GitHub artifact-attestation flow can:

1. sign one deterministic research-only subject with the pinned official `actions/attest` implementation;
2. independently verify the resulting Sigstore bundle with GitHub CLI against exact repository, signer workflow, source revision/ref, subject, and predicate constraints;
3. fail closed under identity, source, subject, predicate, or bundle tampering;
4. leave ExecSurface authority, completeness, and verdict semantics unchanged.

## Frozen external implementation

Official repository: `actions/attest`
Pinned commit: `1e69f48acb82d1966a394da916b4c1698aa569d6`
Action path: `action.yml`
Action identity: `actions/attest@1e69f48acb82d1966a394da916b4c1698aa569d6`

The pinned action declares:

- subject path/digest/checksum inputs;
- local `bundle-path` output;
- attestation ID and URL outputs;
- JSON-serialized Sigstore bundle output;
- GitHub/Sigstore signing through the official action implementation.

The live gate will use **default provenance mode** only. No custom ExecSurface predicate is introduced.

## Verification implementation

Independent verification is performed with `gh attestation verify` from the GitHub-hosted runner and the generated bundle supplied explicitly via `--bundle`.

The run must record the exact `gh` version used. This first bounded A4 gate therefore carries a verifier-version reproducibility caveat unless/until a later gate pins the GitHub CLI binary and checksum.

Verification must constrain:

- repository: `AETHERXGLOBAL/execsurface`;
- signer workflow: `AETHERXGLOBAL/execsurface/.github/workflows/p5-a4-signed-attestation.yml`;
- source digest: the exact workflow `GITHUB_SHA`;
- source ref: the exact workflow `GITHUB_REF`;
- predicate type: `https://slsa.dev/provenance/v1`.

## Deterministic signed subject

Path:
`experiments/p5-signed-attestation/subject.txt`

Exact UTF-8 content, including terminal newline:

```text
execsurface:p5-a4-signed-attestation-prototype
scope=research-only
p5_a3_accepted_source=d0ee3d0d1bd5a491ddc5796be08458026e1cbd8a
public_alpha4_source=48e0b9a0707553349e75e97a9dfa096d13f9ab5d
```

Expected SHA-256:
`41d288d8bff5b1088d451a9ce7193192f7ce3204031085a7134e207df4062015`

The subject is not a public release artifact and conveys no product/release status.

## Permissions — least privilege

The signing job is allowed only:

```yaml
permissions:
  contents: read
  id-token: write
  attestations: write
```

No package, deployment, repository-content write, or artifact-metadata write permission is authorized by A4.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — reuse official GitHub/Sigstore/in-toto mechanisms and avoid inventing a signing format.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks signature->authority, GitHub-brand->authority, SLSA-level inference, release claims, and public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — attacks subject, repository, signer workflow, source digest/ref, predicate and bundle integrity.
4. **Independent Critical-Milestone Reviewer** — verifies source pins, permissions, exact checks, retained failures, live evidence and promotion boundaries.

Dynamic specialists: Sigstore/keyless signing, GitHub OIDC/certificate identity, in-toto/SLSA, artifact-attestation verification, CI supply-chain security, reproducibility.

## Immutable boundaries

- public `v0.1.0-alpha.4` and stable `v0.1` remain at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no public crate/default-observer change;
- no v2->v3 reinterpretation;
- no signature validity -> semantic PASS rule;
- no signer/GitHub/Sigstore/SLSA brand -> authority rule;
- no SLSA level claim;
- no custom proprietary predicate;
- invalid/missing signatures or identity mismatch fail closed;
- successful cryptographic verification does not override ambiguous/lost/incomplete evidence;
- all failures and created external attestation IDs remain part of the evidence history.

## Frozen live checks — exactly 12

1. **A4-01 deterministic subject identity**
   - subject file exists with exact preregistered SHA-256 before signing.

2. **A4-02 pinned live attestation creation**
   - `actions/attest@1e69f48...` executes successfully in default provenance mode;
   - non-empty `bundle-path`, `attestation-id`, and `attestation-url` are returned;
   - bundle file exists and is non-empty.

3. **A4-03 exact positive cryptographic verification**
   - `gh attestation verify` succeeds for the exact subject and local bundle with exact repo, signer workflow, source digest, source ref, and default SLSA provenance v1 predicate.

4. **A4-04 mutated subject rejection**
   - changing one byte of the subject causes verification to fail.

5. **A4-05 wrong repository rejection**
   - verification constrained to a different repository fails.

6. **A4-06 wrong signer-workflow rejection**
   - verification constrained to a different workflow path fails.

7. **A4-07 wrong source-digest rejection**
   - verification constrained to a different source commit digest fails.

8. **A4-08 wrong source-ref rejection**
   - verification constrained to a different git ref fails.

9. **A4-09 wrong predicate-type rejection**
   - verification constrained to a non-SLSA predicate fails.

10. **A4-10 tampered signed bundle rejection**
    - a structure-preserving mutation of signed bundle payload/signature material must fail verification; malformed-only rejection is insufficient if the bundle schema permits a structure-preserving mutation.

11. **A4-11 verified result exact binding inspection**
    - successful JSON verification output must bind the exact subject digest/name and `https://slsa.dev/provenance/v1` predicate;
    - verification output must contain parsed certificate identity and at least one verified timestamp/transparency/timestamp witness when exposed by the installed GitHub CLI;
    - exact `gh` version is recorded.

12. **A4-12 semantic non-inflation guard**
    - the signed-attestation flow must not modify ExecSurface authority/completeness/verdict fields;
    - P5-A3 `a3_12_provenance_presence_does_not_infer_slsa_level_or_semantic_authority` is re-executed and must PASS after live signature verification.

**Acceptance requires 12/12 PASS. No check, identity constraint, negative control, or expected outcome may be weakened after observing the live result.**

## Mandatory same-run predecessor reproofs

After all 12 A4 checks pass:

- full P5-A3 corpus 12/12 PASS;
- P5-A2 original 14/14 PASS;
- P5-A2 verifier-ID addendum 4/4 PASS;
- P5-A1 12/12 PASS;
- P5-A0 18/18 PASS;
- critical P4 cross-proposition + bounded live B1/B2/B3 PASS;
- Semantics v3 PASS;
- public M11 PASS;
- immutable alpha.4/stable boundaries PASS.

## Decision rule

Only after the complete live + predecessor gate passes may A4 close as:

`P5_A4_SIGNED_ATTESTATION_PASS_BOUNDED`

A4 success means only that a real GitHub/Sigstore attestation can be generated and identity-constrained/cryptographically verified for the bounded research subject. It does not authorize public release or signature-derived semantic authority.

## Failure rule

Any failure must be retained and classified before correction as one of:

- permissions/OIDC/platform setup defect;
- action/API interoperability defect;
- verifier/tool-version defect;
- harness defect;
- cryptographic/identity falsification failure;
- semantic-authority regression.

No threshold or identity requirement may be relaxed merely to obtain a green run.
