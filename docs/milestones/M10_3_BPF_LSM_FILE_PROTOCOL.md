# M10.3 — BPF-LSM-FILE-001 Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103`
Parent research HEAD: `research/m10-hybrid-observer@e1d763cede420d78236446a687933e57daddecee`
Status: **PREREGISTERED — AUDIT-ONLY RESEARCH / NO PRODUCT AUTHORITY**

## Objective

Test whether an audit-only BPF LSM `file_open` hook can provide stronger kernel-object-grounded file evidence than the current ptrace pathname/fd model for a bounded proposition, without changing the public ExecSurface runtime.

The target proposition is deliberately narrow:

> For one session-registered process that opens one controlled file after an explicit start barrier, the BPF LSM event's `(s_dev, i_ino)` identity equals the target's independent `fstat(2)` ground truth for the successfully opened file, with zero unaccounted producer loss.

Passing this proposition does **not** authorize BPF-LSM `learn/check`, PASS, baseline interchangeability, backend auto-selection, enforcement, or public integration.

## Why this gate exists

M10.1 produced `PTRACE_PATH_TOCTOU_COUNTEREXAMPLE_OBSERVED`: entry-time pathname intent can differ from the pathname/object consumed by the kernel under concurrent userspace mutation.

M10.2 produced `PTRACE_FD_ATTRIBUTION_COUNTEREXAMPLE_OBSERVED`: accepted-complete ptrace evidence can omit successful reads under shared fd close/reuse concurrency.

M10.3 therefore tests an object-grounded kernel hook rather than another userspace snapshot.

## Fixed roles

- **Innovative Systems Architect** — seek stronger object-grounded evidence while keeping product-default selection separate.
- **Anti-Deviation / Skeptical Reviewer** — reject promotion merely because BPF-LSM is closer to the kernel; require explicit loss, privilege, licensing, and proposition evidence.

## Dynamic team for this gate

| Role | Responsibility |
|---|---|
| Linux LSM / VFS Specialist | Validate `file_open` hook semantics and object fields. |
| eBPF / libbpf Engineer | Build minimal CO-RE BPF LSM program and loader. |
| Kernel Object Identity Reviewer | Define the bounded `(s_dev, i_ino)` proposition and its limitations. |
| Concurrency / Race Reviewer | Ensure the barrier and ground truth do not recreate the ptrace race. |
| BPF Loss/Health Engineer | Account ring-buffer reservation failure and session membership. |
| Linux Privilege / Container Engineer | Record BPF-LSM availability and privilege requirements. |
| Licensing Reviewer | Preserve the mandatory GPL-compatible BPF-program boundary from Apache-2.0 userspace code. |
| CI/Reproducibility Engineer | Preserve kernel/LSM configuration, build logs, report and artifact digest. |
| Internal Red Team | Attempt to explain a PASS through accidental capture, wrong task membership, or hidden loss. |

## Architecture under test

```text
controlled target
   |
   | explicit SIGSTOP start barrier
   v
userspace loader registers target TGID -> session_id
   |
   v
BPF LSM: lsm/file_open  (audit only; preserve prior ret)
   |
   +--> current task/session membership check
   +--> struct file -> inode -> superblock object identity
   +--> ring buffer event
   +--> producer drop counter
   |
   v
userspace collector
   |
   v
compare BPF `(dev, ino)` against target `fstat` ground truth
```

## Licensing boundary

Linux requires BPF LSM programs to declare a GPL-compatible license. The experiment therefore keeps the BPF object as an explicitly separated research component with SPDX `GPL-2.0-only`. The Rust loader remains an isolated research userspace component and no GPL BPF object is added to the public ExecSurface release or crate in this gate.

This gate evaluates technical feasibility and evidence authority only; distribution/licensing integration remains a separate future decision.

## Environment gate

Before interpretation, record:

- `uname -a`;
- `/sys/kernel/security/lsm` when readable;
- presence of `/sys/kernel/btf/vmlinux`;
- `bpftool feature probe` summary when permitted;
- effective UID/capabilities relevant to loading/attaching BPF;
- whether BPF LSM attachment is available.

If the host does not expose the BPF LSM program type / `bpf` LSM, classify:

`M10_3_ENVIRONMENT_BLOCKED`

This is not a failure of the proposition and not evidence against the architecture. It is portability/deployment evidence.

## Controlled fixture

The fixture:

1. starts and immediately raises `SIGSTOP` before the tested open;
2. loader waits for the stop;
3. loader registers the fixture TGID in the BPF session map;
4. loader sends `SIGCONT`;
5. fixture opens one path supplied by the workflow;
6. fixture calls `fstat` on the returned fd;
7. fixture prints machine-readable ground truth containing raw `st_dev` and `st_ino`;
8. fixture exits normally.

The BPF program does not capture pathname bytes, argv, environment, file content, network payload, or arbitrary process memory.

## Event schema under test

Each accepted BPF event must include at minimum:

- `session_id`;
- `tgid`;
- `tid`;
- raw `s_dev`;
- inode number `i_ino`;
- file flags (diagnostic only).

## Health / completeness contract

The report must contain:

- `attachment_available`;
- `session_registered`;
- `target_exit_code` or signal;
- BPF event count for the registered session;
- producer drop count;
- decode error count;
- exact environment gate result.

A proposition PASS is forbidden if:

- producer drops > 0;
- decode errors > 0;
- session was not registered before the tested open;
- target did not exit normally;
- no unique relevant BPF event exists;
- `fstat` ground truth is missing;
- BPF LSM attachment availability was not established.

## Frozen classifications

### `M10_3_OBJECT_IDENTITY_MATCH`

All health gates pass and exactly one relevant BPF event matches target `fstat` on raw `(dev, ino)`.

Interpretation: **COMPUTATIONAL_EVIDENCE** that this bounded BPF-LSM proposition is object-grounded and closes the specific userspace-path ambiguity for this fixture.

### `M10_3_OBJECT_IDENTITY_MISMATCH`

All health gates pass but BPF `(dev, ino)` differs from target `fstat`.

Interpretation: **KILLED** for the proposed object-identity proposition until root cause is resolved.

### `M10_3_EVIDENCE_INCOMPLETE`

Attachment works but loss/decode/session/ground-truth conditions prevent interpretation.

Interpretation: no authority promotion.

### `M10_3_ENVIRONMENT_BLOCKED`

The host cannot load/attach BPF-LSM under the recorded configuration.

Interpretation: portability/deployment evidence only; proposition remains OPEN.

## Non-claims

Even a clean match does not prove:

- full path identity;
- mount-namespace stable path naming;
- read/write attribution;
- fd lifecycle equivalence;
- exec/network equivalence;
- zero loss under load;
- all-kernel portability;
- GitHub-hosted viability;
- lower end-to-end performance cost;
- public product readiness.

## Stop rule

After the first accepted result, record the outcome before widening scope. Do not add pathname reconstruction, descendants, enforcement, persistent daemon behavior, or public integration inside M10.3.
