# M8.6d — Post-target latency root-cause gate

Date: 2026-09-26
Status: **CLOSED — DOMINANT PER-INVOCATION DETACH COST MEASURED / PERSISTENT-SESSION FEASIBILITY NEXT**
Tracking: #44
Parent: `docs/milestones/M8_6_PERFORMANCE_COMPATIBILITY.md`

## Decision

The dominant fixed cost in the current experimental libbpf execution path is **not target runtime** and is **not BPF open/load/attach** on the tested host. The largest measured contributor is per-invocation eBPF skeleton/link teardown.

This finding does **not** authorize weakening lifecycle drain, loss accounting, semantic parity, privilege boundaries, or eBPF PASS authority.

## Fixed governance roles

- **Innovation Architect:** identify an architecture that amortizes lifecycle cost without weakening evidence semantics.
- **Deviation Prevention / Scientific Integrity:** reject timer shortening, incomplete drain, detached evidence, or benchmark-only shortcuts presented as product improvements.
- **Independent Red Team:** attack session isolation, privilege lifetime, event attribution, loss accounting, stale-event contamination, crash/restart behavior, and cleanup semantics.

## Evidence identity

Commit: `12b35dcdd294c5d971b9e61f6e856e1b44685624`

Workflow evidence:

- normal CI: **PASS** — run `36258725637`;
- M8.6 Performance Compatibility: **PASS** — run `36258725625`;
- raw evidence artifact ID: `10911163713`;
- artifact ZIP SHA-256: `f1d60a740ce7a61df2c818ba916b00fd972281c7d9b3f509834d80029fb75150`.

Measured host scope:

- Ubuntu 24.04.5 LTS;
- GitHub-hosted runner image `20260920.314.1`;
- Azure region `westcentralus`;
- kernel `6.17.0-1022-azure`;
- architecture `x86_64`;
- kernel BTF readable;
- privileged eBPF load/attach available;
- unprivileged load denied on this host/policy.

## Phase attribution

Median phase timing from seven clean samples:

| Mode | Pre-target ms | Target runtime ms | Post-target ms | Total ms |
|---|---:|---:|---:|---:|
| direct | 0.631275 | 2.330107 | 0.210678 | 3.168992 |
| ptrace | 2.456199 | 7.728608 | 0.649499 | 10.925994 |
| libbpf | 3.829123 | 2.653922 | 598.991069 | 605.486332 |

Interpretation:

- target-time libbpf instrumentation overhead is small in this specific fixture relative to ptrace;
- the current product-path regression is overwhelmingly post-target;
- this remains exact-host evidence, not a universal performance claim.

## Isolated skeleton lifecycle probe

The exact observer skeleton was opened, loaded, attached, and detached seven times with no target workload in the probe.

Median timings:

| Phase | Median ms | Min ms | Max ms |
|---|---:|---:|---:|
| open | 0.122599 | 0.102962 | 0.134100 |
| load | 0.995327 | 0.830520 | 1.029451 |
| attach | 0.699074 | 0.659811 | 0.720674 |
| detach | **490.150901** | 454.182798 | 508.054501 |
| total lifecycle probe | 497.001462 | 461.006108 | 514.966226 |

The independently measured detach median is approximately 81.8% of the independently measured libbpf post-target median on this run. Because these are separate sample sets, this percentage is an attribution signal rather than an exact additive decomposition.

The remaining difference is consistent with the existing fail-closed quiescence/drain/report path and must remain measured separately.

## KILLED hypotheses

### KILLED — shorten lifecycle timeout as the primary optimization

Reducing the lifecycle timeout or quiescence gate would not address the dominant measured cost and would risk weakening M8.4 lifecycle completeness semantics.

### KILLED — BPF load/attach is the fixed ~600 ms floor

On this host, open + load + attach are approximately 1.8 ms median in the isolated probe. They do not explain the observed ~600 ms end-to-end floor.

## Supported architecture direction

The evidence supports investigating a **persistent attached observer with bounded sessions** so BPF links are attached once and teardown cost is amortized outside individual command observations.

This is an architecture hypothesis, not an approved product design.

## Mandatory persistent-session gates

A persistent-session prototype must satisfy all of the following before it can influence the product path:

1. **Session attribution:** every accepted event must be attributable to the active root/descendant session; ambient host events cannot contaminate evidence.
2. **Session epoch isolation:** events from a previous session cannot be promoted into a later session after tracker reset.
3. **Drain semantics:** session completion must retain M8.4 fail-closed lifecycle/quiescence behavior; no fixed sleep may substitute for established completion.
4. **Loss accounting:** producer drops and event-budget truncation must be attributable to the correct session or must fail the session closed.
5. **Privilege lifetime:** long-lived BPF attachment privileges must be explicit, bounded, and never silently broaden CLI authority.
6. **No automatic PASS:** `ebpf_pass_authorized=false` remains unchanged.
7. **Crash/restart:** daemon/service failure must not yield a clean observation; restart creates a new epoch and invalidates ambiguous in-flight work.
8. **No stale maps/state:** per-session tracker state, pending events, and warning state must reset deterministically without resetting global loss evidence incorrectly.
9. **Concurrency boundary:** the first prototype is single-active-session only. Multi-session concurrency remains unsupported until independent attribution is proved.
10. **Packaging boundary:** the normal `cargo install execsurface --locked` path must remain unchanged during feasibility work.

## Next experiment — M8.6e

Build an isolated, non-product **persistent-session feasibility harness** that:

- opens/loads/attaches the current skeleton once;
- runs multiple sequential observation sessions without detaching between sessions;
- uses an explicit session epoch and root/descendant attribution;
- drains each session using the existing lifecycle rules;
- verifies zero producer drops for accepted clean samples;
- injects a stale-event/session-boundary counterexample and requires fail-closed behavior;
- measures per-session latency separately from one-time startup and final teardown;
- remains outside default workspace/product CLI authority.

Only after M8.6e passes may a persistent collector/service architecture be considered for a later product milestone.

## Authority boundary

Unchanged:

- ptrace remains the correctness reference;
- `full_surface_comparable=false`;
- `ebpf_pass_authorized=false`;
- cross-backend baseline interchangeability is unauthorized;
- no production-readiness claim is permitted from M8.6d.
