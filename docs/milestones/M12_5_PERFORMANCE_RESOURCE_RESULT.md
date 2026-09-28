# M12.5 — Performance / Resource Regression — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`
Status: **CLOSED**
Classification: **`M12_5_NO_GATE_BLOCKING_PERFORMANCE_RESOURCE_REGRESSION`**

## Team

Fixed roles retained:

- **Innovative Systems Architect** — required a paired exact-ref benchmark and added bounded resource sentinels rather than treating runtime timing as the only release cost.
- **Anti-Deviation / Skeptical Reviewer** — froze thresholds before measurement, required incomplete evidence to block PASS, prohibited microbenchmark-only conclusions, required event-shape sanity, and rejected universal performance claims from shared CI.

Dynamic specialists: Linux/ptrace performance, Rust runtime/build engineering, paired-CI methodology, resource measurement, and release engineering.

## Frozen references

- public main: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- tested portable candidate: `2d6b32c30c6062d7aea035d4a59a70ba834647f4`

The performance workflow was committed later, but both benchmark jobs checked out and built the exact frozen refs above.

## Preregistered thresholds

Before numbers were observed, `M12_5_PERFORMANCE_RESOURCE_PROTOCOL.md` froze:

- performance material signal for a nontrivial workload only if candidate observed median > `1.25×` main **and** absolute increase >= `1000 us`;
- two nontrivial material performance signals required for an environment-local block;
- RSS material signal only if ratio > `1.20×` and increase >= `16,384 KiB`;
- RSS can block only if the same workload materially regresses on both Ubuntu environments;
- CLI binary-size material signal only if ratio > `1.10×` and increase >= `524,288 bytes`;
- unexplained median event-count change requires review rather than accepting an apparent speedup.

## Primary workflow evidence

Workflow: `M12.5 Performance and Resource Regression`
Run: `36462911570`
Conclusion: **success**

Both matrix jobs passed build, paired workloads, evaluator, evidence validation, artifact upload, and rejection checks:

- Ubuntu 22.04 / kernel `6.8.0-1064-azure` / Intel Xeon Platinum 8370C;
- Ubuntu 24.04 / kernel `6.17.0-1022-azure` / AMD EPYC 7763.

### Ubuntu 22.04

Per-environment classification: `NO_PERFORMANCE_BLOCKING_REGRESSION`

| workload | main observed median | candidate observed median | ratio | delta | main/candidate max RSS KiB | event median main/candidate | material perf | material RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `/bin/true` | 1938 us | 1977 us | 1.0201× | +39 us | 2304 / 2304 | 8 / 8 | no (descriptive) | no |
| descendant spawn | 11591 us | 11020 us | 0.9507× | -571 us | 2304 / 2176 | 23 / 23 | no | no |
| burst-128 | 61190 us | 61706 us | 1.0084× | +516 us | 2560 / 2688 | 526 / 526 | no | no |

Performance material signals: **0**.
RSS material signals: **0**.
Event-shape review required: **false**.

CLI release-binary size:

- main: `1,824,624 bytes`
- candidate: `1,825,912 bytes`
- delta: `+1,288 bytes`
- ratio: `1.0007059×`
- material: **no**

### Ubuntu 24.04

Per-environment classification: `NO_PERFORMANCE_BLOCKING_REGRESSION`

| workload | main observed median | candidate observed median | ratio | delta | main/candidate max RSS KiB | event median main/candidate | material perf | material RSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `/bin/true` | 1910 us | 1934 us | 1.0126× | +24 us | 2448 / 2480 | 6 / 6 | no (descriptive) | no |
| descendant spawn | 5827 us | 6838 us | 1.1735× | +1011 us | 2420 / 2452 | 19 / 19 | no | no |
| burst-128 | 88333 us | 85087 us | 0.9633× | -3246 us | 2836 / 3052 | 524 / 524 | no | no |

Performance material signals: **0**.
RSS material signals: **0**.
Event-shape review required: **false**.

The Ubuntu 24.04 spawn median increased by 1011 us but the ratio was only ~1.1735×, below the preregistered `>1.25×` condition, so it is not a material signal under the frozen rule. This number is retained rather than hidden.

CLI release-binary size:

- main: `1,824,624 bytes`
- candidate: `1,825,976 bytes`
- delta: `+1,352 bytes`
- ratio: `1.0007410×`
- material: **no**

## Artifact provenance

Run `36462911570`:

- Ubuntu 22.04 artifact id `10987124566`, digest `sha256:c88f40db8b13d858a45ae924aac590443c30170f70812e848c86a96f7db332af`;
- Ubuntu 24.04 artifact id `10987239375`, digest `sha256:95360edf5e01932b397f1f9491e632c15085fe563936459d46f53a7437a310b0`.

Each artifact includes exact SHAs, environment evidence, raw benchmark JSON, GNU-time resource output, stderr/return-code records, binary sizes, and the frozen evaluator result.

## Interpretation

The exact-ref paired gate found no preregistered blocking performance or resource regression on either tested hosted-CI environment.

The candidate also preserved benchmark event-count medians for every paired workload in each environment, so the measured result is not explained by silently observing fewer events in these workloads.

The result does **not** claim the candidate is faster. The 22.04 burst workload was slightly slower while spawn was faster; the 24.04 spawn workload was slower while burst was faster. Shared GitHub runner noise and host/kernel differences remain material limitations.

GNU `/usr/bin/time -v` max RSS is a bounded process-level sentinel, not whole-system memory accounting. It cannot establish universal memory equivalence.

## Explicit non-claims

M12.5 does not prove:

- universal speed improvement;
- tail-latency guarantees;
- whole-system memory equivalence;
- lower production cost;
- performance across every kernel/container/namespace;
- production readiness;
- hybrid/BPF-LSM end-to-end performance.

## M12.5 decision

**`M12_5_NO_GATE_BLOCKING_PERFORMANCE_RESOURCE_REGRESSION`**

M12.5 is closed.

Next authorized gate: **M12.6 — Release artifact / provenance dry run**.