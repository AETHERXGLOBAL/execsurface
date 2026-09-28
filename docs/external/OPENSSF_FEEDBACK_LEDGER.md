# OpenSSF External Feedback Ledger

Tracking: #94

Purpose: preserve every material OpenSSF / Linux Foundation / external community criticism as evidence, including negative, inconclusive and no-fit outcomes.

## Processing rule

`CLAIM -> CRITICISM -> TEST -> EVIDENCE -> DECISION`

Do not replace this with a defense narrative.

## Status vocabulary

- `OPEN` — criticism recorded, test not complete.
- `REPRODUCED` — criticism/counterexample reproduced.
- `NOT_REPRODUCED_TESTED_SCOPE` — not reproduced under declared scope; not proof of impossibility.
- `ACCEPTED` — evidence supports changing claim/architecture/docs/product.
- `REJECTED_WITH_EVIDENCE` — evidence contradicts criticism under declared scope.
- `INCONCLUSIVE` — evidence insufficient.
- `NO_FIT` — technically valid but outside ExecSurface's intended layer/scope.

## Severity vocabulary

- `S0` informational / wording.
- `S1` usability / documentation / interoperability friction.
- `S2` bounded semantic or completeness risk that must constrain claims.
- `S3` material evidence-authority / false-PASS / security boundary risk.
- `S4` disclosure-sensitive security issue; stop public discussion and use the security process.

## Ledger

| ID | Source | Date | Claim criticized | Criticism | Severity | Reproducibility | Status | Resulting action |
|---|---|---|---|---|---|---|---|---|
| EXT-0001 | Greg Kroah-Hartman | 2026-09-28 | ptrace as correctness-reference mechanism | Asked why ptrace is used rather than Linux Security Module mechanisms such as SELinux/AppArmor; highlighted architectural/security concerns with ptrace-centric observation. | S3 | Architecture review + adversarial M10/M12 experiments | ACCEPTED | `HYBRID_ARCHITECTURE_RECOMMENDED`; bound ptrace authority; reproduce PATH-TOCTOU/shared-FD counterexamples; alpha.4 fail-closed shared-FD hardening; BPF-LSM/kernel-hook research retained non-default. |
| DOC-0001 | Internal OpenSSF baseline freeze | 2026-09-28 | current independent-evaluation documentation | `TECHNICAL_EVALUATION.md` and `INDEPENDENT_EVALUATION.md` still named alpha.3 after alpha.4 publication. | S1 | Direct repository inspection | ACCEPTED | Update both documents to exact alpha.4 public artifact and current authority/limitation boundaries before OpenSSF submission. |

## External-submission rule

If a community participant reports a failure while using only the public repository and pack, record it here **before** offering implementation guidance.

Do not convert a later assisted success into the original attempt's PASS evidence.

## Disclosure rule

If feedback plausibly reveals a vulnerability that should not be public, classify it `S4`, stop public reproduction details, and follow the repository security policy before continuing engagement.
