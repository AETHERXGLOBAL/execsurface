# M6.6 — Release Metadata Negative Evidence 002

Date: 2026-09-26

Release:

`v0.1.0-alpha.1`

Release workflow:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36194046171

## Observation

The release artifact, checksum, attestation, source tag and all zero-contact consumer gates succeeded.

However, the human-readable GitHub Release body displayed an empty value after:

`Source commit:`

## Root cause

The release workflow used:

```bash
echo "Source commit: `$GITHUB_SHA`"
```

Inside a double-quoted Bash string, the backticks are command-substitution syntax. The SHA text was therefore treated as a command substitution instead of literal Markdown formatting.

## Scope

This defect affected **release-page metadata only**.

It did not change:

- the immutable `v0.1.0-alpha.1` tag;
- the release archive bytes;
- the SHA-256 checksum;
- the GitHub build-provenance attestation;
- `BUILD_INFO.txt`;
- the stable `v0.1` tag;
- any ExecSurface runtime semantics.

## Correction

A separate main-branch metadata repair workflow:

1. verifies that the immutable tag peels to the expected source commit;
2. reads the existing published archive digest from GitHub;
3. edits release notes only;
4. verifies the corrected public body;
5. leaves tags and assets untouched.

The normal release workflow is also corrected for future versions using `printf` rather than command-substitution-sensitive backticks.

Status: **NEGATIVE EVIDENCE PRESERVED**.
