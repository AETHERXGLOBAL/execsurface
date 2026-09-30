# ExecSurface — P5-A2 SCAI + SVR Verification Binding Gate

Date: 2026-09-30
Branch: `development/post-alpha4-behavioral-integrity`
Parent protocol: `docs/development/P5_ATTESTATION_PROVENANCE_PROTOCOL.md`
Predecessor decision: `P5_RUNTIME_TRACE_MAPPING_ESTABLISHED_BOUNDED`
Status: **PREREGISTERED — EXACTLY 14 TESTS / RESEARCH-ONLY**

## Question

Can the bounded ExecSurface verification result be represented as a detailed SCAI v0.3 assertion plus an SVR v0.2 concise summary over the same subject, policy, verifier and Runtime Trace evidence without allowing cross-statement substitution, semantic laundering, or digest-rebinding attacks?

## Hypothesis H5-A2

The existing standards composition is sufficient if all semantic bindings are explicit and verified:

1. Runtime Trace, SCAI and SVR use the same subject identity;
2. SCAI target equals that subject;
3. SCAI evidence content-binds the exact Runtime Trace statement;
4. SCAI conditions bind policy, authority, completeness, observer profile, baseline/current surface, evidence, source/workflow identity and verifier identity;
5. SCAI producer is the same verifier identity declared in the conditions;
6. SVR policy is the exact policy bound by SCAI;
7. SVR contains the expected recorded-result marker and a content binding to the exact SCAI statement;
8. SVR PASS property is present if and only if the detailed SCAI verdict is PASS;
9. recomputing outer digests cannot rescue a substituted semantic component;
10. no new in-toto predicate is required.

## Fixed roles

- **Innovation Scientist / Systems Architect:** preserve the smallest standards composition; add no proprietary predicate unless a retained representation counterexample requires it.
- **Anti-Drift / Scientific Integrity Reviewer:** reject signature/brand/backend authority inflation, threshold changes, and any public-alpha reinterpretation.
- **Independent Falsifier / Red Team:** attack subject/policy/verifier/evidence substitution, stale/fresh mix-and-match, replay, loss/completeness laundering and digest rebinding.
- **Independent Critical-Milestone Reviewer:** require exact source SHA, pinned standards source, run/job/artifact digests, predecessor reproof and retained failures.

Dynamic specialists: in-toto SCAI/SVR, deterministic serialization, Rust typed validation, provenance/replay analysis, CI evidence engineering.

## Immutable boundaries

- public `v0.1.0-alpha.4` and stable `@v0.1` remain unchanged;
- no public crate modification is authorized by this gate;
- no new predicate design;
- no signing yet;
- no claim that cryptographic authenticity equals semantic PASS;
- missing/ambiguous/lost evidence cannot be repaired by SCAI/SVR packaging;
- no result-driven test removal or threshold adjustment.

## Exact 14-test falsification corpus

Acceptance requires **14/14 PASS**:

1. **same-subject complete composition** — Runtime Trace, SCAI target/subject and SVR subject/policy/verifier bindings verify together;
2. **SCAI subject substitution** — substituted SCAI subject fails closed even after outer digests are recomputed;
3. **SVR subject substitution** — substituted SVR subject fails closed even after outer digests are recomputed;
4. **SCAI target substitution** — target differing from the common subject fails closed;
5. **SVR policy substitution** — a different policy descriptor cannot preserve verification;
6. **Runtime Trace evidence substitution** — SCAI evidence descriptor cannot point at another trace digest;
7. **Runtime Trace digest-field substitution** — detailed assertion's runtime-trace digest binding cannot be changed independently;
8. **stale/missing SVR-to-SCAI binding** — removal or stale SCAI digest property fails closed;
9. **SVR recorded/PASS summary integrity** — the recorded-result marker is mandatory and PASS property must match the detailed verdict exactly;
10. **producer/verifier substitution with digest rebinding** — changing SCAI producer or declared verifier identity cannot be rescued by recomputing SCAI/SVR/bundle digests;
11. **backend/profile authority inflation** — renaming a weak/incomplete observer profile cannot upgrade authority, completeness or verdict;
12. **loss/completeness laundering** — observer loss, ambiguous authority or incomplete evidence cannot produce or preserve a PASS-class summary;
13. **semantic-field identity binding** — baseline, current-surface, evidence, source and workflow mutations each change detailed assertion/bundle identity, and workflow replay is rejected against the original expected context;
14. **deterministic composition** — identical semantic inputs produce identical Runtime Trace, SCAI, SVR and bundle identities; statement domains remain distinct.

The corpus is frozen before execution. A failure may justify the smallest semantics-preserving validation correction, but the failing case remains in the 14-test corpus and the acceptance threshold remains 14/14.

## Required regression reproof

A positive A2 decision additionally requires:

- P5-A1: 12/12 PASS;
- P5-A0: 18/18 PASS;
- P4 cross-proposition: 12/12 PASS;
- P4 B1/B2/B3 live: 10/10 each;
- Semantics v3: 7/7 PASS;
- public M11 shared-FD: 6/6 PASS;
- immutable alpha.4/stable tag checks PASS.

## Allowed decision

If every requirement passes:

`P5_VERIFICATION_ATTESTATION_BINDING_ESTABLISHED_BOUNDED`

Otherwise retain the exact counterexample and classify as `P5_INCOMPLETE_EVIDENCE` or `P5_KILLED_FOR_FALSE_BINDING_OR_LAUNDERING` as warranted.

No A2 outcome alone authorizes signing, public integration or release promotion.
