# ExecSurface — Post-alpha.4 Promotion A2 P4 Proposition Authority Decision

Date: 2026-10-01
Tracking: #115
Protocol: `POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_PROTOCOL.md`
Candidate branch: `integration/post-alpha4-promotion-candidate`

## Decision

**`POST_ALPHA4_PROMOTION_A2_P4_AUTHORITY_REQUALIFIED_BOUNDED_CANDIDATE`**

P4 proposition-scoped authority / backend-adapter research is eligible to continue in the post-alpha.4 candidate under the repaired Semantics-v3 proof-admission boundary. This is a bounded internal promotion decision only.

It does **not** authorize merge to `main`, public release, stable `@v0.1` movement, backend equivalence, baseline interchangeability, automatic v2→v3 migration, public backend selection, production-readiness claims, external-validation claims, or P8 closure.

## Frozen target identities

- candidate evidence source: `d02151e8f2d1f6b0a6558d52da6f48e74763e0a5`
- repaired A1 floor: `3522cc87b5b23a42e8009e779549e5dcca95da5b`
- P4 closeout source: `320c865d3e81071a8214ad726dca7a587b0fc479`
- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action remains `AETHERXGLOBAL/execsurface@v0.1`

## Accepted execution evidence

Workflow:
- run: `36855716538`
- workflow: `.github/workflows/post-alpha4-promotion-a2-p4-authority.yml`
- conclusion: `SUCCESS`

Artifact:
- id: `11159141703`
- name: `post-alpha4-promotion-a2-p4-authority-36855716538-1`
- digest: `sha256:1be78620a347019db2461745a88897d8f82ec44ff190070f5598d6d1faee7ed2`
- expiry: `2026-12-30T11:29:45Z`

## Passed frozen gates

The accepted run completed all required jobs successfully:

1. candidate ancestry / immutable public boundary;
2. exact A2 authority-gap matrix;
3. P4 core proposition-authority library;
4. promotion-specific S01-S10 sensitivity corpus;
5. P4 B0 success-evidence contract;
6. cross-proposition falsification;
7. live ptrace open-object corpus;
8. live ptrace rename/delete corpus;
9. live ptrace connect corpus;
10. pinned Tetragon external-import corpus;
11. repaired Semantics-v3 D01-D06 + A1 reproof;
12. M11 shared-FD fail-closed reproof.

Observed bounded test counts in the accepted A2 authority job:
- A2 authority-gap matrix: `16/16 PASS`;
- historical authority-gap matrix: `10/10 PASS`;
- attempt-authority contract: `5/5 PASS`;
- P4 authority library: `8/8 PASS`;
- B0 success-evidence contract: `18/18 PASS`;
- S01-S10 promotion sensitivity attacks: `10/10 PASS`.

## What survived falsification

Within the tested scope:

- backend/profile naming cannot rescue a proposition mismatch;
- a matching direct record satisfies only its exact bound proposition contract;
- `AttemptOnly` cannot be laundered into successful-open authority;
- `Unsupported`, `Ambiguous`, and `Lost` cannot satisfy complete proof requirements;
- `DerivedBounded` requires an explicit named derivation;
- different propositions remain different even under an empty/default requirement;
- the default/empty proof requirement cannot admit an otherwise valid record;
- unknown ambiguity does not silently grant authority;
- stronger guarantees cannot rescue a mismatched proposition;
- open-object, rename/delete and connect success authority remain proposition-specific and fail closed under their frozen counterexamples;
- external Tetragon import remains schema/proposition-scoped evidence and does not create backend equivalence.

## Retained limitations

This decision deliberately retains the following boundaries:

- `BACKEND_EQUIVALENCE_NOT_ESTABLISHED` remains true;
- ptrace, kernel-hook and external-import evidence are not baseline-interchangeable by default;
- public v2 semantics remain unchanged;
- unsupported proposition families remain unsupported;
- collector identity never acts as a global authority score;
- historical P4 negative evidence remains part of the record.

## Independent-review conclusion

No executed frozen A2 attack established a path to authority laundering, cross-proposition substitution, unsupported/ambiguous/lost promotion, backend-name inflation, or silent public-v2 reinterpretation.

The bounded A2 candidate therefore qualifies for the next internal gate only.

## Next authorized gate

**A3 — P5 Runtime Attestation / Provenance Promotion Eligibility.**

A3 must preserve the separation between cryptographic validity and semantic authority and must replay the retained P5 substitution, duplicate-property, verifier-identity and provenance-binding attacks before any attestation path is eligible for candidate integration.
