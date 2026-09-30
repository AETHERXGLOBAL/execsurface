# ExecSurface — P6-A2 Adversarial False-PASS / Identity-Binding Protocol

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Predecessor: `P6_A1_OBSERVATION_CORPUS_COMPLETE_BOUNDED`
Status: **PREREGISTERED — NO A2 RESULT YET**

## 1. Objective

A2 attempts to falsify ExecSurface's bounded verification claims before using P6 evidence competitively.

Primary question:

> Can a changed, substituted, incomplete, replayed, or tampered execution/evidence state be accepted as PASS under a contract that is supposed to reject or distinguish it?

A2 is not a penetration test of competitors. External systems are tested only against verification/identity properties directly demonstrated in A0/A1 or explicitly documented by their frozen artifacts.

## 2. Fixed roles

1. **Innovation Scientist / Systems Architect** — design attacks against semantic composition rather than superficial event counts.
2. **Anti-Drift / Scientific Integrity Reviewer** — prevent post-result weakening, competitor claim invention, and public-v2/P5 boundary collapse.
3. **Independent Falsifier / Red Team** — own all attack cases and attempt to create false PASS / identity laundering.
4. **Independent Critical-Milestone Reviewer** — verify frozen attack corpus, exact sources, retained failures and decision wording.

Dynamic specialists:
- evidence-integrity / canonicalization;
- CI provenance / in-toto / SLSA / Sigstore;
- Linux ptrace observation completeness;
- adversarial testing / reproducibility.

## 3. Frozen boundaries

A2 MUST NOT:
- change alpha.4 public semantics;
- reinterpret P5 research claims as public alpha.4 features;
- require an external product to reject an attack it never claims to verify;
- call an unsigned artifact a vulnerability merely because it is unsigned;
- treat a native `passed`/`audit`/`success` label as ExecSurface `PASS`;
- alter the frozen A1 workload in-place;
- change acceptance criteria after the first scientifically executable run;
- delete negative evidence.

## 4. Track E — ExecSurface public alpha.4 false-PASS attacks

Exact target:
- `v0.1.0-alpha.4`;
- source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.

The A2 workflow learns the frozen S0 baseline once, then never relearns.

### E1 — Process expansion cannot PASS
Run frozen S1 using the S0 baseline.

Acceptance:
- exit/result may be REVIEW/BLOCK/ERROR according to native semantics;
- **exit `0` / native PASS is forbidden**.

Counterexample marker:
`FALSE_PASS_COUNTEREXAMPLE_PROCESS_EXPANSION`

### E2 — File write expansion cannot PASS
Run frozen S3 using the same S0 baseline.

Acceptance:
- native PASS is forbidden.

Counterexample marker:
`FALSE_PASS_COUNTEREXAMPLE_FILE_WRITE`

### E3 — Network expansion / observer incompleteness cannot PASS
Run frozen S2 using the same S0 baseline.

Acceptance:
- complete observed drift may REVIEW/BLOCK;
- incomplete/lost observation may ERROR/fail closed;
- native PASS is forbidden.

A1 already retained one real `ERROR/INCOMPLETE`; A2 independently re-executes rather than assuming the same scheduler outcome.

Counterexample marker:
`FALSE_PASS_COUNTEREXAMPLE_NETWORK_OR_INCOMPLETE`

### E4 — Baseline payload tamper cannot validate
After learning S0, copy the lockfile and modify one semantic payload field while leaving the stored baseline digest unchanged.

The tampered lock must not yield native PASS for S0.

Counterexample marker:
`FALSE_PASS_COUNTEREXAMPLE_BASELINE_TAMPER`

### E5 — Baseline digest substitution cannot validate
Copy the original S0 lock and replace the top-level baseline digest with a syntactically valid wrong SHA-256 value while preserving the payload.

The substituted lock must not yield native PASS.

Counterexample marker:
`FALSE_PASS_COUNTEREXAMPLE_BASELINE_DIGEST_SUBSTITUTION`

### E6 — Frozen command-contract mismatch cannot silently PASS if alpha.4 declares it incompatible
Use the S0 baseline with the same `/bin/bash` executable but a deliberately different top-level argument-count shape.

