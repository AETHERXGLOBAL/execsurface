# M9.2 — Independent Adoption Evidence Closeout

Date: 2026-09-27
Status: **KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE**

## Decision

M9.2 positive independent-adoption closure is **not satisfied**.

The preregistered public GitHub discovery scope S1–S7 was executed after its rules were merged to `main`. No discovered result satisfied the frozen `INDEPENDENT_USER` provenance and workflow requirements.

Exact accepted count:

- accepted independent records: **0**;
- countable positive-close records: **0**;
- distinct accepted external repositories: **0**;
- accepted comparable/complete independent drift cases: **0**.

The correct evidence label is therefore:

**KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE (2026-09-27)**

This is a bounded negative discovery result, not a claim that ExecSurface has no users or market interest outside the declared scope.

## What remains proved from M9.1

M9.1 zero-contact compatibility evidence remains valid on its own terms. It demonstrates AETHER X-controlled reproduction against pinned external workloads and preserves both successful and negative results. It is not relabeled as adoption.

## eBPF decision

The post-trigger same-workload eBPF experiment remains negative/partial:

- the current per-invocation libbpf path failed its completeness/health gate on the triggered workloads before accepted measured samples could be collected;
- no eBPF performance-value claim was established;
- ptrace remains the correctness reference and public backend;
- eBPF remains research-only.

M9.2 does not reopen or broaden eBPF authority.

## Product interpretation

The result supports a narrow operational conclusion only:

- the product now has stronger public self-service, external-workload compatibility, and independent-evidence intake infrastructure than before M9;
- the declared public GitHub search did not supply the external evidence needed for an adoption claim;
- therefore adoption claims must remain withheld until a future third party independently produces qualifying evidence.

No runtime semantics, privacy boundary, policy threshold, verdict meaning, backend authority, or public-install path is changed by this closeout.

## Evidence

- frozen canonical M9 protocol: `docs/milestones/M9_PROTOCOL.md`
- frozen M9.2 adoption protocol: `docs/milestones/M9_2_INDEPENDENT_ADOPTION_PROTOCOL.md`
- frozen M9.2 gate: `docs/milestones/M9_2_INDEPENDENT_ADOPTION_GATE.md`
- external evaluator quickstart: `docs/milestones/M9_2_EXTERNAL_QUICKSTART.md`
- preregistered discovery scope: `docs/milestones/M9_2_DISCOVERY_SCOPE.md`
- executed search ledger: `docs/milestones/M9_2_DISCOVERY_LEDGER.md`
- eBPF negative evidence: `docs/milestones/M9_1_EBPF_TRIGGER_VALUE_SCREEN_RESULT.md`

## Next gate

**M9.3 — Evidence Synthesis / Product Decision**

M9.3 should synthesize:

1. what M9.1 proved about compatibility, drift detection, performance and friction;
2. what the eBPF value screen killed or left open;
3. what M9.2 did and did not establish about independent adoption;
4. which product changes are actually evidence-backed;
5. whether the current public alpha should remain alpha, receive a bounded release update, or require further external evidence before a broader product claim;
6. the exact public wording that may be used without overstating adoption, performance, security or eBPF capability.

M9.3 must not manufacture adoption evidence or reopen already killed claims.
