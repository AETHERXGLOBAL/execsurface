# ExecSurface — P7-A2 Dual-Layer Closeout Requirement

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED BEFORE A2 RESULT REVIEW**

## Purpose

P7-A2 now contains two separately preregistered, complementary research surfaces:

1. **A2-E — self-hosted evidence envelope**
   - protocol: `P7_A2_SELF_HOSTED_EVIDENCE_PROTOCOL.md`;
   - frozen corpus: 16 tests;
   - focus: package identity, runner context, baseline/verdict/authority/completeness separation.

2. **A2-P — executable package contract**
   - protocol: `P7_A2_SELF_HOSTED_CI_PROTOCOL.md`;
   - frozen corpus: 18 tests plus an executable alpha.4 release-package probe;
   - focus: exact release SHA256, explicit preapproved baseline bytes, native exit-code preservation and S0-S3 execution.

These are not substitutes. A2-E proves the abstract evidence boundary; A2-P tests the package boundary against the actual public alpha.4 artifact.

## Final A2 closeout rule

A positive P7-A2 closeout is permitted only if **both** separately frozen layers pass their original criteria without weakening:

- A2-E: exact 16/16 plus its frozen boundary/reproof gates;
- A2-P: exact 18/18 plus the exact-release executable probe, including native S0-S3 vector `0/10/2/10` and zero S1-S3 false PASS.

No failure in one layer may be hidden by success in the other.

If either layer exposes a surviving scientific flaw, final A2 is FAIL until the frozen flaw is addressed under an explicitly recorded smallest correction and the same criterion is re-executed.

If infrastructure prevents one layer from scientific execution, final A2 remains INCOMPLETE.

## Shared immutable boundaries

Neither layer may:
- modify public alpha.4/stable tags or public crates;
- promote arm64 after A0's negative result;
- upgrade authority from self-hosted ownership/labels/provider/root status;
- auto-approve a same-job learned baseline;
- allow CI environment variables to steer baseline or verdict;
- reinterpret incomplete/error evidence as PASS;
- claim public self-hosted support.

## Positive consolidated decision label

Only after both layers satisfy their frozen gates may A2 close as:

`P7_A2_SELF_HOSTED_CONTRACTS_PASS_BOUNDED`

This label means bounded research contract evidence only. It does not authorize public self-hosted support or zero-assistance deployment claims.
