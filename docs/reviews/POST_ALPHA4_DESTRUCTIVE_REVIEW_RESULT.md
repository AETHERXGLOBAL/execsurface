# ExecSurface — Post-alpha.4 Destructive Falsification Review Result

Date: 2026-10-01  
Tracking: #116  
Promotion gate: #115  
External validation: #114

## Final classification

**`POST_ALPHA4_DESTRUCTIVE_REVIEW_MATERIAL_GAP_FOUND`**

This is an internal independent-style destructive review. It is **not** P8 external evidence and does not close #114.

## Frozen source under attack

- promotion candidate: `5079a990b924d8ccd7ac6414f8a9a2571b54e240`
- review branch: `review/post-alpha4-destructive-falsification`
- immutable public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`

The public alpha.4 tag and stable Action remained pinned to the immutable source throughout the review.

## Material findings

The preregistered D01–D03 destructive corpus produced **0 passed / 3 failed** on the frozen Semantics-v3 promotion candidate.

### D01 — proposition binding gap

A record for a distinct proposition/subject can satisfy the same generic `ProofRequirement` when guarantees and completeness dimensions match, because the current requirement/satisfaction API does not bind the required proposition identity.

Impact: a proof contract can be reused across a semantically different proposition instance.

### D02 — invalidating ambiguity gap

A record may claim a completeness dimension as `Complete`, carry an explicit invalidating ambiguity code for that dimension, and still satisfy `ProofRequirement` because `satisfies()` does not inspect `ambiguity_codes`.

Impact: contradictory proof state can remain admissible.

### D03 — empty requirement bypass

`ProofRequirement::default()` admits a schema-v3 record because empty guarantee/completeness constraints are vacuously satisfied.

Impact: callers can bypass the design rule that proposition kinds declare mandatory proof requirements unless a higher layer separately prevents the empty contract.

## Decision impact

The previous A1 9-test promotion corpus remains valid historical evidence and still replays successfully. The new counterevidence extends the attack surface and therefore supersedes the current promotion-eligibility conclusion.

Current decision for the Semantics-v3 promotion item:

**`A1_BLOCKED_REWORK_REQUIRED — MATERIAL_COUNTEREVIDENCE_FROM_ISSUE_116`**

The smallest future correction must pass the exact unchanged D01–D03 corpus. No assertion weakening, test deletion, broad exception, threshold change, or post-hoc semantic redefinition is permitted.

## What survived the destructive review

### Existing Semantics-v3 A1 corpus

PASS — the previously accepted 9-test promotion corpus still passes. This confirms the new result is a coverage extension rather than a falsification of the historical test execution.

### P3 variance falsifier

PASS — the P3 V4 poisoning / false-PASS falsifier replay succeeded. No new frequency-based authorization or variance-poisoning path was exposed by this review.

### P4 proposition-scoped authority

PASS — replay succeeded for:
- cross-proposition falsification;
- backend-name anti-inflation;
- live successful-open authority;
- live rename/delete authority;
- live connect success/failure/pending authority.

No new P4 false-authority path was exposed.

### P5 attestation / provenance

PASS after preserving and correcting review-wrapper harness defects only.

The review retained two harness failures:
1. missing experiment lockfile regeneration in the first wrapper;
2. missing pinned SLSA fixture in a later wrapper.

Neither reached the scientific assertion corpus.

After reproducing the original pinned fixture contract, the following replayed successfully:
- A5 cross-attestation red-team: 12/12;
- A3 pinned SLSA provenance binding: PASS;
- A2 SCAI/SVR binding: PASS;
- A2 verifier-identity addendum: PASS;
- A1 runtime-trace mapping: PASS;
- A0 standards composition: PASS.

Dedicated completion run: `36846427493` — SUCCESS.

### M11 fail-closed shared-FD hardening

PASS — the shared-FD ambiguity regression remains fail-closed.

### M12 portable adversarial hardening

PASS on both Ubuntu 22.04 and Ubuntu 24.04:
- bounded PATH-TOCTOU fixture;
- bounded shared-FD race fixture;
- `m12_adversarial` suite;
- existing ptrace Linux regression.

No regression was exposed in the public-alpha.4 portable hardening path.

### Immutable public boundary

PASS — `v0.1.0-alpha.4` and stable `v0.1` remained pinned to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.

## Key execution evidence

- initial destructive run: `36845569114`
- harness-corrected replay: `36846001705`
- expanded P3/P4/P5/M11/M12 replay: `36846283772`
- dedicated pinned P5 completion: `36846427493` — SUCCESS

The intentionally failing D01–D03 tests remain retained as negative evidence. They were not repaired or weakened in this review.

## Scope of the failure

The review does **not** establish that ExecSurface as a whole is unsound.

The material finding is localized to the current Semantics-v3 proof-requirement admission contract proposed for promotion. The following remain separately supported by the replayed evidence:
- public alpha.4 fail-closed hardening;
- P3 bounded variance falsification;
- P4 proposition-scoped backend authority experiments;
- P5 bounded standards/attestation composition;
- M11/M12 portable shared-FD and PATH-TOCTOU hardening.

No public v3 integration has occurred, so the discovered gap was found before public promotion.

## Required next gate

Open a separate Semantics-v3 rework gate. It must address at minimum:
1. proposition/subject binding in proof requirements or an equivalent non-bypassable verifier boundary;
2. explicit mapping from ambiguity codes/states to invalidated proof requirements;
3. rejection of empty/underspecified semantic proof contracts for acceptance/PASS eligibility;
4. proof that the fix does not break P4/P5/M11/M12 or v2 compatibility;
5. unchanged D01–D03 destructive corpus plus additional anti-bypass attacks.

Do not modify public alpha.4 or stable `@v0.1` as part of this rework.

## External-validation boundary

This review was intentionally executed by AETHER X as an independent-style internal falsification exercise. It cannot be relabeled as external reproduction, external architecture criticism, external interoperability guidance, adoption, endorsement, or P8 closure evidence.
