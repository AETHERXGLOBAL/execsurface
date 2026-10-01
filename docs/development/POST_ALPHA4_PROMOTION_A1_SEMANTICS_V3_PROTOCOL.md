# ExecSurface — Promotion A1: Semantics v3 Eligibility / v2 Preservation Protocol

Date: 2026-10-01
Tracking: #115
Parent research: #101 / #100
Candidate branch: `integration/post-alpha4-promotion-candidate`
A0 source: `c3c4cdadb34c77c2dba3d554c17d98d562a6e4c4`
Status: **PREREGISTERED — NO V3 PRODUCT INTEGRATION AUTHORIZED**

## Question

Can the bounded P2 Semantics v3 design be admitted to a future product-candidate implementation **without** reinterpreting or weakening the immutable public v2 contracts?

## Hypothesis

A side-by-side, version-dispatched v3 path is eligible for candidate implementation if and only if:

1. every current v2 schema/digest/verdict domain remains exactly v2;
2. v2 artifacts remain verified only by v2 rules;
3. v3 proof-carrying evidence occupies an explicit distinct version domain;
4. v2 evidence cannot become PASS-eligible v3 evidence by defaulting, projection, backend naming, canonical-string equality or format conversion;
5. cross-schema verification defaults to `INCOMPARABLE_SCHEMA`;
6. proof incompleteness/ambiguity/unsupported state remains fail-closed;
7. candidate v3 work can be disabled/removed without altering historical v2 verification;
8. no public CLI/default/release/tag change occurs in A1.

A1 tests **eligibility for candidate implementation**, not release readiness.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — seek the smallest versioned architecture that gains proposition-scoped authority/completeness without duplicating or replacing stable v2 unnecessarily.
2. **Anti-Drift / Scientific Integrity Reviewer** — block silent reinterpretation, convenience projection, threshold changes and research-to-product claim inflation.
3. **Independent Falsifier / Red Team** — attack schema confusion, authority laundering, downgrade/upgrade substitution and fail-open defaults.
4. **Independent Critical-Milestone Reviewer** — independently verify v2 anchors, attack coverage, result provenance and bounded wording.

## Dynamic specialists

- runtime semantics / programming-languages engineer — lead;
- schema/versioning and backward-compatibility engineer;
- Linux runtime proposition-authority specialist;
- reproducibility/test-infrastructure engineer;
- supply-chain attestation specialist only where identity/version binding is relevant.

## Frozen evidence inputs

A1 reuses, but does not reopen, the following closed P2 evidence:

- `SEMANTICS_V3_CURRENT_CONTRACT_AUDIT.md` — `S0_CURRENT_CONTRACT_AUDIT_ACCEPTED`;
- `SEMANTICS_V3_AUTHORITY_COMPLETENESS_MODEL.md` — `S2_S3_AUTHORITY_COMPLETENESS_MODEL_ACCEPTED_FOR_PROTOTYPE`;
- `SEMANTICS_V3_S4_PROTOTYPE_RESULT.md` — `S4_PROOF_CARRYING_PROTOTYPE_PASS`;
- `SEMANTICS_V3_S7_COMPATIBILITY_RESULT.md` — `S7 CLOSED — PASS_RESTRICTED (DESIGN FREEZE)`;
- `SEMANTICS_V3_S8_REDTEAM_RESULT.md` — `P2_SEMANTICS_V3_DESIGN_ACCEPTED_BOUNDED`.

The retained S4 CI failures and all earlier negative evidence remain part of the provenance; A1 does not erase them.

## Immutable v2 anchors

A1 freezes these implementation facts as compatibility inputs:

- raw observation schema: `2`;
- canonical surface schema: `2`;
- baseline lock schema: `2`;
- baseline digest format: `2`;
- diff schema: `2`;
- policy schema: `2`;
- verdict schema: `2`.

The research proof-carrying prototype uses explicit schema `3`.

A1 MUST fail if any of the v2 constants above change.

## Frozen semantic delta

### v2

- session/backend-scoped metadata;
- global observation completeness;
- canonical behavior without proposition-scoped proof requirements;
- deterministic v2 baseline/digest contract;
- no proof-carrying proposition authority.

### candidate v3 semantic direction

