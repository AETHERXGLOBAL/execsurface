# ExecSurface — Stage-2 R8 Full Qualification / FINAL_INTERNAL_GATE Result

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Protocol: `docs/development/STAGE2_R8_FINAL_QUALIFICATION_PROTOCOL.md`  
Qualified product source: `923ca9bc7a027cf9aa0cd6d2a47a6baa840c068d`  
Decision: **FINAL_INTERNAL_GATE_PASS_BOUNDED**

## Decision

R8 is GREEN for the preregistered final internal qualification proposition:

> The repaired R0–R7 invariants remain composable under the qualified Linux x86_64 candidate when custody, policy-v3 semantics, runtime observation, object identity, verdict materialization, compatibility and CI triggering are exercised together.

This closes the internal Stage-2 remediation gate. It does **not** merge PR #166, alter Alpha.5, authorize a release, or claim generic filesystem isolation or universal runtime completeness.

## Source-of-truth qualification

Qualified remediation source:

`923ca9bc7a027cf9aa0cd6d2a47a6baa840c068d`

At qualification:

- `main` remained `c7c9d317e95ec4837dcce64b311cfdc13b512676`;
- PR #166 remained open and draft;
- Alpha.5 remained immutable historical evidence;
- no predecessor acceptance assertion was weakened.

## Retained RED history

The first R8 workflow candidate at:

`e682c2956efc4597de805202aef20ea9c97f32df`

produced a CI failure before scientific execution because the new R8 test file was not rustfmt-clean.

CI run:

- `37716848936` — FAILURE

The failure was classified formatting-only from the log diff. No R8 assertion, expected exit code or hostile condition was changed.

A rustfmt-only correction was committed as:

`923ca9bc7a027cf9aa0cd6d2a47a6baa840c068d`

The RED run remains retained evidence and is not rewritten as semantic falsification or success.

## R8 composed hostile corpus

On both Ubuntu 22.04 and Ubuntu 24.04:

`r8_final_internal_gate.rs`: **5/5 PASS**

### R8-C1 — custody + policy-v3 + report materialization

PASS.

A correctly pinned baseline plus exact-byte pinned schema-v3 policy blocked a truncating open through `open_intent.truncate=true` and produced BLOCK/20 with safe report materialization.

### R8-C2 — unauthorized baseline pin dominates execution

PASS.

A well-formed but unauthorized baseline digest produced ERROR/2 before target execution. The target marker remained absent and no policy verdict report was materialized.

### R8-C3 — protected policy/output alias

PASS.

Using the trusted policy object as verdict output was rejected before target execution. Policy bytes remained unchanged.

### R8-C4 — moved trusted baseline object

PASS.

The target moved the already verified baseline inode onto the selected report path. Postflight object-identity protection returned ERROR/2 and ExecSurface did not overwrite the moved trusted object.

### R8-C5 — post-preflight policy mutation

PASS.

The target replaced the on-disk policy after custody verification. Evaluation still used the verified in-memory schema-v3 BLOCK policy and returned BLOCK/20, proving that policy evaluation does not re-read and silently substitute the changed file.

## Predecessor replay

The dedicated Stage-2 Final Internal Gate replayed the predecessor acceptance corpora without weakening:

- R1 external falsification acceptance — PASS
- R2 FD state-machine — PASS
- R3 object identity/path authority — PASS
- R4 CLI custody — PASS
- R4 Action custody — PASS
- R5 policy expressiveness — PASS
- R6 CI trigger contract — PASS
- R7 hostile cross-gate destruction — PASS
- R8 composed hostile corpus — PASS
- full workspace tests — PASS
- workspace Clippy with warnings denied — PASS
- rustfmt — PASS
- lockfile integrity — PASS

The dedicated gate passed on both supported qualification environments:

- Ubuntu 22.04 — SUCCESS
- Ubuntu 24.04 — SUCCESS

Workflow:

- Stage-2 Final Internal Gate — run `37716904783` — SUCCESS

## Adjacent qualification workflows

All required adjacent workflows completed SUCCESS on the same qualified source:

- CI — run `37716904617` — SUCCESS
- Adversarial Regression — run `37716904549` — SUCCESS
- Ptrace Lifecycle Regression — run `37716904490` — SUCCESS
- P9.3 Compatibility Contract — run `37716904590` — SUCCESS
- Registry Packaging Gate — run `37716904618` — SUCCESS
- Public Consumer Smoke — run `37716904571` — SUCCESS
- P8 A3.4 consumer contract red team — run `37716904606` — SUCCESS
- P8 A3 typed report prototype — run `37716904687` — SUCCESS
- P8 A3 typed evidence output — run `37716904708` — SUCCESS
- Stage-2 Final Internal Gate — run `37716904783` — SUCCESS

## Residual boundaries

The final internal gate does not prove:

- generic sandboxing or prevention of target-side workspace mutation;
- correctness outside the currently qualified Linux x86_64 / native ptrace boundary;
- integrity of a compromised CI/repository administrator control plane;
- wisdom of the human/process that authorized a baseline or policy;
- authority of experimental eBPF/libbpf paths;
- that unchanged observed behavior implies software safety.

The product remains a runtime behavioral-integrity / semantic-evidence system, not antivirus, EDR, SIEM or a malware detector.

## Anti-drift review

Confirmed:

- Alpha.5 was not modified or retagged;
- profile-3 history was not silently reinterpreted under profile 4;
- policy v1/v2 semantics were not rewritten by v3;
- incomplete/ambiguous evidence was not promoted into PASS;
- negative/fail-first evidence from R1–R7 remains preserved;
- the R8 formatting-only RED is preserved;
- no release or merge is implicitly authorized.

## Final internal decision

**FINAL_INTERNAL_GATE_PASS_BOUNDED**

The Stage-2 remediation program is internally qualified at product source:

`923ca9bc7a027cf9aa0cd6d2a47a6baa840c068d`

Next action is a separate governance decision about PR #166 merge/release disposition. That decision is outside R8 and requires explicit authorization.
