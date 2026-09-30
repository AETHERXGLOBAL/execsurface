# ExecSurface — P5-A3 SLSA Provenance Binding Decision

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Decision: **P5_A3_SLSA_PROVENANCE_BINDING_PASS_BOUNDED**
Scope: research/development only; no public integration, release, SLSA-level, or signed-attestation claim.

## Decision

The preregistered P5-A3 hypothesis is accepted for the tested bounded profile.

ExecSurface can consume the pinned upstream SLSA provenance v1 fixture and produce a deterministic provenance binding that remains fail-closed across the frozen subject/source/revision/workflow/replay/absence/unverified/predicate adversarial cases.

The accepted contract is deliberately narrower than cryptographic attestation verification:

- in-toto Statement type must equal `https://in-toto.io/Statement/v1`;
- predicate type must equal `https://slsa.dev/provenance/v1`;
- the exact statement SHA-256 is bound;
- selected subject name and SHA-256 are bound;
- source repository and source revision are bound;
- build-config/workflow path is bound;
- builder identity is retained;
- verification state is explicit and external to the parser;
- a DSSE envelope/signature being present does not set `verified=true`;
- unverified provenance cannot become a verified `ProvenanceReference`;
- provenance absence remains absence;
- provenance presence or verification cannot raise ExecSurface authority, completeness, or verdict;
- no SLSA level is inferred or stored.

## Pinned upstream evidence

Repository: `slsa-framework/slsa-verifier`

Commit:
`30d0be3bbab553fc51557377baba2f7572dfc212`

Path:
`verifiers/internal/gcb/testdata/v1.0-gcloud-container-github-single.json`

Git blob SHA:
`7102e40887a758e01184ac79ca10cb34b7281627`

Fetched fixture SHA-256 observed in CI:
`881b0d07d0a1d43e8fcc38d629db144d390d53ea866a85ea1b4b42a42422cc03`

The fixture contains a DSSE-wrapped Google Cloud Build provenance record with embedded in-toto Statement v1 / SLSA provenance v1. A3 does not claim to verify that DSSE signature.

## Accepted source and CI evidence

Accepted source SHA:
`d0ee3d0d1bd5a491ddc5796be08458026e1cbd8a`

Full gate:
- workflow run: `36753014493`
- result: PASS
- evidence artifact: `11114644663`
- artifact name: `p5-a3-36753014493-1`
- artifact size: `10738` bytes
- artifact SHA-256: `050205a25cde61b66a443cd35dabded60ab909e91d487146c0efb2bdb4a4ed67`

Same-run acceptance evidence:
1. immutable alpha.4/stable boundaries — PASS;
2. exact pinned upstream commit/blob — PASS;
3. lock/rustfmt/clippy static gates — PASS;
4. frozen P5-A3 corpus — **12/12 PASS**;
5. P5-A2 original corpus — **14/14 PASS**;
6. P5-A2 verifier-ID addendum — **4/4 PASS**;
7. P5-A1 — **12/12 PASS**;
8. P5-A0 — **18/18 PASS**;
9. critical P4 cross-proposition and bounded live B1/B2/B3 gates — PASS;
10. Semantics v3 — PASS;
11. public M11 — PASS;
12. candidate decision emitted only after the full gate passed.

## Retained negative/pre-scientific evidence

No failure was deleted or rewritten.

### Run `36751913033`
- source: `a514a83c4ca6ea24e86ebc3e15bf9ff00e6c5842`
- classification: static/rustfmt failure before the A3 corpus ran;
- artifact: `11114882875`;
- artifact SHA-256: `7edcf25540e042b80ca1c152c1e849f9c0bf6439e419b2ad071424e733c8e367`.

### Run `36752092785`
- source: `c011c752b283b90fe7e49fab41bae639f152d5ea`;
- classification: remaining static/rustfmt failure in the frozen test file;
- artifact: `11115360292`;
- artifact SHA-256: `f83c67546689e0645ad1a7857d9e0145b785ffc5749f0550a446e64ac9f5eebc`.

### Run `36752246957`
- source: `23a859e9fd80046bde54a40fe62316147b9965b1`;
- classification: final one-line rustfmt failure before corpus execution;
- artifact: `11115187118`;
- artifact SHA-256: `7fe0f46a2ebb488e9859eb3c4e4e39bf8b945d41745cf092c5b78ae99d3b4210`.

### Run `36752639233`
- source: `be4bc4eca69f2a47d6130c80d11ccfe301295563`;
- classification: compile-time Rust lifetime defect (`E0106`) before corpus execution;
- correction: lifetime explicitly tied to the digest input only; no semantic/test change;
- artifact: `11114634141`;
- artifact SHA-256: `5cf337886d4251569c73980aab08bc73cf6d7b26b51cbbc0fae2c872f8b7fba3`.

### Run `36752814588`
- source: `ad36fc750cb68e2bc9ae1c6f572751a73b353777`;
- static gates: PASS;
- observed corpus result: 0/12 because all tests failed before assertions with `No such file or directory` for the upstream fixture;
- classification: test-harness path defect, not a SLSA-binding counterexample;
- root cause: workflow exported a relative fixture path while Cargo executed tests from the experiment crate directory;
- correction: workflow-only absolute `$GITHUB_WORKSPACE` fixture path; no test or binding semantic change;
- artifact: `11115083301`;
- artifact SHA-256: `983c52f290d734a169ae25fb45edbd0d78ca86143a750add76cd27aa4729b979`.

## Scientific interpretation

The accepted evidence supports only a bounded provenance-binding claim. It does not establish that:

- the upstream DSSE signature was cryptographically verified by ExecSurface;
- a signer identity is authorized by ExecSurface policy;
- any SLSA build level has been achieved;
- provenance proves runtime truth;
- a valid signature can upgrade ambiguous/lost/incomplete runtime evidence;
- this research implementation is ready for public integration.

The strongest result is compositional: a verified provenance reference can be bound into the existing P5 verification model without changing the pre-existing runtime authority/verdict semantics.

## Boundaries preserved

This decision does **not**:
- change public `v0.1.0-alpha.4` or stable `v0.1`;
- touch public crates or default observer behavior;
- reinterpret raw/canonical/baseline v2;
- introduce a proprietary in-toto predicate;
- authorize signature-based authority inflation;
- authorize public signing/publishing or release promotion.

## Next gate

Proceed to **P5-A4 — Signed GitHub/Sigstore Attestation Prototype** only under a separately preregistered gate.

A4 must distinguish cryptographic validity, signer/workflow identity, subject binding, policy authorization, and ExecSurface semantic authority. A valid signature must never convert ambiguous/lost/incomplete evidence into PASS.
