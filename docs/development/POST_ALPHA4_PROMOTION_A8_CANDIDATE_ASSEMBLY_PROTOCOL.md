# ExecSurface — Promotion A8 Clean Product-Candidate Assembly Protocol

Date: 2026-10-01
Tracking: #115
Parent program: #100
External-validation gate: #114
Promotion evidence branch: `integration/post-alpha4-promotion-candidate`
Frozen promotion closeout source: `cef8c5b8acb3f3c9e0671c497ad04837709bb024`
Immutable public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Status: **PREREGISTERED — CLEAN ASSEMBLY ONLY / NO PUBLIC RELEASE AUTHORIZED**

## Decision premise

A7 established bounded internal survival of the composed promotion evidence, but the promotion branch contains research history, experiments, workflows and product-adjacent changes accumulated after alpha.4. It is therefore **not** itself a releasable product branch.

A8 forbids a wholesale merge of the promotion branch into `main` or a release branch.

The release-candidate implementation MUST instead be assembled from the immutable alpha.4 source and admit product changes one bounded item at a time.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — choose the smallest product delta that captures the strongest proven value without importing research breadth.
2. **Anti-Drift / Scientific Integrity Reviewer** — block research-history leakage, silent semantics changes, scope creep, claim inflation and post-result gate weakening.
3. **Independent Falsifier / Red Team** — attack migration, rollback, authority, ambiguity, baseline compatibility, public-action behavior and clean-branch provenance.
4. **Independent Critical-Milestone Reviewer** — independently verify ancestry, admitted-file inventory, evidence locators, release blockers and decision wording.

## Dynamic specialists

- Rust API / semver / feature-gating;
- runtime semantics / schema evolution;
- Linux observer / proposition authority;
- supply-chain attestation / provenance;
- CLI / GitHub Action compatibility;
- release packaging / crates.io / Marketplace;
- reproducibility / CI / rollback;
- privacy / data-minimization.

## Clean assembly branch rule

The dedicated internal assembly branch is:

`release/post-alpha4-candidate-assembly`

It MUST be created directly from:

