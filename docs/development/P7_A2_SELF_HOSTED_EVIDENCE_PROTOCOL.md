# ExecSurface — P7-A2 Self-Hosted CI Packaging & Evidence Contract

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Predecessor: `P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO A2 RESULT YET**

## 1. Question

Can ExecSurface define a deterministic self-hosted CI packaging/evidence contract that preserves the existing public alpha.4 behavioral semantics while treating runner ownership, labels, machine identity and CI provider metadata only as untrusted context?

A self-hosted machine MUST NOT gain semantic authority merely because the user owns it, labels it trusted, or runs it inside a private network.

This is a research-only packaging/evidence-contract gate. It is not a public self-hosted-runner compatibility or installation claim.

## 2. Hypothesis

A bounded contract is semantics-preserving if it separates three things:

1. **package identity** — exact ExecSurface source/version/platform eligibility;
2. **runner context** — provider/runner/OS/architecture/labels as `context_only` metadata;
3. **verification evidence** — verdict, evidence digest, observer authority and completeness copied from the actual ExecSurface result, never synthesized from runner metadata.

Runner metadata may bind provenance, but cannot:
- select or mutate a baseline;
- change PASS/REVIEW/BLOCK/ERROR;
- upgrade observer/proposition authority;
- turn incomplete/lost evidence into complete evidence;
- make an unsupported architecture supported.

## 3. Fixed roles

1. **Innovation Scientist / Systems Architect** — minimize the deployment contract while preserving separations between package, runner context and behavioral evidence.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks trust-by-ownership, label-based authority, architecture exemptions, baseline auto-selection and public-support inflation.
3. **Independent Falsifier / Red Team** — attacks trusted-label laundering, forged runner identity, baseline/verdict environment injection, unsupported-platform laundering, replay and completeness upgrading.
4. **Independent Critical-Milestone Reviewer** — verifies frozen files, exact source/version, historical A0/A1 evidence retention, immutable public boundaries and decision wording.

Dynamic specialists:
- self-hosted CI operations and packaging;
- supply-chain / runner identity;
- runtime evidence semantics / PL;
- Linux platform compatibility;
- deterministic serialization and reproducibility.

## 4. Immutable boundaries

A2 MUST NOT:
- modify public `v0.1.0-alpha.4`, stable `v0.1`, `main`, public v2 semantics, or the default observer;
- call Linux arm64 supported after the negative A0 result;
- infer trust or authority from `self-hosted`, `trusted`, `prod`, `root`, runner group, machine ownership, or provider name;
- auto-select or mutate a baseline from runner environment;
- permit CI environment to override a verdict;
- upgrade incomplete/lost/unsupported evidence;
- add self-hosted-only thresholds/whitelists;
- delete A0/A1 negative evidence;
- claim public self-hosted support from this experiment.

## 5. Frozen package identity

The research contract binds:
- source SHA `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- version `0.1.0-alpha.4`;
- supported public runtime for this package contract: Linux x86_64 only.

The contract may represent another OS/architecture as **ineligible**, but must never silently reinterpret it as supported.

## 6. Runner-context contract

Required fields:
- provider;
- runner ID;
- OS;
- architecture;
- labels.

Output authority is always:
`context_only`

Labels are canonicalized for deterministic provenance only. They have zero behavioral-authority meaning.

## 7. Baseline boundary

A baseline reference, when supplied by the caller, must be an explicit SHA256 digest. The contract MUST NOT read baseline selection from arbitrary environment variables or infer a baseline from workspace paths, runner labels, project names or previous jobs.

No baseline value means `baseline_reference = null`; it does not authorize baseline learning.

## 8. Verification-result boundary

A verification evidence input contains only values already produced by an ExecSurface verification path:
- verdict;
- evidence digest;
- observer authority;
- observer completeness.

The packaging layer may bind these values into a deterministic evidence envelope but MUST NOT raise or replace them.

If completeness is not `complete`, the envelope must expose that exact non-complete state; runner trust/labels cannot turn it into complete or PASS.

## 9. Frozen falsification corpus

A2 must execute exactly these 16 tests:

1. valid Linux x86_64 self-hosted context maps to an eligible deterministic package record;
2. label ordering/duplication canonicalizes deterministically;
3. runner-ID substitution changes package digest;
4. label substitution changes provenance digest but not authority;
5. `trusted`/`prod`/`root`/`self-hosted` labels cannot upgrade `context_only` authority;
6. missing required runner context fails closed;
7. newline/control injection in runner metadata fails closed;
8. arm64 is explicitly ineligible under frozen alpha.4 package identity;
9. non-Linux OS is explicitly ineligible under frozen alpha.4 package identity;
10. environment baseline variables cannot select/set a baseline;
11. environment verdict variables cannot supply/override a verdict;
12. explicit baseline reference must be null or exact SHA256;
13. verification verdict/evidence/authority are bound exactly and cannot be upgraded by runner metadata;
14. incomplete/lost verification evidence remains non-complete and cannot be laundered by trusted labels;
15. runner-context replay/substitution rebinds the evidence-envelope digest;
16. unknown environment variables do not change canonical package/evidence output.

Acceptance threshold: **16/16**. No test deletion, weakening, architecture exemption or threshold change after execution begins.

## 10. Decision classes

A2 may close as:
- `P7_A2_SELF_HOSTED_EVIDENCE_CONTRACT_PASS_BOUNDED` — 16/16 pass and all boundary/reproof gates pass;
- `P7_A2_SELF_HOSTED_EVIDENCE_CONTRACT_FAIL` — scientifically executable corpus exposes a surviving authority/baseline/verdict/completeness flaw;
- `P7_A2_INCOMPLETE` — infrastructure or harness failure prevents scientific execution.

## 11. Required reproofs

After the corpus passes, verify:
- alpha.4 and stable `v0.1` still resolve to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no P7 research change touches public `crates/`;
- P5 and P6 closeouts remain ancestral;
- `P7_A0_ARM64_NOT_PORTABLE` remains retained;
- `P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED` remains retained;
- the historical A1 13/14 run remains recorded, not deleted or rewritten.

## 12. Promotion boundary

A positive A2 result establishes only the packaging/evidence contract. Public self-hosted support requires a separate live zero-assistance installation/execution gate on an actual self-hosted runner with explicit operational prerequisites and retained failure evidence.
