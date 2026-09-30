# ExecSurface — P5 Runtime Attestation & Provenance Interoperability Protocol

Date: 2026-09-30
Parent program: #100
Predecessor: `docs/development/P4_CLOSEOUT.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — STANDARDS-FIRST / NO-NEW-PREDICATE**

## Objective

Turn an ExecSurface verification result into a deterministic, attestable, provenance-linkable result without inventing an isolated ecosystem format when existing standards can express the required semantics.

North-star continuation:

`accepted runtime behavior -> authority-aware evidence -> deterministic drift -> explicit verification -> attestable result`

P5 is not a generic supply-chain-security feature expansion. It is the interoperability layer that binds ExecSurface's already-established evidence semantics to existing attestation/provenance ecosystems.

## Fixed roles

1. **Innovation Scientist / Systems Architect**
   - seek the smallest standards composition that preserves ExecSurface semantics;
   - prefer reusable ecosystem primitives over proprietary wrapper formats;
   - search for a durable moat in verifiable semantic composition, not signature plumbing.
2. **Anti-Drift / Scientific Integrity Reviewer**
   - block a new predicate unless a preregistered representation gap proves it necessary;
   - block claims that a signature implies behavioral correctness;
   - block SLSA/Sigstore/GitHub-brand authority inflation;
   - preserve the distinction between authenticated metadata and the semantic evidence it describes.
3. **Independent Falsifier / Red Team**
   - attack subject substitution, digest substitution, policy substitution, provenance replay, workflow/source substitution, evidence replay, completeness laundering, backend-name inflation and cross-attestation mismatch.
4. **Independent Critical-Milestone Reviewer**
   - require exact source SHA, pinned external specifications/actions, workflow run, artifacts/digests and failure history before any positive gate decision.

## Dynamic specialists

- in-toto Attestation Framework specialist
- SLSA v1.2 provenance specialist
- Sigstore / DSSE / GitHub artifact-attestation engineer
- Rust deterministic-serialization engineer
- runtime evidence-semantics / formal methods specialist
- CI identity / OIDC / provenance specialist
- reproducibility / adversarial fixture engineer

## External standards snapshot

The implementation MUST target these existing standards/interfaces unless a later gate explicitly changes the pin:

### in-toto Statement

- statement type: `https://in-toto.io/Statement/v1`

### Runtime Trace

- predicate type: `https://in-toto.io/attestation/runtime-trace/v0.1`
- role in P5: transport the monitored-operation identity, monitor identity/configuration and runtime trace data.
- important boundary: monitor-specific process/network schemas and file-access evidence do not by themselves define ExecSurface authority/completeness.

### SCAI

- predicate type: `https://in-toto.io/attestation/scai/v0.3`
- role in P5: represent a domain-specific behavioral-verification attribute with explicit conditions and evidence ResourceDescriptors while retaining a standard predicate.

### Simple Verification Result (SVR)

- predicate type: `https://in-toto.io/attestation/svr/v0.2`
- role in P5: concise verifier/policy/property summary.
- boundary: SVR is not the complete reproduction record and must not replace detailed evidence bindings.

### SLSA

- specification baseline: SLSA v1.2
- build-provenance predicate type: `https://slsa.dev/provenance/v1`
- role in P5: optional link to independently produced build provenance when it exists.
- boundary: ExecSurface MUST NOT manufacture SLSA provenance for a build it did not authoritatively observe as the provenance producer.

### GitHub / Sigstore attestation path

Current pinned research action:

- GitHub action: `actions/attest`
- release observed at protocol freeze: `v4.2.2`
- immutable commit pin: `1e69f48acb82d1966a394da916b4c1698aa569d6`

The action supports custom existing predicate types via `predicate-type` + `predicate`/`predicate-path` and signs/stores the resulting attestation through GitHub's Sigstore-backed artifact-attestation system.

No signing step is authorized until the local deterministic representation/falsification gate passes.

## P5-A0 — representation contract / no-new-predicate gate

### Question

Can the required ExecSurface verification bindings be represented by composing existing vetted predicate types without semantic loss that could create a false PASS or false provenance claim?

### Hypothesis H5-A0

A standards composition is sufficient:

1. Runtime Trace represents the observed operation and monitor-specific trace;
2. SCAI represents the detailed ExecSurface behavioral-verification assertion and evidence bindings;
3. SVR provides a concise policy-verification summary over the same subject;
4. SLSA provenance is referenced only when independently available and digest-bound;
5. the statements are deterministic for identical semantic inputs;
6. no new in-toto predicate is necessary for this bounded prototype.

### Mandatory ExecSurface binding set

The detailed verification assertion MUST bind, at minimum:

1. command identity;
2. source identity when available;
3. artifact identity when available;
4. workflow identity when available;
5. accepted/baseline digest;
6. current-surface digest;
7. observer/backend semantic-profile identity;
8. observer capability state/digest;
9. completeness state/digest;
10. policy digest;
11. verdict;
12. raw/derived evidence digest;
13. runtime-trace attestation/predicate digest;
14. SLSA provenance digest + predicate type when provenance exists;
15. ExecSurface verifier/version identity.

Unknown/unavailable identity MUST remain explicit. Missing identity MUST NOT be replaced with a fabricated value.

### Verdict boundary

The prototype recognizes the existing product verdict states only:

- `PASS`
- `REVIEW`
- `BLOCK`
- `ERROR`

Attestation/signature/provenance presence MUST NOT upgrade any verdict.

