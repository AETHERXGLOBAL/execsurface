# ExecSurface — P3 V1 Provenance-Safe Multi-Run Variance Analyzer Result

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Threat model: `docs/development/P3_VARIANCE_THREAT_MODEL.md`

## Formal result

**V1 CLOSED — `P3_V1_ANALYZER_PASS_RESEARCH_ONLY`**

The isolated research analyzer can deterministically summarize recurrence across explicitly supplied trusted baseline artifacts without creating authorization or verdict semantics.

This is not a public baseline replacement, not a multi-run PASS mechanism, and not a release authorization.

## Accepted implementation

Research path:
`experiments/p3-variance-analyzer/`

Accepted CI run:
`36549710912`

Accepted source:
`093185de9146bccaea4a3bfe389afd95abe63f98`

Accepted run result:
- `cargo fmt ... -- --check` — PASS
- `cargo clippy --locked ... --all-targets -- -D warnings` — PASS
- `cargo test --locked ... --all-targets` — PASS
- library tests: **6 passed / 0 failed**

## Proven properties

### 1. Descriptive recurrence, not authorization

For a set of trusted comparable runs the analyzer computes:
- invariant intersection;
- union;
- per-effect support count;
- exact source evidence digests;
- deterministic learning-set digest;
- `invariant` vs `variable_candidate` classification.

It emits no ALLOW/PASS/REVIEW/BLOCK decision and has no automatic accepted-variance output.

### 2. Input-order determinism

The same distinct learning artifacts supplied in different order produce byte-identical serialized report output.

### 3. Duplicate-artifact resistance

Duplicate `evidence_digest` values are rejected rather than counted twice, preventing one artifact from inflating recurrence support.

### 4. Incomplete-run rejection

`observation_complete=false` is rejected before recurrence analysis. Missing effects in incomplete evidence therefore cannot masquerade as legitimate absence.

### 5. Baseline integrity

Each supplied baseline is verified through the existing v2 baseline verifier before use. A corrupted baseline digest is rejected.

### 6. Comparability gating

Runs must match the frozen research profile across tool, command, platform, observer contract, canonical schema, normalization profile and semantic roots. The test suite proves an architecture mismatch is rejected.

## Preserved failures and corrections

The accepted result followed several preserved engineering failures:

1. run `36549051579` failed at rustfmt before clippy/tests; this was formatting-only and was corrected without semantic changes;
2. run `36549272076` passed formatting but failed because the hand-constructed nested lockfile did not match Cargo's generated dependency graph/checksum state;
3. one-shot diagnostic run `36549415757` regenerated the nested lockfile and printed the exact delta;
4. intermediate invalid lockfile edits were not accepted as evidence;
5. the final committed nested lockfile matches the Cargo-generated content identity (`0c81c318e8816472497894c1fc0341df37a3dcd7`) and the locked CI then passed.

No test, comparability rule, completeness prerequisite or anti-poisoning rule was weakened to obtain the final PASS.

## Scientific interpretation

V1 establishes that the project can represent multi-run recurrence as auditable evidence without collapsing recurrence into permission.

It does **not** establish that:
- any variable candidate is safe;
- majority/frequency is authorization;
- FZF false REVIEW is solved;
- broad cache/module/source normalization is valid;
- a multi-run baseline should replace the public single-run baseline;
- incomplete evidence may enter learning.

Those questions remain for V2–V5.

## Public state

Unchanged:
- public release `v0.1.0-alpha.4`;
- public baseline/check semantics remain v2 single-run;
- no public CLI integration;
- no stable Action change;
- no automatic baseline mutation.

## Next authorized gate

**V2 — Producer-Specific Ephemeral Grammar Gate**

V2 must inspect preserved #62 evidence and derive any GCC temporary-file grammar from actual observed paths rather than assumption. A normalization candidate is admissible only with bounded-root, positive, negative and collision/adversarial fixtures, and only if meaningful path changes remain distinguishable.
