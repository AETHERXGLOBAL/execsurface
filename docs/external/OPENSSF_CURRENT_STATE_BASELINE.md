# OpenSSF Current State Baseline

Date: 2026-10-01
Purpose: state the current public ExecSurface facts that govern OpenSSF / Linux Foundation technical engagement.

The exact pre-Alpha.5 baseline is retained unchanged at `docs/archive/engagement/OPENSSF_CURRENT_STATE_BASELINE_ALPHA4.md`.

## Current source of truth

- Repository: `AETHERXGLOBAL/execsurface`
- Public release: `v0.1.0-alpha.5`
- Release source: `9e73b925d55557e33de1b0813995609aaefdc037`
- Frozen product source used for Alpha.5 qualification: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Stable Action: `AETHERXGLOBAL/execsurface@v0.1` -> Alpha.5 release source
- Immutable Action: `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`
- Registry package: `execsurface = 0.1.0-alpha.5`
- Public maturity: Public Alpha
- Public platform scope: Linux x86_64

## Current product model

ExecSurface is a runtime behavioral-integrity and verification layer:

`accepted runtime behavior -> authority-aware evidence -> deterministic drift -> explicit verification -> attestable result`

The baseline records an accepted canonical observed execution surface. It is not policy and is not proof that accepted behavior is safe.

## Architecture boundary

- Native Linux `ptrace` remains the bounded public reference observer.
- Evidence is proposition-scoped; backend identity alone does not create universal authority.
- Incomplete or ambiguous evidence cannot silently become PASS.
- Pathname access-attempt metadata is not represented as kernel-object identity.
- Exact universal shared-FD attribution is not claimed.
- BPF-LSM/kernel-hook/hybrid work remains research/managed and non-default unless separately promoted by evidence.
- No ptrace/hybrid baseline equivalence is claimed.
- ARM64 support is not claimed.

ExecSurface is not antivirus, EDR, SIEM, malware detection, a sandbox, a general MAC system or proof of software safety.

## Distribution and post-release evidence

Alpha.5 completed deterministic build, compatibility, checksum, provenance, archive/privacy, public binary, source/artifact equivalence, immutable Action, stable Action and registry-install gates in release run `36910725515`.

These are internal/public-artifact release proofs, not independent external validation.

## External validation state

Publication was owner-authorized with P8 waived only as a pre-publication gate. P8 remains open after publication for independent reproduction, failures, counterexamples, architecture criticism, interoperability guidance and external real-workload evidence.

Current bounded state:

`ALPHA5_PUBLICLY_RELEASED — P8_POST_RELEASE_EXTERNAL_VALIDATION_OPEN`

Open review hub: issue `#118`. P8 evidence qualification: issue `#114`.

OpenSSF/community participation, routing, acknowledgement or praise must never be represented as endorsement or independent validation.
