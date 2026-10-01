# ExecSurface — Current Status

Date: 2026-09-28

This file states the **current public product state**. Historical milestone documents and earlier roadmap entries remain evidence records and may name the release that was current when they closed.

## Current public product state

- Current live `main`: resolve from the GitHub branch head at read time; documentation/evaluation commits may advance it independently of the immutable release source
- Current public release: `v0.1.0-alpha.5`
- Immutable alpha.4 release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable GitHub Action channel `AETHERXGLOBAL/execsurface@v0.1`: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Public support scope: Linux x86_64
- Public default/reference observer: native `ptrace`
- Evidence/baseline contract: baseline v2 remains compatible; legacy schemas are not silently reinterpreted
- Verdicts / exit codes: PASS `0`, ERROR `2`, REVIEW `10`, BLOCK `20`

The commits after the immutable alpha.4 source are currently documentation/evaluation/community-engagement changes. They do not silently move the stable Action channel or change alpha.4 runtime semantics.

## Current architecture boundary

ExecSurface is an observer-backed runtime evidence system with canonicalization, accepted baseline, deterministic diff and explicit drift policy evaluation.

It is not antivirus, EDR, malware detection, a sandbox, a general mandatory-access-control system or proof that software is safe.

Incomplete evidence cannot silently become PASS.

## Alpha.4 hardening

Alpha.4 changes the known clone/shared-FD false-completeness class to explicit incomplete / non-PASS-eligible evidence.

This is conservative hardening, not an exact shared-FD attribution repair. Some clone/thread concurrency may therefore be marked incomplete even when exact fd-table sharing is not proven.

Ptrace pathname observations are represented as pathname access-attempt metadata, not kernel-object identity.

## Kernel-hook / hybrid research

The ptrace-vs-LSM architecture review concluded `HYBRID_ARCHITECTURE_RECOMMENDED` for stronger authority-sensitive propositions.

BPF-LSM/kernel-hook work remains managed/research-only and non-default. Alpha.4 does not authorize automatic hybrid selection, new portable-path privilege requirements, public hybrid `learn/check`, or ptrace↔hybrid baseline interchangeability.

## Distribution

Current public distribution surfaces:

1. checksum-verifiable GitHub Release binary for `v0.1.0-alpha.5`;
2. crates.io package installation for Rust users;
3. GitHub Action stable channel `AETHERXGLOBAL/execsurface@v0.1`;
4. immutable Action pin `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`.

## Historical-roadmap note

`ROADMAP.md` is cumulative milestone history. Statements inside earlier closed milestones such as M9.3 that called alpha.3 the current release describe the state **at that milestone's closure** and are not current product status.

For current public facts, use this file, the README, the latest GitHub Release and `docs/releases/v0.1.0-alpha.5.md`.

## OpenSSF engagement

OpenSSF technical engagement is tracked in issue #94.

The public OpenSSF review pack and zero-assistance rehearsal evidence were merged through PR #95. The primary community path is the OpenSSF ORBIT Working Group; Supply Chain Integrity is secondary only for a concrete provenance/attestation integration question.

The OpenSSF/Linux Foundation email reply has been sent. A direct external GitHub issue attempt against `ossf/wg-orbit` was blocked by the connected GitHub integration's external-write permission (`403 Resource not accessible by integration`); that is a tooling limitation, not an ORBIT response or rejection.
