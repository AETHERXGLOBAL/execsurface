# ExecSurface — P3 V5-R2 R3-C Normalization Comparability Correction Protocol

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO CORRECTED MEASUREMENT BEFORE THIS FREEZE**

## Purpose

Correct one research-tool comparability defect discovered after the first completed R3 analysis: the R2 baseline helper used `NormalizationConfig::default()`, producing no semantic roots, while public CLI learn/check default normalization derives workspace/home and includes `/tmp` (plus `$TMPDIR` when present).

The correction exists only to make the research baseline normalization comparable to the public CLI default semantics required by the already-frozen V2 GCC grammar.

It does not change the grammar, workload, thresholds, accepted effects, run counts, observer, or public product path.

## Preserved evidence

The following remain immutable negative evidence:
- original V5 Stage L: `P3_INCOMPLETE_EVIDENCE`;
- C6 direct integration: `C6_COMPATIBILITY_REGRESSION`;
- first V5-R2 R2 six-run artifact `11032959833`;
- first completed R3 artifact `11033972162`;
- analyzer output `P3_V5_TARGETED_VARIANCE_NOT_REPRODUCED`;
- all pre-analysis R3 tooling failures.

The prior final interpretation `P3_NO_MATERIAL_VALUE` is superseded because the R2 normalization configuration did not make the targeted V2 grammar evaluable.

No old R2 lockfile may be re-canonicalized and substituted as a corrected learning sample.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — keep correction minimal; no feature expansion.
2. **Anti-Drift / Scientific Integrity Reviewer** — verify this is a comparability repair, not post-hoc success tuning.
3. **Independent Falsifier / Red Team** — attack root ambiguity, path collisions, wrong actor/role, and public/research divergence.
4. **Independent Milestone Reviewer** — verify exact source, run counts, artifacts, hashes and decision boundaries.

## Dynamic specialists

- normalization / canonical-path semantics
- Rust research-tool correctness
- Go/GCC build nondeterminism
- CI reproducibility / artifact sealing
- filesystem root semantics
- supply-chain false-PASS analysis

## Exact correction

The research baseline helper must construct the same *default root selection semantics* used by the public CLI for the frozen invocation context:

- `workspace = frozen FZF WORK_ROOT`;
- `home = $HOME` when present;
- `tmp_roots = [$TMPDIR if present, /tmp if not already present]`;
- `run_tmp = None`;
- `caches = {}`.

No additional cache/module/stdlib/source root may be declared.

The helper must receive the frozen workspace root explicitly; it must not infer the ExecSurface checkout directory as the target workspace.

## Static gates before measurement

1. helper rustfmt PASS;
2. helper clippy `-D warnings` PASS;
3. helper build PASS;
4. public workspace regressions remain green;
5. one synthetic normalization fixture proves `/tmp/ccABC123.s` becomes `PathClass::Temp` with `$TMP/ccABC123.s` under the corrected config;
6. the same fixture with no temp root remains outside declared roots, proving the correction is configuration-driven rather than a grammar change;
7. V2 GCC grammar tests remain unchanged and green.

## Corrected learning campaign — R2-C

Run from a new source SHA.

Sequence is unchanged:
- exactly 3 direct priming runs;
- exactly 6 certificate-aware learning attempts;
- no replacement;
- all six must be exit-zero, complete, warning-free and certificate-positive;
- every corrected lockfile must record the expected normalization profile and semantic roots including `tmp`;
- all six lockfiles and observations are independently checksum-verified.

Any inadmissible sample => `P3_R2C_INCOMPLETE_EVIDENCE` and stop.

## Corrected R3-C

Analyze only the six new R2-C lockfiles.

Freeze/report the same preregistered metrics:
- raw variable candidate count;
- targeted GCC variable candidate count;
- non-target variable candidate count;
- projected variable candidate count;
- targeted residual random-path count;
- non-target support/provenance mismatch count;
- accepted variable effect count.

The V2 grammar and V3 acceptance contract are unchanged.

## Required anti-drift discriminator

Before accepting any R3-C interpretation, verify directly from the new locks that:
- at least one observed `/tmp/cc*.s` effect, if present, is represented as `$TMP/cc*.s` with `PathClass::Temp`;
- V2 eligibility is determined only by the frozen actor/role grammar;
- zero targeted candidates may be interpreted as non-reproduction only if the normalization discriminator above passes.

## Allowed outcomes

- `P3_R3C_VALUE_REQUALIFIED_BOUNDED`
- `P3_R3C_TARGETED_VARIANCE_NOT_REPRODUCED_VALID`
- `P3_R2C_INCOMPLETE_EVIDENCE`
- `P3_R3C_NORMALIZATION_COMPARABILITY_FAILED`
- `P3_R3C_FALSE_PASS_FOUND`

No outcome authorizes public integration or release. R4 remains forbidden until a corrected R3-C freeze is positively valid under the original V5-R2 value condition.
