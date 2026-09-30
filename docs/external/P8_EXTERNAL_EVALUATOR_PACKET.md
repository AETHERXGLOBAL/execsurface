# ExecSurface — P8 External Evaluator Packet

Status: **RESEARCH FREEZE CANDIDATE — does not claim external validation**
Tracking: #114, external engagement #94
Date: 2026-09-30

This packet is intentionally designed for criticism, failure reports, no-fit findings, and independent reproduction. A positive result is welcome, but it is not the preferred or assumed outcome.

## What ExecSurface claims, narrowly

ExecSurface is a runtime behavioral-integrity / verification layer for accepted execution behavior. It compares a previously accepted canonical execution surface with a later observed surface under an explicit observer and policy.

It is **not** antivirus, EDR, malware detection, a sandbox, or proof that a program is safe.

Public alpha.4 is Linux x86_64 only. The public correctness-reference backend is ptrace. Incomplete evidence cannot silently become PASS.

## Frozen public target

Repository:
`https://github.com/AETHERXGLOBAL/execsurface`

Release:
`v0.1.0-alpha.4`

Release source:
`48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

Current public main at P8-A0 freeze:
`b47721ce675444ce029b048edce6fc94631d6408`

Linux x86_64 release asset:
`execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu.tar.gz`

Expected asset SHA256:
`55776cd130784d1c03451a14ab05ed113ffae4fa714c986666cbcadf242d8d1d`

Stable GitHub Action channel:
`AETHERXGLOBAL/execsurface@v0.1`

## Pick one evaluator track

### Track R — zero-assistance reproduction

Please do not ask AETHER X for troubleshooting until you have recorded your initial result.

Use the public repository and its public docs only. The shortest route is the published Linux x86_64 release and the Five-Minute / Independent Evaluation material.

Record:
- operating system / architecture;
- install route used;
- whether checksum verification succeeded;
- `execsurface --version`;
- `execsurface doctor` result;
- whether the controlled PASS -> REVIEW walkthrough behaved as documented;
- any ERROR/incompleteness result;
- any false PASS or false REVIEW you believe occurred;
- documentation/setup friction;
- total assistance received before the initial result (`ZERO_ASSISTANCE` expected for this track).

A failed or partial attempt is a valid result. Please preserve it before trying an assisted retry.

### Track C — architecture criticism / counterexample

Please try to break the claims, not validate them.

Useful questions:
1. Is ptrace an inappropriate correctness-reference boundary for one or more propositions even with explicit incompleteness and bounded authority?
2. Can you construct a situation where incomplete/lost/ambiguous observation becomes PASS?
3. Can baseline identity, policy, observer identity, or backend name be confused in a way that creates false authority?
4. Is the accepted-surface / current-surface distinction semantically insufficient for real CI/developer workflows?
5. Is ExecSurface's intended layer redundant with a mature existing tool or standard? A precise `NO_FIT_OR_REDUNDANCY` result is useful.
6. Are there filesystem/network/process object-identity counterexamples not covered by current documented limitations?
7. Does any current wording overstate what the evidence supports?

Historical note: prior external criticism of the ptrace-centric architecture was accepted and materially constrained subsequent architecture. Repeating that criticism is still useful if it applies to the current bounded design, but please state the concrete surviving failure mode.

### Track I — standards / interoperability review

Research question: is the bounded attestation composition representable/interoperable using existing structures rather than a new ExecSurface-specific predicate standard?

Research decision under review:
`P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`

Composition explored:
- in-toto Statement v1;
- in-toto Runtime Trace v0.1;
- SCAI v0.3;
- SVR v0.2;
- SLSA provenance reference where available;
- GitHub/Sigstore artifact-attestation identity / verification.

Please look for:
- semantic mismatch between behavioral verification and those predicate structures;
- ambiguous producer/verifier identity binding;
- inability to express observer authority/completeness without proprietary interpretation;
- duplicate/conflicting semantic fields;
- replay/substitution or cross-attestation binding gaps;
- a strong reason a narrowly scoped new predicate is actually necessary;
- a better existing interoperability path.

This is a request for technical criticism, **not standards-body endorsement**.

### Track W — external real workload

Use a workload/project that AETHER X did not author.

Please record:
- repository/workload identity if it can be shared;
- command monitored;
- baseline creation/approval process;
- current run result;
- setup/privilege friction;
- observed drift that was useful or noisy;
- false PASS, false REVIEW or false incompleteness;
- data/privacy footprint you observed;
- whether the tool's intended layer fits the workload at all.

Do not send secrets, proprietary file contents, tokens, environment variables or network payloads.

## Result vocabulary

You do not need to use our exact labels, but these map cleanly into the research ledger:
- reproduction succeeded;
- reproduction failed;
- partial / unsupported environment;
- architecture criticism;
- counterexample;
- no-fit / redundancy;
- interoperability guidance;
- external real-workload report;
- security-sensitive finding.

## Security-sensitive findings

If you believe you found a vulnerability rather than a normal architecture/product criticism, do **not** post exploit details publicly. Use the repository security policy / private disclosure path.

## What we will do with your result

AETHER X will:
1. preserve the original result before assistance/fixes;
2. classify whether it qualifies as independent external evidence;
3. attempt reproduction where appropriate;
4. retain negative/no-fit outcomes;
5. separate an assisted follow-up from the initial attempt;
6. avoid describing participation or praise as endorsement/adoption.

## Minimal response template

- Track: R / C / I / W
- Relationship to ExecSurface/AETHER X: unaffiliated / ecosystem maintainer / user-evaluator / contributor / other
- Target release/commit/docs:
- Environment/workload:
- Assistance before initial result: zero / docs-only / assisted / N/A
- Result:
- Reproduction steps or technical reasoning:
- Evidence/link/log excerpt (redacted for secrets):
- What claim should change, if any:
- May we cite your public handle/name with this technical result? yes/no

Negative results are explicitly welcome.
