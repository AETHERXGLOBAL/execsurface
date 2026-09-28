# M12.7 Failure 002 — Privilege/Hybrid Sentinel Literal

Date: 2026-09-28
Tracking: #90
Branch: `integration/m12-portable-clean`

## Classification

`HARNESS_SENTINEL_LITERAL_FAILURE_NOT_PRODUCT_BLOCKER`

## Failed run

- Workflow: `M12.7 Independent Internal Release Red-Team`
- Run: `36467881428`
- Job: `109082427639`
- Candidate SHA: `49e3063a7b214dbdb27b3d332b139f08f352cdb8`
- Conclusion: **failure**

## What passed before failure

The complete code-focused attack surface passed before this sentinel:

- provenance/public-line freeze;
- runtime-source freeze after M12.6 semantic candidate;
- diff-scope check;
- fixture compilation;
- rustfmt + full-workspace Clippy `-D warnings`;
- shared-FD fail-closed tests;
- full M12 adversarial replay;
- ptrace regressions;
- baseline digest and CLI compatibility tests;
- full workspace regression.

## Failure

The privilege/hybrid step required the literal text:

`no ptrace↔hybrid baseline interchangeability`

The release draft instead already states the stronger complete sentence:

`This prospective alpha does not authorize automatic hybrid selection, new privilege requirements for the portable path, ptrace↔hybrid baseline interchangeability, or claims of end-to-end hybrid production readiness.`

The first three privilege/hybrid assertions passed; the final grep failed only because its literal fragment did not occur independently.

## Interpretation

No privilege expansion or hybrid promotion was found by this failure. It is a sentinel-text mismatch. The failed run is retained and is not reclassified as PASS.

Later red-team checks, including the explicit conservative clone-guard disclosure check, were skipped in this run, so M12.7 remains OPEN.

## Authorized repair

Change only the privilege/hybrid sentinel to assert the existing complete non-authorization sentence. Do not weaken or remove any product boundary. Rerun the complete gate.