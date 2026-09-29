# ExecSurface — P3 V5-R2 R5 Independent False-PASS / Falsification Protocol

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO FINAL P3 DECISION MAY PRECEDE THIS GATE**

## Frozen predecessors

R3-C: `P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED`.
R4: `P3_R4_CHECK_SET_COMPLETE_SENTINEL_PROTECTED`.

Immutable references:
- R2-C learning artifact `11035133723`, SHA-256 `sha256:a98a1f022e601f0b0653474df8c4500f33fa5670f7e25f105740b2e79bd2fd26`;
- R4 artifact `11037682900`, SHA-256 `sha256:99f0acef1213a94f08ea6ff5d5908da927098a80505cbc1f4fec1eb74ee4afd1`;
- raw learning-set digest `sha256:437a9f8fca9f0ff4c8573c1cfee06b46cb325d1d1a890649ba536acd8e2e648d`;
- projected learning-set digest `sha256:16e8519c83ea374359d1e44872d570ccc15be88b7ccf8f9e3b54b5e3d6b7ab30`;
- explicit accepted-variable set: **empty**.

No R5 attack may modify those references, relax a threshold, broaden the GCC grammar, or turn recurrence into authorization.

## Team

Fixed roles:
1. **Innovation Scientist / Systems Architect** — search for structural false-PASS paths rather than adding heuristics.
2. **Anti-Drift / Scientific Integrity Reviewer** — owns frozen inputs, forbidden relaxations and claim boundary.
3. **Independent Falsifier / Red Team** — owns all attack construction and must attempt to break the result.
4. **Independent Milestone Reviewer** — verifies exact artifacts, test inventory, failure retention and allowed decision.

Dynamic specialists:
- supply-chain attack modeling;
- canonical effect identity / provenance semantics;
- Rust property/adversarial testing;
- build-system nondeterminism;
- CI artifact integrity;
- Linux observer completeness;
- normalization collision analysis.

## Mandatory attack matrix

R5 must execute all of the following without post-hoc replacement:

1. **Single-run poisoning** — malicious effect in one learning run remains descriptive, never authorized.
2. **Repeated poisoning** — recurrence in multiple learning runs still grants no permission.
3. **Duplicate evidence inflation** — duplicated evidence digest is rejected.
4. **Incomplete evidence poisoning** — incomplete observation is rejected before analysis.
5. **Run-order manipulation** — report/contract remains deterministic under permutation.
6. **Profile / observer / normalization mismatch** — incomparable evidence fails closed.
7. **GCC grammar mimicry** — wrong actor, wrong role, wrong root or structurally similar path is not projected.
8. **Actor substitution** — exact acceptance cannot transfer to a different actor.
9. **Causal-chain substitution** — same target and actor under a different execution chain cannot inherit exact acceptance.
10. **Unseen-similar effect** — similar identity cannot inherit an accepted exact identity.
11. **Invariant laundering** — invariant behavior cannot be selected as variance.
12. **Non-target projection preservation** — bounded GCC projection must leave every non-target effect unchanged.
13. **Meaningful-drift sentinel replay** — the frozen R4 sentinel evidence must remain unseen/residual and unaccepted.
14. **Artifact/source integrity** — exact R2-C and R4 artifact digests must verify; an intentionally corrupted local copy must fail digest verification.
15. **Public-path anti-drift** — M11 public shared-FD contract remains 6/6 PASS and public/default semantics remain legacy conservative.

## Failure policy

Any false authorization, hidden non-target effect, accepted sentinel, profile bypass, digest bypass, or public-contract regression closes R5 negatively. No attack may be removed after observing failure.

Allowed outcomes:
- `P3_R5_FALSIFICATION_PASS_BOUNDED`
- `P3_R5_FALSE_PASS_FOUND`
- `P3_R5_NON_TARGET_COLLAPSE_FOUND`
- `P3_R5_PROVENANCE_BYPASS_FOUND`
- `P3_R5_PUBLIC_CONTRACT_REGRESSION`
- `P3_R5_INCOMPLETE_EVIDENCE`

A bounded PASS authorizes only a P3 product decision. It does not authorize public integration or release.