`48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

It MUST NOT be created from, merged from, or fast-forwarded to the 595-commit promotion/research lineage.

Every admitted product change must be independently attributable to an A1–A7 eligible item and must have a separately testable rollback/safe-disable path.

## Candidate scope freeze

### Eligible for product-candidate implementation

1. **P2 / Semantics v3**
   - explicit side-by-side version domain only;
   - v2 remains the default public semantic domain;
   - no silent v2→v3 projection;
   - candidate integration must be opt-in or otherwise explicitly version-selected;
   - proof admission remains proposition-bound and non-vacuous;
   - incomplete/ambiguous/unsupported evidence remains non-admissible.

2. **P4 / proposition authority**
   - only proposition-scoped authority contracts needed by the candidate v3 path;
   - backend/profile names never confer authority;
   - unsupported proposition families remain unsupported;
   - no backend equivalence or baseline interchangeability claim.

3. **P5 / attestation-provenance binding**
   - only if bound to an actual candidate artifact/evidence path;
   - cryptographic/signature/provenance validity never manufactures semantic authority;
   - no custom-standard novelty claim.

4. **P3 / bounded GCC legitimate variance**
   - only the exact frozen GCC producer/role/grammar projection;
   - no learned acceptance, frequency authorization or broad temporary-path suppression;
   - raw evidence retained.

### Validation/tooling only — not shipped as runtime features by default

- P6 comparison/falsification tooling;
- P7 GitLab/self-hosted contract harnesses;
- historical research experiments and milestone workflows.

### Explicitly excluded

- Linux arm64 support/parity claim;
- BPF-LSM default backend promotion;
- backend interchangeability;
- automatic v2→v3 migration;
- automatic baseline mutation;
- generic variance learning;
- broad cache/temp whitelists;
- P8 external-validation claims.

## A8-A0 — clean provenance / product-delta freeze

Before any candidate implementation:

- create `release/post-alpha4-candidate-assembly` from exact alpha.4 source;
- verify merge-base and branch head initially equal alpha.4;
- record the promotion evidence source `cef8c5b...` as evidence only, never as branch ancestry;
- freeze an admitted-file manifest before each implementation tranche;
- forbid research workflows/experiments/docs from being copied merely because they exist on the promotion branch.

A8-A0 success decision:

`POST_ALPHA4_PROMOTION_A8_A0_CLEAN_ASSEMBLY_BASE_PASS`

## A8-A1 — Semantics v3 candidate implementation

The first product tranche is Semantics v3 because downstream contracts depend on it.

Mandatory implementation boundary:

- explicit version discriminator;
- v2 default behavior unchanged;
- v3 candidate API must not be silently reachable through v2 parser/verifier/digest paths;
- prefer a default-off Cargo feature or equivalently explicit compile/runtime selection until the release gate says otherwise;
- no CLI default behavior change in A8-A1;
- no baseline format rewrite;
- rollback/safe-disable must restore alpha.4-compatible v2 behavior without data mutation.

Mandatory tests:

- repaired D01–D06 destructive corpus;
- original A1 promotion corpus;
- v2 model/baseline/diff/policy regression;
- feature/default-off or explicit-selection proof;
- v2↔v3 `INCOMPARABLE_SCHEMA` boundary;
- deterministic serialization;
- unsupported schema fail-closed;
- clean-branch admitted-file manifest check.

## A8-A2 — proposition-authority candidate implementation

Only after A8-A1 passes. Must reprove:

- proposition binding;
- backend-name anti-inflation;
- success/failure/attempt distinction;
- incomplete/lost/ambiguous fail-closed;
- no unsupported-family promotion;
- no ptrace/external-import equivalence assumption.

## A8-A3 — bounded variance candidate implementation

Only after A8-A2. Must prove lower bounded false REVIEW with zero new false PASS and exact GCC grammar confinement.

## A8-A4 — attestation/provenance candidate binding

Only after a real candidate artifact/evidence identity exists. Must bind subject/source/current/baseline/verifier/provenance without upgrading semantic authority.

## A8-A5 — public-surface compatibility reproof

Before any release decision:

- CLI alpha.4-compatible default behavior;
- GitHub Action install and execution path;
- crates/package build from exact candidate source;
- baseline v2 read/check compatibility;
- rollback/safe-disable;
- privacy/loss/truncation fail-closed regression;
- no arm64/public GitLab/public self-hosted claim leakage;
- immutable alpha.4 and stable `@v0.1` remain untouched.

## P8 release blocker

A8 may assemble and test an internal candidate, but **public release authorization remains blocked while P8 is `P8_EXTERNAL_EVIDENCE_INSUFFICIENT`** under the frozen promotion protocol.

Internal A8 evidence cannot be relabeled as external validation.

## Kill criteria

Any of the following blocks the affected tranche:

- research branch merged wholesale into the clean assembly branch;
- v2 default semantics change without explicit versioned release decision;
- v3 becomes silently accepted as v2 or vice versa;
- ambiguity/loss/incomplete/unsupported becomes PASS-admissible;
- backend/CI/signer/verifier metadata manufactures authority;
- variance hides meaningful drift or repetition becomes authorization;
- release artifact cannot be traced to exact clean-branch source;
- rollback requires destructive migration;
- arm64 or external-validation claims appear without new evidence;
- negative evidence is removed or relabeled.

## Allowed A8 outcomes

For each tranche:

- `ELIGIBLE_BOUNDED_CANDIDATE_IMPLEMENTATION`
- `DEFER`
- `BLOCKED_MATERIAL_GAP`
- `INCOMPLETE`

A8 as a whole may close internally only as:

`POST_ALPHA4_PROMOTION_A8_INTERNAL_CANDIDATE_ASSEMBLED_BOUNDED_RELEASE_BLOCKED_P8`

or a bounded blocked/incomplete decision.

No A8 outcome by itself moves `main`, `v0.1`, `v0.1.0-alpha.4`, publishes crates/releases, or closes P8.
