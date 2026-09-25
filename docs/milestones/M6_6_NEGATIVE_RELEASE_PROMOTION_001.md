# M6.6 — Release Promotion Negative Evidence 001

Date: 2026-09-26

Production promotion run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36193609548

Result: **FAIL — BEFORE TAG CREATION**

## Failure

The release request expected:

`v0.1.0-alpha.1`

but `action/release-tag.txt` contained the literal characters:

`v0.1.0-alpha.1\n`

rather than a trailing newline byte.

The promotion identity check failed before:

- creating the immutable tag;
- dispatching the release workflow;
- publishing any artifact.

## Impact

No release tag or GitHub Release was published by the failed run.

Therefore `v0.1.0-alpha.1` remains unused and may be retried after correcting the source file. A new prerelease number is not required.

## Correction

- write `action/release-tag.txt` with a real newline;
- make the distribution dry-run consume the same file rather than a hard-coded tag;
- add an explicit byte-length/tag assertion;
- record `request_revision: 2`;
- improve promotion diagnostics while preserving the same hard identity checks.

## Governance conclusion

**NEGATIVE EVIDENCE PRESERVED.**

The gate behaved correctly: inconsistent release identity stopped promotion before immutable/public state was created.
