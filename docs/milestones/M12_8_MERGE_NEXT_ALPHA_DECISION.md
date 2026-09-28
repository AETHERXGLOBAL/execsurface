# M12.8 — Merge / Next-Alpha Decision

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`
Status: **DECISION CLOSED**

## Final classification

**`MERGE_AND_NEXT_ALPHA_AUTHORIZED`**

This classification authorizes:

1. merge of the clean portable hardening PR #92 into `main`;
2. creation of a separate release-preparation branch from the merged `main`;
3. coherent version/pointer/workflow/documentation preparation for `0.1.0-alpha.4`;
4. fresh release-readiness CI on that exact release-preparation source;
5. if those release-preparation gates pass, publication of `v0.1.0-alpha.4`, sequential crates.io publication, and movement of the stable `v0.1` Action channel only after immutable release validation.

It does **not** authorize hybrid/BPF-LSM default promotion or ptrace↔hybrid baseline equivalence.

## Evidence considered

- M12.0: clean integration required; raw M11/M10 research branch rejected as direct merge shape.
- M12.1: portable semantic hardening passed fresh clean-branch CI.
- M12.2: baseline/backward compatibility passed, including frozen v2 digest and stable CLI exit codes.
- M12.3: public install / Marketplace / packaging compatibility passed.
- M12.4: adversarial PATH-TOCTOU and shared-FD counterexamples reproduced and remained fail-closed across Ubuntu 22.04/24.04.
- M12.5: no preregistered gate-blocking performance/resource regression.
- M12.6: prospective alpha.4 artifact/provenance/consumer dry run passed without public mutation; sequential crates.io registry visibility constraint explicitly preserved.
- M12.7: independent internal role-separated red-team found and forced repair of a real conservative-incompleteness disclosure gap; after documentation-only remediation, the complete red-team passed.

## Failure-first record retained

The decision does not erase failed attempts from M12.4, M12.6, or M12.7. In particular:

- PATH-TOCTOU remains a real ptrace authority limitation;
- exact shared-FD attribution remains unrepaired;
- conservative false incompleteness is possible under some clone/thread concurrency and is now publicly disclosed;
- M12.6 proved the full dependent crates.io chain cannot truthfully be registry-dry-run before prospective dependency versions become visible sequentially;
- M12.7 retained two harness failures and one real release-documentation blocker before final success.

## Merge boundary

PR #92 is the accepted clean integration shape. It contains the bounded runtime hardening, adversarial/regression fixtures and workflows, evidence documents, README disclosure, and prospective release-note draft. It does not include the live hybrid product backend, M10 research backend implementation, baseline migration, stable Action pointer movement, or alpha.4 version plumbing.

The runtime/product source used to justify M12.7 remained frozen after semantic candidate:

`16ff98b205f951cf463b6ae5f811f164f4ba4572`

Later changes were M12 evidence/workflow/documentation only.

## Release-preparation boundary

Version and public distribution plumbing will be prepared **after the clean hardening merge**, on a dedicated release branch created from the resulting `main`. This avoids weakening or rewriting the already-closed M12.7 freeze checks.

Before creating tag `v0.1.0-alpha.4`, release preparation must prove on the exact source:

- coherent workspace/package version `0.1.0-alpha.4`;
- exact internal dependency pins updated coherently;
- Cargo.lock coherence;
- final release notes and README/install references;
- `action/release-tag.txt` points to `v0.1.0-alpha.4`;
- release/publish workflows contain no stale alpha.3 release-specific references that would break publication;
- full fmt/Clippy/tests;
- packaging topology validation;
- release bundle/checksum/provenance dry run;
- public consumer smoke plan remains valid;
- tag `v0.1.0-alpha.4` is still absent until readiness is closed.

## Residual public claims

The next alpha remains a public **alpha** for Linux x86_64. It does not establish production readiness, universal completeness, universal performance improvement, software safety, or end-to-end hybrid readiness.

## Decision

The bounded portable hardening has survived the mandatory M12 evidence chain and the internal release-kill review. No remaining evidence-supported blocker prevents clean integration.

**PR #92 merge is authorized. Next-alpha release preparation is authorized subject to its exact-source readiness checks before publication.**