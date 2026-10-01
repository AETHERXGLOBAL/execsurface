# ExecSurface Documentation

This directory separates **current public documentation** from **historical engineering evidence**.

For the current product state, start with:

1. [Current Status](STATUS.md)
2. [Five-Minute Start](QUICKSTART_5_MIN.md)
3. [GitHub Action](GITHUB_ACTION.md)
4. [Troubleshooting](TROUBLESHOOTING.md)
5. [Independent Evaluation](INDEPENDENT_EVALUATION.md)

## Current product documentation

- [STATUS.md](STATUS.md) — authoritative current public product/distribution state.
- [SELF_SERVICE_START.md](SELF_SERVICE_START.md) — self-service entry path.
- [QUICKSTART_5_MIN.md](QUICKSTART_5_MIN.md) — controlled first evaluation.
- [GITHUB_ACTION.md](GITHUB_ACTION.md) — stable and immutable Action usage.
- [EXAMPLES.md](EXAMPLES.md) — command examples.
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — common setup and observation issues.
- [CRATES_IO_PUBLISHING.md](CRATES_IO_PUBLISHING.md) — registry publication model.

## Evaluation and external review

- [INDEPENDENT_EVALUATION.md](INDEPENDENT_EVALUATION.md) — independent evaluation rules.
- [TECHNICAL_EVALUATION.md](TECHNICAL_EVALUATION.md) — technical evaluator pack.
- [external/](external/) — current external-review protocols, ledgers and engagement records.
- GitHub issue `#118` — public Alpha.5 post-release review hub.
- GitHub issue `#114` — P8 evidence qualification and tracking.

Negative, partial, unsupported and no-fit findings are retained as first-class evidence. Internal testing does not become independent external validation by relabeling it.

## Architecture and engineering records

- [architecture/](architecture/) — architecture contracts and reviews.
- [milestones/](milestones/) — milestone and gate evidence.
- [releases/](releases/) — release-specific records and evidence.
- [development/](development/) — active/development protocols that remain relevant.

These records are not automatically statements about the current public release. Historical documents must be read in their dated context.

## Historical archive

- [archive/](archive/) — superseded operational documents preserved for provenance and auditability.

Files are archived rather than deleted when they contain scientific, release, failure or governance evidence. Archiving does not convert a failed gate into a pass and does not rewrite project history.

## Documentation authority order

When documents disagree because the project advanced, use this order for **current public facts**:

1. immutable published release/tag metadata;
2. [STATUS.md](STATUS.md);
3. root [README.md](../README.md);
4. latest release record under [releases/](releases/);
5. dated milestone/development/archive records for historical context.

Security-sensitive reports must follow [SECURITY.md](../SECURITY.md) rather than public evidence channels.
