# ExecSurface — Stage-2 R7 Hostile Cross-Gate Destruction Result

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Protocol: `docs/development/STAGE2_R7_HOSTILE_CROSS_GATE_PROTOCOL.md`  
Expansion record: `docs/development/STAGE2_R7_HOSTILE_CROSS_GATE_EXPANSION.md`  
Qualified product source: `6bba030d7993b011398cc9569e18231311849d52`  
Decision: **R7_HOSTILE_CROSS_GATE_DESTRUCTION_PASS_BOUNDED**

## Decision

R7 is GREEN for the bounded composed-state proposition:

> ExecSurface verdict outputs cannot overwrite custody-consumed baseline/policy artifacts, cannot alias each other, cannot overwrite an object the observed workload wrote, and cannot be redirected after target execution into the originally verified trusted object through symlink or trusted-object rename rebinding.

This result is about correctness of ExecSurface's own report materialization relative to R4 custody and observed workload state. It is not a claim of generic filesystem isolation or target sandboxing.

## Original fail-first result

After formatting-only corrections and before product remediation, the hostile corpus reached semantic execution against the vulnerable implementation.

Source: `4ea63d7f9b0d9b49e6aec025e14145655c30a675`  
CI run: `37715140225`  
Job: `113109784802`

Result: **0 PASS / 6 FAIL**

Failures covered:

- direct JSON-output -> baseline collision;
- Markdown symlink alias -> policy;
- JSON hardlink alias -> baseline;
- JSON/Markdown same-object collision;
- verdict output -> workload-written object;
- post-target symlink rebinding of output -> baseline.

The earlier formatting-only RED runs are retained but are not counted as semantic falsification evidence.

## First correction and second destruction

The first correction introduced:

- path/object alias checks;
- pre-target protected-input/output disjointness;
- JSON/Markdown output disjointness;
- post-target revalidation;
- workload-written/output disjointness.

At source `55fb5dbe1582cf07bb242eb2a2013e012cf98510`, the six-case corpus passed **6/6**.

The destruction team then introduced R7-X2, the trusted-object move attack. Against the first correction:

- source: `437896a4e25de3644ed9ad2225000b561db938e5`
- CI run: `37715601621`
- job: `113111234007`
- result: **6 PASS / 1 FAIL**

The failure proved that re-resolving only the original baseline pathname after target execution was insufficient.

## Final correction

The final bounded correction preserves two layers:

1. **pathname/object alias checks**
   - direct spelling;
   - resolved symlink identity;
   - hardlink identity;
   - non-existing output lexical identity with resolved parent;
2. **preflight trusted-object snapshot**
   - existing baseline/policy object identity is captured before target execution;
   - postflight output identities are checked against the captured trusted objects;
   - a target rename cannot make ExecSurface forget which inode it consumed as authoritative input.

Postflight also retains workload-write collision checks before verdict output is materialized.

## Final hostile corpus

At qualified product source `6bba030d7993b011398cc9569e18231311849d52`:

`r7_cross_gate_destruction.rs`: **7/7 PASS**

- direct baseline/output collision — PASS
- policy symlink alias — PASS
- baseline hardlink alias — PASS
- JSON/Markdown alias — PASS
- workload-written output — PASS
- post-target symlink rebinding — PASS
- trusted baseline object moved onto output path — PASS

Earlier gate corpora remain green, including R5 **14/14** and R6 **2/2** in the same CI run.

## Qualification workflows

All qualification workflows completed SUCCESS on the same qualified product source:

- CI — `37715748713` — SUCCESS
- Adversarial Regression — `37715748558` — SUCCESS
- P9.3 Compatibility Contract — `37715748447` — SUCCESS
- Registry Packaging Gate — `37715748544` — SUCCESS
- Public Consumer Smoke — `37715748671` — SUCCESS
- P8 A3.4 consumer contract red team — `37715748595` — SUCCESS
- P8 A3 typed report prototype — `37715748452` — SUCCESS
- P8 A3 typed evidence output — `37715748637` — SUCCESS
- Ptrace Lifecycle Regression — `37715748682` — SUCCESS

## Residual boundary

R7 does not prevent the target itself from deliberately modifying workspace files. Existing policy/evidence semantics govern observed target behavior.

R7 specifically proves that ExecSurface's own verdict report writer will not subsequently overwrite:

- a protected input object it consumed;
- another verdict output;
- an object observed as workload-written,

within the tested Linux filesystem identity boundary.

## Anti-drift decision

- Alpha.5 remains immutable.
- No R0-R6 acceptance test was weakened.
- The original R7 protocol was not rewritten to pretend later discoveries were preregistered.
- No merge or release is authorized by R7.
- R8 full qualification / FINAL_INTERNAL_GATE may now open.
