# M8.2 Negative Evidence 009 — vendored packaging requires Rust 1.82 rustfmt

Date: 2026-09-26
Status: **PRESERVED NEGATIVE EVIDENCE / HARNESS DEPENDENCY DISCOVERY**

## Experiment

Workflow: `M8 eBPF Packaging Probe`
Run: `36242381034`
Job: `108405189866`
Head: `e414dc73b4b78fba105c9caf7b528e0dead19bf9`

After explicitly adding `autopoint`, `libbpf-sys` successfully progressed through the vendored dependency build far enough for the ExecSurface feasibility probe build script to run.

## Failure

Skeleton generation then stopped with:

```text
error: 'rustfmt' is not installed for the toolchain '1.82.0-x86_64-unknown-linux-gnu'
Failed to rustfmt
```

The normal M8.2 libbpf feasibility workflow already installs Rust 1.82 with the `rustfmt` component. The isolated packaging workflow omitted that component, so this is a packaging-harness dependency discovery rather than a semantic or kernel-compatibility failure.

## Governance ruling

- Preserve the failed run.
- Install the Rust 1.82 `rustfmt` component explicitly in the packaging workflow.
- Re-run the same vendored-runtime acceptance criteria without weakening them.
- Keep this tool requirement isolated from the current ExecSurface ptrace-only installation path.

This additional build-time requirement remains part of the final packaging-complexity comparison even if the corrected run succeeds.
