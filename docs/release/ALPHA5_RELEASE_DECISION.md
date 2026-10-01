# ExecSurface v0.1.0-alpha.5 — Public Release Decision & Closeout

Date: 2026-10-01
Status: **PUBLICLY RELEASED — P8 POST-RELEASE EXTERNAL VALIDATION OPEN**

The exact pre-publication blocked decision is retained unchanged at `docs/archive/release-decisions/ALPHA5_PRE_PUBLICATION_DECISION.md`.

## Governance transition

The preregistered pre-publication state initially blocked Alpha.5 on P8 external evidence. The owner subsequently gave explicit authorization to **waive P8 only as a pre-publication gate**, publish Alpha.5, and continue independent validation after publication.

This governance change does not relabel P8 as passed and does not convert internal testing, internal red-team work, outreach, participation or routing into independent external validation.

## Release identity

- Public release: `v0.1.0-alpha.5`
- Release source: `9e73b925d55557e33de1b0813995609aaefdc037`
- Frozen product source used for internal qualification: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Stable Action: `AETHERXGLOBAL/execsurface@v0.1` -> Alpha.5 release source
- Immutable Action: `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`
- Registry package: `execsurface = 0.1.0-alpha.5`
- Canonical public release workflow run: `36910725515`

## Public-release proof chain

The release workflow completed immutable identity/governance checks; full source gates; deterministic dual builds; GLIBC compatibility enforcement; deterministic public bundle construction; archive safety/privacy; GitHub provenance; GitHub prerelease publication; source/public-artifact byte equivalence; public binary consumption and tamper rejection on Ubuntu 22.04/24.04; fresh immutable-tag install; immutable Action PASS/REVIEW/BLOCK/ERROR semantics; stable `v0.1` promotion only after public proofs; stable-channel proofs; registry publication; zero-contact registry install/smoke; and closeout recording.

## Retained negative evidence

Publication does not erase earlier failures. Material retained failures include the A4 workflow-permission syntax failure, A5 harness assumption failure, A6 governance-locator harness failure, initial A7 reproducibility/Ubuntu-22.04 GLIBC failure, and redundant ABI-floor rehearsal failure.

Corrections did not authorize weakening scientific acceptance thresholds.

## P8 external validation

P8 remains open for current independent evidence such as zero-assistance reproduction/failure, substantive architecture criticism/counterexample, standards/interoperability guidance, external real-workload reports and qualified external contributions.

Public review hub: issue `#118`. P8 qualification/evidence tracking: issue `#114`.

Current bounded statement:

`ALPHA5_PUBLICLY_RELEASED — INTERNAL_AND_PUBLIC_ARTIFACT_PROOFS_PASS — P8_INDEPENDENT_EXTERNAL_VALIDATION_OPEN`

Do not claim independent validation, endorsement, adoption or universal correctness from this release.

## Support boundary

- Linux x86_64 only for this public alpha.
- Native ptrace is the bounded public reference observer.
- ARM64 support is not claimed.
- Observed behavior is not all possible behavior.
- Incomplete or ambiguous evidence cannot silently become PASS.
- ExecSurface is not antivirus, EDR, SIEM, malware detection, a sandbox or proof of software safety.
