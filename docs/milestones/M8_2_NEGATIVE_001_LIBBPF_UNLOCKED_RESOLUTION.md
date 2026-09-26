# M8.2 Negative Evidence 001 — libbpf unlocked dependency resolution

Date: 2026-09-26
Status: **PRESERVED NEGATIVE EVIDENCE — SETUP/REPRODUCIBILITY DEFECT, NOT STACK KILL**
Workflow run: `36239268088`
Job: `libbpf-rs / build + lifecycle probe`

## What failed

The first clean Ubuntu 24.04 libbpf-rs feasibility build used Rust/Cargo 1.82 as intended, but the isolated probe had no committed lockfile and allowed broad transitive resolution.

Cargo resolved a 2026-era `clap_lex 1.1.1`. Its manifest requires Cargo's stabilized Edition 2024 support, so Cargo 1.82 failed before compiling libbpf itself:

```text
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.82.0 ...)
```

## Classification

- libbpf-rs/libbpf runtime or verifier failure: **NOT ESTABLISHED**
- ExecSurface experiment reproducibility defect: **PROVED**
- Product MSRV increase required: **NOT ESTABLISHED**
- Candidate killed: **NO**

## Corrective experiment

The upstream `libbpf-rs v0.27.0` workspace declares Rust 1.82 and its release lock resolves `clap 4.5.60` with `clap_lex 1.0.0`.

The follow-up experiment therefore aligns both `libbpf-rs` and `libbpf-cargo` to `0.27.0` and pins the relevant upstream-compatible clap dependency envelope. It will rerun under Rust 1.82.

This failure remains in history even if the corrective run succeeds.
