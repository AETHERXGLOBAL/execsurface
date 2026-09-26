# M8.6 — Performance / Compatibility Gate

Date: 2026-09-26
Status: **CLOSED / ACCEPTED — EXACT-HOST MEASUREMENT ONLY**
Tracking: #44
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
Red-team: `M8_6_RED_TEAM_REVIEW.md`
Post-target root cause: `M8_6D_POST_TARGET_LATENCY.md`

## Objective

Measure the current experimental libbpf observer against direct execution and the ptrace correctness reference, identify the dominant performance costs, and record the exact Linux/kernel/BTF/privilege compatibility boundary supported by evidence.

M8.6 is a measurement gate. It does not authorize semantic shortcuts or eBPF PASS authority.

## Fixed governance roles

- **Innovation Scientist / Architect:** identify useful scaling and architecture signals without weakening evidence semantics.
- **Deviation Prevention / Scientific Integrity:** reject incomparable workloads, cherry-picked samples, hidden warm/cold mixing, loss-contaminated timing, or unsupported platform generalization.
- **Independent Performance / Compatibility Red Team:** attempt to falsify methodology, attribution, host claims, privilege assumptions, and performance interpretation.

## Authority boundary — unchanged

- ptrace remains the correctness reference;
- eBPF remains experimental observation-only;
- `full_surface_comparable=false`;
- `ebpf_pass_authorized=false`;
- cross-backend baseline interchangeability remains unauthorized;
- M8.4 loss/lifecycle semantics and M8.5 parity semantics remain authoritative.

## Frozen measurement protocol

The primary quantity is **end-to-end command observation latency** for one ExecSurface invocation, including observer setup, target execution, evidence drain, and process/report finalization.

Three modes use the same native fixture under the same timed root privilege context:

1. `direct` — target only;
2. `ptrace` — `execsurface observe -- TARGET ...`;
3. `libbpf` — experimental libbpf observation path.

Workloads:

- P1 short process/exec;
- P2 deterministic process tree;
- P3 128 successful open/read/close iterations;
- P4 1500 successful open/read/close iterations.

For each workload/mode:

- 2 untimed warmups;
- 11 timed samples;
- rotating mode order;
- no outlier deletion;
- raw samples retained;
- median, min, max, MAD and nearest-rank p90 derived deterministically.

A libbpf sample is eligible only with no hard incomplete state, no collector failure, `dropped_events=0`, and complete lifecycle drain. `incomplete_capability` remains expected and visible because the backend is intentionally partial.

## M8.6b — first executable benchmark

Evidence commit: `978fbd2b20fc08cb41b1c1f0b0c63a5364b0ab71`

- CI: **PASS** — run `36257784461`;
- M8.6 workflow: **PASS** — run `36257784487`;
- artifact ID: `10910658203`;
- artifact ZIP SHA-256: `024fbd714b3b0e2c452abe086ad181ff6da5bcb61178c2bb82205602b2221bf6`.

Exact measured host:

- Ubuntu 24.04.5 LTS;
- GitHub runner image `20260920.314.1`;
- kernel `6.17.0-1022-azure`;
- x86_64;
- readable `/sys/kernel/btf/vmlinux`;
- required current tracepoints present;
- privileged eBPF load/attach succeeds;
- unprivileged load is denied with EPERM before target start on this host/policy.

### End-to-end results from the first accepted run

| Workload | Direct median ms | ptrace median ms | libbpf median ms | ptrace/direct | libbpf/direct | libbpf/ptrace |
|---|---:|---:|---:|---:|---:|---:|
| short_exec | 3.315181 | 11.412272 | 607.646575 | 3.442428 | 183.292126 | 53.245013 |
| process_tree | 1.870716 | 7.112906 | 610.012217 | 3.802237 | 326.084888 | 85.761321 |
| file_open | 1.339436 | 22.556213 | 604.767881 | 16.840083 | 451.509352 | 26.811588 |
| high_event_rate | 6.810072 | 269.134424 | 619.185921 | 39.520056 | 90.922081 | 2.300657 |

**KILLED on this tested host:** the claim that the current per-invocation libbpf product path is already faster end-to-end than ptrace for these workloads.

This is not a claim that eBPF itself is intrinsically slower.

## M8.6c — phase attribution

Evidence commit: `d6d48356074390d9777d5878fa68fe92c26c266d`