If completeness/evidence state is insufficient for a semantic PASS under the frozen verifier contract, the attestation layer must preserve non-PASS; it cannot repair or reinterpret the verifier result.

## P5-A0 preregistered falsification corpus

Exactly **18 tests** are required for the first gate:

1. complete fixture represents every mandatory binding field;
2. identical semantic input serializes identically;
3. map/object insertion-order changes do not change canonical identity;
4. baseline digest mutation changes detailed-assertion identity;
5. current-surface digest mutation changes detailed-assertion identity;
6. policy digest mutation changes detailed-assertion identity;
7. evidence digest mutation changes detailed-assertion identity;
8. command identity mutation changes detailed-assertion identity;
9. subject/artifact substitution changes statement identity and fails cross-statement binding;
10. source/workflow substitution changes detailed-assertion identity;
11. backend/profile-name substitution cannot upgrade authority, completeness or verdict;
12. completeness downgrade cannot preserve a PASS-class verification assertion;
13. observer-loss state cannot become PASS through attestation construction;
14. ambiguous/unsupported evidence cannot become PASS through attestation construction;
15. Runtime Trace / SCAI / SVR subject mismatch fails closed;
16. SLSA provenance digest/predicate substitution fails closed;
17. replay of evidence or verification assertion against another subject fails closed;
18. unrecognized injected fields cannot upgrade authority/completeness/verdict or preserve a substituted semantic digest.

Acceptance: **18/18 PASS** with no weakened fixture and no post-result threshold change.

## P5-A1 — Runtime Trace mapping gate

Only after P5-A0 passes.

Map a bounded ExecSurface fixture into `runtime-trace/v0.1`:

- `monitor.type` identifies ExecSurface, not ptrace/Tetragon as semantic authority;
- monitor configuration/policy is content-addressed where available;
- `monitoredProcess` identifies the exact job/command instance;
- process/network/fileAccess data maps only what the standard can represent truthfully;
- ExecSurface-specific proposition/authority/completeness details remain separately bound in the SCAI verification assertion rather than being laundered into generic Runtime Trace fields.

Required adversarial cases:

- missing monitor identity;
- subject substitution;
- trace truncation/loss;
- backend-name substitution;
- monitor-log mutation;
- replay under a different workflow/command.

No claim that a generic Runtime Trace consumer understands ExecSurface proposition authority.

## P5-A2 — detailed verification assertion + concise verification summary

Only after A0/A1 pass.

Use SCAI v0.3 for the detailed domain-specific assertion and SVR v0.2 for the concise summary over the **same subject identity**.

The SCAI attribute identifier will be project-specific, but the predicate remains standard. The SCAI evidence descriptor must content-bind the runtime evidence/trace. SVR policies must include the exact policy ResourceDescriptor when policy material exists.

The verifier must reject SCAI/SVR subject mismatch, policy mismatch and digest mismatch.

## P5-A3 — SLSA provenance binding

When a real SLSA provenance statement is available, bind it by exact digest and predicate type.

Negative controls are mandatory:

- wrong provenance digest;
- correct digest attached to wrong subject;
- provenance for another workflow/source;
- unsigned/unverified provenance treated as if verified;
- provenance absence silently represented as provenance success.

P5 does not assign a SLSA level merely because a provenance statement exists.

## P5-A4 — GitHub/Sigstore signed prototype

Only after deterministic local gates pass.

Generate a bounded research attestation for a deterministic fixture/artifact using the pinned `actions/attest` commit.

Required permissions must be explicit and minimal:

- `contents: read`
- `id-token: write`
- `attestations: write`

Acceptance requires:

- exact subject digest;
- expected predicate type;
- successful attestation generation;
- returned attestation/bundle identity recorded;
- verification through an independent GitHub-supported verification path;
- subject mutation fails verification;
- source/workflow identity is inspectable from the attestation identity material;
- no claim that cryptographic verification implies ExecSurface semantic PASS.

## P5-A5 — cross-attestation red team / phase decision

Attack the Runtime Trace + SCAI + SVR + optional provenance composition as a graph, not as isolated JSON files.

Required attacks:

- mix-and-match subjects;
- stale runtime trace + fresh verdict;
- fresh trace + stale policy;
- provenance replay;
- baseline substitution;
- current-surface substitution;
- evidence substitution;
- backend/profile-name authority inflation;
- completeness/loss masking;
- workflow/source substitution;
- duplicate/reordered bundle items;
- deterministic digest collision/domain-confusion attempts.

## Allowed P5 decisions

- `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`
- `P5_RUNTIME_TRACE_MAPPING_ESTABLISHED_BOUNDED`
- `P5_VERIFICATION_ATTESTATION_BINDING_ESTABLISHED_BOUNDED`
- `P5_SIGNED_ATTESTATION_PATH_ESTABLISHED_BOUNDED`
- `P5_EXISTING_STANDARDS_HAVE_PROVEN_REPRESENTATION_GAP`
- `P5_INCOMPLETE_EVIDENCE`
- `P5_KILLED_FOR_FALSE_BINDING_OR_LAUNDERING`

A new predicate design is forbidden unless `P5_EXISTING_STANDARDS_HAVE_PROVEN_REPRESENTATION_GAP` is reached with retained counterexample evidence.

## Promotion boundary

No P5 outcome alone changes public alpha.4 semantics, stable `@v0.1`, public baseline v2 meaning, default observer or `main`.

P5 artifacts remain development/research evidence until a separate promotion/release gate passes.

## First execution step

Implement **P5-A0 only** as an isolated research crate and execute the exact 18-test corpus. Do not sign or publish attestations until A0 is green.
