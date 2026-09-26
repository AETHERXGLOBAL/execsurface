# M9.1 — Same-Workload eBPF Value Experiment Pre-Registration

Status: **FROZEN BEFORE MEASUREMENT**

This experiment is opened because accepted M9.1 zero-contact evidence crossed the predeclared M6.5 real-workload trigger on two workloads:

- `casey/just` full `cargo test` workload;
- `junegunn/fzf` full `go test ./...` workload.

This satisfies M8.8 value trigger V1 and permits a same-workload eBPF value experiment. It does **not** authorize any public eBPF product surface.

## Authority remains unchanged

- ptrace remains the correctness reference;
- eBPF full-surface comparability remains `false`;
- eBPF `learn` remains unauthorized;
- eBPF `check` remains unauthorized;
- eBPF PASS authority remains unauthorized;
- backend auto-selection remains unauthorized;
- ptrace/eBPF baseline interchangeability remains unauthorized;
- this experiment cannot change those facts.

## Candidate under test

First screen only: the existing per-invocation libbpf research collector.

Reason: it already accepts an arbitrary command and has explicit completeness/loss fields. Do not modify the persistent observer architecture merely to create a favorable result. If the existing per-invocation path cannot show a useful end-to-end value signal on these long real workloads, further integration work is not justified by this screen.

## Frozen workloads

### EBPF-V1-JUST

Repository: `casey/just`

Revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Command:

```bash
cargo +1.90.0 test --all >/dev/null 2>&1
```

The repository checkout and dependency cache may be primed before timed samples, matching the practical warm-cache condition of the M9.1 performance series.

### EBPF-V1-FZF

Repository: `junegunn/fzf`

Revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`

Command:

```bash
go test ./... >/dev/null 2>&1
```

The checkout/cache may be primed before timed samples.

## Host / privilege protocol

- GitHub-hosted Ubuntu 24.04 runner, same host class as M9.1.
- `/sys/kernel/btf/vmlinux` must be readable.
- libbpf native dependencies are installed explicitly.
- direct, ptrace, and eBPF modes are all invoked from one root harness so privilege context is equal across timed modes.
- host/kernel/toolchain/binary hashes are recorded.

## Timing protocol

Modes:

1. direct;
2. ptrace `execsurface observe`;
3. per-invocation libbpf collector.

For each workload:

- 3 warmups per mode;
- 15 measured samples per mode;
- rotating mode order across repetitions;
- monotonic `perf_counter_ns` wall clock;
- no outlier removal;
- same command and working directory for all modes.

## eBPF health gate

Every eBPF warmup and measured sample must satisfy all of:

- process return code `0`;
- report file exists and parses;
- `dropped_events == 0`;
- `lifecycle_drain_complete == true`;
- `collector_failure == null`;
- `completeness` is not one of:
  - `incomplete_loss`
  - `incomplete_limit`
  - `incomplete_decode`
  - `incomplete_collector`
  - `incomplete_lifecycle`

Any failed health sample makes the workload screen `PARTIAL / NO VALUE CLAIM`.

## Predeclared value-screen threshold

A workload passes this **research value screen** only if all health gates pass and both conditions hold:

1. `median_eBPF <= 0.80 * median_ptrace` (at least 20% lower median wall time), and
2. `median_ptrace - median_eBPF >= 250 ms`.

This threshold is intentionally frozen before measurement. Passing it means only `VALUE_SIGNAL_FOR_FURTHER_RESEARCH`; it does not authorize public integration.

If the threshold is not met, record `NO_VALUE_SIGNAL` for this candidate path. Do not tune the threshold, remove samples, or change the workload after measurement.

## Outputs

For each workload preserve:

- exact repository revision;
- exact command;
- host/toolchain data;
- binary SHA-256 values;
- every warmup/sample wall time;
- every eBPF report health result;
- medians and ratios;
- predeclared threshold result;
- explicit authority block showing all eBPF product permissions remain false.
