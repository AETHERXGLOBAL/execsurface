# M9-V1R — Same-Workload eBPF Value Experiment Result

Date: 2026-09-27
Status: **COMPUTATIONAL_EVIDENCE — NO E2E VALUE SIGNAL ON TESTED CLI WORKLOADS**
Run: `36280952967`
Research head: `ae133b5208abda348adcc96377310593f8b0eb57`
Current product baseline used by the experiment: `main@a77f7ed7b2bed381bcdfe95605e4cd57f368685a`

## Scope

This is a research-only post-M9.1 value screen. It does not reopen M9.1 and does not change backend authority.

V1 Run 1 (`36280685573`) is retained as `PARTIAL — command-transport harness defect`: all three workloads failed direct warmup 0 with rc=127 because shell `%q` escaping was written literally through `GITHUB_ENV`. Repair V1R was preregistered before rerun and changed command transport only.

## Frozen protocol

For each pinned workload on one GitHub-hosted Ubuntu 24.04 job:

- modes: direct, current ptrace, per-invocation libbpf, persistent two-session libbpf;
- 3 clean warmups per mode;
- 15 timed samples per mode;
- rotating mode order;
- no excluded samples or retries;
- all modes invoked from one root harness for identical privilege context;
- eBPF loss/lifecycle/integrity health gates retained;
- ptrace must remain `complete=true` with target exit 0;
- authority flags remain false for eBPF PASS/product integration.

## V1R-RIPGREP

Pinned upstream: `BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`
Command: `./target/release/rg --files . >/dev/null`

Artifact: `10918956945`
Artifact digest: `sha256:bef03241422ca007d13a7502c9d8e8e8a9aade5e5da7e904a5a47dab418c0da1`

All warmups and all 15 timed samples per mode passed health gates.

Medians:

- direct: `7.170477 ms`
- ptrace: `57.024246 ms`
- per-invocation libbpf: `612.832157 ms`
- persistent two-session amortized: `506.339876 ms`
- persistent internal session: `119.150627 ms`

Ratios vs ptrace:

- per-invocation libbpf / ptrace: `10.74687x`
- persistent amortized / ptrace: `8.87938x`

Value signal: **false** for both eBPF modes.

## V1R-JUST

Pinned upstream: `casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
Command: `./target/release/just --list >/dev/null`

Artifact: `10919011613`
Artifact digest: `sha256:73a970c1c5d21d0e026e4566b262bb12f298face9ef083284c8e0abaed451337`

All warmups and all 15 timed samples per mode passed health gates.

Medians:

- direct: `9.440203 ms`
- ptrace: `73.726749 ms`
- per-invocation libbpf: `616.085324 ms`
- persistent two-session amortized: `508.212034 ms`
- persistent internal session: `120.487089 ms`

Ratios vs ptrace:

- per-invocation libbpf / ptrace: `8.35633x`
- persistent amortized / ptrace: `6.89318x`

Value signal: **false** for both eBPF modes.

## V1R-FD

Pinned upstream: `sharkdp/fd@ce97e473ebaec49697c07daa50a7bc2b32f713d2`
Command: `./target/release/fd --hidden --type f --exclude target . >/dev/null`

Artifact: `10919385026`
Artifact digest: `sha256:2e1713481b8dc509fa74457a4b324aab983e6bd95523e789d74f2497ce48819d`

All warmups and all 15 timed samples per mode passed health gates.

Medians:

- direct: `12.549505 ms`
- ptrace: `89.334170 ms`
- per-invocation libbpf: `615.785132 ms`
- persistent two-session amortized: `513.0436745 ms`
- persistent internal session: `123.797323 ms`

Ratios vs ptrace:

- per-invocation libbpf / ptrace: `6.89305x`
- persistent amortized / ptrace: `5.74297x`

Value signal: **false** for both eBPF modes.

## Health evidence

Across all three workloads:

- all 3 warmups and all 15 samples per mode were clean;
- per-invocation eBPF reported no hard incomplete state in accepted samples;
- persistent sessions reported zero stale-epoch events, integrity errors, decode errors, producer drops, and routing errors;
- persistent membership and pending-mechanism maps were empty after sessions;
- final maps were empty;
- no unexpected events were accepted outside an active session;
- target outcomes were successful;
- eBPF authority remained unchanged.

## Interpretation

Within these exact-host CLI workloads, the tested research eBPF architectures are materially slower end-to-end than current ptrace. The persistent internal session time is lower than the full persistent end-to-end time, but the preregistered gate forbids using internal latency alone as a value claim.

This evidence therefore strengthens the existing M9.1 decision to keep eBPF research-only and public integration deferred.

The different absolute timings relative to the earlier M9.1 Batch 2R records do **not** rewrite those immutable records. Batch 2R used a different measurement/privilege harness including Python `subprocess.run(..., timeout=...)`; V1R used one root harness and direct `subprocess.run` without the timeout polling path. Cross-harness absolute timing is not treated as directly comparable.

## Authority decision

Unchanged:

- ptrace remains the correctness reference;
- `full_surface_comparable=false`;
- eBPF learn/check remain unauthorized;
- eBPF PASS authority remains unauthorized;
- backend auto-selection remains unauthorized;
- ptrace/eBPF baseline interchangeability remains unauthorized;
- public eBPF integration remains deferred.

No further eBPF experiment is required for M9.2. The product program should proceed to independent adoption evidence.
