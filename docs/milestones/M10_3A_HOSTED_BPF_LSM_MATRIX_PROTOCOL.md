# M10.3A — Hosted BPF-LSM Capability Matrix Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103`
Parent result: `M10_3_ENVIRONMENT_BLOCKED`
Status: **PREREGISTERED — ENVIRONMENT DISCOVERY ONLY**

## Objective

Determine whether any standard GitHub-hosted Linux runner currently exposes the kernel prerequisites required to execute the already-frozen M10.3 BPF-LSM file-object proposition without changing the proposition or public ExecSurface product.

This gate is environment discovery only. It cannot establish BPF-LSM object-identity correctness.

## Fixed roles

- **Innovative Systems Architect** — find the lowest-complexity reproducible environment that can execute the frozen BPF-LSM proposition.
- **Anti-Deviation / Skeptical Reviewer** — prevent interpreting runner availability as semantic correctness and prevent weakening the M10.3 protocol to fit a host.

## Dynamic team

| Role | Responsibility |
|---|---|
| Linux Kernel Configuration Specialist | Interpret active LSM and BPF prerequisites. |
| GitHub Actions Reproducibility Engineer | Probe only documented standard runner labels. |
| BPF Privilege Specialist | Record BTF, unprivileged BPF state and relevant capabilities without attempting privilege escalation. |
| CI Evidence Reviewer | Preserve a machine-readable record for every matrix leg. |

## Frozen runner matrix

- `ubuntu-22.04`
- `ubuntu-24.04`
- `ubuntu-26.04`

The matrix must not silently fall back from one label to another.

## Recorded facts per runner

Each leg records:

- requested runner label;
- `uname -a`;
- UID/GID;
- active LSM list from `/sys/kernel/security/lsm` when readable;
- whether `bpf` appears in the active LSM list;
- whether `/sys/kernel/btf/vmlinux` is readable;
- `/proc/sys/kernel/unprivileged_bpf_disabled` when readable;
- effective capabilities via `/proc/self/status`;
- final environment classification.

## Classifications

### `M10_3A_HOST_CAPABLE`

`bpf` is present in the active LSM list and kernel BTF is readable. This classification authorizes rerunning the unchanged M10.3 proposition on that runner label only. It does not authorize product use.

### `M10_3A_HOST_BLOCKED`

The runner does not expose `bpf` as an active LSM, or other required kernel discovery state is unavailable. This is deployment/portability evidence only.

### `M10_3A_MATRIX_MIXED`

At least one runner is capable and at least one is blocked.

### `M10_3A_ALL_STANDARD_HOSTS_BLOCKED`

Every successfully provisioned matrix leg is classified `M10_3A_HOST_BLOCKED`.

If a documented runner label itself cannot provision, that leg must be recorded separately rather than silently removed.

## Stop rule

After the matrix completes, record the result before attempting nested virtualization, a custom kernel, larger runner, or self-hosted infrastructure.

No changes are authorized to `main`, public runtime semantics, Marketplace behavior, public ptrace authority, PASS semantics, baseline comparability, or release packaging.
