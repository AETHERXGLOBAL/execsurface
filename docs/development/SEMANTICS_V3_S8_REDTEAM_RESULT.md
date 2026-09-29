# ExecSurface — Semantics v3 S8 Consolidated Independent Red-Team Result

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/SEMANTICS_V3_S8_REDTEAM_PROTOCOL.md`

## Formal result

**S8 CLOSED — PASS_RESTRICTED**

The combined S1–S7 Semantics v3 design survived the declared red-team attack set without exposing an unclassified path to false authority, false completeness, silent backend equivalence, or silent v2/v3 reinterpretation.

This closes the **P2 design phase only**. It does not authorize public v3 runtime integration, public schema migration, or a new release.

## Evidence basis

S8 deliberately reused preserved executable evidence where the proposition was already established rather than rerunning closed gates merely to obtain a newer timestamp.

Primary evidence includes:
- M12.4 adversarial PATH-TOCTOU/shared-FD regression — run `36460904199`, both Ubuntu 22.04 and 24.04 matrix jobs PASS while reproducing the counterexamples;
- M8.4 loss/lifecycle semantics — real producer loss, event-budget truncation, lifecycle timeout, decode/collector failure all fail closed;
- M8.5 bounded ptrace/eBPF parity and preserved contradictions;
- S4 proof-carrying evidence prototype — accepted CI `36492226283`;
- S5 shared-FD exactness — accepted CI `36493951241`, 10 classifier tests + 6 live ptrace adversarial tests;
- S6 proposition support/authority mapping;
- S7 v2/v3 compatibility design freeze.

## Mandatory attacks

### A1 — PATH-TOCTOU
**Disposition:** `counterexample_preserved_and_model_blocks_promotion`

M12.4 reproduced ptrace pathname-intent vs kernel-consumed-object mismatches:
- Ubuntu 22.04: 153 / 300 valid truth runs mismatched;
- Ubuntu 24.04: 157 / 300 valid truth runs mismatched;
- every reproduced mismatch in the controlled fixture remained non-PASS-eligible.

Semantics v3 response:
- `file.pathname_attempt_observed` is userspace-argument/pre-kernel evidence;
- it is explicitly not `file.open_fd_associated` or kernel-object identity;
- no guarantee-entailment rule promotes it to object-grounded proof.

The counterexample remains real and is not claimed solved.

### A2 — clone without `CLONE_FILES`
**Disposition:** `bounded_fixture_pass`

S5 proved in executable classifier and live ptrace evidence that known clone flags with `CLONE_FILES` clear are not classified as shared. Public v2 remains fail-closed because it does not retain this relation evidence.

No shared authority is inferred merely from clone/thread-like activity.

### A3 — actual `CLONE_FILES` sharing with close/dup/reuse races
**Disposition:** `bounded_fixture_pass`

S5's shared-table fixture closes the original fd, opens a replacement, uses `dup2` where required, and performs a covered read through the reused fd number. The accepted test attributed the read to the replacement path, not stale old-path state.

This is bounded fixture evidence, not a universal fd-race proof.

### A4 — fd-number reuse
**Disposition:** `bounded_fixture_pass`

Covered by the same S5 adversarial path. Numeric fd continuity is not treated as object-identity continuity when state changes.

### A5 — exec / de-threading interaction
**Disposition:** `bounded_fixture_pass`

S5's thread-originated exec fixture preserved the inherited fd identity for the declared case and kept public v2 fail-closed. The design does not generalize this into proof for every Linux exec/de-thread edge case.

Unknown or unsupported transitions remain non-admissible for propositions that require exact fd-table continuity.

### A6 — producer loss / resource truncation
**Disposition:** `counterexample_preserved_and_model_blocks_promotion`

M8.4 proved real ring-buffer producer loss, event-budget truncation, consumer-lag-induced loss, lifecycle timeout, decode failure and collector failure all become explicit incomplete states rather than clean evidence.

S5 additionally proved resource truncation remains fail-closed even when fd-table relation evidence is otherwise exact.

Semantics v3 keeps transport/resource/lifecycle completeness independent of behavioral match.

### A7 — causal-lineage gaps
**Disposition:** `counterexample_preserved_and_model_blocks_promotion`

M8.5 preserved a real nested-thread lineage contradiction caused by TGID/TID parent confusion before correction. The v3 model treats lineage as an explicit proposition/dependency and uses structural actor roles for cross-run comparison.

Unknown lifecycle/lineage dependencies propagate incompleteness rather than silently becoming a valid causal chain.

### A8 — mixed-backend unsupported proposition
**Disposition:** `unsupported_non_comparable`

S6 explicitly maps unsupported eBPF read/write/network/path-attempt families as unsupported rather than projecting them from supported process/open evidence.

Same event string or backend name cannot manufacture proposition support.

### A9 — v2/v3 comparison confusion
**Disposition:** `unsupported_non_comparable`

S7 freezes cross-schema default incomparability:
- v2 baseline vs v3 candidate -> schema-incomparable;
- v3 baseline vs v2 candidate -> schema-incomparable.

v2 is never parsed as proof-carrying v3 evidence and no default PASS-eligible v2->v3 projection exists.

### A10 — authority laundering
**Disposition:** `counterexample_preserved_and_model_blocks_promotion`

S2/S3 authority is guarantee-set based, not backend-ranked. S4 proves weak evidence cannot satisfy stronger proof requirements. S6 confirms pathname-attempt, successful-open, fd-effect and kernel-object propositions remain distinct.

No declared path converts a backend label, canonical string or missing proof field into stronger authority.

## Additional synthesis attacks

### A11 — same behavior / weaker proof
**Blocked.** Future v3 diff must surface proof downgrade/inadmissibility separately from behavioral equality.

### A12 — target exit 0 / incomplete observer
**Blocked.** Target success never substitutes for observer completeness. This is already demonstrated by E4 and multiple loss/resource gates.

### A13 — backend label as trust score
**Rejected.** No scalar backend authority exists; guarantees are proposition-scoped and partially ordered.

### A14 — positive existence generalized into absence
**Blocked.** M8.5 preserved the distinction: a focused resolved positive witness may prove existence while unresolved relevant identity blocks absence.

### A15 — exact fd-table relation generalized into all fd effects complete
**Blocked.** `process.fd_table_relation` is a dependency, not a universal completeness certificate. close/dup/reuse, identity, lifecycle, resource and transport prerequisites remain separate.

### A16 — local/global completeness scoping error
**Classified conservatively.** S2/S3 allows local scoping only when dependency propagation is known. Unknown dependency graph fails broader, not narrower.

### A17 — unstable forensic IDs enter baseline equality
**Rejected.** Canonical proof profile is separated from run-local forensic trace links; TIDs, event sequence numbers, timestamps and transient transport IDs are not baseline-equality inputs by default.

### A18 — unsupported neighboring proposition inferred
**Rejected.** Proposition families have explicit support/capability states; support does not transit laterally without a declared entailment/dependency rule.

## Independent falsifier decision

No mandatory or synthesis attack produced an unclassified false-authority or false-completeness path within the declared design scope.

The strongest surviving limitations remain explicit:
- ptrace pathname observations are not kernel-object identity;
- shared-FD exactness is proved only for bounded S5 fixtures and relation semantics, not universal fd attribution;
- current research eBPF support is a strict subset;
- clone3 and unresolved identity cases remain fail-closed/non-comparable where not proved;
- v3 runtime integration itself has not been proven;
- external independent review may still produce new counterexamples and must override this bounded closure if valid.

## P2 design acceptance checklist

1. current-contract audit closed — **YES**;
2. proposition taxonomy accepted for falsification — **YES**;
3. authority/completeness semantics accepted — **YES**;
4. proof-carrying record prototype deterministic and CI-passing — **YES**;
5. shared-FD experiment produced a falsifiable bounded result — **YES / PASS_RESTRICTED**;
6. v2 preservation/v3 compatibility plan frozen — **YES**;
7. consolidated red team found no unclassified false-authority path in the declared suite — **YES / BOUNDED**.

## P2 design decision

**`P2_SEMANTICS_V3_DESIGN_ACCEPTED_BOUNDED`**

The Semantics v3 design is materially more precise than v2 for proposition-scoped authority/completeness and is suitable to serve as the semantic foundation for later development.

This decision does **not** authorize replacing public v2. A future runtime-integration effort must be separately gated and must preserve all P2 counterexamples and compatibility constraints.

## Public state remains unchanged

- public release: `v0.1.0-alpha.4`;
- public raw/canonical/baseline contract: v2;
- public/default backend: native ptrace;
- research eBPF: no PASS/learn/check/auto-selection authority;
- stable `@v0.1`: unaffected by P2 development.

## Next program step

P2 design is complete. Per parent program #100, the next development gate is **P3 — legitimate variance / nondeterminism model**, while any Semantics v3 runtime integration remains a separately gated future effort and cannot bypass S7/S8 constraints.
