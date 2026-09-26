# M8.6 — Independent Performance / Compatibility Red-Team Review

Date: 2026-09-26
Status: **ACCEPT — NARROW MEASUREMENT CLAIMS ONLY**
Tracking: #44

## Scope reviewed

The review covers M8.6a–M8.6d only:

- frozen benchmark protocol;
- same-host direct / ptrace / libbpf measurements;
- exact-host compatibility probe;
- monotonic phase attribution;
- isolated open/load/attach/detach lifecycle probe;
- preservation of M8.4/M8.5 authority boundaries.

It does not approve a persistent observer, production eBPF backend, automatic backend selection, or eBPF PASS authority.

## Attack 1 — benchmark theater / cherry-picked workload

**Result: NOT FOUND in the accepted evidence.**

The protocol was committed before interpretation, uses the same compiled fixture across modes, retains all raw samples, uses no outlier deletion, and reports median plus dispersion. The first result in which libbpf lost badly was preserved rather than replaced.

The measurements remain limited to four synthetic local workloads and one hosted environment.

## Attack 2 — hidden privilege asymmetry

**Result: CONTROLLED for timing; compatibility boundary remains explicit.**

The timed harness launches direct, ptrace, and libbpf from one root harness process. Privilege availability is measured separately. On the tested host, unprivileged eBPF load fails before target start with EPERM; privileged load/attach succeeds.

This is exact-host/policy evidence only.

## Attack 3 — claim that eBPF itself is intrinsically slow

**Result: REJECTED.**

M8.6c separates target runtime from process-lifecycle overhead. In the phase fixture, libbpf target runtime was close to direct execution while the dominant regression was post-target. Therefore the accepted claim is about the **current per-invocation architecture**, not eBPF as a technology.

## Attack 4 — attribution of the fixed floor to load/attach

**Result: FALSIFIED by M8.6d.**

The isolated lifecycle probe measured approximately:

- open median: `0.122599 ms`;
- load median: `0.995327 ms`;
- attach median: `0.699074 ms`;
- detach median: `490.150901 ms`.

The current ~600 ms end-to-end floor is therefore not explained by load/attach on the tested host. Per-invocation teardown is the dominant measured contributor.

The ~81.8% comparison between detach median and post-target median is an attribution signal from separate sample sets, not an exact additive decomposition.

## Attack 5 — optimize benchmark by shortening lifecycle drain

**Result: REJECTED.**

M8.4 established fail-closed lifecycle semantics. M8.6 must not weaken that contract for performance. The measured dominant cost is detach anyway, so shortening lifecycle timeout/grace would be both semantically risky and technically misdirected as the primary optimization.

## Attack 6 — avoid detach by leaking links/resources at process exit

**Result: REJECTED.**

Skipping explicit lifecycle cleanup, leaking BPF resources, or relying on uncontrolled process teardown is not an acceptable product optimization. Any amortization must come from an explicitly designed persistent lifecycle with bounded authority and session isolation.

## Attack 7 — infer universal Linux compatibility

**Result: REJECTED.**

Accepted compatibility evidence covers the exact tested Ubuntu 24.04 / x86_64 / kernel `6.17.0-1022-azure` hosted runner with readable BTF and the observed tracepoints. Untested kernels, architectures, distributions, container policies, seccomp/LSM policies, and capability configurations remain OPEN.

## Attack 8 — promote eBPF to correctness/PASS authority from parity + performance work

**Result: REJECTED.**

The following remain unchanged:

- ptrace is the correctness reference;
- `full_surface_comparable=false`;
- `ebpf_pass_authorized=false`;
- cross-backend baseline interchangeability is unauthorized;
- clone3 universal parity remains unresolved;
- experimental libbpf observations remain partial-capability evidence.

## Attack 9 — persistent observer as an implicit approved design

**Result: REJECTED.**

M8.6d supports persistent attachment as the next architecture hypothesis because it could amortize the measured teardown cost. It does not approve the design.

A separate milestone must prove at minimum:

- root/descendant session attribution;
- session epoch isolation against stale events;
- per-session loss accounting;
- fail-closed lifecycle completion;
- bounded privilege lifetime;
- crash/restart invalidation;
- deterministic state reset;
- no cross-session contamination;
- packaging and upgrade behavior;
- no automatic PASS promotion.

## Red-team verdict

**ACCEPT M8.6 for narrow measured performance and exact-host compatibility evidence.**

Accepted conclusions:

1. The current per-invocation libbpf path is slower end-to-end than ptrace on all four tested workloads on the measured host.
2. Target-time overhead in the phase fixture is small relative to the current post-target cost.
3. Per-invocation eBPF skeleton/link teardown is the dominant isolated lifecycle cost measured on this host.
4. A persistent attached observer is a justified next hypothesis for reducing the fixed floor, subject to a separate architecture/security/evidence milestone.

Not accepted:

- universal eBPF performance claims;
- general Linux compatibility claims;
- production readiness;
- superiority over ptrace;
- full semantic equivalence;
- eBPF PASS authority.
