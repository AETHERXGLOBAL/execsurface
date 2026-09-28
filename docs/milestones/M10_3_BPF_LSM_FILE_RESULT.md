# M10.3 — BPF-LSM-FILE-001 Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer`
Protocol: `docs/milestones/M10_3_BPF_LSM_FILE_PROTOCOL.md`
Accepted source SHA: `8bb035b97d179c0820d8e27154ee098c74478aa8`
Workflow run: `36426361362`
Artifact: `10971299575`
Artifact digest: `sha256:3bb34d57837645ce2501b8a56eee3692b3dde98b41b209a484331b808dc5c1ff`
Kernel: `6.17.0-1022-azure`

Status: **CLOSED FOR GITHUB-HOSTED RUNNER — CAPABILITY UNAVAILABLE**
Classification: **`BPF_LSM_FILE_HOST_UNAVAILABLE`**

## Decision

The preregistered M10.3 semantic proposition was **not executed** on the tested GitHub-hosted Ubuntu 24.04 runner because BPF LSM was not enabled in the active Linux Security Module list.

This is a deployment/capability result, not a semantic counterexample to BPF LSM and not evidence that the prototype is correct.

No product/runtime semantics changed.

## Preserved host evidence

The accepted workflow recorded:

```text
kernel=6.17.0-1022-azure
uid=1001
btf_vmlinux=yes
lsm_list=lockdown,capability,landlock,yama,apparmor,ima,evm
```

Frozen classification output:

```text
classification=BPF_LSM_FILE_HOST_UNAVAILABLE reason=bpf_not_in_active_lsm_list lsm_list=lockdown,capability,landlock,yama,apparmor,ima,evm
```

Because `bpf` is absent from the active LSM list, the workflow intentionally skipped dependency installation, BPF program build, attachment, and semantic interpretation. This prevents an unsupported host from being mislabeled as a BPF-LSM semantic failure.

## Earlier infrastructure attempts

Three earlier workflow attempts failed during the build-dependency/bpftool bootstrap before executing the scientific gate. They are retained as infrastructure failures and are not interpreted as architecture evidence.

The accepted run moved the host-capability preflight ahead of toolchain bootstrap. That change did not alter the preregistered proposition or success/kill criteria; it prevented irrelevant build setup from obscuring host capability.

## What this establishes

Within the exact tested GitHub-hosted environment:

- `/sys/kernel/btf/vmlinux` is available;
- the active LSM stack does not include `bpf`;
- therefore an actual BPF-LSM `file_open` authority experiment cannot be executed on that runner without changing the boot/kernel LSM configuration;
- a BPF-LSM backend cannot currently be assumed to be a drop-in default for the existing GitHub-hosted Marketplace path.

## What this does NOT establish

This result does not establish:

- that BPF LSM is semantically better or worse than ptrace;
- that the audit-only prototype loads correctly on a BPF-LSM-enabled host;
- file-object authority parity;
- event-loss behavior;
- performance;
- public-product suitability on self-hosted or differently configured Linux systems;
- any reason to change current `v0.1.0-alpha.3` behavior.

## Architecture consequence

The hybrid architecture remains a research direction, but BPF-LSM authority and public portability must be treated as separate gates.

M10 should not replace ptrace merely because kernel hooks offer potentially stronger object authority. A production hybrid would need an explicitly supported host class or another kernel observation primitive that is available on ordinary hosted CI while preserving proposition authority.

The next work should therefore proceed on two tracks without changing the current product:

1. **Authority track:** execute the audit-only BPF-LSM prototype only on a host where `bpf` is genuinely active in the LSM list; do not emulate or silently substitute another hook.
2. **Portability track:** continue evaluating kernel primitives already available on GitHub-hosted Linux (stable tracepoints/fentry where applicable, fanotify/Audit as bounded complements) against the M10.1/M10.2 counterexamples.

## Product impact

**None.**

- `main`: unchanged by M10 research.
- `v0.1.0-alpha.3`: unchanged.
- Marketplace Action: unchanged.
- public ptrace backend: unchanged.
- baseline compatibility: unchanged.
- BPF-LSM PASS/learn/check authority: none.
- automatic backend selection: none.
