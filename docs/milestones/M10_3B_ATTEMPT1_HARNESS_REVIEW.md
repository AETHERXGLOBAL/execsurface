# M10.3B — Attempt 1 Harness Review

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103b`
Workflow run: `36427677800`
Source SHA: `7f85a889822b9c26cf45c2021370b529883fb387`
Artifact: `10972880684`
Artifact digest: `sha256:76766eb67c254aab9bfe4f3a1384ae01fee1d0474999261ce0a75a815658b5ad`
Status: **INVALID FOR ENVIRONMENT CLASSIFICATION — HARNESS ARTIFACT PRESERVED**

## Raw frozen evaluator output

The workflow produced:

`M10_3B_BOOT_BLOCKED`

with kernel prerequisites reported as present:

- `CONFIG_BPF=y`
- `CONFIG_BPF_SYSCALL=y`
- `CONFIG_BPF_LSM=y`

However, that classification is not accepted as evidence about guest bootability or BPF-LSM activation.

## Root cause

The selection logic used the highest versioned `/boot/vmlinuz-*` file rather than specifically selecting the generic kernel installed for this experiment.

It therefore selected:

`/boot/vmlinuz-6.17.0-1022-azure`

while the workflow had installed `linux-image-generic 6.8.0-142.142`.

QEMU exited before boot with:

`qemu: could not open kernel file '/boot/vmlinuz-6.17.0-1022-azure': Permission denied`

No guest serial probe marker was emitted.

## Scientific interpretation

This is a **harness failure**, not:

- evidence that a boot-controlled Linux guest cannot run on GitHub-hosted CI;
- evidence that BPF LSM cannot be activated;
- evidence against the M10.3 object-identity proposition;
- authorization to change the architecture or public product.

The raw artifact and frozen evaluator output are retained rather than rewritten.

## Corrective action allowed

A second attempt may change only environment harness mechanics:

1. select the `*-generic` kernel intentionally installed by `linux-image-generic`;
2. copy that kernel into the workflow workspace using root privilege and make the disposable copy readable by QEMU;
3. retain the same required config gate;
4. retain the same QEMU TCG model, `lsm=bpf` boot argument, serial markers, classifications and non-authorizations.

No success criterion or semantic proposition may be weakened.
