# ExecSurface — Post-alpha.4 Promotion A2 P4 Proposition Authority Protocol

Date: 2026-10-01
Tracking: #115, historical P4 #107
Status: PREREGISTERED — CANDIDATE-BRANCH REQUALIFICATION / NO PUBLIC PROMOTION

## Target

- candidate branch: `integration/post-alpha4-promotion-candidate`
- A1 repaired candidate floor: `3522cc87b5b23a42e8009e779549e5dcca95da5b`
- P4 closeout source: `320c865d3e81071a8214ad726dca7a587b0fc479`
- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action remains `AETHERXGLOBAL/execsurface@v0.1`

P4 historical evidence and negative runs remain authoritative. A2 is not allowed to rewrite P4 closure or claim backend equivalence.

## Team

Fixed roles:
1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team
4. Independent Critical-Milestone Reviewer

Dynamic specialists:
- proposition authority / formal semantics specialist;
- Linux ptrace and syscall-state authority engineer;
- Rust backend-adapter/API engineer;
- capability/completeness modeling specialist;
- cross-proposition substitution/replay adversarial tester;
- compatibility/migration engineer.

## Question

Can the P4 proposition-scoped authority model be admitted to the post-alpha.4 candidate without allowing collector identity, backend labels, proposition substitution, incompleteness, ambiguity, replay, or cross-proposition evidence to create semantic authority?

## Frozen architectural boundary

1. A collector/backend is an evidence source, never a global authority source.
2. Authority is proposition-scoped and proof-condition-scoped.
3. Backend name/profile cannot upgrade weak evidence.
4. `Unsupported`, `Ambiguous`, `Lost`, incomplete observer state, malformed evidence, replay or substitution cannot become success authority.
5. There is no global backend score.
6. ptrace/eBPF/external-import evidence is not baseline-interchangeable without explicit proof.
7. `BACKEND_EQUIVALENCE_NOT_ESTABLISHED` remains true.
8. Research modules that were intentionally non-exported from the P4 crate root are not promoted merely to enlarge API surface.
9. Public v2 semantics, alpha.4 and stable `@v0.1` remain unchanged.

## Frozen P4 inventory invariants

The A2 authority-gap matrix must retain exactly 16 proposition rows and exactly three success-authority requalification rows:

- `P4.FILE.OPEN_OBJECT`
- `P4.FILE.RENAME_DELETE`
- `P4.NET.CONNECT_DESTINATION`

No new success-authority row may be inferred from backend capability or collection coverage.

## Frozen success-authority semantics

### File open object
Success requires proposition-specific object evidence; pathname attempt alone is insufficient.

### Rename/delete
Success requires the bound syscall/result/object relation. Entry-only, wrong actor/object, replay, unsupported operation, observer loss or ABI ambiguity must fail closed.

### Connect destination
- return code `0` only is synchronous Success;
- `-EINPROGRESS` is Pending, never Success;
- other negative errno is Failure;
- entry-only is AttemptOnly;
- actor + entry + socket FD + destination binding are mandatory;
- malformed/truncated/unsupported sockaddr fails closed;
- FD reuse/replay cannot transfer authority;
- later socket IO/state cannot reclassify the connect result;
- observer loss/incompleteness blocks success.

## External import boundary

P4-C remains **schema-level / proposition-scoped import evidence**, not a second-backend authority claim.

Pinned Tetragon identity:
- source commit: `666efe6f91e3605ad58683ad226d759d9cf970ca`
- `api/v1/tetragon/events.proto` blob: `d6bd56769241da983f7e0042a54d5b3a81f40817`
- profile: `external-tetragon-json-import-v1`

External import must retain explicit completeness limits and cannot establish backend equivalence.

## Preregistered promotion-specific sensitivity attacks

Before execution, A2 freezes these additional attacks:

- S01 backend/profile-name mutation cannot rescue a proposition mismatch;
- S02 a matching direct record can satisfy only its explicitly bound proof contract;
- S03 `AttemptOnly` may be admissible only for exact `P4.PATH.ACCESS_ATTEMPT`, never relabeled as open-object success;
- S04 `Unsupported` cannot validate as `Complete`;
- S05 `Ambiguous` and `Lost` cannot validate as `Complete`;
- S06 `DerivedBounded` requires a named derivation identity;
- S07 different propositions compare as `DifferentProposition` even under an empty requirement;
- S08 default/empty proof requirement cannot admit an otherwise valid record;
- S09 unknown ambiguity cannot silently grant authority;
- S10 stronger guarantees cannot rescue a mismatched proposition.

## Frozen acceptance gate

A2 requires every item below to PASS:

1. candidate/public identity boundary;
2. exact A2 matrix invariants (16 rows / 3 requalifications / exact IDs);
3. P4 core library authority invariants;
4. promotion-specific S01-S10 sensitivity corpus;
5. P4 B0 success-evidence contract;
6. P4 cross-proposition falsification;
7. B1 live ptrace open-object corpus;
8. B2 live ptrace rename/delete corpus;
9. B3 live ptrace connect corpus;
10. P4-C external Tetragon import 14-test corpus using pinned schema identity;
11. repaired Semantics v3 D01-D06 and A1 reproof;
12. public M11 shared-FD fail-closed reproof;
13. immutable alpha.4/stable tags and no public v2 migration.

There is no composite score and no partial-pass promotion.

## Kill criteria

Any reproducible path that creates authority from backend identity, wrong proposition, wrong actor/object/destination, unsupported/ambiguous/lost evidence, incomplete observer state, replay/substitution, or later unrelated state blocks A2.

Any evidence that backend equivalence or baseline interchangeability was silently inferred also blocks A2.

Harness/static failures are retained separately and may receive only the smallest correction that does not alter assertions, attack meaning, semantics, or thresholds.

## Decision vocabulary

Only if all frozen gates pass:

`POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_REQUALIFIED_BOUNDED_CANDIDATE`

Otherwise:

`POST_ALPHA4_PROMOTION_A2_REQUALIFICATION_BLOCKED`

## Claim boundary

A PASS authorizes only continuation to A3 — P5 Attestation/Provenance promotion eligibility. It does not authorize `main` merge, public release, stable-tag movement, backend-equivalence claims, public backend selection, automatic v2→v3 migration, production-readiness claims, external validation, or P8 closure.
