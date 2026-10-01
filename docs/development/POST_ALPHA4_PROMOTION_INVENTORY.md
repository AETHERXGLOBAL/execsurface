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
| P3 | Legitimate-variance / ephemeral-identity work | ELIGIBLE_BOUNDED | Bounded GCC ephemeral projection only; accepted source `96ef1e2f6f316460cc88af4f89403455a910b2b4`; run `36861859874`; closeout artifact `11161694287`; attack artifact `11161424642`; decision `POST_ALPHA4_PROMOTION_A4_P3_VARIANCE_ELIGIBLE_BOUNDED_CANDIDATE`; failed pre-scientific runs `36860850659`, `36861039687`, `36861324520`, `36861475284` retained | Eligible only for the frozen GCC producer/role/grammar projection. Frequency, recurrence and similarity never authorize; raw evidence remains retained; generic learned variance and broad temp suppression remain out of scope. |
| P4 | Backend Adapter / Proposition Authority architecture | ELIGIBLE_BOUNDED | P4 closeout source `320c865d3e81071a8214ad726dca7a587b0fc479`; A2 candidate evidence source `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`; run `36855716538`; artifact `11159141703`; decision `POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_REQUALIFIED_BOUNDED_CANDIDATE` | Proposition-scoped authority eligible for continued candidate integration only. Backend name never raises authority; backend equivalence and baseline interchangeability remain unproved. |
| P5 | Runtime attestation/provenance composition using existing standards | ELIGIBLE_BOUNDED | P5 bounded closeout retained; A3 accepted source `0f6cbc68bea380ff25889e762cb8cd5287a9c5a3`; run `36858724331`; closeout artifact `11160521675`; decision `POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_ELIGIBLE_BOUNDED_CANDIDATE`; initial formatting-only failure `36858589653` retained | Existing standards composition eligible for continued candidate integration only. Cryptographic/signature/provenance validity never raises semantic authority; no custom-standard claim. |
| P6 | Competitive falsification/comparison harness and factual matrix | ELIGIBLE_BOUNDED | Validation/tooling only; accepted source `5e1ed9ae3640bb6ffa3f33cc2be735041399022f`; run `36864447147`; closeout artifact `11163263388`; attack artifact `11163336806`; decision `POST_ALPHA4_PROMOTION_A5_P6_VALIDATION_TOOLING_ELIGIBLE_BOUNDED_CANDIDATE`; first A5 run `36864270080` retained as governance-harness failure | Eligible only as internal proposition-local falsification/comparison tooling. No score, winner, superiority, semantic-authority, release-readiness, P8 or external-validation inference is authorized. |
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

## Closed assessment — A3 / P5

A3 requalified the bounded P5 standards-composition path while explicitly attacking signature/provenance authority laundering.

Accepted evidence:
- candidate evidence source: `0f6cbc68bea380ff25889e762cb8cd5287a9c5a3`;
- workflow run: `36858724331` — SUCCESS;
- closeout artifact: `11160521675`, digest `sha256:d56abd784e9c6ee5f23e36417177756c118781f2a0c8be15c8b5220bf78414ca`;
- A3 promotion-specific attack artifact: `11160171982`, digest `sha256:13c5dc170767fbc6e37b02ae0df6e9f3c72da5e078cc0df1c95e07ded10adb5e`;
- Sigstore reverification artifact: `11160531607`, digest `sha256:a40bae0e52c1ea63b66c824cf11b7feb72d84f1d6671fa1abc73388a98fe6f14`;
- A3 frozen attestation/provenance authority corpus: 12/12 PASS;
- P5-A5 historical cross-attestation red-team: 12/12 PASS;
- P5 A3/A2/A1/A0 standards corpora: PASS;
- repaired Semantics-v3, P4 authority sensitivity/cross-proposition rejection and M11 fail-closed: PASS;
- historical P5-A5 negative run `36755962689` retained and explicitly rechecked.

Retained negative A3 execution evidence:
- initial run `36858589653` at `f39ec43916c1971f2ea728ec05a19f1b2aaa4f28` failed at rustfmt before scientific tests;
- correction `0f6cbc68bea380ff25889e762cb8cd5287a9c5a3` was formatting-only; no test/assertion/threshold/model rule changed.

Current A3 decision:
`POST_ALPHA4_PROMOTION_A3_P5_ATTESTATION_ELIGIBLE_BOUNDED_CANDIDATE`

Retained A3 boundaries:
- cryptographic/signature/provenance validity never manufactures semantic authority;
- standards labels, signer/workflow/verifier identity do not create PASS eligibility;
- subject/source/baseline/current/verifier/provenance bindings remain explicit and fail closed;
- existing standards composition remains sufficient only within the tested bounded scope;
- no custom predicate/schema novelty claim;
- no public integration, `main` merge, release/tag movement or P8 closure authorized.

## Closed assessment — A4 / P3

A4 evaluated only the bounded GCC ephemeral-identity projection that survived prior P3 research. It did not reopen generic variance learning or convert recurrence into authorization.

