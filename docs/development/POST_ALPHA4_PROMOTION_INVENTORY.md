# ExecSurface — Post-alpha.4 Promotion Inventory

Date: 2026-10-01
Tracking: #115
Candidate branch: `integration/post-alpha4-promotion-candidate`
A0 rule: **inventory records research state; it does not promote research into the public product.**

## Status vocabulary

`UNASSESSED` | `ELIGIBLE_BOUNDED` | `DEFER` | `BLOCKED` | `NEGATIVE_RETAINED`

No product-affecting row may start A0 as `ELIGIBLE_BOUNDED`.

## Inventory

| Program | Research result | Status | Evidence source / boundary | Promotion note |
|---|---|---|---|---|
| P2 | Semantics v3: proposition-scoped evidence, explicit authority/completeness, proof-carrying observation, bounded compatibility work | ELIGIBLE_BOUNDED | Historical A1 success `36840921782`; destructive counterevidence #116 retained; repaired source `0200f09557906118dd0e96f8a5a73aa4f5c4a9cc`; independent falsification `36853136002`; final candidate requalification run `36853799391`, artifact `11157785131` | Requalified after material-gap repair. Side-by-side candidate only; v2 remains v2; cross-schema default is `INCOMPARABLE_SCHEMA`; no public/default v3 or migration authorized. |
| P3 | Legitimate-variance / ephemeral-identity work | UNASSESSED | Bounded research decision retained on parent branch | Must prove lower false REVIEW with zero new false PASS; frequency never authorizes behavior. |
| P4 | Backend Adapter / Proposition Authority architecture | ELIGIBLE_BOUNDED | P4 closeout source `320c865d3e81071a8214ad726dca7a587b0fc479`; A2 candidate evidence source `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`; run `36855716538`; artifact `11159141703`; decision `POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_REQUALIFIED_BOUNDED_CANDIDATE` | Proposition-scoped authority eligible for continued candidate integration only. Backend name never raises authority; backend equivalence and baseline interchangeability remain unproved. |
| P5 | Runtime attestation/provenance composition using existing standards | UNASSESSED | Final bounded decision `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`; closeout lineage includes `1f4df8f45a115a1638c997107e069c6acd74c51b` | Cryptographic validity remains separate from semantic authority; no custom-standard claim. |
| P6 | Competitive falsification/comparison harness and factual matrix | UNASSESSED | P6 bounded closeout; successful closeout workflow run `36773688443` | Assess as validation/tooling infrastructure separately from runtime feature promotion. No winner score. |
| P7-arm64 | Native arm64 parity path tested and not established portable in bounded experiment | NEGATIVE_RETAINED | Retained P7 A0 failure and bounded arm64 not-portable decision | Must not be promoted as arm64 support. Fresh gate required for any future claim. |
| P7-other | Additional CI/platform research results other than failed arm64 parity | UNASSESSED | P7 bounded closeout; workflow run `36776286847` | Assess individually; platform count cannot weaken evidence contracts. |
| P8 | Current external validation / independent reproduction / criticism | BLOCKED | Issue #114; A5 prereg source `7bd806258ecdca415ef16ffb3689aefa13c0ece2` | Current external evidence minimum not yet satisfied. Internal work cannot manufacture independence. |

## Immutable public anchors

- Public release: `v0.1.0-alpha.4`
- Public release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable Action: `AETHERXGLOBAL/execsurface@v0.1`
- Public raw/canonical/baseline semantics remain v2.

## Closed assessment — A1 / P2

A1 is requalified after destructive review #116 found three material proof-admission gaps and the repaired candidate passed the unchanged destructive corpus plus expanded independent falsification.

Frozen A1 boundaries remain:
- no silent v2 -> v3 reinterpretation;
- v2 artifacts remain under v2 parser/verifier/digest rules;
- v3 proof semantics use an explicit distinct version domain;
- v2/v3 cross-schema comparison defaults to `INCOMPARABLE_SCHEMA`;
- no default PASS-eligible v2 -> v3 projection;
- missing/incomplete/ambiguous/unsupported required evidence remains non-admissible;
- backend names do not confer authority;
- any future migration must be explicit, non-destructive and reacquire missing proof evidence;
- public alpha.4, `main`, tags and stable `@v0.1` remain unchanged.

Current A1 decision:
`POST_ALPHA4_PROMOTION_A1_SEMANTICS_V3_REQUALIFIED_BOUNDED_CANDIDATE`

## Closed assessment — A2 / P4

A2 requalified the P4 proposition-authority architecture against the repaired A1 contract.

Accepted evidence:
- candidate evidence source: `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`;
- workflow run: `36855716538` — SUCCESS;
- artifact: `11159141703`;
- artifact digest: `sha256:1be78620a347019db2461745a88897d8f82ec44ff190070f5598d6d1faee7ed2`;
- S01-S10 authority-laundering sensitivity corpus: 10/10 PASS;
- A2 authority matrix: 16/16 PASS;
- B0 success-evidence contract: 18/18 PASS;
- P4 live open/rename-delete/connect reproof: PASS;
- pinned Tetragon external-import reproof: PASS;
- repaired Semantics-v3 + M11 fail-closed reproof: PASS.

Current A2 decision:
`POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_REQUALIFIED_BOUNDED_CANDIDATE`

Retained A2 boundaries:
- `BACKEND_EQUIVALENCE_NOT_ESTABLISHED` remains true;
- backend labels/profile names never confer authority;
- ptrace/kernel-hook/external-import evidence is not baseline-interchangeable by default;
- unsupported proposition families remain unsupported;
- public v2 semantics remain unchanged.

## Next assessment

A3 evaluates **P5 Runtime Attestation / Provenance promotion eligibility**.

A3 must prove that attestation, signature, SLSA/in-toto/Sigstore identity, provenance links and verifier metadata cannot manufacture semantic authority. It must replay retained P5 substitution/replay, duplicate-semantic-property, verifier-identity and source/baseline/current binding attacks before any P5 path is eligible for candidate integration.
