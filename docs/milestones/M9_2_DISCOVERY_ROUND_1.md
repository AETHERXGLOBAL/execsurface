# M9.2 — Discovery Round 1 Scope

Date: 2026-09-27
Status: **FROZEN BEFORE SEARCH RESULTS**
Base main: `a6260344c5a21c4fe5d4ea6cb7fc682092077ebb`

## Purpose

Search public GitHub surfaces for independently attributable ExecSurface use without contacting, guiding, or executing anything for third parties.

This round is discovery only. Search hits do not become adoption evidence until they pass the frozen M9.2 provenance/intake rules and manual attribution review.

## Team

- OSS Adoption Evidence Lead
- GitHub/CI Provenance Specialist
- Developer Experience Reviewer
- Privacy/Attribution Reviewer
- Runtime Semantics Reviewer
- fixed Innovation Scientist / Adoption Architect
- fixed Deviation Prevention / Scientific Integrity
- Independent Red Team

## Frozen search scope

Search public GitHub code, issues, and pull requests using the following exact discovery terms/patterns:

1. `AETHERXGLOBAL/execsurface@`
2. `cargo install execsurface`
3. `execsurface.lock.json`
4. `execsurface check`
5. `execsurface learn`
6. `execsurface-policy.json`
7. `ExecSurface`

For code search, inspect up to the first 20 results per query returned by the connected GitHub search capability. For issue/PR search, inspect up to the first 20 relevant public results for `ExecSurface` outside `AETHERXGLOBAL/execsurface` where the API supports it.

Duplicates across queries are collapsed by repository/reference identity.

## Exclusions

The following cannot count as independent candidates:

- `AETHERXGLOBAL/execsurface` itself;
- other AETHER X-controlled repositories or branches where AETHER X authored the integration;
- generated mirrors/caches that merely copy repository text;
- search-index pages without attributable third-party execution;
- stars, forks, downloads, traffic, or social reactions without execution evidence;
- AETHER X-authored external PRs/issues;
- M9.1 zero-contact reproductions;
- synthetic fixtures and documentation examples.

## Classification

Every unique external hit is classified as exactly one of:

- `DISCOVERY_ONLY` — mention/config/example with no qualifying independent execution evidence;
- `CANDIDATE_INDEPENDENT` — potentially third-party initiated/executed and requires provenance/workflow review;
- `ACCEPTED_L1`, `ACCEPTED_L2`, or `ACCEPTED_L3` — only after canonical validators plus manual attribution review;
- `KILLED` — fails independence, provenance, privacy, or workflow requirements.

## Evidence handling

For every candidate preserve, where available:

- repository/reference URL;
- owner/project identity;
- file/issue/PR path or number;
- visible execution/integration signal;
- whether AETHER X appears to have authored or materially assisted it;
- provisional classification and reason.

Do not copy secrets, credentials, private logs, file contents unrelated to the public evidence, environment values, stdin, network payloads, or unrestricted argv.

## Stop condition

Round 1 ends after all seven frozen queries have been executed and all unique returned hits in the declared result windows have been classified.

If no candidate qualifies, the result is `0 accepted independent adoption records in Round 1 scope`. That is negative discovery evidence only; it is not proof that no independent users exist outside the declared scope and is not a positive adoption result.
