# P8-A3.5 — Zero-Assistance External Typed-Evidence Trial Packet

Date: 2026-10-04  
Tracking: #157  
Implementation PR: #158  
External evidence authority: #114  
Real-workload authority: #140

## Decision

`EXTERNAL_TYPED_EVIDENCE_TRIAL_PACKET_READY_BOUNDED`

This decision means the packet is internally qualified for use by a future external evaluator.

It does **not** mean:
- external validation occurred;
- an external real workload was tested;
- P8-A1, P8-A4 or P9.5 passed;
- adoption occurred;
- the typed-evidence interface is stable or released.

## T0 — Frozen product and protocol boundary

The experimental product target is frozen at:

`cea214244d7efe05fea2108a5f5d8ed6287d9a3f`

The packet is separate from the product target.

The trial remains distinct from the published Alpha.5 independent-reproduction track because typed evidence is unreleased and available only from the frozen source commit.

Qualification of any later external result remains under #114 / #140.

Accepted result:

`T0_PRODUCT_TARGET_AND_EXTERNAL_EVIDENCE_BOUNDARY_FROZEN`

## T1 — Trial manifest and evaluator guide

Added:
- `docs/external/P8_A3_TYPED_EVIDENCE_TRIAL_MANIFEST.json`;
- `docs/external/P8_A3_TYPED_EVIDENCE_EXTERNAL_TRIAL.md`.

The manifest freezes:
- packet identity;
- exact experimental product source SHA;
- Linux x86_64 scope;
- evidence schema/reference consumer;
- evaluator-selected workload rule;
- initial-result preservation rule;
- privacy collection boundary;
- non-automatic qualification boundary.

The guide requires the evaluator to:
- select the workload independently;
- preserve the first result;
- use a fresh output directory for any retry;
- inspect raw typed evidence for path sensitivity before sharing;
- avoid posting secrets/private content;
- record assistance received before the initial result.

Accepted result:

`T1_ZERO_ASSISTANCE_CAPABLE_GUIDE_AND_MANIFEST_PASS`

## T2 — Privacy-minimized capture harness

Added stdlib-only harness:

`scripts/p8_a3_external_trial_capture.py`

The harness:
1. verifies the exact packet revision supplied by the evaluator;
2. verifies the packet tracked worktree is clean;
3. verifies the target source is exactly the frozen product commit;
4. rejects a dirty frozen target worktree;
5. requires the workload directory to be outside packet/product trees;
6. requires a fresh trial output directory outside packet/product/workload trees;
7. builds the frozen target using `cargo build --locked -p execsurface`;
8. runs `execsurface doctor`;
9. executes one evaluator-supplied command through `observe --evidence-output`;
10. independently validates the evidence using the existing reference consumer/schema;
11. records bounded result metadata and evidence hash.

The retained summary deliberately does not record:
- command/argv;
- target stdout;
- target stderr;
- environment values;
- file contents.

The typed-evidence artifact may contain paths and therefore must be locally reviewed before sharing.

The harness never marks its own output as external evidence. Normal output remains:

`UNQUALIFIED_PENDING_EXTERNAL_EVIDENCE_REVIEW`

Internal rehearsal output is forced to:

`DISQUALIFIED_SELF_EVIDENCE`

Accepted result:

`T2_PRIVACY_MINIMIZED_CAPTURE_HARNESS_PASS`

## T3 — Failure-first packet red team

Accepted run:

`37200933060` — **SUCCESS**

Red-team attacks passed:
- wrong product source SHA rejected before workload execution;
- dirty frozen target rejected before workload execution;
- missing schema rejected;
- packet/target used as workload directory rejected;
- invalid schema caused independent consumer rejection;
- target exit 7 was preserved as target outcome rather than rewritten as harness failure;
- evidence-output collision remained fail-closed and preserved workload-owned bytes;
- output-directory reuse was rejected without altering the first result;
- a real write-bearing artifact still passed the 27-case mutation/substitution rejection corpus.

Accepted result:

`T3_TRIAL_PACKET_SUBSTITUTION_PRIVACY_AND_PRESERVATION_ATTACKS_PASS`

## T4 — External report / intake surface

Added:
- `docs/external/P8_A3_TYPED_EVIDENCE_TRIAL_REPORT_TEMPLATE.md`;
- `.github/ISSUE_TEMPLATE/typed-evidence-trial.yml`.

