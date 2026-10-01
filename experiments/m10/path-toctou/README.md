# M10 PATH-TOCTOU adversarial fixture

Status: **RESEARCH-ONLY**
Tracking: #88
Protocol: `docs/milestones/M10_1_PATH_TOCTOU_PROTOCOL.md`

This fixture tests one narrow question: can the pathname string copied by the unchanged ptrace observer at syscall entry differ from the A/B object the kernel actually opens when another thread mutates the same userspace pathname buffer during the ptrace stop/resume window?

## Files

- `path_toctou_target.c` — two-thread adversarial target with an atomic one-byte shared pathname.
- `crates/execsurface-observe/tests/m10_path_toctou.rs` — ignored research harness that repeatedly invokes the unchanged observer, decodes target exit-code ground truth, and writes machine-readable evidence.
- `.github/workflows/m10-path-toctou.yml` — dedicated runner for the frozen protocol.

## Ground truth

The isolated working directory contains:

- file `A` with byte `A`;
- file `B` with byte `B`.

The target exits:

- `11` when the successfully opened fd reads byte `A`;
- `12` when the successfully opened fd reads byte `B`.

This result is compared against the observer's relevant `FilePathAccess(Open)` path.

## Interpretation

A complete warning-free mismatch is a counterexample to universal pathname-authority for the tested ptrace proposition. It does not prove BPF-LSM correctness, real-world race frequency, exploitability, or authorization to alter the public backend.

No file in this experiment changes public runtime behavior.
