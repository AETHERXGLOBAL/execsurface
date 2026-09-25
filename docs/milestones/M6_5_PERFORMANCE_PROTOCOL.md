# M6.5 — Performance / Backend Decision Protocol

Status: **CLOSED / ACCEPTED**

## Purpose

Measure the current ptrace reference observer before deciding whether an eBPF fast-path is justified.

This protocol does **not** make a low-overhead claim.

## Metrics

For each controlled command:

- direct wall-time median / p95 / min / max;
- observed wall-time median / p95 / min / max;
- median absolute observer overhead;
- median slowdown ratio;
- canonicalization wall time;
- observed event-count min / median / max.

## Controlled workloads

1. `/bin/true` — fixed-cost floor / tracer startup.
2. descendant spawn fixture — process lifecycle.
3. 128-file burst — syscall/event-volume sensitivity.

## Interpretation

Tiny commands are dominated by fixed tracer cost, so relative slowdown alone is not a production-backend criterion.

### Pre-M7 decision rule

Keep ptrace as the **reference correctness backend** for M7 unless either condition is reproduced:

1. event loss/incompleteness under the controlled stress workload; or
2. median absolute observer overhead exceeds 500 ms on the 128-file controlled burst.

If those conditions are not met, M7 proceeds with ptrace and records overhead on the external workload.

### eBPF authorization rule

A production eBPF fast-path is authorized when a real external workload reproduces material ptrace cost, defined for the next gate as either:

- median slowdown > 2x on a command whose direct median is >= 100 ms; or
- median absolute observer overhead > 500 ms.

This does not authorize weakening completeness semantics. Any eBPF backend must implement explicit lost-event accounting and equivalent privacy/comparability metadata before it can produce PASS.

## Why there is no automatic backend switch

The two backends have different completeness and loss modes. Performance alone cannot make their evidence interchangeable.

M4 comparability must continue to reject observations whose backend semantics differ.


## Final accepted measurement

Final branch commit:

`a5d454d39e5ad9ad56c4609c69d5b92846d36cc8`

Performance workflow:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36188370027

Shared-runner measurements:

| Workload | Direct median | Observed median | Median absolute overhead | Median slowdown | Median events | Canonicalization median |
|---|---:|---:|---:|---:|---:|---:|
| `/bin/true` | 0.375 ms | 1.435 ms | 1.060 ms | 3.827× | 6 | 0.004 ms |
| descendant spawn fixture | 0.956 ms | 4.301 ms | 3.345 ms | 4.499× | 19 | 0.023 ms |
| 128-file burst | 18.944 ms | 70.513 ms | 51.569 ms | 3.722× | 524 | 0.571 ms |

The shared runner is noisy. These values are engineering evidence, not a performance guarantee.

The controlled burst remained far below the pre-M7 500 ms absolute-overhead trigger and did not produce incomplete evidence.

Decision: retain ptrace as the M7 reference correctness backend and measure the first real external workload before authorizing an eBPF production fast-path.
