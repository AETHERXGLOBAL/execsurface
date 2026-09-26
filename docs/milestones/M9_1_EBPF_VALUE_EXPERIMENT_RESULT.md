# M9.1 — Same-Workload eBPF Value Experiment Result

Status: **PARTIAL / NO VALUE CLAIM — per-invocation candidate fails lifecycle completeness gate**

The experiment was preregistered before measurement because accepted M9.1 external workloads crossed the M6.5 ptrace performance trigger. The threshold and health gates were frozen before execution.

This result does **not** change backend authority.

## Candidate

Existing research-only per-invocation libbpf collector:

`experiments/m8-ebpf/libbpf-observer/target/release/execsurface-m8-libbpf-observer`

Modes were executed from the same root harness on GitHub-hosted Ubuntu 24.04:

1. direct;
2. ptrace `execsurface observe`;
3. per-invocation libbpf collector.

The frozen protocol required 3 clean warmups per mode before any 15-sample measured series could begin.

## Health rule

Every eBPF sample had to satisfy:

- process return code `0`;
- report present and parseable;
- `dropped_events == 0`;
- `lifecycle_drain_complete == true`;
- `collector_failure == null`;
- completeness not equal to a hard incomplete state including `incomplete_lifecycle`.

Any failure required `PARTIAL_NO_VALUE_CLAIM`, regardless of observed wall time.

## EBPF-V1-FZF

Pinned workload:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Command:

```text
go test ./... >/dev/null 2>&1
```

Result:

- direct warmups: clean, about `436.6–445.6 ms`;
- ptrace warmups: clean and complete, about `1939.96–1985.02 ms`, roughly `8,576–8,604` observed events;
- eBPF warmup #1:
  - return code: `0`
  - wall time: `1072.189955 ms`
  - `dropped_events=0`
  - `lifecycle_drain_complete=true`
  - `collector_failure=null`
  - `completeness=incomplete_capability`
- eBPF warmup #2:
  - return code: `0`
  - wall time: `3005.291339 ms`
  - `dropped_events=0`
  - `lifecycle_drain_complete=false`
  - `collector_failure=null`
  - `completeness=incomplete_lifecycle`

The second eBPF warmup failed the preregistered health gate, so no measured 15-sample series was started and no eBPF performance value claim is permitted.

Evidence:

- run: `36280366182`
- artifact ID: `10918064543`
- artifact ZIP SHA-256: `889bd643dcb64950b8d0ce91a0452f9abb68bda38ef2753219b0f52bb35e86b4`
- `value-screen.json` SHA-256: `ec9d68c980624e1725b67be80dc453735f93df3b8d03b0a981c5dc3afb0db6d3`
- status: `PARTIAL_NO_VALUE_CLAIM`

## EBPF-V1-JUST

Pinned workload:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Command:

```text
cargo +1.90.0 test --all >/dev/null 2>&1
```

Result:

- direct warmups: clean, about `4064.90–4094.15 ms`;
- ptrace warmups: clean and complete, about `14096.59–14169.53 ms`, exactly `214,281` observed events in each warmup;
- eBPF warmup #1:
  - return code: `0`
  - wall time: `6911.697939 ms`
  - `dropped_events=0`
  - `lifecycle_drain_complete=false`
  - `collector_failure=null`
  - `completeness=incomplete_lifecycle`
  - event count: `52,813`

The first eBPF warmup failed the preregistered health gate, so no measured 15-sample series was started and no eBPF performance value claim is permitted.

Evidence:

- run: `36280366182`
- artifact ID: `10918857441`
- artifact ZIP SHA-256: `b35a3d33df190b9f59e60f75e731f55280a3d4ae08efee7167081a4151e2b59f`
- `value-screen.json` SHA-256: `97814a293b911d80bc9eece99e069b94ad6f8d8a27f5413ab6b36670832c22b5`
- status: `PARTIAL_NO_VALUE_CLAIM`

## Interpretation

The timing observations before the health failure show why eBPF was worth testing: individual eBPF warmups could be materially faster than ptrace on these process-dense workloads.

However, the experiment was explicitly designed so speed cannot compensate for incomplete lifecycle evidence. Both real workloads produced `incomplete_lifecycle` before the measured series could begin.

Therefore:

- **no eBPF value claim is accepted from this experiment**;
- the existing per-invocation eBPF path is **not justified for public integration** by M9.1 evidence;
- no threshold is retuned and no samples are excluded;
- no pivot to another eBPF architecture is made post hoc inside this gate;
- any future eBPF work must begin as a new preregistered research gate that first closes lifecycle completeness.

## Authority decision

Unchanged:

- ptrace remains the correctness reference;
- `full_surface_comparable=false`;
- `ebpf_learn_authorized=false`;
- `ebpf_check_authorized=false`;
- `ebpf_pass_authorized=false`;
- backend auto-selection remains unauthorized;
- ptrace/eBPF baseline interchange remains unauthorized;
- public eBPF integration remains deferred.

M9.1 therefore closes its performance investigation with a clear result: ptrace has a real overhead pressure point, but the tested eBPF candidate does not meet the evidence-completeness gate required to act on the apparent speed advantage.
