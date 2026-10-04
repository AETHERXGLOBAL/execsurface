# Experimental Typed-Evidence External Trial — Initial Result Template

Use this template for the **initial result only**. Do not overwrite it with a later assisted retry.

## Identity and independence

- External actor / handle:
- Relationship to AETHER X / ExecSurface:
  `UNAFFILIATED | UPSTREAM_OR_ECOSYSTEM_MAINTAINER | USER_OR_EVALUATOR | CONTRIBUTOR_TO_EXECSURFACE | PRIOR_CONTACT_NO_PROJECT_ROLE | OTHER`
- Who selected the workload:
- Assistance before the initial result:
  `ZERO_ASSISTANCE | DOCS_ONLY_CLARIFICATION | ASSISTED | COLLABORATIVE`
- Did AETHER X operate the trial for you? `yes | no`

## Frozen targets

- Trial packet revision:
- Product source SHA:
  `cea214244d7efe05fea2108a5f5d8ed6287d9a3f`
- Product status:
  `experimental-unreleased`

## Environment

- OS:
- Kernel:
- Architecture:
- CI / VM / container context if relevant:
- Rust / Cargo version if relevant:

## Workload

- Workload/repository identity if shareable:
- Workload origin:
  `EVALUATOR_OWN | PUBLIC_INDEPENDENTLY_SELECTED | PRIVATE_NON_SENSITIVE_DESCRIPTION | OTHER`
- Sanitized command description if safe:
- Why this workload was selected:

Do not paste secrets, credentials, environment values, private file contents, stdin or network payloads.

## Machine-readable trial result

From `trial-summary.json`:

- status:
- failure_code:
- build_exit_code:
- doctor_exit_code:
- observe_exit_code:
- consumer_exit_code:
- target_outcome:
- collection_health:
- typed_effect_count:
- evidence_sha256:

From `consumer-report.json`, if present:

- decision:
- raw_event_count:
- effect_count:

## Human observations

- Setup friction:
- Permission / privilege friction:
- Documentation friction:
- Unexpected behavior:
- Suspected false completeness:
- Suspected false typed effect:
- Missing effect you expected:
- Noisy/unhelpful effect:
- Privacy concern:
- Performance concern:
- Fit assessment:
  `USEFUL_FIT | USEFUL_BUT_NOISY | INCOMPLETE_FOR_WORKLOAD | UNSUPPORTED | FALSE_REVIEW | SUSPECTED_FALSE_PASS_OR_COUNTEREXAMPLE | OPERATIONALLY_TOO_COSTLY | NO_FIT_OR_REDUNDANT | INCONCLUSIVE`

## Evidence handling

- Raw typed evidence reviewed locally for sensitive paths? `yes | no`
- Raw typed evidence shared publicly? `yes | no`
- Safe artifact/log locator, if any:
- Trial summary SHA256, if independently computed:
- Consumer report SHA256, if independently computed:

Raw typed evidence is **not required** to be posted publicly.

## Security-sensitive check

- Could this be a vulnerability/security-sensitive finding? `yes | no | uncertain`

If `yes` or `uncertain`, stop posting technical exploit details publicly and use the repository security policy.

## Initial-result preservation

- Is this the untouched initial result before AETHER X troubleshooting? `yes | no`
- If this is an assisted retry, link the original initial result:

## Claims boundary

Submission does not itself establish:
- endorsement;
- adoption;
- production readiness;
- stable API/schema compatibility;
- P8-A1/P8-A4/P9.5 qualification.

AETHER X will qualify the record separately under #114 / #140.
