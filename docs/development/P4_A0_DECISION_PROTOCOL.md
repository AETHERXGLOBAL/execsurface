# ExecSurface — P4-A0 Decision Protocol

Date: 2026-09-29
Parent: #100
Branch: `development/post-alpha4-behavioral-integrity`

This decision protocol freezes how P4-A0 is reviewed before the review result is observed.

## Inputs

- `P4_BACKEND_AUTHORITY_PROTOCOL.md`
- `P4_A0_PROPOSITION_INVENTORY.md`
- `P4_A0_REVIEW_MATRIX.md`
- accepted P2 Semantics-v3 contracts and retained counterexamples
- current public ptrace limitations / M11 fail-closed contract

## Independent review procedure

1. Verify every proposition has subject/object identity requirements.
2. Verify every proposition has explicit authority and completeness semantics.
3. Verify unsupported/ambiguous/lost states cannot be interpreted as absence.
4. For each A0 review matrix row, derive the only allowed safe state from the contract.
5. Search the frozen contract for any wording/path that could still permit `direct+complete` despite missing identity, causal evidence, or health prerequisites.
6. Verify no global backend-equivalence predicate or implication exists.
7. Verify the contract does not change public v2 observation/verdict behavior.
8. Record any contradiction as a blocking falsification; do not patch after deciding PASS.

## Pass criteria

All are mandatory:
- all ten propositions are scoped;
- all twelve review rows have a safe classification;
- no review row permits a false `direct+complete` result;
- global backend equivalence remains impossible by construction;
- public alpha.4 behavior is unchanged;
- no runtime adapter implementation has occurred before this review.

## Allowed decisions

- `P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`
- `P4_A0_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A0_INCOMPLETE_CONTRACT`

Only an accepted bounded decision may authorize P4-A1 implementation.
