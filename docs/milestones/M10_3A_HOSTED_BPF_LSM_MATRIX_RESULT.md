# M10.3A — Hosted BPF-LSM Capability Matrix Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103`
Protocol: `docs/milestones/M10_3A_HOSTED_BPF_LSM_MATRIX_PROTOCOL.md`
Source HEAD tested: `cc61fd687560dd4cff9fa2685d20c666bf014f56`
Workflow run: `36424752820`
Status: **CLOSED — M10_3A_ALL_STANDARD_HOSTS_BLOCKED**

## Classification

`M10_3A_ALL_STANDARD_HOSTS_BLOCKED`

All three preregistered standard GitHub-hosted Linux runner labels provisioned successfully and completed the frozen capability probe. None exposed `bpf` in the active LSM list.

## Results

| Runner | Active LSMs | BPF LSM active | Kernel BTF readable | unprivileged_bpf_disabled | Classification |
|---|---|---:|---:|---:|---|
| `ubuntu-22.04` | `lockdown,capability,landlock,yama,apparmor` | no | yes | `2` | `M10_3A_HOST_BLOCKED` |
| `ubuntu-24.04` | `lockdown,capability,landlock,yama,apparmor,ima,evm` | no | yes | `2` | `M10_3A_HOST_BLOCKED` |
| `ubuntu-26.04` | `lockdown,capability,landlock,yama,apparmor,ima,evm` | no | yes | `2` | `M10_3A_HOST_BLOCKED` |

Artifacts:

- `ubuntu-22.04`: artifact `10971391366`, digest `sha256:b9219ccccdebef1b3269e522d18769cf3dbef8ebdb9aafd3f2de908d42ea5d74`;
- `ubuntu-24.04`: artifact `10970697077`, digest `sha256:06e623aae02c6d6546561610ca2902b5d102d899848cbc21d2da90bb7c90e507`;
- `ubuntu-26.04`: artifact `10971660354`, digest `sha256:24e1c86deae869000d2d67901e3f04016c39c5b8659a096c25f40e532bfeef9d`.

## Interpretation

This result is **deployment/portability evidence**, not an architecture failure.

It establishes only that the tested standard GitHub-hosted runner matrix cannot execute the frozen M10.3 BPF-LSM proposition because the required `bpf` LSM is not active. Kernel BTF being readable does not compensate for an inactive BPF LSM attachment surface.

The result does not prove that:

- BPF-LSM is unavailable on Linux generally;
- BPF-LSM object evidence is correct or incorrect;
- a custom/self-hosted kernel is product viable;
- ptrace should be replaced;
- any public authority should change.

## Anti-deviation decision

Do **not** weaken M10.3 to tracepoints-only merely to obtain a green hosted-runner experiment. That would test a different proposition and would evade the external criticism rather than answer it.

Do **not** merge experimental BPF components into the public product merely because they compile on hosted runners.

## Next gate

Proceed to a separately preregistered environment-provisioning gate for the unchanged M10.3 proposition. Candidate environments must explicitly activate BPF LSM and preserve:

- audit-only behavior;
- object-grounded `(s_dev, i_ino)` proposition;
- target/session containment;
- producer-loss accounting;
- metadata-only privacy;
- no public product integration.

A custom/self-hosted or boot-controlled Linux kernel environment is therefore required before M10.3 semantic authority can be decided.
