# ExecSurface Documentation

This directory separates **current public guidance** from **engineering evidence and historical records**. For the present product state, start with the documents in the first section rather than inferring current behavior from an older milestone record.

## Current public product

- [Current Status](STATUS.md) — authoritative public support, distribution and validation state.
- [Production Readiness](PRODUCTION_READINESS.md) — P9 evidence contract and v1.0 qualification gates; not a claim that v1.0 is already authorized.
- [Compatibility & Stability Contract](COMPATIBILITY.md) — P9.3 candidate v1 compatibility boundary and the executable evidence required before it may become stable.
- [Five-Minute Start](QUICKSTART_5_MIN.md) — shortest controlled PASS → REVIEW walkthrough.
- [Self-Service Start](SELF_SERVICE_START.md) — installation and first-use path for independent users.
- [GitHub Action](GITHUB_ACTION.md) — CI integration and stable Action usage.
- [Troubleshooting](TROUBLESHOOTING.md) — environment, ptrace and evidence-health diagnostics.
- [Examples](EXAMPLES.md) — command-line usage examples.
- [Why provenance is not the same as runtime behavioral integrity](WHY_RUNTIME_BEHAVIORAL_INTEGRITY.md) — shareable technical explanation of the problem ExecSurface addresses.
- [Ecosystem Positioning](ECOSYSTEM_POSITIONING.md) — factual scope/interoperability map relative to adjacent runtime-security and provenance systems; not a winner ranking.

## Evaluation and external evidence

- [Independent Evaluation](INDEPENDENT_EVALUATION.md) — how an external evaluator can reproduce, challenge or falsify the public release.
- [Technical Evaluation](TECHNICAL_EVALUATION.md) — repeatable technical evaluation pack.
- [Team & Evidence Governance](TEAM_AND_EVIDENCE_GOVERNANCE.md) — evidence discipline, role separation and anti-drift rules.
- [`external/`](external/) — external engagement protocols, baselines and evidence ledgers.
- GitHub issue `#118` — stable-v1 independent external evaluation and post-release review hub.
- GitHub issue `#114` — P8 evidence qualification and tracking.
- GitHub issue `#129` — P9 production-readiness and v1.0 qualification program.
- GitHub issue `#134` — P9.3 compatibility and stability contract gate.

Negative, partial, unsupported, reproduction-failure and no-fit findings are retained as first-class evidence. A contact, referral, invitation, internal test or self-evaluation PASS is not independent validation by itself.

## Contributor entry points

New contributors can start from repository Issues labeled `good first issue` and `help wanted`. Current newcomer-scoped work includes deterministic examples, Python/pytest and Node.js/npm recipes, a minimal GitHub Action consumer example, and current-documentation auditing.

Contributions must preserve product boundaries, negative evidence, fail-closed behavior and the distinction between internal/community engineering work and independent external validation. See [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Architecture and engineering

- [`architecture/`](architecture/) — current and historical architecture records.
- [`development/`](development/) — bounded development/research protocols and engineering records.
- [`milestones/`](milestones/) — milestone evidence. These files are historical records unless a current-status document explicitly incorporates them.

## Releases and distribution

- [crates.io Publishing](CRATES_IO_PUBLISHING.md) — registry publication process and safeguards.
- [`release/`](release/) — current release-decision material.
- [`releases/`](releases/) — release-specific public notes and retained evidence.

## Historical archive

- [`archive/`](archive/) — superseded state documents and retained historical material.

Historical failures, negative evidence and closed gates are intentionally preserved. Moving a record into an archive changes its operational location, not its evidentiary meaning and never converts a failure into a pass.

## Source-of-truth order

When documents from different dates appear to conflict, use this order for current public facts:

1. immutable published release/tag metadata for release identity and artifacts;
2. `docs/STATUS.md` for current support, distribution and validation state;
3. root `README.md` for the current user-facing path;
4. the latest release-specific document under `docs/releases/`;
5. dated milestone/development/archive records for chronology and historical evidence only.

ExecSurface Alpha.5 remains bounded to Linux x86_64. ARM64 is not claimed. Native ptrace remains the bounded public reference observer. Independent external validation is not claimed unless qualified external evidence is explicitly recorded.

Security-sensitive findings must follow [`SECURITY.md`](../SECURITY.md) rather than public evidence channels.
