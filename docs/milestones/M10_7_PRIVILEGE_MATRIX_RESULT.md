# M10.7 — PRIVILEGE-MATRIX-001 Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m107`
Protocol: `docs/milestones/M10_7_PRIVILEGE_MATRIX_PROTOCOL.md`

## Classification

`M10_7_PRIVILEGE_MATRIX_CONFIRMED`

## Accepted execution

Workflow run: `36442056048`  
Job: `108994790907`  
Source SHA: `c992e81d1dada732a008ba25b34224fec71d5f19`  
Artifact: `10978956771`  
Artifact ZIP digest: `sha256:80cce53e0460a9f8152d49afb9e97b603705ba3574d79613ed5b271ba762fb32`

Workflow conclusion: `success`.

## Matrix evidence

### P1 — standard GitHub-hosted Ubuntu 24.04 host

- runner kernel: `6.17.0-1022-azure`;
- active LSM list: `lockdown,capability,landlock,yama,apparmor,ima,evm`;
- `bpf` LSM active: **false**;
- `/proc/sys/kernel/unprivileged_bpf_disabled`: `2`;
- root minimal `BPF_MAP_CREATE`: **success** (`bpf_rc=0`).

Interpretation: generic privileged BPF use can be available while BPF LSM itself is unavailable because it is not active in the host LSM stack. Generic BPF capability is therefore not equivalent to BPF-LSM deployment capability.

### P2 — default local container

Same static probe, no added capability or privilege:

- effective UID: `0` inside container;
- `BPF_MAP_CREATE`: **denied**;
- errno: `1` (`EPERM`).

Interpretation: container-local UID 0 alone is insufficient for this BPF operation under the default container boundary on the tested hosted runner.

### P3 — privileged local container

Same local image under `--privileged`:

- effective UID: `0`;
- `BPF_MAP_CREATE`: **success** (`bpf_rc=0`).

The host still lacks active BPF LSM, so this does not make BPF-LSM attach available as the current Marketplace default.

### P4 — boot-controlled guest root

Guest:

- kernel: `6.8.0-142-generic`;
- active LSM list: `lockdown,capability,bpf`;
- BPF LSM active: **true**;
- BTF readable: **true**;
- guest `/proc/sys/kernel/unprivileged_bpf_disabled`: `2`;
- root `BPF_MAP_CREATE`: **success** (`bpf_rc=0`).

This is consistent with the previously accepted real BPF-LSM file/exec/connect attachment evidence in the same controlled architecture family.

### P5 — boot-controlled guest unprivileged UID/GID

After dropping to UID/GID `65534`:

- `BPF_MAP_CREATE`: **denied**;
- errno: `1` (`EPERM`).

### P6 — boot-controlled guest user namespace

- `unshare(CLONE_NEWUSER)`: **success**;
- process identity inside the unmapped namespace appeared as overflow UID/GID `65534`;
- `BPF_MAP_CREATE`: **denied**;
- errno: `1` (`EPERM`).

No claim is made that this exact errno or identity representation is universal across Linux configurations.

## Frozen evaluator output

`M10_7_PRIVILEGE_MATRIX_CONFIRMED`

The evaluator also retained:

- `public_default_authorized=false`;
- `product_integration_authorized=false`.

## Interpretation

The matrix confirms the architectural split that M10 must preserve:

1. **Evidence authority:** boot-controlled BPF-LSM/kernel-hook observation can provide stronger bounded kernel-object evidence than current ptrace for selected file/exec/connect propositions.
2. **Default deployability:** standard GitHub-hosted runners do not currently expose BPF LSM in the active security-module stack, and default containers deny even the minimal BPF operation tested here.

Therefore semantic superiority for selected propositions does **not** justify replacing the current portable public backend with a mandatory BPF-LSM backend.

A managed/self-hosted or otherwise boot-controlled environment can support the stronger kernel-hook path, subject to explicit privilege, kernel, loss, and capability requirements.

## Product consequence

For the current product line:

- BPF-LSM MUST NOT silently become the Marketplace default;
- ExecSurface MUST NOT silently elevate privilege or install a privileged service;
- ptrace cannot continue to be described as universally kernel-authoritative for pathname/fd identity;
- a future hybrid backend, if exposed, must be capability-versioned and explicitly selected/diagnosed;
- unsupported BPF-LSM capability must not degrade into a false-equivalent baseline or PASS authority.

## Decision

M10.7 is **CLOSED / MATRIX CONFIRMED**.

This completes the mandatory M10 gates and authorizes **M10 architecture closeout only**. It does not itself authorize product integration.

No change to `main`, `v0.1.0-alpha.3`, Marketplace behavior, public ptrace authority, or existing baselines is made by this result.
