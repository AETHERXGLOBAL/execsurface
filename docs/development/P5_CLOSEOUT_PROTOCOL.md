# ExecSurface — P5 Runtime Attestation & Provenance Closeout Protocol

Date: 2026-09-30
Parent program: #100
Parent protocol: `docs/development/P5_ATTESTATION_PROVENANCE_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — P5 CLOSEOUT / EVIDENCE-BOUND / RESEARCH-ONLY**

## Question

Does the completed bounded P5 evidence establish that ExecSurface can compose existing in-toto Runtime Trace, SCAI, SVR, SLSA provenance, and GitHub/Sigstore attestation mechanisms into a deterministic fail-closed verification graph for the tested scope, without demonstrated need for a new custom predicate/schema?

This closeout is a phase-decision gate. It does not add features, change public behavior, weaken any predecessor test, or reinterpret a failed run.

## Team

Fixed roles remain mandatory:

1. **Innovation Scientist / Systems Architect** — identify the smallest durable standards composition and avoid unnecessary ecosystem invention.
2. **Anti-Drift / Scientific Integrity Reviewer** — block universal-sufficiency claims, signature/provenance authority inflation, hidden threshold changes, and public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — require retained counterexamples and verify that the corrected graph closes the exact frozen attacks.
4. **Independent Critical-Milestone Reviewer** — require exact source SHA, run, artifact identity/digest, predecessor ancestry, public-boundary checks, and reproducibility evidence before a positive decision.

Dynamic closeout specialists:
- in-toto/SCAI/SVR composition;
- SLSA provenance semantics;
- Sigstore/GitHub Attestations identity;
- deterministic serialization/domain separation;
- CI evidence and artifact provenance;
- runtime evidence semantics/formal review.

## Frozen predecessor evidence

The closeout must establish ancestry of all accepted P5 predecessor sources:

- P5-A0 accepted source: `9ff34d9baaa01a90cf6d2cb95e1930dc4c011e11`
- P5-A1 accepted source: `74dc06a3f5a091f2f70e5f73290adf37ef3f2f44`
- P5-A2 accepted source: `63b9a9fcde16d9dbb66aa484e73a1e2338afd6cb`
- P5-A2 decision source: `8ef427a57d085f6cc6d45a0beef9599c5c4f002f`
- P5-A3 accepted source: `d0ee3d0d1bd5a491ddc5796be08458026e1cbd8a`
- P5-A4 accepted source: `862f0830511fe1421de20717187645d6e0a9a52e`
- P5-A4 decision source: `9fe4d1f81a79bd75064f8c14571b6580948af569`
- P4 closeout source: `320c865d3e81071a8214ad726dca7a587b0fc479`
- immutable public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

## Frozen A5 negative evidence

The first scientifically executable P5-A5 result remains retained and mandatory evidence:

- source SHA: `a7753bee6cc9f145adc7823dd82837fa40cfca01`
- workflow run: `36755962689`
- result: **8 passed / 4 failed** from the exact frozen 12-test corpus
- artifact ID: `11116268894`
- artifact name: `p5-a5-36755962689-1`
- artifact digest: `sha256:97aa4db53e27355d8b2d62e86d6f1ba5c2ee6d7d6e10607b965e4730fa878314`

This evidence MUST NOT be deleted, relabeled as non-scientific, or replaced by the later positive run. Its four counterexamples remain the causal justification for the bounded verifier hardening.

## Frozen A5 successful evidence

The corrected exact A5 gate completed successfully with all predecessor reproofs in the same run:

- source SHA: `d6d0d45b686ed8c9332ea6b0924537a345049bfb`
- workflow run: `36758531046`
- conclusion: `success`
- frozen A5 corpus: **12/12 passed**
- accepted P5-A4 live GitHub/Sigstore re-verification: PASS
- P5-A3/A2/A1/A0 reproofs: PASS
- critical P4 authority reproofs: PASS
- Semantics v3 reproof: PASS
- public M11 reproof: PASS
- artifact ID: `11117568130`
- artifact name: `p5-a5-36758531046-1`
- artifact digest: `sha256:d1ba8f88a42b0b3f23880445f6063735cef99b306861241ed2e04cdeee57d343`

The closeout must query GitHub for these run/artifact identities rather than trusting this document alone.

## Mandatory payload representation check

The research implementation must still represent the P5 mandatory semantic payload, including at minimum:

- command identity;
- source identity when available;
- artifact identity when available;
- workflow identity when available;
- baseline digest;
- current-surface digest;
- observer/profile identity;
- capability state/digest;
- completeness state/digest;
- policy digest;
- verdict;
- evidence digest;
- Runtime Trace statement digest;
- optional exact SLSA provenance reference;
- verifier identity.

Missing optional identities remain explicit; they may not be fabricated.

## Standards boundary

P5 intentionally reuses existing predicate types:

- `https://in-toto.io/attestation/runtime-trace/v0.1`
- `https://in-toto.io/attestation/scai/v0.3`
- `https://in-toto.io/attestation/svr/v0.2`
- `https://slsa.dev/provenance/v1`

ExecSurface-specific SCAI attribute/conditions and verifier-side graph bindings are domain data within the standards composition. A positive closeout means only:

> **No demonstrated need for a new custom predicate/schema in the bounded P5 experiment.**

It does NOT mean that every runtime-verification system is representable by this composition, that every consumer understands ExecSurface semantics, or that custom domain data is absent.

## Immutable boundaries

A positive closeout requires all of the following:

- public `v0.1.0-alpha.4` remains exactly `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable `v0.1` remains exactly the same source;
- no public `crates/` modification is introduced by P5 research after the P4 closeout boundary;
- no signature, SLSA record, GitHub identity, backend name, or standards label upgrades semantic authority;
- observer loss/incompleteness cannot become PASS;
- historical negative evidence remains retained;
- no post-result threshold or corpus weakening;
- no public release/promotion is authorized by P5 closeout.

## Closeout executable gate

The independent closeout workflow must:

1. verify all predecessor ancestry and immutable public tags;
2. verify that no P5 research changed public crates;
3. verify this protocol and the retained-counterexample marker;
4. query GitHub for the exact successful A5 run and artifact identity/digest;
5. query GitHub for the exact historical 8/12 run and artifact identity/digest;
6. verify mandatory payload fields and standard predicate constants in the research implementation;
7. independently reverify the accepted P5-A4 signed attestation through GitHub's supported attestation verifier;
8. record a candidate decision only after every check succeeds;
9. seal and upload closeout evidence.

## Allowed closeout decisions

Only these phase-level outcomes are allowed:

- `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`
- `P5_EXISTING_STANDARDS_HAVE_PROVEN_REPRESENTATION_GAP`
- `P5_INCOMPLETE_EVIDENCE`
- `P5_KILLED_FOR_FALSE_BINDING_OR_LAUNDERING`

If the executable closeout is fully green, it may additionally retain the accepted A5 statement:

- `P5_A5_CROSS_ATTESTATION_GRAPH_PASS_BOUNDED`

No other positive wording is authorized.

## Promotion boundary

Even a fully positive P5 closeout remains **research-only**. It does not alter `main`, public alpha.4 behavior, stable `@v0.1`, public raw/canonical/baseline v2 meaning, the default observer, or release status. Any such change requires a separate promotion/release gate.