Accepted evidence:
- accepted candidate source: `96ef1e2f6f316460cc88af4f89403455a910b2b4`;
- workflow run: `36861859874` — SUCCESS;
- closeout artifact: `11161694287`, digest `sha256:6b94dc9e849359350f744b1489d89893b946ff7c95b109b287304e518a230730`;
- frozen A4 attack artifact: `11161424642`, digest `sha256:39d4fae1ef0d36be55b76c6501fdc0adee4971434a68ac88a09e5f95706a8f5b`;
- prior-P3 falsifier artifact: `11162255036`, digest `sha256:ecf9fa6f7562425d505964b70ae2ff9f01518ea50a075ba360b3f9c1588bd68c`;
- exact frozen A4 corpus: **12/12 PASS**;
- existing GCC candidate unit corpus: **7/7 PASS**;
- P3 V4 poisoning / false-PASS falsifier replay: PASS;
- repaired Semantics-v3 admission reproof: PASS;
- rustfmt + Clippy `-D warnings`: PASS;
- immutable public anchors: PASS.

Value and safety established within the bounded scope:
- the legitimate GCC ephemeral pair becomes equal only in the derived projection, reducing identity-only false REVIEW;
- meaningful non-ephemeral drift remains visible;
- wrong actor/root/grammar/role, collision and rename cases remain ineligible;
- 256-fold repetition does not manufacture eligibility;
- raw evidence is not mutated;
- projection is deterministic under the tested effect-order variation;
- unrelated Go-build normalization remains distinct.

Retained failed A4 execution evidence:
- `36860850659` — pre-scientific static-gate failure; A4 scientific corpus did not execute;
- `36861039687` — pre-scientific static-gate failure; A4 scientific corpus did not execute;
- `36861324520` — lockfile PASS, rustfmt FAIL; Clippy/scientific corpus did not execute;
- `36861475284` — diagnostic rustfmt FAIL; exact Rust 1.90 output retained in artifact `11161608989`, digest `sha256:a454432f96671e79fb4044779fa3b7b98b8df893468a407b1055e21cada34b23`.

No test name, scientific assertion, threshold, acceptance rule or candidate semantic mechanism was weakened to obtain the successful run.

Current A4 decision:
`POST_ALPHA4_PROMOTION_A4_P3_VARIANCE_ELIGIBLE_BOUNDED_CANDIDATE`

Retained A4 boundaries:
- only exact GCC producer/role/grammar ephemeral projection is eligible;
- frequency, recurrence and similarity remain non-authoritative;
- no learned acceptance or broad temp/cache suppression;
- raw evidence and prior P3 negative evidence remain retained;
- public v2 semantics remain unchanged;
- no public integration, `main` merge, release/tag movement or P8 closure authorized.

## Closed assessment — A5 / P6

A5 evaluated P6 strictly as internal validation/falsification tooling and attacked the comparison method itself for ranking, missing-data laundering, authority inflation, evidence suppression, scope/equivalence inflation and release/P8 claim laundering.

Accepted evidence:
- accepted candidate source: `5e1ed9ae3640bb6ffa3f33cc2be735041399022f`;
- workflow run: `36864447147` — SUCCESS;
- closeout artifact: `11163263388`, digest `sha256:547f48bb3d43805003afc24bffff727b3b0a38083ba81ccf36219dcc9c5011c8`;
- frozen A5 attack artifact: `11163336806`, digest `sha256:ef5c9b6b1a43c4fe0f69ce6632f328dc1f509bf8cede52d07ef8e76cd2b7d0cd`;
- exact frozen A5 corpus: **12/12 PASS**;
- original P6 non-ranking and negative-evidence boundaries: PASS;
- retained historical P6 harness failure `36770551463`: independently rechecked as failure;
- A1 Semantics-v3, A2 proposition authority, A3 attestation authority, A4 variance and P3 poisoning reproofs: PASS;
- immutable public anchors: PASS.

Retained first A5 execution:
- run `36864270080` at `f543f6c62c9939caa321cc5eccc2e8a4c4470b00` failed only in a governance grep whose literal text did not account for Markdown emphasis in the frozen P6 protocol;
- the A5 12-test scientific corpus itself already passed in that run;
- correction `5e1ed9ae3640bb6ffa3f33cc2be735041399022f` changed only that grep to a Markdown-safe invariant substring;
- no test, source pin, classification, negative-evidence requirement, acceptance rule or semantic boundary changed.

Current A5 decision:
`POST_ALPHA4_PROMOTION_A5_P6_VALIDATION_TOOLING_ELIGIBLE_BOUNDED_CANDIDATE`

Retained A5 boundaries:
- proposition-local facts only;
- no composite score/rank/tier/winner or overall superiority/inferiority verdict;
- source-pin mismatch remains `STALE_SOURCE`/incomparable;
- missing/incomplete/not-testable evidence remains explicit;
- mandatory negative evidence cannot be filtered out of closeout;
- comparison results cannot confer Semantics-v3 authority, backend equivalence, product correctness, release readiness or P8 external validation;
- A5 is validation/tooling only and authorizes no public runtime feature.

## Next assessment

A6 evaluates **P7-other platform/CI research** individually, while keeping `P7-arm64` fixed at `NEGATIVE_RETAINED`.

A6 must not infer portability from platform count. Each candidate CI/platform path must preserve the same semantic/evidence contract, fail closed on unsupported/incomplete observation, retain platform-specific limitations, and must not use the successful P7 closeout to overwrite the bounded arm64 not-portable result.
