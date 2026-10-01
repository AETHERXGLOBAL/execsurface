# ExecSurface — Post-alpha.4 Promotion & Integration Protocol

Date: 2026-10-01
Tracking: #115
Parent program: #100
External-validation gate: #114
Candidate branch: `integration/post-alpha4-promotion-candidate`
Candidate base: `7bd806258ecdca415ef16ffb3689aefa13c0ece2`
Status: **A0 PREREGISTERED — NO PRODUCT PROMOTION AUTHORIZED**

## Purpose

This protocol defines how bounded research results from the post-alpha.4 program may be assessed for a future product candidate without converting research evidence into public-product claims by default.

The central rule is:

`RESEARCH DECISION -> PROMOTION ELIGIBILITY -> COMPATIBILITY/FALSIFICATION -> CANDIDATE IMPLEMENTATION -> REPROOF -> RELEASE DECISION`

Never:

`RESEARCH PASS -> PUBLIC FEATURE`

## Fixed roles

1. **Innovation Scientist / Systems Architect** — identify the smallest high-leverage integration that advances the north star without copying competitor breadth.
2. **Anti-Drift / Scientific Integrity Reviewer** — block gate skipping, claim inflation, convenient reinterpretation, threshold weakening and hidden compatibility changes.
3. **Independent Falsifier / Red Team** — construct false-PASS, false-REVIEW, migration, rollback, provenance, authority and compatibility counterexamples.
4. **Independent Critical-Milestone Reviewer** — independently verify source evidence, decision boundaries, product delta, rollback path and claims wording.

## Dynamic specialists

Select per gate:
- release / promotion engineer;
- runtime semantics / programming-languages engineer;
- Linux runtime / backend-authority engineer;
- supply-chain attestation / in-toto / SLSA / Sigstore specialist;
- CI / portability engineer;
- backward-compatibility / migration engineer;
- reproducibility / test-infrastructure engineer;
- privacy / data-minimization reviewer when evidence shape changes.

## Immutable public boundary

Until a separately closed release gate authorizes otherwise:

- public release remains `v0.1.0-alpha.4`;
- immutable public release source remains `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable Action remains `AETHERXGLOBAL/execsurface@v0.1` and its current tag target must remain the alpha.4 source;
- public alpha.4 semantics are not silently reinterpreted;
- public raw/canonical/baseline v2 meaning is not silently rewritten;
- public default/correctness-reference observer is not silently replaced;
- `main` is not a target of this A0 gate.

A future version may intentionally change semantics only through an explicit versioned migration and release decision. Compatibility must be described, tested and reversible/fail-closed.

## Hard scientific boundaries

The candidate MUST preserve these rules:

- no silent v2 -> v3 reinterpretation;
- no automatic baseline poisoning or authorization by observation frequency;
- unsupported evidence is not evidence of absence;
- ambiguous/lost/incomplete observation cannot become PASS;
- backend name does not confer semantic authority;
- signature, provenance or standards-label presence does not confer behavioral authority;
- no global backend score may replace proposition-scoped authority/completeness;
- ptrace/eBPF/external traces are not interchangeable without proposition-specific proof;
- no broad whitelist used to manufacture compatibility;
- no deletion or rewriting of retained negative evidence;
- no threshold or success-criterion change after seeing results.

## P8 dependency boundary

P8 is externally dependent and remains open until its preregistered current-evidence minimum is satisfied.

Internal promotion work:
- MUST NOT close P8;
- MUST NOT relabel internal CI as external validation;
- MUST NOT describe outreach/routing/praise as validation, adoption or endorsement;
- MAY prepare a candidate branch and tests while keeping release authorization blocked.

The candidate release decision remains blocked while P8 is in `P8_EXTERNAL_EVIDENCE_INSUFFICIENT` unless the parent release gate is explicitly amended by new evidence and a separately documented decision. A0 does not amend it.

## Platform boundary

The retained P7 arm64 result is negative/bounded. Therefore:
- Linux arm64 is **not** promoted as supported by this candidate;
- no parity claim is authorized;
- any future arm64 promotion requires a fresh preregistered portability/evidence gate and must retain the original failure.

## Promotion states

Every inventory item uses exactly one of:

- `UNASSESSED` — research result exists but product-promotion eligibility has not been tested.
- `ELIGIBLE_BOUNDED` — a dedicated promotion gate established bounded eligibility; still not a release.
- `DEFER` — useful research result intentionally withheld from the current candidate.
- `BLOCKED` — dependency or unresolved material risk prevents promotion assessment/implementation.
- `NEGATIVE_RETAINED` — tested direction failed or was bounded negatively; must remain visible and cannot be promoted as success.

A0 permits no product-affecting item to begin as `ELIGIBLE_BOUNDED`.

## Required evidence before ELIGIBLE_BOUNDED

For a product-affecting item, all of the following are mandatory:

1. exact research source decision / evidence locator;
2. explicit proposition and claim boundary;
3. public semantic delta analysis;
4. backward-compatibility and migration analysis;
5. fail-closed behavior reproof;
6. adversarial falsification specific to the integration;
7. rollback or safe-disable path;
8. negative-evidence retention check;
9. reproducible CI evidence;
10. independent critical-milestone review.

If any mandatory element is absent, the item remains `UNASSESSED`, `DEFER`, `BLOCKED`, or `NEGATIVE_RETAINED`.

## Candidate ordering

Assessment order is dependency-driven, not feature-count-driven:

1. **P2 Semantics v3** — defines contracts used by downstream authority/evidence work.
2. **P4 proposition authority / backend adapter architecture** — only after P2 compatibility boundary is known.
3. **P5 attestation/provenance composition** — only after semantic/authority identifiers are stable enough to bind.
4. **P3 legitimate variance** — integrate only if it demonstrably lowers false REVIEW without creating false PASS or implicit authorization.
5. **P6 comparison/falsification tooling** — assess separately from runtime product code; benchmark evidence does not itself become a runtime feature.
6. **P7 CI/platform results** — promote only individually; arm64 remains `NEGATIVE_RETAINED` unless separately re-proved.
7. **P8 external findings** — enter the inventory only when qualified external evidence exists; never synthesize it internally.

Ordering may change only if a documented dependency/falsification result requires it. Anti-drift review must record the reason.

## A0 success criterion

A0 passes only if:
- candidate branch is separate from `main`;
- `v0.1.0-alpha.4` resolves to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- `v0.1` resolves to the same public alpha.4 source;
- the candidate contains the immutable public source in ancestry;
- the promotion inventory starts with no product-affecting `ELIGIBLE_BOUNDED` entry;
- P8 is represented as `BLOCKED`/external-input pending;
- arm64 is represented as `NEGATIVE_RETAINED`;
- A0 itself changes governance/test artifacts only;
- no release, `main` merge, or tag movement occurs.

## A0 decisions

Permitted outcomes:

- `POST_ALPHA4_PROMOTION_A0_PROTOCOL_PASS`
- `POST_ALPHA4_PROMOTION_A0_PROTOCOL_FAIL`
- `POST_ALPHA4_PROMOTION_A0_INCOMPLETE`

Only PASS allows A1 promotion assessment to start. PASS still authorizes **zero** public product integration by itself.
