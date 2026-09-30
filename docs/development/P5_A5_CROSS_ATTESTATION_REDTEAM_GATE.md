# ExecSurface — P5-A5 Cross-Attestation Red Team / Phase Decision Gate

Status: **PREREGISTERED — EXACTLY 12 GRAPH ATTACKS / RESEARCH-ONLY**
Date: 2026-09-30
Parent program: #100
Predecessor: `P5_A4_SIGNED_ATTESTATION_PASS_BOUNDED`
Development branch: `development/post-alpha4-behavioral-integrity`

## Question

Does the accepted Runtime Trace + SCAI + SVR + optional SLSA provenance composition remain fail-closed when attacked as one verification graph, including attacks that recompute internally consistent statement digests after substituting semantic context?

## Hypothesis

The standards composition is sufficient for the bounded P5 goal if a final verifier binds the internally valid graph to an explicit caller-supplied expected semantic context and a deterministic role-scoped graph manifest.

Internal consistency alone is intentionally not the A5 acceptance criterion. A graph that is valid for *some other* baseline, current surface, source, workflow, evidence set, provenance statement, or observer profile must be rejected when verified against the expected execution context.

No new in-toto predicate is required for this hypothesis. The A5 graph manifest is verifier-internal research state, not an attestation format.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — add only the smallest final graph/context binding needed to compose the accepted standards safely.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks self-consistency-only acceptance, new proprietary predicates, signature/brand authority inflation, hidden threshold changes, or public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — executes the twelve frozen cross-graph attacks below, including recomputation/replay attempts.
4. **Independent Critical-Milestone Reviewer** — verifies predecessor SHAs, exact attack count, retained failures, same-run reproofs and the final P5 decision boundary.

Dynamic specialists: attestation-graph semantics, replay resistance, context binding, deterministic hashing/domain separation, in-toto/SLSA interoperability, adversarial Rust test engineering.

## Immutable boundaries

- public `v0.1.0-alpha.4` and stable `v0.1` remain at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no public crate/default-observer change;
- A0-A4 accepted evidence is not rewritten;
- no new proprietary in-toto predicate;
- no backend/signature/GitHub/Sigstore/SLSA name grants authority;
- ambiguous/lost/incomplete/unsupported evidence cannot become PASS;
- a recomputed, internally self-consistent graph for a different context is not accepted for the expected context;
- failures are retained.

## A5 expected-context contract

The research-only A5 verifier must first execute the accepted `verify_bundle` internal-consistency checks, then bind the graph to explicit expectations for at least:

- subject identity;
- source identity;
- workflow identity;
- command and host identity;
- baseline digest;
- current-surface digest;
- evidence digest;
- observer profile;
- policy descriptor;
- authority;
- completeness state;
- verdict;
- exact optional SLSA `ProvenanceReference` including statement digest, subject digest and verified state.

A5 also introduces a verifier-internal graph manifest with role-scoped entries for Runtime Trace, SCAI, SVR and optional provenance. The manifest must:

- contain each required role exactly once;
- reject duplicate or unknown roles;
- bind each role to the correct role digest;
- canonicalize role ordering deterministically;
- domain-separate its own digest from P5 statement digests.

The graph manifest is not serialized as a new in-toto predicate and carries no independent semantic authority.

## Frozen attacks — exactly 12

1. `a5_01_mix_and_match_subjects_fail_closed`
   - mix a statement/subject from another valid graph;
   - rejection is required even if outer digests are recomputed.

2. `a5_02_stale_runtime_trace_with_fresh_verdict_fails_closed`
   - combine stale Runtime Trace with a fresh SCAI/SVR verdict graph;
   - recomputation must not repair the runtime-trace binding.

3. `a5_03_fresh_trace_with_stale_policy_fails_closed`
   - substitute an older/different policy into an otherwise fresh graph;
   - graph verification against the expected policy must reject it.

4. `a5_04_provenance_replay_fails_expected_context_binding`
   - replace the expected verified SLSA provenance reference with another syntactically valid, verified reference for the same subject but a different statement digest;
   - self-consistency is insufficient; expected-context verification must reject replay.

5. `a5_05_baseline_substitution_fails_expected_context_binding`
   - substitute baseline digest and recompute all affected statement/bundle digests;
   - expected baseline binding must still reject it.

6. `a5_06_current_surface_substitution_fails_expected_context_binding`
   - substitute current-surface digest and recompute all affected digests;
   - expected current-surface binding must reject it.

7. `a5_07_evidence_substitution_fails_expected_context_binding`
   - substitute evidence digest consistently across trace policy/process log/SCAI conditions and recompute outer digests;
   - expected evidence binding must reject it.

8. `a5_08_backend_profile_name_cannot_inflate_authority`
   - rename observer/backend profile to a trust-sounding profile and attempt to present PASS semantics;
   - name changes cannot create Direct authority, Complete evidence, or PASS.

9. `a5_09_completeness_or_loss_masking_cannot_become_pass`
   - start from ambiguous/incomplete or lost evidence and attempt PASS laundering;
   - accepted P5 semantics must reject it even after digest recomputation.

10. `a5_10_workflow_or_source_substitution_fails_expected_context_binding`
    - substitute workflow and source identities with well-formed alternate descriptors and recompute graph digests;
    - expected-context verification must reject each substitution.

11. `a5_11_duplicate_or_reordered_graph_items_are_handled_safely`
    - duplicate graph roles must fail closed;
    - reordering the same unique role items must canonicalize to the same deterministic manifest identity rather than create a second semantic identity.

12. `a5_12_digest_role_swap_and_domain_confusion_attempts_fail_closed`
    - swap valid Runtime Trace/SCAI/SVR digests between role labels and attempt manifest acceptance;
    - role/digest confusion must fail;
    - the graph-manifest digest must be domain-separated and deterministic.

**Acceptance requires 12/12 PASS with no renamed/removed attack, no relaxed expected context, and no post-result threshold change.**

## Mandatory same-run predecessor reproofs

After the frozen A5 corpus passes:

- P5-A4 decision source remains an ancestor and its immutable public/product boundaries remain satisfied;
- P5-A3 12/12 PASS;
- P5-A2 original 14/14 PASS;
- P5-A2 verifier-ID addendum 4/4 PASS;
- P5-A1 12/12 PASS;
- P5-A0 18/18 PASS;
- critical P4 cross-proposition + bounded live B1/B2/B3 PASS;
- Semantics v3 PASS;
- public M11 PASS;
- public alpha.4 / stable `v0.1` boundaries PASS.

A5 does not create another live signature merely to re-prove A4. The accepted A4 signed artifact, attestation ID, certificate identity, Rekor evidence and full-run artifact remain pinned predecessor evidence; A5 verifies ancestry and decision identity.

## Phase decision rule

If and only if all 12 graph attacks and all mandatory predecessor reproofs pass, P5 may close as:

`P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`

with subordinate established results retained for Runtime Trace mapping, verification attestation binding, provenance binding and signed-attestation interoperability.

If a frozen attack reveals a binding that cannot be expressed/verified without changing the standards composition, retain the counterexample and classify whether it reaches:

`P5_EXISTING_STANDARDS_HAVE_PROVEN_REPRESENTATION_GAP`

If false binding/laundering survives the final verifier, close negatively as:

`P5_KILLED_FOR_FALSE_BINDING_OR_LAUNDERING`

A new predicate remains forbidden unless an actual retained representation gap is established.

## Promotion boundary

Even a positive P5 phase decision does not change public alpha.4, stable `v0.1`, `main`, public baseline-v2 meaning, or default observer. Product promotion remains a separate gate.
