# M8.2 Negative Evidence 002 — libbpf transitive MSRV drift

Date: 2026-09-26
Status: **PRESERVED NEGATIVE EVIDENCE — DEPENDENCY-RESOLUTION OPERABILITY RISK**
Workflow runs: `36239433995`, `36239566175`

## What happened

After the first unlocked-resolution failure on a newer `clap_lex`, the experiment aligned `libbpf-rs` and `libbpf-cargo` to `0.27.0` and pinned the upstream release-line clap versions.

A later clean Rust/Cargo 1.82 resolution still selected `getrandom 0.4.3` through the broad `tempfile 3.27.0` constraint. Cargo 1.82 could not parse that dependency's Edition-2024 manifest, so the build failed before compiling libbpf itself.

Adding `getrandom = 0.3.1` directly to the experiment did not constrain the independent semver-compatible dependency edge; Cargo can retain more than one incompatible 0.x line in the graph.

## Evidence interpretation

- libbpf verifier/runtime defect: **NOT ESTABLISHED**
- libbpf source incompatibility with Rust 1.82: **NOT ESTABLISHED**
- unlocked modern resolution is insufficient for the declared Rust-1.82 experiment: **PROVED**
- a deterministic lock or MSRV-aware resolver is required for this candidate: **PROVED for this experiment**
- candidate killed: **NO**

## Corrective gate

The next run uses a newer Cargo only to create an MSRV-aware lock against the experiment's declared `rust-version = 1.82`, then performs the actual build with Rust/Cargo 1.82 using `--locked`.

If that path succeeds, the extra resolver/lock machinery remains a comparative packaging/reproducibility cost. If it fails, the failure is retained and the candidate may be deferred rather than forcing a product-MSRV increase.