- CI: **PASS** — run `36258238597`;
- M8.6 workflow: **PASS** — run `36258238632`;
- phase evidence used one shared monotonic-clock protocol with seven clean samples per mode.

Median phase timing:

| Mode | Pre-target ms | Target runtime ms | Post-target ms | Total ms |
|---|---:|---:|---:|---:|
| direct | 0.543662 | 1.907784 | 0.184495 | 2.653630 |
| ptrace | 2.504252 | 8.691757 | 0.580087 | 12.099102 |
| libbpf | 3.034996 | 1.947832 | **593.580342** | 598.906652 |

The measurement localized the dominant current libbpf regression to post-target lifecycle/finalization rather than target execution.

## M8.6d — isolated lifecycle root-cause probe

Evidence commit: `12b35dcdd294c5d971b9e61f6e856e1b44685624`

- CI: **PASS** — run `36258725637`;
- M8.6 workflow: **PASS** — run `36258725625`;
- raw artifact ID: `10911163713`;
- artifact ZIP SHA-256: `f1d60a740ce7a61df2c818ba916b00fd972281c7d9b3f509834d80029fb75150`.

The same run repeated phase attribution and measured a libbpf post-target median of `598.991069 ms`.

The exact observer skeleton was then measured independently over seven open/load/attach/detach cycles:

| Lifecycle phase | Median ms | Min ms | Max ms |
|---|---:|---:|---:|
| open | 0.122599 | 0.102962 | 0.134100 |
| load | 0.995327 | 0.830520 | 1.029451 |
| attach | 0.699074 | 0.659811 | 0.720674 |
| detach | **490.150901** | 454.182798 | 508.054501 |
| total lifecycle probe | 497.001462 | 461.006108 | 514.966226 |

The detach median is approximately 81.8% of the independently measured post-target median on that run. Because they are separate sample sets, this ratio is an attribution signal, not an exact additive decomposition.

### M8.6d conclusions

**KILLED:** BPF open/load/attach explains the ~600 ms floor.

**KILLED as primary optimization:** shorten lifecycle timeout/quiescence to chase the benchmark. It does not address the dominant measured cost and risks M8.4 semantics.

**MEASURED:** per-invocation skeleton/link teardown is the dominant isolated lifecycle cost on this exact host.

**SUPPORTED NEXT HYPOTHESIS:** persistent attachment can potentially amortize the measured teardown cost. This is not yet an approved product architecture.

See `M8_6D_POST_TARGET_LATENCY.md` for the mandatory persistent-session safety/evidence gates.

## Compatibility matrix

| Environment | Arch | Kernel | BTF | Current attachment points | Privileged collector | Unprivileged collector | Claim status |
|---|---|---|---|---|---|---|---|
| GitHub Ubuntu 24.04.5 image `20260920.314.1` | x86_64 | `6.17.0-1022-azure` | readable | present for current experimental program | PASS for load/attach/transport | EPERM before target start | **COMPATIBILITY_EVIDENCE — exact host only** |
| Other Linux kernels/distros/policies | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | **UNTESTED** |
| non-x86_64 | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | **UNTESTED** |

No broader Linux support claim is authorized by this matrix.

## Red-team closure

`M8_6_RED_TEAM_REVIEW.md` independently attacked:

- benchmark theater/cherry-picking;
- privilege asymmetry;
- conflating current architecture cost with intrinsic eBPF cost;
- incorrect attribution to load/attach;
- timeout-shortening as a benchmark shortcut;
- leaking links/resources to avoid detach;
- universal Linux compatibility claims;
- eBPF PASS promotion;
- treating persistent attachment as already approved.

Verdict: **ACCEPT M8.6 for narrow measured performance and exact-host compatibility evidence only.**

## Final M8.6 decision

**CLOSED / ACCEPTED.**

Established:

1. The current per-invocation libbpf path is slower end-to-end than ptrace on all four measured workloads on the tested host.
2. The dominant regression is post-target, not target runtime.
3. Per-invocation detach is the dominant isolated lifecycle cost measured on the tested host.
4. The current host supports the experimental collector only under the recorded privileged conditions.
5. Persistent attachment is justified as the next performance architecture hypothesis, but requires its own session-isolation, privilege, loss, lifecycle and crash/restart proof.

Not established:

- universal eBPF performance;
- universal Linux compatibility;
- production readiness;
- full semantic equivalence;
- eBPF PASS authority.