The intake surface explicitly records:
- relationship/independence;
- assistance before the initial result;
- packet revision;
- frozen product SHA;
- evaluator-selected workload;
- environment;
- machine-readable outcomes;
- suspected false completeness/effect claims;
- friction/privacy/performance/no-fit observations;
- safe hashes/evidence locator;
- security-sensitive routing.

Raw typed evidence is not required to be posted publicly.

Accepted result:

`T4_INITIAL_RESULT_AND_PRIVACY_SAFE_INTAKE_PASS`

## T5 — Internal rehearsal

Internal rehearsals executed the packet against an AETHER X-controlled synthetic workload.

Both environments passed:
- Ubuntu 22.04: PASS;
- Ubuntu 24.04: PASS.

Every rehearsal is mechanically marked:

`DISQUALIFIED_SELF_EVIDENCE`

No internal rehearsal is counted as independent evidence.

Accepted artifacts:
- Ubuntu 22.04: `11301619820`
  - digest `sha256:5e26a37a1a652422c3279f6c73169907595c1c28e4fb3a3fa7bb4472cdfc2958`
- Ubuntu 24.04: `11302153777`
  - digest `sha256:66a509f2399558e376cd91349e9b8cff74b3c88b70b67df56e482a2da07c1401`
- packet red-team artifact: `11301624861`
  - digest `sha256:1d662d17f1c97b1d744ed2102c0f50c680d0434edc5925c046e369ddd2749697`

Accepted result:

`T5_INTERNAL_REHEARSAL_EXECUTABLE_BUT_SELF_EVIDENCE_DISQUALIFIED`

## Retained negative evidence

Initial packet run:

`37200876012` — **FAILURE retained**

The failure occurred only in the final mutation-corpus step.

Cause:
the mutation red team was mistakenly pointed at the target-failure artifact produced by `/bin/sh -c 'exit 7'`. That artifact correctly contained no typed file-write effect, while the mutation corpus requires a write-bearing base artifact.

Observed failure:

`fixture has no typed effect`

The correction changed only the mutation fixture locator to the real write-bearing artifact already generated by the invalid-schema attack.

No:
- product behavior;
- harness semantic rule;
- privacy rule;
- verifier rule;
- mutation assertion;
- threshold;
- independence rule

was weakened.

Retained red-team artifact:
- `11302073694`
- digest `sha256:a69773bd8cde102fb5d6fc41da447182c14cfcbaf3e09e1646a7aee2ca474063`

This is a test-fixture selection failure, not evidence of a product or verifier bypass.

## General CI

Accepted code/test source before this documentation-only closeout:

`39765e24839e1216681e5f9d4f5034797ed5f9da`

Required runs:
- P8-A3.5 external trial packet `37200933060`: **SUCCESS**
- full CI `37200933049`: **SUCCESS**

## Fixed-role review

### Innovation Lead

High-value choice:
separate the trial packet from the frozen product target so documentation/harness changes cannot silently change the product under test.

The packet also uses a privacy-minimized summary rather than requiring public raw evidence.

### Anti-Drift / Goal Alignment

Blocked:
- new product features;
- release/tag work;
- GitHub Action exposure;
- selecting workloads for evaluators;
- rewriting initial failures;
- treating CI/self-rehearsal as external evidence;
- automatic P8/P9.5 qualification.

### Independent Falsifier

Attacked:
- source substitution;
- dirty source;
- schema substitution;
- internal-workload confusion;
- target failure;
- evidence collision;
- output reuse;
- mutation/substitution;
- privacy sentinels.

The first failed packet run remains retained.

### Critical Milestone Reviewer

The accepted claim is only:

> The packet can reproducibly drive the exact frozen experimental source through a privacy-minimized, independently verified trial flow and preserve negative/partial results without claiming external evidence.

It is not a claim that any external evaluator has used it.

## T6 — Final bounded decision

`P8_A3_5_EXTERNAL_TYPED_EVIDENCE_TRIAL_PACKET_READY_BOUNDED`

After merge, the exact **packet revision** must be recorded in #157. Future evaluators must use that packet SHA while the product target remains:

`cea214244d7efe05fea2108a5f5d8ed6287d9a3f`

The next legitimate step is a genuinely external initial trial.

P8 #114 and P9.5 #140 remain open.