This attack tests the actual public v2 command compatibility contract only. It does **not** claim alpha.4 binds workflow/source identity or argument values beyond the fields present in the lock.

If alpha.4 explicitly rejects command mismatch, retain that result. If it accepts because its declared v2 compatibility model considers the command shape compatible, record the bounded behavior and do not relabel it as a security flaw without contract evidence.

E6 therefore has no preregistered false-PASS marker until the public lock contract is checked against the observed case.

## 5. Track P — P5 research cross-attestation reproof

A2 must independently re-run the already frozen P5-A5 12-test adversarial corpus on the current branch without editing its tests.

Required attacks already include:
- baseline substitution;
- current-surface substitution;
- source substitution;
- semantic duplication/canonicalization attacks;
- cross-attestation graph binding failures.

Acceptance:
- exact accepted P5-A5 corpus remains `12/12 PASS`;
- no P6 code may weaken P5 tests;
- public `crates/` remain unchanged since P5 closeout.

Any new failure is a P6 blocker and must be investigated as a potential regression before competitive claims continue.

## 6. Track C — cicd-sensor run-predicate identity/replay facts

Frozen A1 artifact:
- run `36770813526`;
- attestation artifact `11123269064`;
- artifact digest `sha256:6dad33b00832f1569fa1f3dd3af2fb98771b9d8761a2215ecb15d0e8499b8ce9`.

Frozen source/action docs state the predicate artifact is not signed by the action.

A2 tests only these demonstrated facts:

### C1 — Unchanged replay exposes original run identity
Given an expected run ID different from `36770813526`, the unchanged predicate's embedded `github_run_id` must remain the original value, so an equality-checking consumer can detect replay-context mismatch.

Classification if true:
`IDENTITY_MISMATCH_EXPLICITLY_DETECTABLE_BY_CONSUMER`

This is not a claim that cicd-sensor itself performs that consumer verification.

### C2 — Local JSON identity tamper has no embedded cryptographic self-verification
Modify only the copied predicate's embedded run ID.

Because the frozen artifact is documented as unsigned, A2 expects no embedded signature verification to reject the modified JSON by itself.

Classification if structurally valid:
`EXTERNAL_INTEGRITY_REQUIRED_FOR_TAMPER_DETECTION`

This is a standards/integration fact, **not** a vulnerability label and not a negative product score.

### C3 — Native `passed` is not proof of ExecSurface baseline conformance
The A1 predicate's native result extension remains recorded but must not be accepted by the P6 harness as equivalent to ExecSurface baseline/policy PASS.

Acceptance:
P6 parser retains the label as `native_predicate_result` only and produces no common verdict.

## 7. Harden-Runner A2 boundary

A1 demonstrated job/run correlation and network observation, but no local attestation-equivalent artifact contract. P6 therefore does not invent an offline replay-verification obligation for Harden-Runner in A2.

Harden-Runner is classified `NOT_APPLICABLE_TO_OFFLINE_ATTESTATION_REPLAY` for this gate.

Its process/network/file evidence may be used later for proposition-specific falsification if an exact supported verification contract is frozen.

## 8. Tetragon boundary

Still `NOT_TESTABLE_YET` for live P6 execution until an exact-source build workflow or immutable image digest is frozen. No A2 penalty or positive inference is allowed.

## 9. A2 evidence and decision rules

Every attack records:
- exact source SHA;
- input/baseline/artifact digests;
- mutation performed;
- native exit/result;
- whether PASS occurred;
- stderr/error class;
- artifact/log checksums.

A2 may close positively only if:
1. E1–E5 produce no ExecSurface false PASS;
2. E6 is classified against its actual public contract without claim inflation;
3. P5-A5 remains 12/12;
4. C1–C3 are recorded as bounded identity/integrity facts without semantic laundering;
5. all historical failures remain retained.

Allowed decisions:
- `P6_A2_NO_FALSE_PASS_IN_FROZEN_SCOPE`
- `P6_EXECSURFACE_COUNTEREXAMPLE_FOUND`
- `P6_A2_INCOMPLETE`

A positive result never authorizes a global superiority claim.