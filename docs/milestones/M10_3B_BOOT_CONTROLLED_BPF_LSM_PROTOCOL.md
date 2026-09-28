# M10.3B — Boot-Controlled BPF-LSM Environment Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103b`
Parent: `M10_3A_ALL_STANDARD_HOSTS_BLOCKED`
Parent HEAD: `ccc133f69b385e6062adb2db03285311e5d44e2b`
Status: **PREREGISTERED — ENVIRONMENT PROVISIONING ONLY / NO PRODUCT AUTHORITY**

## Objective

Determine whether a reproducible, boot-controlled Linux guest launched from a standard GitHub-hosted runner can expose the BPF LSM attachment surface required by the unchanged M10.3 `BPF-LSM-FILE-001` proposition.

This gate does **not** test object-identity correctness. It only establishes whether the required kernel environment can be provisioned without changing `main`, the public product, or the frozen M10.3 proposition.

## Fixed roles

- **Innovative Systems Architect** — find the smallest reproducible boot-controlled environment that exposes BPF LSM without widening product privilege requirements.
- **Anti-Deviation / Skeptical Reviewer** — reject tracepoint substitution, host-policy weakening, or interpreting environment availability as semantic correctness.

## Dynamic team

| Role | Responsibility |
|---|---|
| Linux Kernel Configuration Specialist | Verify `CONFIG_BPF_LSM` and boot-time LSM selection semantics. |
| QEMU / Boot Engineer | Build a minimal initramfs and deterministic serial-console probe. |
| eBPF Privilege Specialist | Distinguish guest-root feasibility from public-user deployability. |
| CI/Reproducibility Engineer | Preserve package/kernel identity, serial log, config and machine-readable result. |
| Security Boundary Reviewer | Ensure the experiment is confined to the disposable guest and introduces no enforcement. |
| Internal Red Team | Attempt to invalidate any `CAPABLE` result as host leakage, false parsing, or missing prerequisite. |

## Frozen environment design

Runner: `ubuntu-24.04`.

The workflow shall:

1. install QEMU, a static BusyBox, cpio and an Ubuntu generic kernel package;
2. select the installed generic kernel and matching config;
3. record kernel package/version and configuration;
4. require `CONFIG_BPF=y`, `CONFIG_BPF_SYSCALL=y`, and `CONFIG_BPF_LSM=y` before attempting the guest probe;
5. construct a minimal initramfs containing BusyBox and an `/init` probe;
6. boot the guest with QEMU TCG, not KVM-dependent acceleration;
7. pass `lsm=bpf` on the guest kernel command line;
8. mount proc, sysfs and securityfs inside the guest;
9. read `/sys/kernel/security/lsm` inside the guest;
10. emit a deterministic serial marker and machine-readable host-side result;
11. power off the guest immediately after the probe.

No host LSM setting is changed. No BPF program is loaded in M10.3B.

## Required evidence

Preserve:

- host `uname -a`;
- selected guest kernel path/version;
- selected kernel config;
- exact QEMU command line;
- complete guest serial log;
- guest active LSM list;
- whether kernel BTF is exposed in the guest when sysfs is mounted;
- final `result.json`;
- package-install/build logs as available.

## Frozen classifications

### `M10_3B_BOOT_CONTROLLED_CAPABLE`

Requirements:

- selected kernel config contains `CONFIG_BPF_LSM=y`;
- guest boots successfully;
- securityfs is mounted/readable;
- guest active LSM list contains `bpf`;
- serial probe reaches its completion marker.

This authorizes only the next gate: execute the **unchanged M10.3 proposition** inside a boot-controlled BPF-LSM guest.

### `M10_3B_KERNEL_CONFIG_BLOCKED`

The available packaged kernel lacks one of the frozen BPF/BPF-LSM configuration prerequisites.

### `M10_3B_BOOT_BLOCKED`

Kernel prerequisites exist but the minimal guest cannot boot to the completed probe under the frozen workflow.

### `M10_3B_LSM_ACTIVATION_BLOCKED`

Guest boots and securityfs is readable, but `bpf` is absent from the active LSM list despite the frozen `lsm=bpf` boot argument.

### `M10_3B_EVIDENCE_INCOMPLETE`

The workflow runs but evidence required to classify the environment is missing or ambiguous.

## Stop rule

After the first interpretable M10.3B result, record it before adding the M10.3 BPF program, target fixture, path reconstruction, lifecycle widening, or product integration.

## Hard non-authorizations

M10.3B does not authorize:

- changes to `main`;
- changes to `v0.1.0-alpha.3` or Marketplace behavior;
- replacement of ptrace;
- BPF-LSM `learn/check` or PASS authority;
- baseline interchangeability;
- backend auto-selection;
- a privileged daemon/service;
- enforcement behavior;
- claims that a QEMU-capable research environment is deployable to normal GitHub-hosted users.
