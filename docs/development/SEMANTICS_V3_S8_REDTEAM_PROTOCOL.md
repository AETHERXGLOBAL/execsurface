# ExecSurface — Semantics v3 S8 Consolidated Independent Red-Team Protocol

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor: `SEMANTICS_V3_S7_COMPATIBILITY_RESULT.md` — **PASS_RESTRICTED**
Status: **FROZEN BEFORE S8 CLOSURE DECISION**

## Objective

Attempt to invalidate the combined S1–S7 Semantics v3 design before P2 design closure.

S8 is not a documentation consistency check. The red-team question is whether any known counterexample can still create an unclassified path to:
- false authority;
- false completeness;
- false cross-backend equivalence;
- silent v2/v3 reinterpretation;
- or PASS-eligible evidence unsupported by the actual observer.

## Evidence-reuse rule

Historical executable counterexamples may be reused only when:
1. the original result remains preserved and reproducible enough to constrain the current design;
2. S8 does not strengthen the original claim;
3. the Semantics v3 response to that counterexample is explicit;
4. any gap not covered by existing evidence remains open rather than inferred away.

A new runtime rerun is required only if the v3 design depends on a fact not established by preserved evidence.

## Mandatory attack set

The Issue #101 preregistered attacks remain mandatory:

1. PATH-TOCTOU;
2. clone without `CLONE_FILES`;
3. actual `CLONE_FILES` sharing with close/dup/reuse races;
4. fd-number reuse;
5. exec / de-threading interaction;
6. producer loss / resource truncation;
7. causal-lineage gaps;
8. mixed-backend unsupported proposition;
9. v2/v3 comparison confusion;
10. authority laundering — weak evidence represented as stronger proof.

## Additional synthesis attacks

11. same canonical behavior but weaker proof guarantees;
12. successful target exit with incomplete observer evidence;
13. backend label used as a global trust score;
14. positive existence proof generalized into absence proof;
15. exact fd-table relation generalized into complete fd-effect attribution;
16. session-local ambiguity incorrectly made global or global loss incorrectly scoped local;
17. unstable forensic identifiers entering baseline equality;
18. unsupported proposition inferred from a neighboring supported proposition.

## Required disposition per attack

Each attack must be classified as one of:
- `counterexample_preserved_and_model_blocks_promotion`;
- `bounded_fixture_pass`;
- `blocked_incomplete`;
- `unsupported_non_comparable`;
- `design_gap_found`.

Any `design_gap_found` that can produce false authority/completeness blocks S8 closure and returns P2 to the owning S1–S7 gate.

## Closure criterion

S8 can close `PASS_RESTRICTED` only if:
- every mandatory and synthesis attack has an explicit fail-closed disposition;
- no counterexample is erased or reclassified as proof of stronger authority;
- no public alpha.4 semantics change is required;
- no v3 runtime integration is smuggled into the design gate;
- remaining limitations are explicit and proposition-scoped.

S8 cannot authorize public v3 release or runtime integration. It can only close the **P2 design phase** and authorize a separately gated future implementation effort.
