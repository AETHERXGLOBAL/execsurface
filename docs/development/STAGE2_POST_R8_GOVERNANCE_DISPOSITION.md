# ExecSurface — Stage-2 Post-R8 Governance Disposition

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Research classification: runtime behavioral verification / execution semantics / semantic-evidence correctness — **not cybersecurity research**

## Purpose

This record is the governance review required after R8. It does not reopen R0-R8 and does not add product semantics.

The question is narrower:

> Is PR #166 ready to leave Draft status for explicit review, while keeping merge and release as separate controlled decisions?

## Source of truth

Governance-review head:

`9691dd91020409cfa6d4559c6c7f721a7a76ffa1`

Internally qualified product source:

`923ca9bc7a027cf9aa0cd6d2a47a6baa840c068d`

The two commits after the qualified product source are documentation-only:

- R8 qualification result;
- research-scope anti-drift record.

No product code, schema, normalization, policy, observer, compatibility or verdict semantics changed after the qualified product source.

## Exact-head qualification

All required workflows are SUCCESS on the governance-review head:

- CI;
- Adversarial Regression;
- Ptrace Lifecycle Regression;
- P9.3 Compatibility Contract;
- Registry Packaging Gate;
- Public Consumer Smoke;
- P8 A3.4 consumer contract red team;
- P8 A3 typed report prototype;
- P8 A3 typed evidence output;
- Stage-2 Final Internal Gate.

The exact-head rerun therefore confirms that the documentation-only post-R8 commits did not invalidate the qualified state.

## Pull-request audit

At review time:

- PR #166 is open and mergeable;
- no submitted PR reviews exist;
- no unresolved inline review threads exist;
- the full changed-file set is confined to Stage-2 remediation code, tests, compatibility/CI gates, Action custody plumbing, and Stage-2 documentation;
- Alpha.5 historical evidence remains immutable;
- no release is created or authorized by this review;
- no old RED evidence is deleted or rewritten.

## Anti-drift review

Confirmed:

- research scope remains behavioral/semantic verification rather than cybersecurity research;
- policy v1/v2 meanings are not silently reinterpreted as v3;
- profile-3 Alpha.5 baselines remain historical and are explicitly incomparable with corrected profile 4 where required;
- incomplete or ambiguous evidence cannot silently become PASS;
- fail-first and destruction evidence remains retained;
- no qualification result is generalized beyond the bounded Linux x86_64 / native ptrace proposition.

## Governance decision

**PR_READY_FOR_REVIEW_BOUNDED**

PR #166 may leave Draft state and enter explicit review.

This decision does **not** authorize:

- merging PR #166;
- publishing Alpha.6, v1.0 or any other release;
- moving a stable tag;
- claiming external validation or certification;
- expanding the qualified platform/backend boundary.

A merge decision must use the exact reviewed head SHA and re-check required status checks immediately before merge.
