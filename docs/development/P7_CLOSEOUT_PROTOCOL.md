# ExecSurface — P7 Platform / CI Expansion Closeout Protocol

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED CLOSEOUT — NO P7 CLOSEOUT DECISION YET**

## Purpose

Close P7 only if the evidence accumulated across A0-A2 can be reconciled without deleting negative results, weakening public semantics, or opening platform work merely to increase platform count.

## Required evidence

P7 may close positively only if all of the following hold:

1. **A0 arm64 result is retained exactly as negative evidence**
   - decision: `P7_A0_ARM64_NOT_PORTABLE`;
   - original native arm64 workflow remains a failed run;
   - no arm64 public-support claim or release asset is introduced.

2. **A1 GitLab context adapter is bounded and evidence-backed**
   - decision: `P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED`;
   - historical 13/14 harness-defect run remains recorded;
   - corrected accepted run is exact 14/14;
   - no live/public GitLab support claim.

3. **A2 self-hosted contracts satisfy the dual-layer freeze**
   - A2-E exact 16/16 + boundary/reproof gates;
   - A2-P exact 18/18 + exact alpha.4 release-package probe;
   - executable S0-S3 vector is exactly `0 / 10 / 2 / 10` with zero S1-S3 false PASS;
   - consolidated decision is `P7_A2_SELF_HOSTED_CONTRACTS_PASS_BOUNDED`.

4. **Immutable public boundary remains intact**
   - `v0.1.0-alpha.4` and stable `v0.1` still resolve to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
   - no P7 change after P6 closeout touches public `crates/`;
   - public v2 semantics/default observer remain unpromoted by P7.

5. **Anti-drift condition**
   - no additional OS family is opened without a concrete measured use case and separately preregistered evidence contract;
   - live GitLab/self-hosted promotion remains separate from bounded research contracts;
   - runner labels/ownership/provider names never gain behavioral authority.

## Exact workflow evidence to re-verify

- A0 native parity run `36774237512` — expected conclusion `failure`, retained as scientific negative result;
- A1 corrected run `36775178743` — expected `success`;
- A2-P executable package run `36775768709` — expected `success`;
- A2-E evidence-envelope run `36775802501` — expected `success`.

Artifacts to verify by ID/digest:
- A1: `11125646709`, digest `sha256:b86b68056751301db72da23b2fab52939a4212192913e004bafce3254a7fd1fe`;
- A2-P: `11125257579`, digest `sha256:f3736483d24fbfad4b7f2fe110c544d0ef23d438e8b97e49e7e1a1d4d52412b2`;
- A2-E: `11125232540`, digest `sha256:b8455e5ad2eecf26cccfe204bb0466c220f4d5416d3513b62531aee06370a8ed`.

A0 artifacts remain referenced by `P7_A0_ARM64_DECISION.md` and must not be removed.

## Positive closeout label

Only if every condition above passes may P7 close as:

**`P7_PLATFORM_CI_RESEARCH_COMPLETE_BOUNDED`**

This label means the bounded research phase is complete. It does not mean public arm64/GitLab/self-hosted support is complete.

## Failure / incomplete handling

- Any contradiction in run/artifact identity, immutable tags, or retained negative evidence blocks closeout.
- A transient closeout-harness problem is `P7_CLOSEOUT_INCOMPLETE`, not a scientific failure of prior gates.
- Do not change the closeout criteria after execution begins.
