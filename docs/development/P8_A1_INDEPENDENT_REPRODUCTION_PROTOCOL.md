# ExecSurface — P8-A1 Independent Zero-Assistance Reproduction Protocol

Date: 2026-09-30
Parent program: #100
P8 issue: #114
Predecessor: `P8_A0_EXTERNAL_VALIDATION_PACK_READY_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — EXTERNAL ATTEMPT REQUIRED / NO A1 RESULT YET**

## 1. Question

Can at least one evaluator outside AETHER X independently attempt the frozen public ExecSurface alpha.4 reproduction path with no live AETHER X execution/troubleshooting assistance before the initial result is recorded?

A1 measures independence and reproducibility of the public path. It does not require the external result to be positive.

## 2. Frozen public target

- release: `v0.1.0-alpha.4`;
- release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- platform: Linux x86_64 only;
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`;
- evaluator packet frozen under P8-A0;
- public main freeze for the A0 packet: `b47721ce675444ce029b048edce6fc94631d6408`.

If public documentation changes after this freeze, the evaluator must identify which revision they actually used. Do not silently map a newer docs result back to the frozen A0 packet.

## 3. Fixed roles

1. **Innovation Scientist / Systems Architect** — extract useful architecture/product hypotheses from the result rather than optimize for a successful demo.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks assisted-result laundering, cherry-picking, evaluator prestige weighting, and silent exclusion of failed attempts.
3. **Independent Falsifier / Red Team** — challenges evaluator independence, workload provenance, assistance level, target identity and any claimed PASS/FAIL meaning.
4. **Independent Critical-Milestone Reviewer** — verifies the initial external record is preserved before AETHER X troubleshooting/fixes.

Dynamic specialists:
- reproducibility / CI;
- Linux x86_64 runtime;
- developer-experience diagnostics;
- privacy/data minimization;
- evidence integrity.

## 4. Minimum qualifying relationship

The initial evaluator must be declared as one of:
- `UNAFFILIATED`;
- `UPSTREAM_OR_ECOSYSTEM_MAINTAINER`;
- `USER_OR_EVALUATOR`;
- `PRIOR_CONTACT_NO_PROJECT_ROLE`.

`AETHER_X_AFFILIATED` does not qualify.
A current ExecSurface contributor may provide useful evidence but does not satisfy independent A1 for the area they implemented without a separately justified independence boundary.

## 5. Assistance freeze

For A1, the initial attempt must be:

**`ZERO_ASSISTANCE`**

Allowed before the initial result:
- public repository/docs/issues/discussions;
- public release files/checksums;
- public crates.io/docs.rs material;
- public GitHub Action;
- normal third-party tooling documentation.

Not allowed before recording the initial result:
- AETHER X live debugging;
- private step-by-step troubleshooting;
- AETHER X operating the evaluator's machine/CI;
- a custom baseline/policy supplied privately to force the demo through;
- changing the expected outcome after seeing the attempt.

After the initial result is frozen, an assisted retry is allowed only as a separately linked record.

## 6. Minimum initial evidence record

The external record must include enough to establish:
- evaluator/handle or stable evidence source;
- relationship declaration;
- approximate timestamp/date;
- ExecSurface version or release target;
- OS + architecture;
- install route attempted;
- whether `execsurface --version` and `doctor` ran;
- baseline/unchanged/drift path attempted, if reached;
- initial result: success / failure / partial / unsupported;
- material error/friction if any;
- assistance = `ZERO_ASSISTANCE`;
- source/evidence locator or preserved external message/log sufficient for review.

Secrets/private source/environment values are not required.

## 7. Scientific outcome classes

A qualifying external attempt closes A1 into one of:

### `P8_A1_ZERO_ASSISTANCE_REPRODUCTION_PASS_BOUNDED`
The evaluator independently reaches the documented bounded public path without AETHER X live assistance, and the observed result is materially consistent with the documented target.

### `P8_A1_ZERO_ASSISTANCE_REPRODUCTION_FAILURE_BOUNDED`
The evaluator independently attempts the correct target but fails due to install/runtime/docs/usability/product behavior. The failure itself is accepted external evidence and must enter a finding-specific reproduction/falsification gate.

### `P8_A1_ZERO_ASSISTANCE_PARTIAL_OR_UNSUPPORTED_BOUNDED`
The attempt is independent but cannot complete for a declared environmental/support boundary or only reaches part of the flow. The result is retained and may expose documentation/support friction.

### `P8_A1_EXTERNAL_RECORD_INSUFFICIENT`
An external contact/result exists but lacks enough target/attempt information to classify reproducibly. Do not call it independent reproduction.

## 8. No silent retry laundering

If an evaluator reports failure, then succeeds after AETHER X assistance:
- initial record remains `FAILURE` or `PARTIAL`;
- assisted retry receives a new evidence ID;
- the assisted retry may show a fix/workaround is effective but does not convert the original attempt into zero-assistance PASS.

## 9. Product change rule

Do not change code/docs based solely on a vague external complaint. First preserve the raw external result, then reproduce/test it under a finding-specific protocol where feasible.

For an obvious documentation typo that blocks reproduction, the original failed attempt still remains in evidence after the typo is fixed.

## 10. Security boundary

If the external attempt plausibly reveals a vulnerability, stop public reproduction detail and use the private security process. A security report can qualify as external evidence without publishing exploit detail.

## 11. A1 cannot be satisfied internally

The following cannot close A1:
- AETHER X GitHub Actions;
- internal clean VM/container runs;
- compatibility tests against third-party repositories run by AETHER X;
- OpenSSF Slack membership/posting;
- Linux/Rust Foundation routing replies;
- downloads/stars/views;
- historical EXT-0001 architecture criticism (valuable but not reproduction).

## 12. Current state at preregistration

No qualifying current zero-assistance external reproduction record has been received after the P8-A0 freeze.

Therefore:

**`P8_A1_EXTERNAL_ATTEMPT_PENDING`**

This is a genuine external dependency, not a reason to weaken or replace the gate with internal testing.
