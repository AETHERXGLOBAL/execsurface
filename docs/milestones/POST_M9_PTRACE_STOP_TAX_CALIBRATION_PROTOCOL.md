# Post-M9 — Ptrace Stop-Tax Calibration Protocol

Date: 2026-09-27
Status: **PREREGISTERED — DIAGNOSTIC ONLY**
Tracking: GitHub issue #84
Parent protocol: `docs/milestones/POST_M9_PTRACE_COST_ATTRIBUTION_PROTOCOL.md`

## Purpose

The Phase A external measurements show a reproducible association between native ptrace overhead and trace-stop / ptrace-request density, but the parent protocol prohibits converting correlation into a causal claim.

This calibration isolates one mechanism prospectively: the cost added by forcing the current ptrace observer through a controlled number of additional syscalls that do not belong to the current ExecSurface authority-relevant observation set.

No public runtime behavior, observer semantics, evidence authority, baseline format, policy, verdict, privacy rule, supported platform, or release is changed by this experiment.

## Frozen synthetic workload

Compile one local diagnostic executable from a frozen C source. It accepts a non-negative integer `N` and executes exactly `N` raw Linux `getpid` syscalls via `syscall(SYS_getpid)` in a loop, with a volatile accumulator preventing removal of the loop.

`getpid` is intentionally outside the currently selected ExecSurface event families. The target therefore adds syscall-entry/exit trace stops without intentionally adding a new ExecSurface raw-event class.

The executable must emit no stdout/stderr during accepted timing samples and must exit 0.

Frozen N values:

- 0
- 1,000
- 5,000
- 10,000
- 20,000

No N value may be added, removed, or replaced after accepted measurements begin.

## Host and source freeze

For the accepted run record:

- record ExecSurface source SHA;
- record workflow run ID/attempt;
- record OS/kernel/architecture/CPU description;
- record compiler version;
- retain the exact synthetic C source in the artifact;
- build the current ExecSurface source without runtime-code modification.

## Timing protocol

For every frozen N on the same runner/job:

1. run 3 direct warmups;
2. run 3 current-ptrace warmups;
3. require every ptrace warmup to report `complete = true`, exit code 0 and no signal;
4. collect 7 direct measured samples and 7 current-ptrace measured samples;
5. alternate direct/ptrace ordering by sample index;
6. remove no clean sample;
7. record all failures/exclusions explicitly;
8. calculate median and MAD for each mode;
9. calculate median absolute overhead = ptrace median - direct median.

The timing samples must not be wrapped by `strace`.

## Trace-stop counter diagnostic

Separately from timing, run one diagnostic observation per N under `strace` of the ExecSurface observer process and count at minimum:

- `PTRACE_GET_SYSCALL_INFO` requests;
- `PTRACE_SYSCALL` resumes;
- total ptrace requests;
- wait4 calls;
- resulting ExecSurface raw-event count;
- warning count.

The diagnostic strace run is not a timing sample.

## Predeclared causal test

Let `G(N)` be the diagnostic count of `PTRACE_GET_SYSCALL_INFO` requests and `H(N)` the median absolute ptrace overhead in milliseconds.

The controlled stop-tax mechanism is labeled `MEASURED_CAUSAL_CALIBRATION` for this exact synthetic workload/host only if all of the following hold:

1. all accepted ptrace observations are complete and warning-free;
2. for each N > 0, `G(N) - G(0) = 2N` exactly, establishing that each added raw syscall creates the expected entry+exit syscall-info stops in the current observer;
3. the least-squares slope of `H(N)` versus N is positive;
4. the least-squares fit has `R^2 >= 0.98`;
5. no clean timing sample is removed.

If any condition fails, the result remains `PARTIAL` or `OPEN`; thresholds must not be changed after observing the data.

## Interpretation boundary

A passing calibration may establish only that, on the exact tested host and synthetic workload, controlled additional irrelevant syscalls cause additional current-ptrace stop/resume work and a near-linear increase in observer wall overhead.

It does **not** by itself establish:

- a universal Linux per-stop cost;
- that all external-workload overhead is caused by this mechanism;
- that metadata/path/fd bookkeeping is negligible;
- that any specific optimization is semantically safe;
- an eBPF value or authority claim.

The external `ripgrep` and `fzf` Phase A results remain separate evidence and are not rewritten by this calibration.

## Next gate if the causal test passes

A candidate optimization may be designed only in a separate prospective record. It must identify how it reduces irrelevant stop/resume work while preserving every authority-relevant observation and current fail-closed behavior. No observer implementation change is authorized by this protocol itself.

## Execution registration

The calibration workflow was added only after this protocol and all thresholds above were already committed. This note records the first executable run trigger after workflow registration; it changes none of the frozen N values, sample counts, thresholds, decision rules, or interpretation boundaries.
