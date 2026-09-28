# M12.0 — Candidate Provenance / Diff Freeze

Date: 2026-09-28
Tracking: #90
Source review PR: #91

## Frozen provenance

- public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- M11 closeout/source candidate: `520fcf92395bf1e62c9502704596eba1e76b0817`
- raw review branch: `integration/m12-portable-release-review`
- clean integration branch: `integration/m12-portable-clean`
- M11 classification: `M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`

## Fixed review roles

- **Innovative Systems Architect** — search for the strongest safe integration path, not merely the shortest merge path.
- **Anti-Deviation / Skeptical Reviewer** — reject unrelated research baggage, semantic weakening, silent migration, privilege expansion, overclaim, or merge pressure from sunk cost.

Dynamic specialists for this gate: Rust workspace/package architecture, GitHub Actions/release engineering, evidence semantics, baseline compatibility, and repository hygiene.

## Raw candidate inventory

Draft PR #91 compares the full M11 source branch against frozen public `main` and contains:

- 138 commits;
- 104 changed files;
- 11,768 additions;
- 3 deletions.

The raw candidate is mergeable at the Git graph level, but it is **not accepted as the release integration shape**.

The diff contains multiple classes:

1. **portable product hardening**
   - `crates/execsurface-observe/src/lib.rs` shared-FD ambiguity fail-closed behavior;
   - portable authority/completeness semantics and regression tests.

2. **authority model / compatibility support**
   - `crates/execsurface-authority/*`;
   - proposition-level authority and baseline comparability tests;
   - M11 compatibility/red-team/performance evidence.

3. **explicit hybrid research adapter**
   - `crates/execsurface-hybrid-authority/*`;
   - remains non-default and does not establish a live end-to-end BPF-LSM product observer.

4. **M10 research/evidence baggage**
   - numerous `m10-*` workflows;
   - `experiments/m10/*` BPF/C/QEMU fixtures and loaders;
   - M10 protocol/result documents.

5. **M11 research/release-review workflows and documents**
   - authority CI, Marketplace regression, performance/release workflow;
   - M11 protocol/result/closeout documents.

## M12.0 anti-deviation finding

The raw M11 branch is a valid **evidence source**, but merging it wholesale would conflate the portable next-alpha hardening with research-only M10 experiment infrastructure and hybrid exploration.

That is unnecessary scope expansion for the authorized release boundary.

Therefore:

**`RAW_M11_BRANCH_REJECTED_AS_DIRECT_MERGE_SHAPE`**

This does **not** reject the portable M11 hardening. It rejects only a wholesale 104-file merge as the integration mechanism.

## Clean integration rule

A new clean branch was created from frozen public `main`:

`integration/m12-portable-clean`

Only changes justified by the M12 portable release boundary may be ported to it.

### Eligible by default

- shared-FD fail-closed runtime hardening;
- authority semantics needed to prevent overclaim;
- exact regression/compatibility tests required to prove the hardening;
- release/Marketplace tests required by M12;
- bounded release documentation.

### Excluded by default unless a later gate proves necessity

- M10 experiment harnesses and one-off BPF/QEMU workflows;
- live-hybrid research code that would alter packaging or public expectations without product evidence;
- any default backend change;
- any ptrace↔hybrid baseline equivalence;
- any silent privilege/deployment change.

## Compatibility invariant

The clean candidate must preserve:

- public `v0.1.0-alpha.3` until M12.8 authorizes a new release;
- stable `AETHERXGLOBAL/execsurface@v0.1` behavior until explicit promotion;
- current baseline interpretation unless a versioned migration is chosen;
- Linux x86_64 self-service path without BPF-LSM requirements;
- fail-closed behavior for ambiguity/loss/capability gaps.

## M12.0 classification

**`M12_0_DIFF_FROZEN_CLEAN_INTEGRATION_REQUIRED`**

M12.0 is closed.

Next authorized gate: **M12.1 — Semantic integration audit** on the clean branch before porting any broader release surface.
