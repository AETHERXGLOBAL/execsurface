# ExecSurface — P4-A1.3U Attempt-Scoped Authority Usability Gate

Date: 2026-09-29
Parent: #108 / #107 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor: `P4_A1_3_FALSIFICATION_PASS_BOUNDED`
Status: **PREREGISTERED — CONTRACT-UTILITY FALSIFICATION MAY START**

## Why this gate exists

Independent review before the accepted A1.3 scientific result identified a bounded semantic usability gap between the frozen A0 vocabulary and the A1 authority API:

- A0 defines `attempt_only` as evidence that proves an attempted operation/path reference but not successful object-level effect.
- `P4.PATH.ACCESS_ATTEMPT` is explicitly an attempt proposition and is `attempt_only` by default.
- the current `AdapterRecord::admissible_for()` accepts only `direct | derived_bounded`, so a complete exact attempt record cannot satisfy even an attempt-scoped requirement.

The original A1.3 gate was already running when this review finding was recorded, so its frozen assertions were not modified. A1.3U is therefore a separate preregistered gate.

This is a false-negative/usability question, not permission to weaken success/object authority.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — recover useful attempt authority without weakening success semantics.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks any change that makes attempt evidence satisfy successful effect/object requirements.
3. **Independent Falsifier / Red Team** — attacks attempt-to-success laundering and malformed authority/proposition combinations.
4. **Independent Milestone Reviewer** — verifies preregistration, exact tests, retained failures, CI evidence and public boundary.

## Dynamic specialists

- programming languages / type-and-effect semantics
- filesystem pathname versus object identity
- network/rename attempt-versus-success semantics
- Rust API invariants
- CI/reproducibility evidence engineering

## Frozen question

Can `attempt_only` become usable for explicit attempt propositions while remaining structurally incapable of satisfying successful-object/effect propositions or stronger success-grounded proof requirements?

## Immutable boundaries

- no public/runtime collector behavior change;
- raw-v2 bytes remain unchanged;
- no public learn/check/baseline/policy behavior change;
- `P4.FILE.OPEN_OBJECT`, successful rename/delete and successful connect authority are not strengthened by this gate;
- no backend promotion/equivalence claim;
- no change to A1.2 evidence mapping identities;
- A1.3 PASS remains retained and is not relabeled;
- any failure remains retained.

## Frozen falsification cases

A1.3U must test all of the following before A1.4:

1. **Exact attempt usability** — a complete `P4.PATH.ACCESS_ATTEMPT` record with `attempt_only` and the matching pre-operation pathname guarantees must satisfy an attempt-scoped requirement.
2. **Success/object non-laundering** — the same record must fail a requirement demanding post-success/object-grounded guarantees.
3. **Authority/proposition shape guard** — `attempt_only` attached to a non-attempt proposition such as successful exec or FD attribution must be rejected by record validation.
4. **Rename/connect attempt containment** — raw-v2 rename/connect attempt records may remain descriptive `attempt_only`, but must not become admissible as the frozen successful `P4.FILE.RENAME_DELETE` / `P4.NET.CONNECT_DESTINATION` product propositions without explicit success semantics.
5. **Comparison containment** — an attempt-only record and a successful/direct record must never compare as equivalent when their proposition semantics differ.
6. **Regression reproof** — A1.1, A1.2, A1.3, Semantics-v3, and public M11 remain green.

## Predeclared correction boundary

If case 1 reproduces the false-negative while cases 2–5 show a bounded safe distinction is representable, the only authorized model correction is:

- allow `attempt_only` admissibility only for an explicit attempt proposition class;
- reject `attempt_only` record validation for non-attempt proposition shapes;
- preserve `proof.satisfies(requirement)` so stronger success/object requirements still fail;
- do not alter the raw-v2 mapping or public runtime.

If the distinction cannot be expressed without widening success authority, close as a retained limitation rather than weakening the contract.

## Allowed outcomes

- `P4_A1_3U_ATTEMPT_AUTHORITY_PASS_BOUNDED`
- `P4_A1_3U_RETAIN_FALSE_NEGATIVE_LIMITATION`
- `P4_A1_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A1_PUBLIC_CONTRACT_REGRESSION`

Only the first, followed by A1.4, may authorize full A1 closure.
