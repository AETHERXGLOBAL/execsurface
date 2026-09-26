# M8.2 Negative Evidence 010 — libbpf E9 file-event decoder gap

Date: 2026-09-26
Status: **CLOSED — HARNESS / USERSPACE SCHEMA GAP**
Tracking: #34

## Context

The libbpf-rs E9 file-metadata experiment added a third metadata event kind for successful `openat` exits while preserving the 16-byte metadata-only event record.

The kernel-side probe emitted the new event, but the first userspace run had not yet extended its decoder beyond the existing exec and lineage event kinds.

## Failing evidence

Commit under test:

`cbe5c5ed079830ca8084875f3526d22c0d40b8ab`

Workflow:

- `M8 eBPF Feasibility` — run `36242801388`
- job `libbpf-rs / build + lifecycle probe` — job `108406370665`

Earlier steps in the same job succeeded:

- declared native dependency installation;
- Rust-1.82-compatible dependency resolution;
- release build on Rust 1.82;
- explicit unprivileged denial;
- invalid-attach diagnostic.

The final semantic proof failed with:

```text
M8_LIBBPF_EVENT_DECODE_ERROR unknown process event kind: 3
Error: Error: Operation not permitted (os error 1)
```

## Root cause

The BPF-side E9 probe introduced `EVENT_FILE_OPEN = 3`, but the Rust userspace event decoder still accepted only the pre-E9 exec and lineage kinds. The ring-buffer callback therefore rejected a valid newly introduced metadata record before the E9 semantic assertions could complete.

This was a schema/harness synchronization defect. It was **not** evidence that libbpf could not emit the file metadata event, and it was not a kernel attach, verifier, privilege, loss-accounting, or privacy failure.

## Correction

The userspace decoder and statistics model were extended to recognize the third 16-byte metadata event kind, retain only process identity plus returned file descriptor metadata, and require a successful file-open event rooted at the controlled child process.

The workflow was also hardened to require the file-metadata PASS marker for both candidate stacks.

Correction commits include:

- `0acf114f24142a65be6a5397404a09e8bdb1b5d0` — prove libbpf file metadata semantic for E9;
- `6e4ba2dc0fde94c77ca63abbdf7e4eac9a9162f1` — enforce E9 file metadata gate for both stacks.

Passing replacement evidence:

- `M8 eBPF Feasibility` — run `36242866289` — **SUCCESS**;
- Host / BTF / privilege audit — **SUCCESS**;
- Aya / build + lifecycle probe — **SUCCESS**;
- libbpf-rs / build + lifecycle probe — **SUCCESS**.

## Governance classification

- Original E9 libbpf run: **KILLED AS VALID SUCCESS EVIDENCE**
- Root cause: **PROVED — userspace decoder/schema drift**
- Corrected E9 file-metadata semantic on reference runner: **PROVED**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace correctness reference: **RETAINED**

The failed run remains part of the evidence ledger and is not rewritten as a successful result.