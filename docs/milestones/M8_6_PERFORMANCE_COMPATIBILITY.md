# M8.6 — Performance / Compatibility Gate

Date: 2026-09-26
Status: **OPEN — M8.6a–M8.6b EVIDENCE RECORDED / M8.6c PHASE ATTRIBUTION NEXT**
Tracking: #44
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`

## Objective

Measure the current experimental libbpf observer against direct execution and the ptrace correctness reference, and define the exact Linux/kernel/BTF/privilege compatibility boundary supported by evidence.

This gate measures the implementation that exists. It does not authorize semantic shortcuts to improve a benchmark.

## Fixed governance roles

- **Innovation Scientist / Architect:** search for useful scaling and architecture signals without changing evidence semantics to win a benchmark.
- **Deviation Prevention / Scientific Integrity:** reject incomparable workloads, cherry-picked samples, hidden warm/cold mixing, loss-contaminated measurements, and unsupported platform generalization.
- **Independent Performance / Compatibility Red Team:** attempt to falsify methodology, statistical summaries, host claims, privilege assumptions, and any claimed performance advantage.

## Dynamic specialists

- Linux scheduler / process-performance specialist;
- eBPF / libbpf / ring-buffer specialist;
- Rust systems benchmarking specialist;
- BTF / CO-RE / kernel compatibility specialist;
- statistics / reproducibility specialist;
- CI / packaging / privilege-boundary engineer.

## Non-negotiable authority

- ptrace remains the correctness reference.
- eBPF remains experimental observation-only.
- `full_surface_comparable = false` remains unchanged.
- `ebpf_pass_authorized = false` remains unchanged.
- cross-backend learned-baseline interchangeability remains unauthorized.
- M8.6 performance evidence must not alter M8.4 loss semantics or M8.5 parity semantics.

## Measurement target

The primary quantity is **end-to-end command observation latency** for one ExecSurface invocation, including observer startup/attachment, target execution, evidence drain, and report serialization where applicable.

This is deliberately not labeled pure tracing cost.

Three modes are measured for the same compiled target binary:

1. `direct` — execute target with no ExecSurface observer;
2. `ptrace` — `execsurface observe -- TARGET ...`;
3. `libbpf` — `execsurface observe --backend experimental-libbpf --collector COLLECTOR -- TARGET ...`.

Because the experimental libbpf collector currently attaches/detaches per invocation, M8.6 must not claim a persistent-session steady-state cost that the product does not implement.

## Workload matrix

All performance fixtures are native, deterministic, local-only workloads. No network is required.

### P1 — short process / exec

Four fork + exec `/bin/true` operations with clean waits.

### P2 — process tree

A deterministic binary process tree of depth 2, with leaf execs and clean waits.

### P3 — successful file opens

128 `open/read/close` iterations against `/dev/zero`.

### P4 — controlled high event rate

1500 `open/read/close` iterations against `/dev/zero`.

If P4 loses events on a measured host, that sample is recorded as incomplete and excluded from clean overhead summaries. The first accepted run did not cross the known loss/truncation gate.

## Sample protocol

For each workload and mode:

- **2 untimed warm-up invocations**;
- **11 timed samples**;
- rotating mode order across repetitions;
- monotonic `perf_counter_ns` around the complete child invocation;
- every raw sample retained;
- no outlier removal;
- median, min, max, MAD, and nearest-rank p90 derived deterministically.

The timed benchmark harness runs as root and launches all three modes under the same EUID so privilege context is not a confounder in the timing comparison. Privilege availability is measured separately by the compatibility probe.

One hosted-runner result is evidence for that runner only. It is not a universal Linux performance result.

## Clean-sample health gate

A libbpf sample is eligible only when there is no hard incomplete state, no collector failure, `dropped_events == 0`, and lifecycle drain is complete.

`incomplete_capability` remains expected because the experimental backend deliberately supports a strict semantic subset. It remains visible in evidence and never becomes PASS authority.

Ptrace timing is eligible only when the reference observation reports `complete=true`.

## M8.6b — first executable measurement — RECORDED

### Evidence identity

Commit: `978fbd2b20fc08cb41b1c1f0b0c63a5364b0ab71`

Workflow evidence:

- normal repository CI: **PASS** — run `36257784461`;
- M8.6 Performance Compatibility: **PASS** — run `36257784487`;
- raw evidence artifact ID: `10910658203`;
- artifact ZIP SHA-256: `024fbd714b3b0e2c452abe086ad181ff6da5bcb61178c2bb82205602b2221bf6`.

Measured runner:

- Ubuntu 24.04.5 LTS;
- GitHub-hosted runner image `20260920.314.1`;
- Azure region `westcentralus`;
- kernel `6.17.0-1022-azure`;
- architecture `x86_64`.

### Exact-host compatibility evidence

- kernel BTF readable: **YES**;
- `/sys/kernel/btf/vmlinux` SHA-256: `95782433dc426daf8ebeb0acc1b04aed8f87168f67c1266f4d267aba7c22a8bf`;
- all currently required sched/syscall tracepoints were present on this host;
- privileged collector load/attach/transport: **PASS**;
- privileged probe: `incomplete_capability`, `dropped_events=0`, lifecycle drain complete;
- unprivileged collector load: **DENIED / EPERM** before target start, recorded as `incomplete_collector`;
- the unprivileged denial is compatibility evidence for this exact host/policy only, not a universal Linux privilege claim.

### Binary identities

- collector SHA-256: `fc6cf11458c0daeb9e5210336afec33740e533b72dce0a69646869177b5ff495`;
- ExecSurface CLI SHA-256: `a25945eb4f0fbf0bd15e8110f99a94e6146c9e0bb75c1a01e4c37a2d75a3c81a`;
- performance fixture SHA-256: `0192204d25787c82c0205248e3d69ca3792af675dfe3520bc62722ce7315e998`.

## M8.6b measured end-to-end results

These numbers are **MEASURED on the single exact runner above**. They include current per-invocation libbpf load/attach, target execution, post-target drain, and report handling. They are not pure kernel tracing overhead.

| Workload | Direct median ms | ptrace median ms | libbpf median ms | ptrace/direct | libbpf/direct | libbpf/ptrace |
|---|---:|---:|---:|---:|---:|---:|
| short_exec | 3.315181 | 11.412272 | 607.646575 | 3.442428 | 183.292126 | 53.245013 |
| process_tree | 1.870716 | 7.112906 | 610.012217 | 3.802237 | 326.084888 | 85.761321 |
| file_open | 1.339436 | 22.556213 | 604.767881 | 16.840083 | 451.509352 | 26.811588 |
| high_event_rate | 6.810072 | 269.134424 | 619.185921 | 39.520056 | 90.922081 | 2.300657 |

Dispersion remained narrow enough for all 11 samples per mode/workload to remain eligible under the frozen protocol. Examples:

- short_exec libbpf MAD `11.073613 ms`, p90 `621.520854 ms`;
- process_tree libbpf MAD `3.349683 ms`, p90 `616.327965 ms`;
- file_open libbpf MAD `10.358342 ms`, p90 `619.287508 ms`;
- high_event_rate libbpf MAD `9.898776 ms`, p90 `629.430842 ms`.

## M8.6b interpretation

### KILLED — naive current-path speed claim

**KILLED on this tested host:** the proposition that the current per-invocation experimental libbpf path is already faster end-to-end than the ptrace reference for these workloads.

It was slower on every tested workload, substantially so for short-lived workloads.

This result must not be hidden, averaged away, or replaced by a different benchmark protocol after seeing the numbers.

### OPEN — source of the fixed floor

The libbpf medians cluster around roughly `605–619 ms` while direct and ptrace cost scale much more visibly with workload size. That is strong evidence of a fixed per-invocation cost in the current architecture, but M8.6b does **not** yet identify whether the dominant source is:

- BPF object load/verifier/attach;
- process startup around the collector;
- target-time polling;
- post-root lifecycle/quiescence drain;
- report finalization;
- or a combination.

The source remains **OPEN** until M8.6c phase attribution measures it.

### MEASURED scaling signal, not a speedup claim

As event volume rises, the measured `libbpf/ptrace` ratio falls sharply—from ~85.8 on the small process-tree workload to ~2.30 on the 1500-open workload. This is a useful architecture signal but is not evidence of a crossover and is not evidence that eBPF is intrinsically slower or faster than ptrace.

The current benchmark measures the product path as implemented today: per-invocation attach/detach plus fail-closed drain semantics.

## Innovation direction after M8.6b

A persistent or preloaded eBPF observation service could potentially amortize the fixed setup/attachment cost, but that is **not yet an approved design**. Before implementation it must pass a separate architecture review covering:

- authority and privilege lifetime;
- target scoping / event attribution;
- cross-session contamination;
- BPF map/ring-buffer reset semantics;
- loss accounting per session;
- lifecycle completion and quiescence semantics;
- process namespace/cgroup/container boundaries;
- daemon crash/restart behavior;
- privacy and metadata retention;
- compatibility with current no-PASS authority.

M8.6c first measures phase attribution. Architecture changes follow evidence rather than precede it.

## Output format

Machine-readable benchmark evidence includes repository SHA, host identity, toolchain/BTF metadata, binary hashes, every raw timed sample with observer health, derived summaries, and computed ratios.

The JSON artifact is the evidence source of truth; this document records the reviewed interpretation.

## Compatibility matrix policy

M8.6 records one row per actually tested environment. Untested platforms remain `OPEN`.

The first matrix row covers only the exact Ubuntu 24.04 / kernel `6.17.0-1022-azure` GitHub-hosted environment above. Broader Linux compatibility remains open.

## Claim taxonomy

Use only:

- **MEASURED** — directly produced by recorded benchmark execution;
- **COMPUTED** — deterministic statistic derived from recorded samples;
- **COMPATIBILITY_EVIDENCE** — observed behavior on an exact environment;
- **OPEN** — not yet established;
- **KILLED** — a tested methodology or assumption was falsified.

## M8.6a conclusion

**CLOSED / ACCEPTED:** the measurement protocol was committed before result interpretation.

## M8.6b conclusion

**EVIDENCE RECORDED / NOT YET FINAL M8.6 CLOSURE.**

The first same-host benchmark and compatibility row are valid under the frozen protocol, and the current per-invocation libbpf speed advantage hypothesis is killed for the tested workloads/host.

M8.6 remains open because phase attribution, broader compatibility evidence, and independent Red Team closure are still required.

## M8.6c next

Measure the fixed libbpf floor by separating, using shared monotonic-clock markers where possible:

1. pre-target collector/setup time;
2. actual target runtime;
3. post-target drain/finalization time.

Do this without weakening or bypassing M8.4/M8.5 semantics.
