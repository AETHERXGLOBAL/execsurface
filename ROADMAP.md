# ExecSurface Roadmap

This file describes the **current forward roadmap**. The detailed M0–M9 historical roadmap is preserved unchanged at `docs/archive/ROADMAP_PRE_ALPHA5.md`.

Current public state: `v0.1.0-alpha.6` is released for Linux x86_64, with native `ptrace` as the bounded public reference observer. See `docs/STATUS.md` for the authoritative current release state.

## Operating principles

ExecSurface advances only through evidence-preserving changes:

- incomplete or ambiguous evidence must not silently become PASS;
- backend identity alone does not create semantic authority;
- baseline learning is separate from authorization/policy;
- failed tests and negative evidence are retained;
- acceptance criteria are not weakened to obtain a favorable result;
- research backends are not promoted to public authority without proposition-specific evidence;
- public claims remain bounded to what the released artifact and qualified evidence establish.

## P9 — Production readiness & v1.0 qualification

**OPEN / ACTIVE**

The production-readiness program is tracked in issue `#129`, the frozen qualification contract is `docs/PRODUCTION_READINESS.md`, the current Alpha.6 gap analysis is `docs/development/V1_PRODUCTION_READINESS_GAP_ANALYSIS.md`, the frozen v1 stable contract is `docs/COMPATIBILITY.md`, and the stable support/deprecation policy is `docs/SUPPORT_POLICY.md`.

The program does not authorize a stable-release claim from internal CI alone. Production qualification requires independent external evidence, a stable compatibility contract, public-consumer reliability, repository/supply-chain controls, real-workload evidence, and an evidence-qualified release candidate.

P8 remains the external-evidence authority for the independent validation portion of P9. Repository-administration controls that cannot be applied through the connected automation remain explicit blockers rather than being treated as complete by documentation.

V1-R0 stable-contract freeze is now the active internal maturity boundary: do not add v1 features merely for breadth. The next internal engineering gate after R0 is V1-R1 release-control-plane hardening, while external Alpha.6 execution/use evidence proceeds independently.

## R1 — Post-release independent validation

**OPEN / ACTIVE**

Obtain independent reproduction, criticism, counterexamples, interoperability findings and real-workload evidence against the current supported public release. Historical Alpha.5 evidence remains retained; current v1-readiness execution/use evidence should target Alpha.6 unless a protocol explicitly freezes another source.

Tracking: issues `#114` and `#118`.

Publication before P8 closeout was explicitly owner-authorized. That authorization did not convert internal evidence into independent validation. P8 remains open until its external-evidence rules are satisfied or explicitly superseded by a documented governance decision.

## R2 — Public-consumer reliability

**ACTIVE / CONTINUOUS**

Maintain a small durable operational CI surface that continuously proves:

- source formatting, linting, tests and lockfile integrity;
- public GitHub Release installation and checksum verification;
- stable `AETHERXGLOBAL/execsurface@v0.1` behavior;
- exact registry installation of the current public release;
- bounded ptrace lifecycle and adversarial regressions;
- PASS / REVIEW / BLOCK / ERROR behavior without silent semantic relaxation.

Completed one-shot research and release workflows are archived rather than left as active operational automation.

## R3 — Evidence semantics and authority hardening

**OPEN — EVIDENCE-GATED**

Continue strengthening proposition-scoped authority, completeness, provenance, executable/artifact binding and legitimate-variance handling without converting frequency, similarity or backend labels into authorization.

Changes that affect current public semantics require explicit adversarial regression and compatibility evidence before release.

## R4 — Runtime backend research

**RESEARCH ONLY**

Native `ptrace` remains the public reference observer. eBPF/BPF-LSM, imported traces and other heterogeneous evidence backends remain research or adapter candidates unless proposition-by-proposition authority/completeness evidence justifies a bounded promotion.

No general backend-equivalence claim is authorized.

## R5 — Platform expansion

**NOT YET CLAIMED**

Linux x86_64 remains the public support boundary for Alpha.6. ARM64 or additional operating-system support may be reconsidered only after independent, reproducible platform evidence closes the relevant compatibility and semantic gates.

Historical ARM64 negative evidence remains retained and must not be reinterpreted as support.

## R6 — Release discipline

Future public releases should preserve the Alpha.6 release standard or strengthen it:

1. freeze the exact release source;
2. run full source and adversarial gates;
3. build reproducibly against the declared compatibility floor;
4. bind checksums and provenance to the exact artifact;
5. publish an immutable version tag/release;
6. consume the public artifact as a fresh user on supported environments;
7. move the stable `v0.1` channel only after public-artifact proofs pass;
8. prove registry installation after publication;
9. retain every failed attempt and material negative result.

## Historical program record

The original milestone roadmap through M9, including closed gates, failed experiments, eBPF research decisions and independent-adoption search results, is preserved at:

`docs/archive/ROADMAP_PRE_ALPHA5.md`

Those records remain evidence. They are not the authoritative statement of the current public version or current support boundary; for current facts use `docs/STATUS.md`.
