# M12.2 — Baseline / Backward Compatibility — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`

## Team

Fixed roles retained:
- **Innovative Systems Architect**
- **Anti-Deviation / Skeptical Reviewer**

Dynamic specialists: baseline/schema compatibility, CLI contract/exit codes, self-service UX, Rust regression/CI.

## Gate objective

Prove that the clean portable hardening does not silently reinterpret existing baselines or change established CLI/public behavior.

## Fresh evidence

Workflow: `M12 clean integration CI`
Run: `36458046832`
Conclusion: **success**

Explicitly passed:

- shared-FD ambiguity fail-closed regression;
- existing ptrace Linux regressions;
- frozen baseline-v2 serialization/digest vector;
- legacy lock schema rejection (`schema_version = 1`) rather than reinterpretation;
- CLI learn/check compatibility;
- stable verdict/exit-code behavior including PASS `0`, REVIEW `10`, BLOCK `20`, invalid-policy error `2`;
- self-service CLI compatibility;
- full workspace tests;
- full-workspace Clippy `-D warnings`;
- lockfile integrity.

## Interpretation

The portable hardening changes one bounded behavior: known clone-based fd-lifecycle ambiguity can no longer remain `complete=true` / PASS-eligible.

It does not:
- change baseline schema or digest format;
- migrate old baselines silently;
- change established policy verdict exit codes;
- require BPF-LSM or new privileges;
- grant ptrace↔hybrid equivalence;
- mutate the public release/tag/Marketplace channel.

## M12.2 classification

**`M12_2_BACKWARD_COMPATIBILITY_PASS_CLEAN`**

M12.2 is closed.

Next authorized gate: **M12.3 — Public install / Marketplace / crates regression**.
