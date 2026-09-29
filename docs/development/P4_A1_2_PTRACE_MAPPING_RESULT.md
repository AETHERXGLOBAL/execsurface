# ExecSurface — P4-A1.2 Ptrace Evidence Mapping Result

Date: 2026-09-29
Tracking: #108 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P4_A1_2_PTRACE_MAPPING_PASS_RESEARCH_ONLY**

## Accepted execution

- source: `8ad1fcf1d9ab474fd6a0d007f198844aef4694fb`
- workflow: `36584250301`
- job: `109460186549`
- artifact: `11040593453`
- artifact SHA-256: `sha256:6e277eff717c83269a55e221e7592f72475b69ee25538987d8d2de9fea299c7d`
- runner: GitHub-hosted Ubuntu 24.04
- Rust: 1.90.0

## Results

- frozen A0/A1/A1.2 boundary verification: PASS
- rustfmt: PASS
- clippy `-D warnings`: PASS
- canonical authority-model tests: **8/8 PASS**
- raw-v2 ptrace mapping tests: **10/10 PASS**
- Semantics-v3 proof-model tests: **7/7 PASS**
- public M11 shared-FD fail-closed regression: **6/6 PASS**

Formal decision:
`P4_A1_2_PTRACE_MAPPING_PASS_RESEARCH_ONLY`

## What A1.2 established

The research mapping now preserves explicit authority distinctions for current raw-v2 evidence:
- lifecycle spawn -> process-create proposition;
- confirmed ptrace exec transition -> exec-success proposition;
- pathname/open/delete and connect argument evidence -> attempt-only authority;
- positive-result runtime-FD read/write evidence -> FD attribution only under declared FD-table prerequisites;
- raw-v2 clone without retained `CLONE_FILES` -> fd-table relation unknown/ambiguous and dependent FD attribution incomplete;
- fork/vfork fd-table copy relation -> named bounded derivation, not direct authority;
- execution-chain records -> named bounded causal derivation;
- incomplete or warning-bearing observation -> fail closed before effect mapping;
- successful-open object remains an explicit raw-v2 capability gap instead of being fabricated from pathname attempts.

## Retained failures

Two A1.2 runs stopped at rustfmt before clippy/tests:
- `36583344327`
- `36583439358` / artifact `11040926419` / artifact digest `sha256:d2450ef0c373247743aac76c810556ce6e894f7a71674dc32f21441ed1637e36`

They remain retained as engineering evidence. The formatting correction was applied by a one-shot branch-scoped helper that was removed immediately afterward; its history remains in Git/Actions.

## Boundary

A1.2 validates the current mapping under the declared synthetic evidence suite only. It does not yet close the ptrace reference adapter.

Next mandatory stage:
**P4-A1.3 — independent adversarial mapping gate plus live ptrace smoke.**

A1.4 public anti-drift regression remains mandatory before full A1 `PASS_RESEARCH_ONLY` and before any A2 authority-gap decision.

No new backend, public integration, release, or stable-channel movement is authorized.