- explicit proposition identity;
- guarantee-set authority, never a scalar backend trust score;
- typed completeness dimensions and dependency propagation;
- canonical proof profile separated from unstable forensic links;
- proof-aware behavior/evidence comparison;
- explicit unsupported/ambiguous/incomplete states.

Canonicalization MUST NOT promote weak source semantics into a stronger proposition.

## Version / migration contract

Frozen default architecture:

```text
bytes
  -> explicit schema discriminator
      -> v2 parser + v2 verifier + v2 digest rules
      -> v3 parser + v3 verifier + v3 proof rules
      -> unsupported version error
```

Rules:

1. a permissive v3 reader MUST NOT be used as the v2 reader;
2. v2 observer names/capability strings MUST NOT imply v3 guarantees;
3. v2 `complete=true` MUST NOT manufacture per-proposition v3 completeness;
4. equal canonical strings MUST NOT establish cross-schema equality;
5. `v2 baseline vs v3 candidate` => `INCOMPARABLE_SCHEMA` by default;
6. `v3 baseline vs v2 candidate` => `INCOMPARABLE_SCHEMA` by default;
7. no default PASS-eligible v2 -> v3 projection exists;
8. any future migration is explicit, non-destructive, preserves the original v2 artifact/digest as provenance, creates a distinct v3 artifact, and reacquires fresh evidence for proof facts absent from v2;
9. format conversion alone can never claim unchanged runtime verification.

A1 does not authorize a migration command.

## Rollback / safe-disable boundary

Any later candidate implementation admitted by A1 must be side-by-side and non-default until a later gate closes.

Required rollback property:

> Removing/disabling candidate v3 code must leave historical v2 parsing, digest verification, diff, policy/verdict and public alpha.4 behavior unchanged.

No shared mutable schema constant may make rollback alter v2 meaning.

## Preregistered attacks

A1 must execute and retain at least these attacks:

### A1-01 — v2-shaped payload presented to v3 proof reader
Expected: parse rejection or otherwise non-admissible; never PASS-eligible.

### A1-02 — fully shaped proof record with schema version `2`
Expected: proof admission rejects it.

### A1-03 — required completeness dimension absent
Expected: non-admissible.

### A1-04 — required completeness marked `incomplete`
Expected: non-admissible.

### A1-05 — required completeness marked `ambiguous`
Expected: non-admissible.

### A1-06 — required completeness marked `unsupported`
Expected: non-admissible.

### A1-07 — weak pathname evidence with authoritative-sounding backend name
Expected: cannot satisfy `kernel_object_grounded` requirement.

### A1-08 — equal behavioral value with different proof authority
Expected: different evidence; no silent equality.

### A1-09 — insertion-order variation in sets/maps
Expected: deterministic tested canonical prototype serialization.

### A1-10 — public v2 constants drift
Expected: gate failure.

### A1-11 — alpha.4 / stable tag drift
Expected: gate failure.

### A1-12 — A1 modifies public runtime paths
Expected: gate failure. A1 may touch only this protocol, its dedicated promotion test, and its workflow.

## Execution gate

The dedicated A1 workflow must:

- verify public `v0.1.0-alpha.4` and stable `v0.1` still resolve to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- assert every frozen v2 constant above remains `2` and prototype v3 remains `3`;
- run Rust formatting;
- run model Clippy with `-D warnings`;
- run the dedicated A1 adversarial promotion test;
- re-run model/baseline/diff/policy tests under the lockfile;
- prove A1 changed no runtime/public paths;
- emit a retained evidence artifact.

No metric or threshold may change after execution.

## Success criterion

A1 passes only if **all** preregistered executable checks pass with no weakening and no public/runtime path changes.

Allowed outcomes:

- `POST_ALPHA4_PROMOTION_A1_P2_ELIGIBLE_BOUNDED_FOR_CANDIDATE_IMPLEMENTATION`
- `POST_ALPHA4_PROMOTION_A1_P2_DEFER`
- `POST_ALPHA4_PROMOTION_A1_P2_BLOCKED`
- `POST_ALPHA4_PROMOTION_A1_P2_FAIL`

Even the first outcome authorizes only a later isolated candidate implementation gate. It does not authorize public v3, migration, `main` merge, release, tag movement or P8 closure.
