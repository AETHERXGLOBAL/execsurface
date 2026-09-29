# ExecSurface — P3 V5-R2 R3 Variance Analysis Result

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P3_NO_MATERIAL_VALUE (BOUNDED TO PREREGISTERED V5-R2 QUESTION)**

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

R0/R1/R2 completed successfully under issue #105.

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

The compile correction was exactly one structural derive change at commit:
`539cfa821b8092f77d5316a70623172060ce9822`.

## Accepted R3 scientific attempt

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

Before analysis:
- frozen R2 artifact ZIP digest matched exactly;
- every extracted file checksum verified;
- analyzer rustfmt PASS;
- analyzer clippy `-D warnings` PASS;
- analyzer build PASS.

The analyzer then executed over exactly the six frozen lockfiles.

## Observed R3 metrics

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

## Preregistered gate interpretation

The R3 value condition required **at least one targeted GCC ephemeral variable to be reproduced in raw evidence** before the bounded GCC projection could establish value.

That prerequisite was not met.

Therefore:
- R4 unchanged checks are **not authorized**;
- R5 falsification for a positive variance-value claim is **not reached**;
- no accepted-variable set is created;
- no frequency-based authorization is permitted;
- no broad Go/cache/module/stdlib normalization is introduced;
- no public integration or release change is authorized.

The presence of 2,075 non-target variable candidates is retained as evidence of substantial genuine/non-target run-to-run variability. It does not itself justify accepting any of those effects.

## Formal decision

For the exact preregistered V5-R2 research question and declared environment:

`P3_NO_MATERIAL_VALUE`

This decision is **bounded**. It means the campaign did not reproduce the previously targeted GCC ephemeral-identity variance and therefore cannot establish the intended material value from that mechanism in this run set. It does not prove that all multi-run variance modeling lacks value on every workload or environment.

## Anti-drift conclusion

The correct response is to retain the negative result, stop V5-R2 before R4, and move to the already-planned P3 V6 product decision. V6 may inspect whether the retained 2,075 non-target variable candidates justify a new narrowly preregistered research question, but it must not retroactively redefine V5-R2 success or authorize behavior by recurrence alone.

Public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, raw/canonical/baseline v2 semantics and the frozen original V5 failure remain unchanged.
