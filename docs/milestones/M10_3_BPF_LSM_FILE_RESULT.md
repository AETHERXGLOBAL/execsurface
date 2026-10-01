# M10.3 — BPF-LSM-FILE-001 Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103`
Protocol: `docs/milestones/M10_3_BPF_LSM_FILE_PROTOCOL.md`
Source HEAD tested: `be42da05176d5fc25b2a4ce7d1ece3c90b45c6db`
Workflow run: `36423567136`
Artifact: `10969894969`
Artifact digest: `sha256:c0d693f0bba1231637ca4a991f167cfef0b53595be7a848813130795e59b25ba`

Status: **CLOSED FOR THIS ENVIRONMENT — M10_3_ENVIRONMENT_BLOCKED**

## Frozen classification

`M10_3_ENVIRONMENT_BLOCKED`

This is portability/deployment evidence only. It does **not** kill the BPF-LSM object-identity proposition and it does **not** promote BPF-LSM authority.

## What executed

The GitHub-hosted Ubuntu 24.04 workflow completed successfully as an evidence-collection workflow. The experimental Rust/libbpf component also built successfully (`build_exit_code = 0`).

The actual audit-only BPF-LSM proposition step was deliberately skipped by the preregistered environment gate because the runner did not expose the `bpf` LSM in its active LSM list.

Recorded environment:

- kernel: `Linux 6.17.0-1022-azure x86_64`;
- runner UID: `1001`;
- active LSMs: `lockdown,capability,landlock,yama,apparmor,ima,evm`;
- `bpf` LSM: **absent**;
- kernel BTF `/sys/kernel/btf/vmlinux`: readable;
- `/proc/sys/kernel/unprivileged_bpf_disabled`: `2`;
- isolated experimental build: **PASS**.

Because `bpf` was absent from the active LSM list, the workflow emitted the preregistered result:

```json
{
  "protocol": "M10_3_BPF_LSM_FILE_001",
  "classification": "M10_3_ENVIRONMENT_BLOCKED",
  "reason": "bpf_lsm_not_present_or_securityfs_lsm_unreadable",
  "active_lsms": "lockdown,capability,landlock,yama,apparmor,ima,evm"
}
```

## Anti-deviation review

A green GitHub Actions run is **not** interpreted as proposition success.

The scientific proposition was not executed, therefore none of the following is established:

- BPF-LSM `(s_dev, i_ino)` matches target `fstat` ground truth;
- BPF-LSM closes the M10.1 pathname ambiguity;
- BPF-LSM closes the M10.2 shared-FD attribution gap;
- BPF-LSM has PASS, `learn/check`, baseline or product authority.

The only accepted result is that the standard GitHub-hosted Ubuntu 24.04 environment used here does not expose the required BPF-LSM attachment surface under the recorded configuration.

## Engineering consequence

The M10.3 proposition remains **OPEN across a suitable BPF-LSM-enabled environment**.

The next implementation step must remain research-only and must obtain an explicit BPF-LSM-enabled kernel environment before rerunning the unchanged proposition. Environment provisioning must not weaken the frozen proposition, loss accounting, metadata-only boundary or audit-only behavior.

No change is authorized to:

- `main`;
- `v0.1.0-alpha.3`;
- Marketplace behavior;
- public ptrace `learn/check`;
- baseline comparability;
- public PASS semantics.
