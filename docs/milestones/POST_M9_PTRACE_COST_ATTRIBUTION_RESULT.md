# Post-M9 — Ptrace Cost Attribution Phase A Result

Date: 2026-09-27
Status: **CLOSED — CAUSAL STOP-TAX COMPONENT ESTABLISHED; OPTIMIZATION NOT YET AUTHORIZED**
Tracking: GitHub issue #84
Authority: `docs/milestones/POST_M9_PTRACE_COST_ATTRIBUTION_PROTOCOL.md`
Calibration protocol: `docs/milestones/POST_M9_PTRACE_STOP_TAX_CALIBRATION_PROTOCOL.md`

## Decision

Phase A is closed for attribution only.

The evidence establishes two bounded facts:

1. on two pinned external runtime workloads, current native ptrace observation carries substantial exact-host wall-time overhead while remaining complete and warning-free; and
2. on one preregistered synthetic calibration, controlled addition of otherwise irrelevant `getpid` syscalls caused the exact expected increase in syscall-info stops and a near-linear increase in ptrace wall overhead.

This is sufficient to establish that all-syscall ptrace stop/resume work is a real causal component of the measured observer cost on the tested host. It is **not** sufficient to authorize a specific optimization, remove any event class, weaken entry/exit semantics, or generalize a per-stop cost to Linux as a whole.

No runtime code, public semantics, evidence authority, policy, verdict, privacy boundary, supported platform, release, or ptrace/eBPF authority changed in Phase A.

## External workload attribution

Accepted hardened diagnostic run:

- workflow: `Post-M9 ptrace cost attribution`
- run ID: `36287349422`
- source SHA: `cef7d8f0d44189efd8be5e81ad25b1e853fc9ef9`
- protocol: 3 warmups + 15 measured samples per direct/ptrace mode; no clean-sample removal

### PTRACE-COST-RG-001 — ripgrep

Pinned target:

`BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`

Timing on the exact accepted runner:

- direct median: `113.885308 ms`
- ptrace median: `514.720651 ms`
- median absolute overhead: `400.835343 ms`
- slowdown ratio: `4.519640505340689x`

Observer diagnostic counts:

- `PTRACE_GET_SYSCALL_INFO`: `20,193`
- `PTRACE_SYSCALL`: `20,627`
- total ptrace requests: `45,499`
- `wait4`: `20,732`
- `PTRACE_PEEKDATA`: `4,438`
- `readlink`: `653`
- ExecSurface raw events: `1,410`
- warnings: `0`

Artifact:

- artifact ID: `10921215244`
- digest: `sha256:de7b76048ff65f118f02f1a827502f36317a55572d7bbdb5d78c0fb145e951e3`

### PTRACE-COST-FZF-001 — fzf

Pinned target:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Timing on the exact accepted runner:

- direct median: `113.750175 ms`
- ptrace median: `364.251797 ms`
- median absolute overhead: `250.501622 ms`
- slowdown ratio: `3.2022086735251176x`

Observer diagnostic counts:

- `PTRACE_GET_SYSCALL_INFO`: `20,916`
- `PTRACE_SYSCALL`: `21,390`
- total ptrace requests: `46,957`
- `wait4`: `21,502`
- `PTRACE_PEEKDATA`: `4,396`
- `readlink`: `644`
- ExecSurface raw events: `1,398`
- warnings: `0`

Artifact:

- artifact ID: `10920996427`
- digest: `sha256:f1613275c330e588a7756c329768f559611daa1b69fa28fe073c05846e010b73`

## Diagnostic-duration boundary

The hardened diagnostic also recorded `strace -T` request durations. Those values are explicitly classified `DIAGNOSTIC_PERTURBED` because tracing the observer changes the measured path. They support decomposition/debugging only and are not treated as production causal wall-cost measurements.

Likewise, the large difference between observer request/stop counts and emitted raw-event counts is a measured structural fact, but it does not imply that every non-emitting syscall stop is safely removable. Some non-emitting operations may still participate in lifecycle, pairing, fd, path, signal, restart, or fail-closed semantics.

## Preregistered stop-tax calibration

Protocol was committed before execution with frozen values:

`N = 0, 1000, 5000, 10000, 20000`

Each additional operation was one raw `syscall(SYS_getpid)`, intentionally outside the selected ExecSurface event families.

Accepted run:

- workflow: `Post-M9 ptrace stop-tax calibration`
- run ID: `36287456574`
- run attempt: `1`
- source SHA: `8a5747549c4898bc3d3316832fe5f20ec0fb7e6d`
- artifact ID: `10920774100`
- artifact digest: `sha256:0ba36d738556f83d7ab98759022a77833270524b533b69db2fc2333fd1fc1600`
- classification produced by the frozen evaluator: `MEASURED_CAUSAL_CALIBRATION`

| N added `getpid` syscalls | `PTRACE_GET_SYSCALL_INFO` | median direct (ms) | median ptrace (ms) | median absolute overhead (ms) |
| ---: | ---: | ---: | ---: | ---: |
| 0 | 59 | 1.340063 | 3.515692 | 2.175629 |
| 1,000 | 2,059 | 1.338840 | 63.840399 | 62.501559 |
| 5,000 | 10,059 | 3.411488 | 264.271948 | 260.860460 |
| 10,000 | 20,059 | 3.408549 | 564.671129 | 561.262580 |
| 20,000 | 40,059 | 7.478846 | 1115.580676 | 1108.101830 |

All preregistered tests passed:

- all accepted ptrace observations complete and warning-free: **PASS**
- `G(N) - G(0) = 2N` for every `N > 0`: **PASS**
- positive least-squares overhead slope: **PASS**
- `R^2 >= 0.98`: **PASS**
- no clean timing sample removed: **PASS**

Frozen fit:

- slope: `0.05539044768140929 ms` per added `getpid` syscall
- equivalent exact-host calibration: `55.39044768140929 us` per added syscall
- intercept: `0.1691882938530398 ms`
- `R^2`: `0.9995502333567049`

## What is established

### PROVED / MEASURED within the declared scope

- The current ptrace observer performs entry/exit stop processing for added `getpid` syscalls even though those syscalls do not create a new ExecSurface raw-event class in the calibration.
- Each added raw syscall produced exactly two additional `PTRACE_GET_SYSCALL_INFO` requests in the accepted calibration.
- Controlled increases in those irrelevant syscalls caused a strongly near-linear increase in median ptrace overhead on the exact tested runner.
- Therefore all-syscall stop/resume processing is a causal contributor to current ptrace observer cost on that host/workload.

### OPEN / not established

- a universal Linux cost per syscall stop;
- the fraction of external `ripgrep`/`fzf` overhead attributable to this mechanism;
- whether path/fd bookkeeping is negligible;
- whether a selective-stop mechanism is semantically transparent;
- whether syscall-phase inference can safely replace any `PTRACE_GET_SYSCALL_INFO` call;
- whether a seccomp-assisted filter is acceptable given target-state/privilege semantics;
- any public performance improvement claim.

## Engineering consequence

The next step is **not** to remove syscall stops immediately.

A separate prospective optimization gate must first identify a mechanism, its target-state side effects, the complete authority-relevant syscall/lifecycle closure, and adversarial counterexamples. Runtime implementation is authorized only after that semantic design gate demonstrates that the candidate can reduce measured cost without hiding evidence or weakening fail-closed behavior.

Until such a gate passes:

- public release remains `v0.1.0-alpha.3`;
- native ptrace remains the correctness-reference and public backend;
- eBPF remains research-only;
- no runtime optimization claim is accepted.
