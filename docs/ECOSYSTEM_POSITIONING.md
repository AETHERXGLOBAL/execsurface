# ExecSurface in the runtime and software-supply-chain ecosystem

Last reviewed: 2026-10-01

This document explains where ExecSurface fits relative to adjacent open-source security and provenance systems. It is a **scope and interoperability map**, not a winner ranking.

ExecSurface Alpha.6 is a public alpha for Linux x86_64. Native `ptrace` is the bounded public reference observer. ARM64 support and independent external validation are not claimed.

## The question ExecSurface is designed to answer

ExecSurface focuses on a per-command integrity question:

> Given behavior that was explicitly learned/accepted for this command, what execution-surface behavior appeared, disappeared or changed now, what evidence supports that observation, and what does the explicit policy say about the drift?

The product path is intentionally narrow:

`accepted runtime behavior -> authority-aware evidence -> deterministic drift -> explicit verification -> attestable result`

ExecSurface does not infer that drift is malicious, and an unchanged observed surface does not prove that software is safe.

## Adjacent systems

| System / class | Primary focus | Relationship to ExecSurface |
|---|---|---|
| **ExecSurface** | Accepted per-command runtime surface, explicit drift, evidence/completeness boundaries, deterministic PASS / REVIEW / BLOCK / ERROR | The subject of this repository. Current public path is a Rust CLI + GitHub Action for Linux x86_64. |
| **Falco** | Runtime security detection and alerting over syscall/kernel and plugin event streams using rules | Complementary. Falco is designed for broad runtime detection/alerting. ExecSurface is centered on comparing a specific execution against an explicitly accepted surface; it does not ship a malware/threat ruleset. |
| **Tetragon** | eBPF-based security observability plus in-kernel runtime enforcement | Complementary and potentially an evidence source under separately proved semantics. Tetragon has stronger kernel-side observation/enforcement capabilities; ExecSurface does not attempt to reproduce that enforcement architecture. |
| **Sigstore** | Artifact signing/identity verification and transparency-log-backed verification | Complementary. Sigstore can help establish artifact identity/provenance; ExecSurface asks what the selected execution actually did under its observer and policy. |
| **SLSA / provenance systems** | Build provenance and supply-chain integrity metadata | Complementary. Provenance answers how/where an artifact was produced; runtime behavioral integrity answers whether the observed execution surface matches what was accepted. |

## What ExecSurface deliberately does not compete on

ExecSurface is not intended to become:

- antivirus or a malware classifier;
- a SIEM;
- a general-purpose EDR;
- a generic host telemetry platform;
- a broad kernel event collector for feature parity;
- a replacement for artifact signing, transparency logs or build provenance;
- a proof that a program is safe.

Falco and Tetragon already cover broad runtime-detection/observability territory. Sigstore and SLSA cover different supply-chain identity/provenance questions. ExecSurface should interoperate where evidence supports composition rather than duplicate mature systems merely for feature count.

## Where composition is useful

A longer-term evidence composition may look like:

```text
source / artifact / workflow identity
                |
                v
        provenance / signing
       (SLSA / Sigstore etc.)
                |
                v
      runtime evidence backend
   (ptrace today; other sources only
      where semantics are proved)
                |
                v
      canonical execution surface
                |
        accepted vs current
                |
                v
       deterministic drift
                |
                v
        explicit policy
                |
                v
 PASS / REVIEW / BLOCK / ERROR
                |
                v
      attestable result
```

Backend identity alone never grants authority. Any future external trace or kernel backend must be qualified proposition-by-proposition for authority, completeness, loss and semantic compatibility before it can support the same verdict claims.

## Current Alpha.6 boundary

Alpha.6 currently provides public distribution through:

- checksum-verifiable GitHub Release artifacts for Linux x86_64;
- `cargo install execsurface --version "=0.1.0-alpha.6" --locked`;
- GitHub Action `AETHERXGLOBAL/execsurface@v0.1`;
- immutable Action pin `AETHERXGLOBAL/execsurface@v0.1.0-alpha.6`.

The public release has internal release, reproducibility, compatibility, artifact-binding and consumer-path evidence. Those results are not independent external validation. External review remains open in [Issue #118](https://github.com/AETHERXGLOBAL/execsurface/issues/118).

## Official references for adjacent projects

These links are provided so readers can verify adjacent-project capabilities from their own documentation:

- Falco documentation: https://falco.org/docs/
- Falco supported events: https://falco.org/docs/reference/rules/supported-events/
- Tetragon overview: https://tetragon.io/docs/overview/
- Tetragon enforcement: https://tetragon.io/docs/concepts/enforcement/
- Sigstore documentation: https://docs.sigstore.dev/
- SLSA specification: https://slsa.dev/

Project capabilities evolve. This page should be updated from primary sources rather than copied from secondary comparison articles.

## Challenge the positioning

If this boundary is wrong, redundant, too broad, or misses a materially overlapping system, that is useful evidence. Please report concrete counterexamples, no-fit findings, prior-art overlap or interoperability constraints through [Issue #118](https://github.com/AETHERXGLOBAL/execsurface/issues/118).

A negative or redundancy finding is not treated as a failed community interaction; it is evidence that should constrain the product direction.