# M12.1 — Semantic Integration Audit — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`

## Fixed roles

- **Innovative Systems Architect** — selected a clean, minimal integration path rather than wholesale M11 merge.
- **Anti-Deviation / Skeptical Reviewer** — rejected importing M10/hybrid research baggage into the portable release candidate and required fail-closed semantics to survive fresh CI on a branch derived directly from public `main`.

Dynamic specialists: Rust observer semantics, ptrace concurrency/fd lifecycle, baseline compatibility, CI/release engineering.

## Integrated semantic delta

M12.1 ports only the bounded portable runtime hardening required by the M10/M11 shared-FD counterexample:

1. clone-based concurrency is treated as an explicit shared-fd lifecycle ambiguity because raw v2 does not retain enough `CLONE_FILES` detail to certify exact shared-fd attribution;
2. that ambiguity forces `observation.complete = false`;
3. warning code `shared_fd_table_ambiguity` is emitted once;
4. backend completeness classifies it as `IncompleteAmbiguity`;
5. `IncompleteAmbiguity` is not PASS-eligible;
6. the existing ptrace regression expectation was updated from false completeness to fail-closed behavior;
7. a no-clone control proves the guard does not invent the ambiguity warning where clone concurrency is absent.

No BPF-LSM/hybrid product path, default backend change, baseline migration, cross-backend equivalence, privilege requirement, or public release mutation was introduced.

## Fresh clean-branch evidence

Workflow: `M12 clean integration CI`
Run: `36457724809`
Conclusion: **success**

Passed steps:

- targeted rustfmt checks;
- full-workspace Clippy with `-D warnings`;
- `m11_shared_fd_ambiguity` regression;
- existing `ptrace_linux` regressions;
- frozen baseline-v2 digest compatibility test;
- full workspace tests;
- Cargo.lock integrity.

## Semantic interpretation

The change is deliberately asymmetric:

- it does **not** claim ptrace became more complete;
- it does **not** repair all possible shared-fd attribution races;
- it prevents the known class of clone-based ambiguity from being represented as complete/PASS-eligible evidence.

This is a fail-closed hardening, not an authority upgrade.

## M12.1 classification

**`M12_1_PORTABLE_SEMANTIC_HARDENING_PASS_CLEAN`**

M12.1 is closed.

Next authorized gate: **M12.2 — Baseline / backward compatibility**.
