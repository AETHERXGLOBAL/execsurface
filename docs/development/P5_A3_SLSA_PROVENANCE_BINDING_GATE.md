# ExecSurface — P5-A3 SLSA Provenance Binding Gate

Status: **PREREGISTERED — EXACTLY 12 TESTS / RESEARCH-ONLY**
Date: 2026-09-30
Parent program: #100
Predecessor: `P5_VERIFICATION_ATTESTATION_BINDING_ESTABLISHED_BOUNDED`
Development branch: `development/post-alpha4-behavioral-integrity`

## Question

Can ExecSurface bind an externally produced SLSA provenance v1 statement to the exact artifact/source/workflow context used by the bounded P5 verification bundle without treating provenance presence, a signature envelope, provider identity, or SLSA branding as semantic authority?

## Hypothesis

A minimal SLSA-v1 binding layer can parse a pinned upstream provenance statement, derive deterministic identity bindings, and fail closed under digest, subject, source, workflow, verification-state, replay, truncation, and predicate substitution while leaving existing P5/P4/public semantics unchanged.

## External evidence source — frozen before implementation

Repository: `slsa-framework/slsa-verifier`
Commit: `30d0be3bbab553fc51557377baba2f7572dfc212`
Path: `verifiers/internal/gcb/testdata/v1.0-gcloud-container-github-single.json`
Git blob SHA: `7102e40887a758e01184ac79ca10cb34b7281627`

The pinned upstream fixture contains a Google Cloud Build provenance record whose embedded in-toto statement has:

- `_type = https://in-toto.io/Statement/v1`
- `predicateType = https://slsa.dev/provenance/v1`
- SHA-256 subject identity
- GitHub build-config source identity and revision
- builder identity and invocation metadata
- a DSSE envelope/signature in the upstream fixture

The presence of that envelope/signature is **not** treated as locally verified by A3. Cryptographic verification/signing is deferred to the separately gated signed-attestation work.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — minimize the binding model and reuse SLSA/in-toto semantics instead of inventing a new predicate.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks SLSA-level inference, signature/brand-based authority, fabricated provenance, and public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — attacks subject/source/workflow substitution, replay, truncation, digest laundering, and unverified-provenance laundering.
4. **Independent Critical-Milestone Reviewer** — checks pinned upstream identity, exact test count, predecessor reproofs, retained failures, and decision boundaries.

Dynamic specialists for A3: SLSA provenance semantics, in-toto statements, CI/source identity, deterministic serialization, Rust schema validation, provenance verification boundaries.

## Immutable boundaries

- public `v0.1.0-alpha.4` and stable `v0.1` stay at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no public crate or observer change;
- no new proprietary in-toto predicate;
- no provenance presence -> PASS rule;
- no signature-envelope presence -> verified rule;
- no provider/backend name -> authority rule;
- no SLSA level is inferred or stored merely because provenance exists;
- missing provenance remains explicit absence, not provenance success;
- unverified provenance remains unverified;
- all P5-A2/A1/A0 and critical P4 fail-closed semantics remain in force.

## Frozen binding model

A3 may introduce a research-only parsed binding record derived from the pinned SLSA v1 statement. At minimum it must retain:

- exact statement SHA-256;
- exact predicate type;
- exact selected subject name and SHA-256 digest;
- source repository identity;
- source revision;
- build-config/workflow path where present in the upstream statement;
- builder identity;
- explicit external verification state supplied to the binding function.

Conversion into the existing `ProvenanceReference` is allowed only after the requested artifact/source/workflow context matches the parsed provenance binding. A3 does not cryptographically verify the DSSE signature.

## Frozen falsification corpus — exactly 12 tests

1. `a3_01_pinned_upstream_fixture_parses_exact_slsa_v1_identity`
   - exact Statement v1 and exact SLSA provenance v1 are required;
   - pinned fixture yields the expected subject/source/revision/build-config/builder fields.

2. `a3_02_statement_digest_is_deterministic_and_exactly_bound`
   - identical embedded statement bytes/representation used by the parser produce one deterministic SHA-256 binding;
   - an expected wrong statement digest is rejected.

3. `a3_03_exact_subject_digest_and_name_bind_successfully`
   - selected provenance subject must match requested artifact name and SHA-256 digest.

4. `a3_04_correct_statement_digest_with_wrong_subject_fails_closed`
   - a valid provenance statement cannot be attached to another artifact merely because its statement digest is correct.

5. `a3_05_source_repository_or_revision_substitution_fails_closed`
   - source repository and revision are independently bound and substitution is rejected.

6. `a3_06_workflow_or_build_config_substitution_fails_closed`
   - the build-config/workflow path present in the provenance must match the requested context.

7. `a3_07_unverified_provenance_cannot_become_verified_reference`
   - DSSE/signature presence in the fixture does not set `verified=true`;
   - explicit unverified state cannot be laundered into a verified `ProvenanceReference`.

8. `a3_08_provenance_absence_cannot_be_laundered_into_success`
   - missing provenance remains `None`/absence and cannot create a verified provenance reference or upgrade a verdict.

9. `a3_09_predicate_type_or_statement_type_substitution_is_rejected`
   - any other predicate URI or Statement type fails closed.

10. `a3_10_replay_under_different_artifact_source_or_workflow_context_is_rejected`
    - a previously valid bound statement cannot be replayed under a different requested context.

11. `a3_11_malformed_truncated_or_missing_required_binding_fields_fail_closed`
    - malformed JSON, truncated structure, absent subject digest, absent source identity/revision, or absent build-config path does not produce a bound provenance reference.

12. `a3_12_provenance_presence_does_not_infer_slsa_level_or_semantic_authority`
    - A3 output contains no inferred SLSA level;
    - provenance presence/verified state alone cannot upgrade ExecSurface authority, completeness, or verdict.

**Acceptance requires 12/12 PASS. No test may be removed, weakened, renamed to hide a failure, or have its acceptance condition changed after execution.**

## Mandatory regression reproofs in the same CI gate

After 12/12 A3 PASS:

- P5-A2 original: 14/14 PASS;
- P5-A2 verifier-identity addendum: 4/4 PASS;
- P5-A1: 12/12 PASS;
- P5-A0: 18/18 PASS;
- critical P4 cross-proposition gate and live B1/B2/B3 gates PASS;
- Semantics v3 PASS;
- public M11 PASS;
- immutable alpha.4/stable boundaries PASS.

## Decision rule

If the full gate passes, record only the bounded research conclusion:

`P5_A3_SLSA_PROVENANCE_BINDING_PASS_BOUNDED`

This is a P5 execution status, not a release/promotion decision and not a SLSA level claim.

If any falsifier demonstrates provenance laundering, false subject/source/workflow binding, or authority inflation, retain the evidence and do not advance to signed-attestation work.

## Next step only after acceptance

Proceed to the separately preregistered signed GitHub/Sigstore attestation prototype. A3 itself does not authorize signing, publishing attestations, or public integration.
