# ExecSurface — Current Status

Date: 2026-09-28

This file states the **current public project state**. Historical milestone documents and earlier roadmap entries remain evidence records and may name the release that was current when they closed.

## Current public state

- Public release: `v0.1.0-alpha.4`
- Public source / `main`: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable GitHub Action channel: `AETHERXGLOBAL/execsurface@v0.1`
- Public support scope: Linux x86_64
- Public default/reference observer: native `ptrace`
- Evidence/baseline contract: baseline v2 remains compatible; legacy schemas are not silently reinterpreted
- Verdicts / exit codes: PASS `0`, ERROR `2`, REVIEW `10`, BLOCK `20`

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

1. checksum-verifiable GitHub Release binary for `v0.1.0-alpha.4`;
2. crates.io package installation for Rust users;
3. GitHub Action stable channel `AETHERXGLOBAL/execsurface@v0.1`;
4. immutable Action pin `AETHERXGLOBAL/execsurface@v0.1.0-alpha.4`.

## Historical-roadmap note

`ROADMAP.md` is cumulative milestone history. Statements inside earlier closed milestones such as M9.3 that called alpha.3 the current release describe the state **at that milestone's closure** and are not current product status.

For current public facts, use this file, the README, the latest GitHub Release and `docs/releases/v0.1.0-alpha.4.md`.

## OpenSSF engagement

Current OpenSSF technical-engagement work is tracked in issue #94 and on branch `external/openssf-engagement-alpha4` until merged.

Primary proposed community path: OpenSSF ORBIT Working Group.

Secondary proposed path: Supply Chain Integrity Working Group for a specific provenance/attestation integration question only.
