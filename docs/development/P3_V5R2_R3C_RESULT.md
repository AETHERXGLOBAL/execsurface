# ExecSurface — P3 V5-R2 R3-C Fresh Corrected Variance Analysis Result

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED / R3C_ACCEPTANCE_FROZEN**

## Governance and team

Fixed roles retained:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team
- Independent Milestone Reviewer

Dynamic specialists:
- Linux ptrace / clone / fd-table completeness
- Go/GCC build nondeterminism
- Rust research-tool correctness
- normalization / canonicalization
- multi-run set analysis
- reproducibility / CI evidence sealing
- provenance / artifact verification

## Why R3-C was required

The first V5-R2 learning campaign used a research baseline helper with `NormalizationConfig::default()`, which emitted no semantic roots. That made real `/tmp/cc*.s` GCC effects structurally ineligible for the already-frozen V2 classifier. The earlier zero-target R3 classification is retained but superseded as `P3_V5R2_R3_INVALID_NORMALIZATION_COMPARABILITY`.

R3-C therefore used a newly preregistered correction protocol and a fully fresh R2-C learning campaign. No old lockfile was re-canonicalized or substituted as corrected evidence.

## Fresh R2-C predecessor

Source:
`e38150f715f8907f7324ce13335e5e9074204823`

Workflow:
`36573676797`

Job:
`109423682625`

Artifact:
`11035133723`

Artifact SHA-256:
`sha256:a98a1f022e601f0b0653474df8c4500f33fa5670f7e25f105740b2e79bd2fd26`

R2-C evidence:
- corrected baseline discriminator tests: 2/2 PASS;
- frozen V2 GCC grammar tests: 7/7 PASS;
- exactly 3 direct priming runs: 3/3 rc=0;
- exactly 6 fresh learning attempts: 6/6 admitted, no replacement;
- every observation: exit 0, `complete=true`, warnings=0, internal clone/fd certificate positive;
- all six baselines: semantic roots `home,tmp,workspace`;
- direct verification found 12 canonical GCC `$TMP/cc<6 alphanumeric>.s` effects with `PathClass::Temp`;
- public M11 shared-FD contract remained 6/6 PASS after learning.

## R3-C execution

Source:
`20ae0934dc38c46c9f2a5cc0b7c49992cb337a76`

Workflow:
`36574213979`

Job:
`109425525106`

Evidence artifact:
`11036313569`

Artifact SHA-256:
`sha256:ce364994c57bcb14a38ca625f46abed4f4a42542691230c56e1d3fdc8c5f388b`

Before analysis:
- exact fresh R2-C artifact ZIP digest verified;
- all extracted files verified against the original `SHA256SUMS`;
- six fresh lockfiles consumed directly with no re-canonicalization;
- normalization discriminator PASS;
- `R3C_FRESH_GCC_TMP_EFFECTS=12`;
- unchanged preregistered variance analyzer fmt/clippy/build PASS.

## R3-C metrics

- run count: **6**
- raw variable candidates: **114**
- targeted GCC raw variable candidates: **12**
- non-target raw variable candidates: **102**
- projected variable candidates: **102**
- canonical GCC invariant effects: **2**
- explicitly accepted variable effects: **0**
- non-target support/provenance mismatches: **0**
- lingering targeted random paths after projection: **0**

Raw learning-set digest:
`sha256:437a9f8fca9f0ff4c8573c1cfee06b46cb325d1d1a890649ba536acd8e2e648d`

Projected learning-set digest:
`sha256:16e8519c83ea374359d1e44872d570ccc15be88b7ccf8f9e3b54b5e3d6b7ab30`

Analyzer decision:
`P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED`

The public M11 shared-FD regression was rerun after analysis and remained 6/6 PASS.

## Interpretation

Within the exact pinned FZF workload/environment and frozen V2 GCC role grammar, the targeted ephemeral GCC identity variance reproduced on fresh evidence and the bounded projection removed those 12 targeted variable identities while retaining all 102 non-target variable candidates without support/provenance mismatch.

This is a bounded value result for the targeted identity mechanism only. It does not authorize the 102 non-target variables, does not prove universal stability, and does not authorize public integration.

## Acceptance freeze

Repository freeze file:
`docs/development/P3_V5R2_R3C_ACCEPTANCE_FREEZE.json`

Freeze commit:
`97d759e78a4573f197568e3cf578e06b155a0940`

Frozen properties:
- explicit accepted-variable set remains empty;
- frequency/recurrence grants no authorization;
- public integration remains false;
- learning and R3-C artifact IDs/digests are immutable references;
- R4 is authorized only as exactly six fresh unchanged checks with no replacement;
- non-target variability must remain visible;
- any incomplete/warning-bearing/certificate-negative check blocks positive interpretation.

## Next

R4 may now execute exactly six fresh unchanged checks against this frozen R3-C boundary. R5 false-PASS/falsification remains mandatory before any P3 product decision.

Public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, and public v2/default observer semantics remain unchanged.
