# M12.7 — Independent Internal Release Red-Team — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`
Status: **CLOSED**

## Classification

**`M12_7_INTERNAL_REDTEAM_RELEASE_ARGUMENT_SURVIVES_BOUNDED`**

This was an **independent internal, role-separated red-team**, not an external or third-party review.

## Team

Fixed roles retained:

- **Innovative Systems Architect** — permitted to propose remediation only after a finding was reproduced and bounded.
- **Anti-Deviation / Skeptical Reviewer** — owned the release-kill hypothesis and attempted to block release on semantics, compatibility, authority, privilege, packaging, provenance, claims, and developer-impact grounds.

Dynamic specialists: Linux ptrace/thread/fd lifecycle, Rust runtime/API, baseline/schema compatibility, GitHub Actions/Marketplace, crates.io publication, software-supply-chain provenance, security/privacy claim review, and developer UX.

## Frozen runtime candidate

The product/runtime source remained frozen after the M12.6 semantic candidate:

`16ff98b205f951cf463b6ae5f811f164f4ba4572`

M12.7 changes after that SHA were red-team harness, evidence, and documentation only. No runtime/product source was changed to make the red-team pass.

Public `main` remained:

`db11761e2d75ebca7d4458dc39094f4aed6a26bb`

throughout the gate.

## Failure-first record

### Failure 001 — harness package ID

Run `36466685070` failed because the red-team called package ID `execsurface-cli`, while the actual Cargo package is `execsurface`.

Classification: `HARNESS_FAILURE_NOT_PRODUCT_BLOCKER`.

The failed run remains recorded in `M12_7_FAILURE_001_REDTEAM_HARNESS_PACKAGE_ID.md`.

### Failure 002 — privilege sentinel literal

Run `36467881428` passed all code-focused gates but failed a literal grep for `no ptrace↔hybrid baseline interchangeability`, while the release draft already contained the stronger complete non-authorization sentence.

Classification: `HARNESS_SENTINEL_LITERAL_FAILURE_NOT_PRODUCT_BLOCKER`.

The failed run remains recorded in `M12_7_FAILURE_002_PRIVILEGE_SENTINEL_LITERAL.md`.

### Failure 003 — real public disclosure blocker

Run `36468075481` passed semantics, adversarial tests, ptrace regressions, baseline/CLI compatibility, full workspace, privilege boundaries, and general claim boundaries. It then failed the required conservative clone-guard disclosure.

Classification: **`M12_7_RELEASE_BLOCKED_CLAIMS`**.

The issue was real: raw observation v2 does not retain enough `CLONE_FILES` detail to prove exact fd-table sharing, so the portable guard intentionally marks observed clone-based concurrency incomplete. That can produce false incompleteness in some concurrent programs even when exact fd-table sharing was not proved.

Shipping that behavior without explicit public disclosure would overstate completeness precision.

Remediation was documentation-only. README and prospective alpha.4 release notes now explicitly state:

- some clone/thread concurrency may conservatively be marked incomplete even when exact fd-table sharing is not proven;
- the tradeoff is intentional to prevent the known false-completeness class;
- this is not an exact shared-FD attribution repair.

No fail-closed runtime guard was weakened.

The failed run remains recorded in `M12_7_FAILURE_003_UNDISCLOSED_CONSERVATIVE_INCOMPLETENESS.md`.

## Final successful red-team run

Workflow: `M12.7 Independent Internal Release Red-Team`
Run: `36468334051`
Job: `109083947890`
Final documentation-remediated SHA: `8e5ed221f481f303facae78c2e12018028a74d4c`
Conclusion: **success**

Passed from the start:

- public-line and tag freeze;
- proof runtime/product source remained frozen after the M12.6 semantic candidate;
- diff-scope kill check;
- adversarial fixture build;
- rustfmt;
- full-workspace Clippy `-D warnings`;
- shared-FD fail-closed kill attempt;
- full M12 adversarial replay;
- existing ptrace regressions;
- baseline and CLI compatibility;
- full workspace regression;
- privilege/hybrid non-promotion sentinels;
- claim-boundary sentinels;
- conservative clone-guard disclosure check;
- Cargo.lock/public pointer immutability.

## Same-SHA independent regression layers

On `8e5ed221f481f303facae78c2e12018028a74d4c`, the repository exposed 16 GitHub check runs. At closeout, there were **no failed and no in-progress checks**.

The set included:

- independent internal red-team;
- staged next-alpha artifact/provenance rehearsal;
- M12 integration CI;
- M12 adversarial jobs on Ubuntu 22.04 and 24.04;
- current public / Marketplace compatibility;
- release-bundle dry run and registry-readiness decision;
- root CI / Action behavior checks.

## Residual limitations preserved

M12.7 does not erase or upgrade the known boundaries:

- ptrace pathname access-attempt metadata is not kernel-object identity;
- PATH-TOCTOU remains a real counterexample class;
- exact shared-FD attribution is not repaired;
- conservative false incompleteness is possible and now explicitly disclosed;
- ptrace can be restricted by host/container/Yama/capability policy;
- public scope remains Linux x86_64;
- BPF-LSM/hybrid remains managed/research-only and non-default;
- no ptrace↔hybrid baseline interchangeability is established;
- no production-readiness, universal-completeness, universal-performance, or software-safety claim is supported.

## Decision

The internal skeptical team failed to find a remaining release-blocking defect after the real disclosure blocker was repaired and the complete gate rerun.

**M12.7 is CLOSED.**

Authorized next gate only:

**M12.8 — Merge / next-alpha decision.**

M12.7 itself does not merge, tag, publish crates, move the stable Action, or publish a release.