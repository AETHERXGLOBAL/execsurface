# M8.6 — Performance / Compatibility Gate

Date: 2026-09-26
Status: **OPEN — M8.6a PROTOCOL FROZEN / MEASUREMENT NEXT**
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

A bounded process-creation workload that repeatedly forks and execs `/bin/true`, then waits for clean child exit.

Purpose: expose observer overhead on process lineage + exec occurrence.

### P2 — process tree

A bounded multi-level or multi-child process workload with deterministic child count and clean waits.

Purpose: exercise parent/child tracking and lifecycle drain under a larger process graph.

### P3 — successful file opens

A bounded loop of `open/read/close` against `/dev/zero`.

Purpose: exercise successful-open event generation and userspace FD-path resolution.

### P4 — controlled high event rate

A larger but bounded open/read/close loop sized to generate materially more events while remaining below known loss/truncation thresholds on a clean accepted run.

Purpose: reveal scaling behavior and ring-buffer/consumer cost before the failure regime.

If P4 loses events on the measured host, that sample is not silently discarded. It is recorded as incomplete and excluded from clean overhead summaries; the clean-rate boundary is then reported for that host only.

## Sample protocol

For each workload and mode:

- perform **2 untimed warm-up invocations**;
- collect **11 timed samples**;
- execute modes in rotating order across repetitions so one mode is not always first or last;
- use a monotonic high-resolution clock around the complete child process invocation;
- retain every raw sample;
- do not remove outliers from the primary summary;
- calculate at minimum:
  - sample count;
  - median;
  - minimum;
  - maximum;
  - median absolute deviation (MAD);
  - p90 using an explicitly recorded deterministic rule.

One hosted-runner result is evidence for that runner only. It is not a universal Linux performance result.

## Warm / cold terminology

M8.6 uses:

- **first measured invocation** as a separately recorded first-run observation;
- **post-warmup repeated samples** as the primary repeated-invocation distribution.

It does not call the libbpf path a persistent warm backend because attachment/setup still occurs per invocation.

## Clean-sample health gate

A libbpf sample is eligible for a clean overhead summary only when its machine-readable report shows:

- no `incomplete_loss`;
- no `incomplete_limit`;
- no `incomplete_decode`;
- no `incomplete_collector`;
- no `incomplete_lifecycle`;
- no collector failure;
- `dropped_events == 0` where exposed.

`incomplete_capability` is expected for the current experimental backend because unsupported semantic classes remain declared. It does not by itself invalidate a performance sample for the capabilities the collector actually executes, but it must remain visible in raw evidence and cannot become PASS authority.

Ptrace samples likewise retain observation health metadata and cannot be used as clean performance evidence if the reference observer reports incomplete collection for the tested run.

## Output format

M8.6 measurement output must be machine-readable JSON containing:

- schema version;
- repository commit SHA;
- UTC timestamp;
- runner/host identity available to the workflow;
- OS release;
- kernel release;
- architecture;
- CPU model / logical CPU count when available;
- Rust/Cargo/clang versions;
- BTF presence and `/sys/kernel/btf/vmlinux` identity/hash when readable;
- privilege/effective capability context used by each mode;
- binary SHA-256 values;
- workload parameters;
- every raw timed sample;
- observer health for every observed sample;
- derived summaries.

A human-readable Markdown summary may be generated from the JSON, but JSON is the evidence source of truth.

## Relative metrics

Derived ratios may be reported only as measured quantities on the recorded host, for example:

- `ptrace_over_direct = median(ptrace) / median(direct)`;
- `libbpf_over_direct = median(libbpf) / median(direct)`;
- `libbpf_vs_ptrace = median(libbpf) / median(ptrace)`.

Do not translate these into universal claims such as “eBPF is N× faster” without broader evidence.

## Compatibility matrix

M8.6 records one row per actually tested environment with:

- distro / image identity;
- architecture;
- kernel release;
- BTF readable: yes/no;
- required BPF tracepoints / attachment points available: yes/no with exact names;
- libbpf observer build: pass/fail;
- unprivileged load/attach outcome;
- privileged load/attach outcome;
- event transport outcome;
- known capability limitations;
- exact workflow run / commit evidence.

Untested platforms are `OPEN`, not inferred compatible.

## Claim taxonomy

Use only:

- **MEASURED** — directly produced by the recorded benchmark run;
- **COMPUTED** — deterministic statistic derived from recorded raw samples;
- **COMPATIBILITY_EVIDENCE** — observed behavior on an exact tested environment;
- **OPEN** — not yet established;
- **KILLED** — a tested methodology or compatibility assumption was falsified.

## M8.6a acceptance

M8.6a closes when this protocol is committed before benchmark results are interpreted.

## M8.6b next

Implement native fixtures + machine-readable benchmark harness + GitHub Actions evidence on Ubuntu 24.04 without modifying the default product semantics or granting eBPF PASS authority.
