# ExecSurface — Current Status

Date: 2026-10-01

This file states the current public product state. Historical milestone, prerelease and engagement documents are evidence records and may describe the release that was current when they closed.

## Current public product state

- Public release: `v0.1.0-alpha.5`
- Release source commit: `9e73b925d55557e33de1b0813995609aaefdc037`
- Frozen product source used for Alpha.5 qualification: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Stable GitHub Action: `AETHERXGLOBAL/execsurface@v0.1`
- Stable `v0.1` resolves to the Alpha.5 release source commit above.
- Immutable Action pin: `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`
- Registry package: `execsurface = 0.1.0-alpha.5`
- Public support scope: Linux x86_64
- Public default/reference observer: native `ptrace`
- Verdict / exit-code contract: PASS `0`, ERROR `2`, REVIEW `10`, BLOCK `20`

`main` may advance with documentation, evaluation or maintenance commits after the immutable release source. Consumers that require immutable review should pin `v0.1.0-alpha.5`.

## Distribution state

Alpha.5 completed the release and post-publication chain: deterministic dual build and compatibility checks; checksum and provenance generation; GitHub prerelease publication; public-artifact byte/source equivalence; clean binary consumption on Ubuntu 22.04 and Ubuntu 24.04; immutable-tag installation and Action verdict checks; stable `v0.1` promotion only after public proofs; stable-channel public Action checks; registry publication; and zero-contact registry install/smoke.

Canonical public-release workflow run: `36910725515`.

## Product boundary

ExecSurface is a **runtime behavioral-integrity and verification layer for observed execution-surface drift**.

It is not antivirus, EDR, SIEM, malware detection, a sandbox, a general mandatory-access-control system or proof that software is safe.

- Observed behavior is not all possible behavior.
- Incomplete or ambiguous evidence cannot silently become PASS.
- A baseline is not policy.
- Backend identity alone does not create semantic authority.
- Native ptrace remains the bounded public reference observer.
- ARM64 support is not claimed.
- BPF-LSM/kernel-hook work remains research/managed and non-default unless a later evidence gate explicitly changes that boundary.

## External validation state

P8 independent external validation was **not closed before publication**. The owner explicitly waived P8 only as a pre-publication gate and authorized publication with post-release validation continuing.

Current bounded state:

`ALPHA5_PUBLICLY_RELEASED — P8_POST_RELEASE_EXTERNAL_VALIDATION_OPEN`

This does not claim independent validation, endorsement, adoption or approval by external reviewers or organizations.

Public review hub: issue `#118`.
Evidence qualification/tracking: issue `#114`.

## Historical evidence handling

Historical failures, prerelease decisions, closed-gate workflows and negative evidence are retained. Operationally obsolete files may be moved out of active paths into `docs/archive/` or `.github/workflow-archive/`; that is repository hygiene, not evidence deletion or history rewriting.

For current public facts, use this file, `README.md`, the latest GitHub Release and `docs/releases/v0.1.0-alpha.5.md`.
