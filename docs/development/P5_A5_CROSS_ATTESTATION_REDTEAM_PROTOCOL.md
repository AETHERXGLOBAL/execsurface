# ExecSurface — P5-A5 Cross-Attestation Graph Red-Team Protocol

Date: 2026-09-30
Parent program: #100
Parent protocol: `docs/development/P5_ATTESTATION_PROVENANCE_PROTOCOL.md`
Predecessor: `P5_A4_SIGNED_ATTESTATION_PASS_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — EXACTLY 12 CROSS-ATTESTATION ATTACKS / RESEARCH-ONLY**

## Objective

Falsify the complete P5 composition as a graph rather than accepting Runtime Trace, SCAI, SVR, SLSA provenance and signed-attestation evidence merely because each component passed an isolated gate.

The gate asks one narrow question:

> Can an attacker mix, replay, substitute, duplicate, reorder, or domain-confuse individually well-formed P5 evidence and still obtain an accepted cross-attestation graph for the wrong semantic context?

A5 is not a feature expansion and does not authorize a new predicate, public integration, release promotion, or signature-derived semantic authority.

## Team

Fixed roles:

1. **Innovation Scientist / Systems Architect** — look for the smallest compositional invariant that closes a real graph-level gap without inventing a parallel attestation ecosystem.
2. **Anti-Drift / Scientific Integrity Reviewer** — block weaker tests, post-result threshold changes, signature/SLSA/backend-name authority inflation, and any silent reinterpretation of earlier accepted semantics.
3. **Independent Falsifier / Red Team** — construct the twelve frozen cross-graph attacks below and retain any surviving counterexample.
4. **Independent Critical-Milestone Reviewer** — require exact source SHA, run, artifact digest, predecessor reproofs and immutable public-boundary checks before a positive decision.

Dynamic specialists for A5:
- in-toto graph/binding semantics;
- SCAI/SVR schema composition;
- SLSA provenance replay/domain separation;
- deterministic serialization and cryptographic digest domain separation;
- Rust adversarial testing;
- CI/OIDC identity and Sigstore verification.

## Frozen predecessor facts

A5 starts only because these bounded predecessor decisions exist:

- P5-A0: existing standards composition accepted bounded;
- P5-A1: Runtime Trace mapping established bounded;
- P5-A2: SCAI/SVR verification binding established bounded;
- P5-A3: SLSA provenance binding passed bounded;
- P5-A4: real GitHub/Sigstore signed attestation path passed bounded.

A5 does not reopen their isolated tests unless a graph-level counterexample demonstrates that an accepted predecessor invariant is insufficient.

## Frozen acceptance rule

Exactly **12 adversarial tests** are required. No threshold may change after execution.

A positive A5 result requires:

- **12/12 attacks fail closed or preserve only explicitly order-insensitive semantics**;
- no false PASS caused by recomputing only outer digests after semantic substitution;
- no stale/fresh cross-attestation laundering;
- no provenance replay across subject/context;
- no backend/profile-name authority inflation;
- no completeness/loss masking;
- no duplicate semantic item accepted as equivalent evidence;
- no digest domain-confusion collision across distinct P5 object classes;
- all predecessor P5 gates reprove;
- critical P4 authority gates reprove;
- Semantics v3 and public M11 reprove;
- public `v0.1.0-alpha.4` and stable `v0.1` remain exactly at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no public crate changes after the accepted P5-A4 source.

## Frozen attack corpus — exactly 12

### A5-01 — mix-and-match subjects
Build two independently valid bundles for different subjects. Replace one graph component with the other subject's component, recompute attacker-visible outer digests, and require rejection.

### A5-02 — stale Runtime Trace + fresh verdict
Mix a Runtime Trace from an older semantic state with a SCAI/SVR pair from a newer state. Recompute outer digests. The graph must reject the stale trace rather than accepting the fresh verdict over stale runtime evidence.

### A5-03 — fresh trace + stale policy
Use fresh runtime evidence with a stale policy-bound SCAI/SVR context. Recompute outer digests. The graph must reject the policy/config split.

### A5-04 — SLSA provenance replay
Replay an otherwise well-formed provenance reference from another subject/context and require rejection. `verified=true` is not sufficient to authorize replay.

### A5-05 — baseline substitution
Mutate the accepted/baseline digest inside the detailed assertion while keeping the remaining graph intact; recompute attacker-visible outer digests. The graph must reject the cross-binding break.

### A5-06 — current-surface substitution
Mutate the current-surface digest in the detailed assertion under the same attack model and require rejection.

### A5-07 — evidence substitution
Mutate the raw/derived evidence digest in only part of the graph and require rejection even after outer digest recomputation.

### A5-08 — backend/profile-name authority inflation
Starting from a non-PASS ambiguous/incomplete bundle, rename the observer/backend profile to a trusted-looking name and attempt to inject PASS/direct/complete claims without supplying the required bound evidence. The graph must reject; backend/profile naming must never grant authority.

### A5-09 — completeness/loss masking
Starting from lost/incomplete evidence, attempt to present a PASS-class SCAI/SVR result while hiding or replacing the loss/completeness state. The graph must reject.

### A5-10 — workflow/source substitution
Substitute workflow/source identity in the detailed assertion while retaining the runtime/evidence graph and require rejection when verifying against the frozen expected execution context.

### A5-11 — duplicate/reordered bundle items
Two subchecks inside one frozen test:

1. duplicate a semantic graph item where cardinality is defined as one and require rejection;
2. reorder an explicitly order-insensitive representation only if semantic identity is preserved by canonical reconstruction; arbitrary reordering must not create a second accepted semantic identity for the same graph.

A duplicate is never allowed to gain authority merely because every duplicated item is individually valid.

### A5-12 — deterministic digest collision / domain confusion
For the same semantic payload material, calculate identities in the distinct P5 digest domains used for Runtime Trace, SCAI, SVR and bundle identity. Require domain-separated unequal identities and deterministic stability on reconstruction. Cross-domain digest substitution must be rejected.

## Attacker model

The adversary may:

- clone a valid typed bundle;
- modify serialized/typed semantic fields;
- recompute public outer statement/bundle digests using the same deterministic code path;
- mix artifacts from independently valid bundles.

The adversary may **not** break SHA-256 cryptographically or forge a valid Sigstore signature for an unauthorized identity. A5 tests semantic graph binding above cryptographic primitive assumptions.

This attacker model intentionally prevents a weak test that fails only because an outer cached digest was not recomputed.

## Decision rules

Allowed A5/P5 phase decisions after execution:

- `P5_A5_CROSS_ATTESTATION_GRAPH_PASS_BOUNDED`
- `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`
- `P5_EXISTING_STANDARDS_HAVE_PROVEN_REPRESENTATION_GAP`
- `P5_INCOMPLETE_EVIDENCE`
- `P5_KILLED_FOR_FALSE_BINDING_OR_LAUNDERING`

A new predicate remains forbidden unless retained evidence proves a standards representation gap. An implementation binding defect is not automatically a standards representation gap.

## Promotion boundary

No A5 outcome alone changes:

- public alpha.4 semantics;
- stable `@v0.1`;
- public raw/canonical/baseline v2 meaning;
- default observer;
- `main`;
- release status.

A5 remains research/development evidence until a separate promotion/release gate authorizes otherwise.
