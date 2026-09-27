# Post-M9 — Native ptrace Optimization Boundary Decision

Date: 2026-09-27
Status: **CLOSED — COST SOURCE ESTABLISHED / NO ACCEPTED >=10% TRANSPARENT PTRACE OPTIMIZATION**
Tracking: GitHub issue #84

## Decision

Close the current native-ptrace optimization search without merging Candidate A2 or Candidate B.

Retain native ptrace unchanged as the public correctness-reference backend.

Do **not** introduce seccomp-assisted selective syscall tracing into the current public ptrace backend as a transparent optimization. That mechanism changes tracee process state and therefore requires a separate architecture/product semantics gate rather than an optimization patch.

The next performance research gate should return to the existing eBPF research path and target the concrete completeness failures already observed there (`incomplete_lifecycle` / `incomplete_capability`) before any new eBPF value claim is measured.

No public runtime semantics, evidence authority, backend selection, baseline format, release, or eBPF authority changes are authorized by this decision.

## Evidence closed under #84

### 1. Causal stop tax established

`POST_M9_PTRACE_COST_ATTRIBUTION_RESULT.md` established on the frozen synthetic calibration that added irrelevant raw syscalls produced exactly two additional `PTRACE_GET_SYSCALL_INFO` requests each and a near-linear increase in ptrace wall overhead:

- slope: `55.39044768140929 us` per added raw syscall on the exact tested host;
- `R^2 = 0.9995502333567049`;
- all accepted observations complete and warning-free.

This establishes all-syscall stop/resume processing as a real causal cost component in the declared scope.

### 2. Candidate A2 rejected

A2 attempted to reduce observer-side syscall-info work without removing syscall stops.

Its external value gate did not establish value:

- pinned ripgrep: effectively zero improvement (`-0.0020677%` reduction; candidate slightly slower);
- pinned fzf: production semantic prerequisite failed closed before timing.

A2 remains unmerged.

### 3. Candidate B rejected for integration

Candidate B replaced bounded tracee-memory `PTRACE_PEEKDATA` reads with bounded `process_vm_readv` where safe and retained conservative fallback behavior.

Semantic and activation gates passed, including preserved privacy bounds, but the frozen external value gate failed the preregistered `>=10%` requirement independently on both targets:

- ripgrep: `0.001497928%` median reduction;
- fzf: `8.113117966%` median reduction.

Candidate B is therefore `SEMANTICALLY_VALID / VALUE_NOT_ESTABLISHED` and remains unmerged.

### 4. Why the next native lever is not a transparent patch

The current public observer explicitly uses `PTRACE_SYSCALL` and therefore requests a stop at syscall entry/exit while tracing. To avoid the majority of irrelevant syscall stops, the kernel must know which syscalls should notify the tracer before they execute.

Linux supports a seccomp-assisted route using `SECCOMP_RET_TRACE` plus `PTRACE_O_TRACESECCOMP`, but installing an unprivileged seccomp filter normally requires `PR_SET_NO_NEW_PRIVS=1` (unless the task has the required capability). `no_new_privs` is irreversible for the task, inherited by descendants, preserved across `execve`, and changes privileged-exec behavior. Seccomp filters are likewise inherited/preserved when fork/clone/exec are allowed.

Therefore adding this mechanism inside ExecSurface's current pre-exec child path would create observable target-state and privilege semantics that do not exist in the current public backend. It is not classified as a transparent native-ptrace optimization.

Kernel contract references reviewed for this boundary:

- Linux `ptrace(2)`: `PTRACE_SYSCALL`, `PTRACE_O_TRACESECCOMP`, `PTRACE_EVENT_SECCOMP`;
- Linux `seccomp(2)` / kernel seccomp-filter documentation: `SECCOMP_RET_TRACE`, filter inheritance and `no_new_privs` requirement;
- Linux `PR_SET_NO_NEW_PRIVS(2const)`: irreversibility, inheritance and privileged-exec behavior.

## Architectural consequence

Within the current public contract, reducing observer-side helper calls can still be useful for narrow workloads, but the accepted evidence does not support another unguided micro-optimization round as the highest-value next step.

The high-leverage cost source is the syscall-stop topology itself. A materially faster architecture therefore needs a separate observation mechanism or an explicitly different target-state contract.

The repository already has such a research mechanism: eBPF. It remains research-only and currently fails its own completeness gates on triggered real workloads. That is the next concrete engineering gap.

## Next gate

Open a separate research-only gate with this order:

1. reproduce the existing eBPF `incomplete_lifecycle` and `incomplete_capability` failures on the pinned trigger workloads;
2. attribute each incompleteness cause to a concrete collector/lifecycle/capability boundary;
3. fix only those boundaries without weakening completeness or promoting backend authority;
4. pass the existing controlled parity/health gates;
5. only then rerun a frozen same-workload performance value experiment;
6. public eBPF exposure, `learn/check`, PASS, auto-selection and baseline interchangeability remain separately prohibited unless future evidence gates explicitly authorize them.

## Authority after this decision

Unchanged:

- public/default backend: **native ptrace**;
- ptrace correctness-reference authority: **RETAINED**;
- Candidate A2: **REJECTED / UNMERGED**;
- Candidate B: **REJECTED FOR INTEGRATION / UNMERGED**;
- seccomp-assisted ptrace in current public backend: **NOT AUTHORIZED**;
- eBPF: **RESEARCH-ONLY**;
- eBPF `learn` / `check`: **NOT AUTHORIZED**;
- eBPF PASS authority: **NOT AUTHORIZED**;
- backend auto-selection: **NOT AUTHORIZED**;
- ptrace/eBPF baseline interchangeability: **NOT AUTHORIZED**.
