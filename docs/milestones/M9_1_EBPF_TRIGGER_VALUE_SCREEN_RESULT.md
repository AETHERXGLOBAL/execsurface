# M9.1 Triggered eBPF Same-Workload Value Screen — Result

Status: **KILLED FOR VALUE CLAIM / PARTIAL_NO_VALUE_CLAIM**

This record preserves the bounded eBPF value experiment permitted by the predeclared M6.5 trigger after M9.1 real-workload evidence crossed the ptrace-cost threshold. It does **not** authorize eBPF for public product use and does not change any backend authority.

## Fixed review roles

- Linux/eBPF Runtime Specialist — inspect collector lifecycle/completeness behavior.
- Performance/Reproducibility Engineer — enforce the frozen 3 warmup + 15 measured-sample protocol.
- Evidence Provenance Lead — bind results to exact source/workload/run/artifact identities.
- Innovation Scientist / Systems Architect — look for a real value signal without relaxing semantics.
- Deviation Prevention / Scientific Integrity — reject incomplete observations as performance evidence.
- Independent Red Team — attempt to expose any path that would convert incomplete eBPF evidence into a value or public-authority claim.

## Experiment identity

- workflow run: `36280366182`
- ExecSurface experiment source: `74737ac825c3d66d67d626b707aa5186a60e1759`
- host class: GitHub-hosted Ubuntu 24.04, Linux `6.17.0-1022-azure`, x86_64
- BTF `/sys/kernel/btf/vmlinux`: readable
- effective experiment UID: `0`
- protocol: 3 warmups per mode, then 15 measured samples per mode if all warmup health gates pass
- mode order: rotating `direct / ptrace / libbpf_per_invocation`
- outlier removal: none

## Immutable authority boundary

The generated evidence verified all of the following:

- `ptrace_correctness_reference = true`
- `full_surface_comparable = false`
- `ebpf_learn_authorized = false`
- `ebpf_check_authorized = false`
- `ebpf_pass_authorized = false`
- `backend_auto_select_authorized = false`
- `baseline_interchange_authorized = false`
- `public_integration_authorized = false`

These values are unchanged by the outcome of the experiment.

## EBPF-V1-JUST

Pinned workload:

- repository: `casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- command: `cargo +1.90.0 test --all >/dev/null 2>&1`

Direct warmups were clean at approximately `4.06–4.09 s`.

Ptrace warmups were all clean and complete:

- event count: `214281` each warmup
- command exit: `0`
- observation complete: `true`
- wall time: approximately `14.10–14.17 s`

The first `libbpf_per_invocation` warmup returned command exit `0` and `dropped_events = 0`, but the collector declared:

- `completeness = incomplete_lifecycle`
- `lifecycle_drain_complete = false`
- event count: `52813`

Therefore the warmup health gate failed before measured samples were collected.

Accepted experiment status: `PARTIAL_NO_VALUE_CLAIM`.

Evidence:

- artifact ID: `10918857441`
- artifact digest: `sha256:b35a3d33df190b9f59e60f75e731f55280a3d4ae08efee7167081a4151e2b59f`

## EBPF-V1-FZF

Pinned workload:

- repository: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- command: `go test ./... >/dev/null 2>&1`

Direct warmups were clean at approximately `0.437–0.446 s`.

Ptrace warmups were all clean and complete:

- event count: `8576–8604`
- command exit: `0`
- observation complete: `true`
- wall time: approximately `1.94–1.99 s`

`libbpf_per_invocation` warmup 1 completed lifecycle drain but declared `incomplete_capability`.

Warmup 2 then returned:

- command exit: `0`
- `dropped_events = 0`
- `completeness = incomplete_lifecycle`
- `lifecycle_drain_complete = false`
- event count: `4293`

Therefore the warmup health gate failed before measured samples were collected.

Accepted experiment status: `PARTIAL_NO_VALUE_CLAIM`.

Evidence:

- artifact ID: `10918064543`
- artifact digest: `sha256:889bd643dcb64950b8d0ce91a0452f9abb68bda38ef2753219b0f52bb35e86b4`

## Scientific conclusion

The predeclared trigger justified running the experiment. The experiment did **not** establish an eBPF performance-value claim because the current per-invocation libbpf path failed its own completeness/health gate on both trigger workloads before accepted measured samples could begin.

The correct result is therefore:

**KILLED FOR VALUE CLAIM at the current collector/lifecycle semantics.**

This is not evidence that eBPF is universally slower, nor that eBPF can never be valuable. It is evidence that the current research collector cannot support an accepted same-workload performance claim under the existing completeness rules.

## Consequences

1. Keep ptrace as the public correctness reference.
2. Keep eBPF research-only.
3. Do not rerun solely to obtain favorable timing numbers.
4. Any future eBPF value experiment must first close the observed `incomplete_lifecycle` / `incomplete_capability` health failures without weakening completeness.
5. Do not block M9.2 independent-adoption work on this research result.
6. Preserve this negative evidence alongside the earlier M8.6/M8.8 negative results.
