# ExecSurface Documentation

This directory separates **current public guidance** from **engineering evidence and historical records**. For the present product state, start with the documents in the first section rather than inferring current behavior from an older milestone record.

## Current public product

- [Current Status](STATUS.md) — authoritative public release, support and validation state.
- [Five-Minute Start](QUICKSTART_5_MIN.md) — shortest controlled PASS → REVIEW walkthrough.
- [Self-Service Start](SELF_SERVICE_START.md) — installation and first-use path for independent users.
- [GitHub Action](GITHUB_ACTION.md) — CI integration and stable Action usage.
- [Troubleshooting](TROUBLESHOOTING.md) — environment, ptrace and evidence-health diagnostics.
- [Examples](EXAMPLES.md) — command-line usage examples.

## Evaluation and evidence

- [Independent Evaluation](INDEPENDENT_EVALUATION.md) — how an external evaluator can reproduce, challenge or falsify the public release.
- [Technical Evaluation](TECHNICAL_EVALUATION.md) — repeatable technical evaluation pack.
- [Team & Evidence Governance](TEAM_AND_EVIDENCE_GOVERNANCE.md) — evidence discipline, role separation and anti-drift rules.
- [`external/`](external/) — external engagement and evidence records. A contact, referral or invitation is not validation by itself.

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

Historical failures, negative evidence and closed gates are intentionally preserved. Moving a record into an archive changes its operational location, not its evidentiary meaning.

## Source-of-truth order

When documents from different dates appear to conflict, use this order for **current public facts**:

1. `docs/STATUS.md`
2. the latest published GitHub Release and immutable version tag
3. `README.md`
4. the release-specific document under `docs/releases/`
5. historical milestone/archive records for chronology and evidence only

ExecSurface Alpha.5 remains bounded to Linux x86_64 and does not claim independent external validation unless qualified external evidence is explicitly recorded.
