# M10.3C — Boot-Controlled BPF-LSM File Authority Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103c`
Parent gate: `M10_3B_BOOT_CONTROLLED_CAPABLE`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Execute the **unchanged M10.3 `BPF-LSM-FILE-001` proposition** inside the reproducible boot-controlled BPF-LSM guest established by M10.3B.

This gate exists only because standard GitHub-hosted kernels do not expose BPF LSM directly. It must not weaken or reinterpret the original M10.3 success/kill criteria.

## Fixed roles

- **Innovative Systems Architect** — find the smallest isolated execution path that permits the already-frozen BPF-LSM proposition to run against a real kernel mediation hook.
- **Anti-Deviation / Skeptical Reviewer** — reject any change to the proposition, event identity, session model, loss accounting, or acceptance criteria merely to make the guest experiment pass.

## Dynamic specialists

- Linux LSM / BPF-LSM
- VFS / inode / superblock semantics
- libbpf / CO-RE / verifier
- static userspace / initramfs engineering
- QEMU / kernel boot
- event-loss and session-membership semantics
- CI/reproducibility
- internal red team

## Frozen proposition lineage

Authoritative parent protocol:
`docs/milestones/M10_3_BPF_LSM_FILE_PROTOCOL.md` from `research/m10-hybrid-observer`.

Frozen research prototype source identities imported **without semantic modification**:

- `experiments/m10/bpf-lsm-file/bpf_lsm_file.bpf.c` blob `5b457dbfa7d1f3e2979313a8e43c17ac3d360008`
- `experiments/m10/bpf-lsm-file/loader.c` blob `97d9f35cf5236544d79801a06c6946f8ffbd86b9`
- `experiments/m10/bpf-lsm-file/run.sh` blob `d18b1ff2606ac4cae92bf8980cc8e7c51ccc0e52`
- `experiments/m10/bpf-lsm-file/README.md` blob `8005bc20cf5b66862084ce6d7f46d534a75c272c`

Any semantic change to those files after preregistration invalidates the first accepted run and requires a recorded protocol amendment before interpretation.

## Environment

Reuse the accepted M10.3B environment model:

- GitHub-hosted `ubuntu-24.04` is only the outer build/boot host;
- Ubuntu generic guest kernel resolved from `linux-image-generic`;
- matching `linux-buildinfo` config evidence;
- require `CONFIG_BPF=y`, `CONFIG_BPF_SYSCALL=y`, `CONFIG_BPF_LSM=y`;
- QEMU TCG;
- guest command line includes `lsm=bpf`;
- guest active LSM list must contain `bpf`;
- guest BTF must be readable.

The BPF-LSM program executes **inside the guest**, not on the GitHub host.

## Build transport boundary

The outer host may compile/package the frozen experiment for the guest, but may not interpret host execution as BPF-LSM evidence.

Preferred transport:

1. build the frozen CO-RE BPF object and skeleton on the outer host;
2. link the frozen loader into a self-contained executable suitable for the minimal initramfs;
3. place the executable in the guest initramfs;
4. run it as guest root only after verifying BPF LSM active and BTF readable.

If static linking is not feasible, a reproducible minimal shared-library closure may be placed in the initramfs. Packaging changes are infrastructure-only and do not change the proposition.

## Frozen fixture and success criteria

The frozen loader creates two distinct files A/B, verifies their `(st_dev, st_ino)` independently, registers the stopped target TGID to a fixed research session, then releases the target to perform 100 deterministic opens/reads of each file.

`BPF_LSM_FILE_AUTHORITY_PASS` requires **all** original M10.3 conditions plus the frozen loader conditions:

- real `lsm/file_open` attachment;
- target exits `0`;
- exactly 100 accepted A-object events;
- exactly 100 accepted B-object events;
- zero unmatched/other object events;
- zero wrong-session events;
- zero recorded ring-buffer reservation drops;
- metadata-only boundary preserved;
- BPF program remains audit-only and preserves prior LSM denial/returns allow otherwise;
- no public ptrace implementation modification.

## Frozen classifications

The top-level scientific classification remains exactly one of the original M10.3 values:

- `BPF_LSM_FILE_AUTHORITY_PASS`
- `BPF_LSM_FILE_AUTHORITY_PARTIAL`
- `BPF_LSM_FILE_HOST_UNAVAILABLE`
- `BPF_LSM_FILE_COUNTEREXAMPLE`
- `BPF_LSM_FILE_INFRA_FAILURE`

Environment/harness facts may be recorded separately but may not invent a new semantic PASS class.

## Kill rule

Any accepted healthy run where the emitted kernel-object identity is inconsistent with the independently established A/B object truth kills the candidate for this proposition.

Any path that can lose selected events or misroute session events while still being labeled authoritative also kills the candidate.

## Stop rule

Record the first scientifically interpretable M10.3C result before starting EXEC, CONNECT, loss-forcing, product integration, or any change to public semantics.

## Hard non-authorizations

M10.3C does not authorize:

- changes to `main`;
- changes to public `v0.1.0-alpha.3`;
- Marketplace behavior changes;
- BPF-LSM public PASS or `learn/check` authority;
- backend auto-selection;
- baseline interchangeability;
- replacement/removal of ptrace;
- host-level privileged daemon/service;
- enforcement.
