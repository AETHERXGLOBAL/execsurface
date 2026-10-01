# M10.3B — Boot-Controlled BPF-LSM Environment Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103b`
Protocol: `docs/milestones/M10_3B_BOOT_CONTROLLED_BPF_LSM_PROTOCOL.md`
Status: **CLOSED — M10_3B_BOOT_CONTROLLED_CAPABLE**

## Accepted result

Accepted source SHA:
`5ff4e4c57ad5dc5dd3f8c98ba40dff4e20dde8fc`

Workflow run:
`36432629161`

Job:
`108962425459`

Artifact:
`10973883064`

Artifact ZIP digest:
`sha256:fc4a71808ac9356e3f0164adb801f5d8527b525bb542733c85de0e26dd81e19a`

## Frozen classification

`M10_3B_BOOT_CONTROLLED_CAPABLE`

The accepted run satisfied every frozen environment criterion:

- Ubuntu generic guest kernel: `6.8.0-142-generic`, package version `6.8.0-142.142`;
- matching `linux-buildinfo-6.8.0-142-generic` package used as the kernel-config evidence source;
- `CONFIG_BPF=y`;
- `CONFIG_BPF_SYSCALL=y`;
- `CONFIG_BPF_LSM=y`;
- guest boot completed under QEMU TCG;
- guest command line contained `lsm=bpf`;
- kernel log reported `LSM support for eBPF active`;
- securityfs LSM list reported `lockdown,capability,bpf`;
- guest BTF was readable;
- deterministic probe completion marker was reached.

The machine-readable result retained:

- `semantic_authority_granted=false`;
- `product_integration_authorized=false`.

## Preserved non-accepted attempts

### Attempt 1 — harness selection/permission artifact

Run `36427677800` did not test the frozen proposition because the harness selected an unsuitable host Azure kernel path and failed before a valid guest boot. It remains negative harness evidence only.

### Attempt 2 — parser framing artifact

Run `36428146126` booted the intended guest and raw serial evidence showed BPF LSM active, but `/sys/kernel/security/lsm` lacked a terminating newline and the parser concatenated the next marker. The malformed machine record was correctly rejected.

### Attempt 3 — dependency-install timeout

Run `36428560620` was cancelled while installing the Ubuntu generic-kernel dependency closure, including irrelevant firmware. It did not reach the frozen classification step.

### Harness repair A — image-only package without config evidence

Run `36432296047` proved that downloading only `linux-image-6.8.0-142-generic` avoids the large firmware dependency closure, but that package alone does not contain the kernel config required by the frozen gate. The emitted `M10_3B_KERNEL_CONFIG_BLOCKED` classification is not accepted as a scientific result because the config source was a harness omission.

The final accepted harness retained the same Ubuntu generic kernel while sourcing its matching config from the same-version `linux-buildinfo` package. No frozen scientific criterion was weakened.

## Decision

A reproducible boot-controlled Linux guest with BPF LSM active can be provisioned from a standard GitHub-hosted Ubuntu runner while leaving the host LSM configuration untouched.

This closes only the environment-provisioning question.

It authorizes the next research gate:

> Execute the **unchanged M10.3 BPF-LSM file-object proposition** inside the boot-controlled environment.

## Non-authorizations

This result does **not** authorize:

- changes to `main`;
- changes to `v0.1.0-alpha.3`;
- Marketplace behavior changes;
- replacement of ptrace;
- BPF-LSM `learn`, `check`, or PASS authority;
- cross-backend baseline interchangeability;
- automatic backend selection;
- a privileged host daemon/service;
- enforcement behavior;
- claims that ordinary GitHub-hosted jobs expose BPF LSM directly.
