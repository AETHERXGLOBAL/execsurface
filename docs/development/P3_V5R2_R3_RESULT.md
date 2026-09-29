# ExecSurface — P3 V5-R2 R3 Variance Analysis Result

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **SUPERSEDED — CLASSIFICATION INVALIDATED BY NORMALIZATION-CONFIG MISMATCH**

> This file preserves the first completed R3 analysis and its metrics as negative evidence. Its prior interpretation `P3_NO_MATERIAL_VALUE` is **not an accepted final scientific decision**. Anti-drift review found that the R2 research baseline collector used `NormalizationConfig::default()` and therefore emitted `semantic_roots=[]`, while the public CLI default normalization derives workspace/home and adds `/tmp` (plus `$TMPDIR` when present). The already-frozen V2 GCC classifier requires `PathClass::Temp` / `$TMP/cc<token>.s`. Real GCC `/tmp/cc*.s` effects were present in the frozen evidence but were structurally ineligible under the mismatched R2 normalization metadata.

## Team

Fixed roles retained:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team
- Independent Milestone Reviewer

Dynamic specialists used:
- Linux ptrace / clone / fd-table completeness
- Go/GCC build nondeterminism
- Rust research-tool correctness
- canonicalization / multi-run set analysis
- reproducibility / CI evidence sealing
- provenance / artifact verification

## Frozen predecessor evidence

R0/R1/R2 originally completed under issue #105.

Accepted learning source:
`dd496fef9f62f452368c908c71b78149b2046d75`

Accepted learning workflow:
`36569389464`

Frozen learning artifact:
`11032959833`

Frozen artifact ZIP SHA-256:
`0129b5c77dfbb3d91967918e9a281dcfb135eff100023be77bd14096e5acae41`

Exactly six learning lockfiles were admitted after exactly three direct priming runs. All six research observations were complete, warning-free, certificate-positive, exit-zero and structurally verified. No sample was replaced.

## Preserved R3 pre-analysis failures

Three failures occurred before scientific analysis and remain retained:

1. workflow `36569997349` — artifact path-prefix verification plumbing;
2. workflow `36570204764` — rustfmt-only failure;
3. workflow `36570542263` — research analyzer compile defect (`EffectEvidence` lacked `Clone`).

None changed the frozen learning evidence, algorithm, threshold, metric, workload, acceptance rule or expected result.

## First completed R3 attempt — retained but not decision-valid

Workflow:
`36570775457`

Job:
`109413933682`

Source:
`539cfa821b8092f77d5316a70623172060ce9822`

Evidence artifact:
`11033972162`

Artifact upload SHA-256:
`9850ef23aa6e95c709ab534c8832e3738b4a5a28a25a4d49a55f0f6409d8872e`

Observed analyzer metrics:
- run count: **6**
- raw variable candidates: **2,075**
- targeted GCC raw variable candidates: **0**
- non-target raw variable candidates: **2,075**
- projected variable candidates: **2,075**
- canonical GCC invariant effects: **0**
- accepted variable effects: **0**
- non-target support/provenance mismatches: **0**
- lingering targeted random paths: **0**

Analyzer classification:
`P3_V5_TARGETED_VARIANCE_NOT_REPRODUCED`

## Anti-drift counterexample

Inspection of the same frozen R3 artifact found variable GCC effects including `/tmp/cc*.s` with `/usr/bin/gcc` actor and matching create/write-capable open + delete roles.

However the R2 locks contain:
- `normalization.semantic_roots=[]`;
- `/tmp/cc*.s` target class `outside_declared_roots`;
- raw `/tmp/...` values rather than `$TMP/...`.

The V2 classifier intentionally requires:
- `PathClass::Temp`;
- `$TMP/cc<6 alphanumeric>.s` identity;
- bounded `/usr/bin/gcc` actor and chain-tail;
- create+write open + delete;
- no conflicting actor/operation.

Therefore zero targeted candidates was not a valid test of whether the GCC variance existed under the intended normalization semantics.

## Corrected scientific status

The first R3 result is classified:

`P3_V5R2_R3_INVALID_NORMALIZATION_COMPARABILITY`

Consequences:
- the earlier `P3_NO_MATERIAL_VALUE` interpretation is superseded;
- R4/R5 remain forbidden;
- the old six R2 lockfiles remain immutable evidence and are not post-hoc re-canonicalized into replacement samples;
- #105 is reopened;
- a fresh preregistered learning campaign from a new source SHA is required after correcting only the research baseline normalization configuration to match public CLI default semantics;
- no grammar, threshold, workload, acceptance rule or success criterion changes.

Public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, public v2 semantics, the original V5 failure and C6 compatibility failure remain unchanged.
