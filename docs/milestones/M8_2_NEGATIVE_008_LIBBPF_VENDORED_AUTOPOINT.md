# M8.2 Negative Evidence 008 — libbpf vendored packaging requires explicit autopoint

Date: 2026-09-26
Status: **PRESERVED NEGATIVE EVIDENCE / HARNESS DEPENDENCY DISCOVERY**

## Experiment

Workflow: `M8 eBPF Packaging Probe`
Run: `36242295030`
Job: `108404954824`
Head: `33761d31b876e3ea873df67cb10eb3fbf4831841`

The isolated libbpf-rs packaging probe enabled `libbpf-rs/vendored` to test whether the final userspace binary could remove runtime dependencies on system `libelf`, `zlib`, and `zstd` while retaining Rust 1.82 compatibility and the M8.2 feasibility semantics.

The runner explicitly installed the declared native build toolchain including `build-essential`, `autoconf`, `automake`, `libtool`, `gettext`, `flex`, and `bison`.

## Failure

`libbpf-sys v1.7.0+v1.7.0` stopped during its build script with:

```text
autopoint is required to compile libbpf-sys with the selected set of features
```

The build script confirmed that the selected feature set enabled vendored and static libbpf, libelf, and zlib.

## Classification

This is **not** evidence that libbpf-rs cannot be vendored and it is **not** an ExecSurface semantic failure.

It is evidence that the vendored packaging path introduces an additional explicit build-time dependency that is not satisfied by installing `gettext` alone on the Ubuntu 24.04 runner image.

## Governance ruling

- Preserve this failed run.
- Add `autopoint` explicitly to the declared packaging-only build dependencies.
- Re-run the same packaging gate without weakening runtime dependency or semantic acceptance criteria.
- Do not add any of these native build requirements to the existing ptrace-only ExecSurface installation path.

The cost of the added build tool remains part of the final Aya vs libbpf-rs packaging comparison even if the corrected run succeeds.
