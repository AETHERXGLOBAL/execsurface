# M9.2 — Public Independent-Adoption Discovery Scope

Date: 2026-09-27
Status: **FROZEN BEFORE DISCOVERY RESULTS**
Base main: `a6260344c5a21c4fe5d4ea6cb7fc682092077ebb`
Protocol: `docs/milestones/M9_2_INDEPENDENT_ADOPTION_PROTOCOL.md`

## Purpose

Search for already-existing public evidence of independent ExecSurface use without manufacturing adoption, contacting maintainers, authoring external PRs, or materially assisting execution.

This scope is intentionally GitHub-centered because independent use that can satisfy M9.2 needs attributable public/consented provenance and reproducible workflow evidence. Absence from this scope is not evidence that nobody uses or wants ExecSurface elsewhere.

## Search date boundary

Evaluate public GitHub evidence discoverable on **2026-09-27**.

## Frozen discovery sources

1. public GitHub code search;
2. public GitHub pull requests;
3. public GitHub issues;
4. public GitHub repositories whose name/description/reference may mention ExecSurface;
5. public workflow/manifests/readmes returned by the above searches;
6. the ExecSurface repository's own public M9.2 intake issues, if any independently submitted evidence exists.

No private repositories, private messages, email, telemetry, secret-bearing logs, or non-consented data are part of this discovery gate.

## Frozen search families

Run the following semantic/exact search families and retain the results that are materially relevant:

### S1 — Package/install usage

- `cargo install execsurface`
- `execsurface 0.1.0-alpha.2`
- `execsurface Cargo.toml`

Goal: find third-party manifests/docs/scripts that install or depend on the public package.

### S2 — CLI workflow usage

- `execsurface learn`
- `execsurface check`
- `execsurface doctor`
- `execsurface.lock.json`

Goal: find public third-party evidence of actual baseline/check usage rather than mentions only.

### S3 — GitHub Action / CI integration

- `AETHERXGLOBAL/execsurface`
- `uses AETHERXGLOBAL execsurface`
- `ExecSurface GitHub Action`

Goal: find third-party workflow integration or CI evaluation.

### S4 — Pull-request evidence

Search public pull requests for `execsurface` and `ExecSurface`.

Goal: find third-party PRs where ExecSurface was independently installed, evaluated, integrated, retained, or removed with attributable results.

### S5 — Issue evidence

Search public issues for `execsurface` and `ExecSurface`.

Goal: find attributable third-party reports with real execution evidence, including failures/friction.

### S6 — Repository/reference discovery

Search public repositories for `execsurface` / `ExecSurface` in repository name or description and inspect materially relevant results.

Goal: find projects that may vendor, integrate, document, or evaluate ExecSurface.

### S7 — Local intake

Inspect public issues in `AETHERXGLOBAL/execsurface` created through or materially equivalent to the M9.2 independent-evidence intake.

Goal: identify third-party submissions that may satisfy the frozen independent-evidence gate.

## Exclusions

Do not count the following as adoption:

- the `AETHERXGLOBAL/execsurface` repository itself;
- AETHER X organization repositories;
- mirrors/forks with no independent execution evidence;
- GitHub stars/watchers/forks by themselves;
- package/release download counts;
- AETHER X-authored external PRs/issues;
- M9.1 zero-contact workloads run by AETHER X;
- synthetic fixtures/test references;
- search-engine snippets with no attributable source;
- copied logs without attributable executor/provenance;
- collaborative runs with material AETHER X execution/debugging assistance.

## Candidate classification

Every materially relevant discovered result is classified as exactly one of:

- `DISCOVERY_ONLY` — mention/reference/install hint but no qualifying independent workflow evidence;
- `CANDIDATE_INDEPENDENT` — potentially qualifying external execution evidence requiring detailed provenance/workflow review;
- `ACCEPTED_INDEPENDENT` — passes the canonical M9 validator, M9.2 admissibility validator, and manual provenance review;
- `KILLED` — fails independence, attribution, privacy, or workflow requirements.

`ACCEPTED_INDEPENDENT` is not automatically countable toward positive close. Countability is separately checked by `scripts/m9_validate_independent_adoption.py --countable`.

## Search integrity rules

- do not change these search families after seeing results merely to improve adoption yield;
- additional search terms may only refine a returned candidate's identity/provenance, not expand the declared negative-close scope after interpretation;
- preserve meaningful negative/ambiguous results;
- do not contact or guide an external user during this discovery gate;
- do not author an external integration and then classify it as independent;
- do not infer execution from a textual mention;
- do not infer retained adoption from one successful trial;
- do not infer broad market adoption from a small number of public results.

## Negative-close meaning

If no result passes M9.2 after executing S1–S7, the permitted conclusion is only:

> `KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE (2026-09-27)`

This does not mean no users exist outside the declared public GitHub scope.
