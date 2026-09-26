# M9-V1R — Same-Workload eBPF Value Experiment Harness Repair

Date: 2026-09-27
Status: **FROZEN BEFORE REPAIR EXECUTION**
Parent protocol: `docs/milestones/M9_V1_EBPF_VALUE_EXPERIMENT.md`
Failed run retained: GitHub Actions run `36280685573`

## Why V1 Run 1 is PARTIAL

All three jobs completed source checkout, native dependency installation, frozen toolchain installation, the research-only persistent adapter, isolated eBPF lock resolution, product/eBPF builds, and exact pinned external workload preparation.

The first timed-protocol step then failed before any valid backend comparison. In all three cases, **direct warmup 0 returned 127**.

Artifacts:

- V1-JUST: artifact `10918387961`, digest `sha256:68ff0f39ce9fc1fb8f30d4ad39948bcb574bb8a3a42d83aafea4fbcb421ba764`
- V1-RIPGREP: artifact `10918427842`, digest `sha256:a4f2db65719ea1b2404a417569bbde94d81fb9114edb56e789ef3c877f26899d`
- V1-FD: artifact `10919105428`, digest `sha256:27c5cf81d50317fe8dfaa2ab48134175ee79e2561599dd03553ff43927ed890d`

The evidence JSON records the workload script with literal shell escape backslashes, for example:

`./target/release/rg\\ --files\\ .\\ \\>/dev/null`

The workflow had written `WORKLOAD_SCRIPT` to `GITHUB_ENV` using shell `printf %q`. GitHub environment-file transport preserves those backslashes as literal characters; it does not perform a later shell unescape. The measurement harness therefore received a different command string than the preregistered workload and attempted to execute an invalid command name.

This is a **measurement-harness command-transport defect**, not an ExecSurface, ptrace, libbpf, or persistent-observer result.

## Repair rule

V1R changes only command transport:

- do not transport `WORKLOAD_SCRIPT` through `GITHUB_ENV`;
- derive the exact frozen command locally inside the measurement step from the matrix project id;
- derive the deterministic `$RUNNER_TEMP` work root locally in the same step;
- invoke the unchanged `m9_v1_performance.py` harness with those exact strings.

No product code, eBPF program, adapter semantics, workload command, pinned revision, toolchain, host class, sample count, mode order, health gate, authority flag, or interpretation gate may change in V1R.

## Frozen V1R workloads

Unchanged from V1:

- JUST: `casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`; script `./target/release/just --list >/dev/null`
- RIPGREP: `BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`; script `./target/release/rg --files . >/dev/null`
- FD: `sharkdp/fd@ce97e473ebaec49697c07daa50a7bc2b32f713d2`; script `./target/release/fd --hidden --type f --exclude target . >/dev/null`

## Evidence discipline

- Run 1 is retained as `PARTIAL — harness defect` and is not overwritten or relabeled.
- V1R starts a new Actions run and new artifacts.
- If V1R fails for any new reason, preserve it; do not repair within the same run.
- Only a fully healthy 3+15 run may support a same-workload value conclusion.
