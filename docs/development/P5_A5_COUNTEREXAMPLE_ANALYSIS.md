# ExecSurface — P5-A5 Cross-Attestation Counterexample Analysis

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P5_A5_CROSS_ATTESTATION_REDTEAM_PROTOCOL.md`
Status: **COUNTEREXAMPLES RETAINED — CORRECTION BOUNDARY PREREGISTERED**

## Evidence retained

The first scientifically executable A5 run was:

- source SHA: `a7753bee6cc9f145adc7823dd82837fa40cfca01`
- workflow run: `36755962689`
- job: `110026152357`
- evidence artifact: `11116268894`
- artifact name: `p5-a5-36755962689-1`
- artifact SHA-256: `97aa4db53e27355d8b2d62e86d6f1ba5c2ee6d7d6e10607b965e4730fa878314`
- frozen A5 corpus result: **8 passed / 4 failed**

The earlier run `36755501882` failed only in `cargo fmt --check` before the A5 corpus ran and remains classified as pre-scientific static evidence. It is not substituted for or merged with the scientific 8/12 result.

## Confirmed counterexamples

### A5-05 — baseline substitution

Observed:
`baseline substitution: adversarial graph unexpectedly verified`

A competent attacker could change the baseline digest and recompute attacker-visible outer SCAI/SVR/bundle digests. The resulting graph remained internally self-consistent and passed the isolated bundle verifier because no trusted expected execution context was supplied at graph verification time.

### A5-06 — current-surface substitution

Observed:
`current-surface substitution: adversarial graph unexpectedly verified`

The same class of gap exists for the current-surface digest. Internal hash consistency is not equivalent to verification against the expected execution state.

### A5-10 — source substitution

Observed:
`source substitution unexpectedly matched expected execution context`

The existing `verify_bundle_for_context` bound workflow, command and host, but not source identity. The workflow subcase rejected; the source subcase survived.

### A5-11 — duplicate SVR semantic property

Observed:
`duplicate SVR semantic property: adversarial graph unexpectedly verified`

The isolated verifier required named SVR properties to be present but did not enforce a single canonical, duplicate-free property representation. Recomputing outer digests therefore allowed a duplicate semantic item to survive verification.

The reorder subcheck in A5-11 did not execute because the duplicate-property assertion failed first. No result is claimed for that subcheck from run `36755962689`.

## Scientific classification

These are **implementation-level cross-attestation binding/canonicalization gaps**.

The run does **not** establish a representation gap in in-toto Runtime Trace, SCAI, SVR, SLSA provenance, GitHub Attestations, or Sigstore. The missing invariant is the verifier-side composition of already representable evidence against a trusted expected context and a canonical graph representation.

Therefore this evidence does not authorize a proprietary predicate or a new attestation standard.

No public alpha.4 behavior is implicated: all affected code is research/development-only P5 composition code.

## Frozen correction boundary

No A5 attack, threshold or expected outcome is changed. Acceptance remains exactly **12/12**.

Only these corrections are authorized:

1. **Trusted expected-context graph verification**
   - require subject, source, artifact, workflow, command, host, baseline, current surface, evidence, observer profile, capability state, observer health, authority, completeness, policy, verdict, verifier identity, verification timestamp and provenance to match the caller-supplied expected execution context;
   - internal self-consistency alone is insufficient.

2. **Canonical graph/SVR semantics**
   - require graph roles to be unique and from the frozen allowed role set;
   - require SVR semantic properties to be strictly sorted and duplicate-free at graph verification time;
   - an arbitrary reorder is rejected as noncanonical;
   - reconstructing the canonical order must recover the original semantic identity.

3. **Harness binding correction only**
   - A5-05, A5-06 and A5-10 will verify the attacked bundle through the preregistered trusted expected-context graph verifier rather than the isolated self-consistency verifier;
   - A5-11 will exercise both duplicate rejection and canonical reconstruction. This is a correction of the verification surface required by the frozen attack prose, not a change to the attack or threshold.

## Anti-drift constraints

The correction must not:

- weaken or remove any of the twelve attacks;
- treat a signature, provenance record, backend/profile name or standards label as semantic authority;
- change the public crates or alpha.4 semantics;
- infer a SLSA level;
- delete the 8/12 failure evidence;
- introduce a new predicate;
- make an internally rehashed graph authoritative without caller-supplied expected context.

## Next execution rule

After the bounded correction, the exact twelve-attack A5 gate must rerun from scratch. A5 cannot close unless it reaches 12/12 and the same run also re-verifies accepted A4, all prior P5 gates, critical P4 authority gates, Semantics v3, public M11, and immutable public tags.
