# M12.7 — Independent Internal Release Red-Team — Protocol

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`

## Status

PREREGISTERED / ADVERSARIAL / NO MERGE AUTHORITY UNTIL CLOSEOUT

## Independence boundary

No external review team is currently available. This gate therefore uses an **independent internal red-team** with role separation from the implementation path. It must not be described as external or third-party review.

## Fixed roles

- **Innovative Systems Architect** — may propose mitigations only after a finding is reproduced and bounded.
- **Anti-Deviation / Skeptical Reviewer** — owns the release-kill hypothesis and may block release for semantic, compatibility, evidence-authority, privilege, packaging, provenance, claim, or usability failures.

## Dynamic red-team specialists

- Linux ptrace/thread/fd lifecycle semantics
- Rust API/runtime review
- baseline/schema compatibility
- GitHub Actions / Marketplace release engineering
- crates.io sequential publication semantics
- software supply-chain provenance / artifact integrity
- security and privacy claim review
- developer UX / false-incompleteness blast-radius review

## Hypothesis under attack

`H_RELEASE`: the portable M12 candidate can be merged and released as the next public alpha without introducing an undisclosed semantic regression, hidden privilege expansion, baseline reinterpretation, distribution break, provenance gap, or inflated public claim.

The red-team starts by trying to **kill** `H_RELEASE`.

## Frozen semantic candidate

Portable runtime candidate already tested through M12.6:

`16ff98b205f951cf463b6ae5f811f164f4ba4572`

Later M12.6/M12.7 commits may add evidence, workflow, or documentation only. Any runtime/product-code change after the frozen semantic candidate requires explicit re-audit.

Public main remains frozen at:

`db11761e2d75ebca7d4458dc39094f4aed6a26bb`

## Mandatory attack surfaces

### A — Diff-scope / hidden product mutation

Reject if the PR contains unreviewed product paths, hybrid/BPF product promotion, baseline migration, stable Action movement, release pointer mutation, or new privilege requirement outside the declared portable hardening.

### B — Semantic fail-closed integrity

Re-run the targeted shared-FD ambiguity tests, ptrace regressions, adversarial suite, full workspace, and `-D warnings` Clippy. Ambiguity or loss must remain non-PASS-eligible.

### C — Conservative-guard blast radius

The clone guard is intentionally conservative because raw v2 does not retain exact `CLONE_FILES` flags. Red-team must reject release if this user-visible limitation is hidden. Public release documentation must explicitly state that some clone/thread concurrency can be marked incomplete even when exact fd-table sharing is not proven.

### D — Baseline / CLI contract

Frozen baseline-v2 serialization, legacy rejection, learn/check behavior, PASS/REVIEW/BLOCK/ERROR exit codes, and self-service paths must stay compatible except for the explicit fail-closed incompleteness behavior.

### E — Privilege / backend boundary

Reject if the portable line silently requires BPF-LSM, daemon/service installation, privilege elevation, changed ptrace host policy, hybrid auto-selection, or ptrace↔hybrid baseline equivalence.

### F — Release / provenance truthfulness

M12.6 dry-run constraints must remain explicit: full dependent crates.io chain cannot truthfully be called registry-dry-run before sequential prospective dependencies exist, and dry-run provenance must not be called a signed GitHub cryptographic attestation.

### G — Public-line immutability before M12.8

`main`, `v0.1.0-alpha.3`, stable `v0.1`, Marketplace behavior, and crates.io public state must remain untouched before the decision gate.

### H — Claim falsification

Reject release if README/release notes imply kernel-object authority for ptrace pathnames, exact shared-FD repair, universal completeness, production readiness, universal performance improvement, hybrid default readiness, or software-safety proof.

## Blocking classifications

Any reproduced blocker produces one of:

- `M12_7_RELEASE_BLOCKED_SEMANTICS`
- `M12_7_RELEASE_BLOCKED_COMPATIBILITY`
- `M12_7_RELEASE_BLOCKED_PRIVILEGE`
- `M12_7_RELEASE_BLOCKED_DISTRIBUTION`
- `M12_7_RELEASE_BLOCKED_PROVENANCE`
- `M12_7_RELEASE_BLOCKED_CLAIMS`

A repair must preserve the failed attempt in the record and rerun the complete gate.

## Pass classification

Only if all mandatory attack surfaces survive:

`M12_7_INTERNAL_REDTEAM_RELEASE_ARGUMENT_SURVIVES_BOUNDED`

This classification still does not itself merge or publish anything. It authorizes only M12.8.