# ADR-0001 — M1 Reference Observation Backend

Status: **ACCEPTED FOR M1**

Date: 2026-09-25

## Decision

Use **strace-based ingestion** as the M1 reference observer.

- Native Rust ptrace: defer.
- procfs: augmentation only.
- eBPF-first: defer.

## Decision criteria

M1 prioritizes:
1. reproducibility,
2. rootless operation for a command launched by ExecSurface,
3. ordinary Linux CI usability,
4. inspectable fixtures and failure modes,
5. minimum implementation surface before validating the core baseline/diff abstraction.

## Evidence

Linux ptrace documents `PTRACE_O_TRACEEXEC`, `PTRACE_O_TRACEFORK` and `PTRACE_O_TRACECLONE`, including automatic attachment to descendants under the corresponding options:

https://man7.org/linux/man-pages/man2/ptrace.2.html

Linux kernel BPF documentation exposes multiple tracing mechanisms but also states that tracepoints are not a stable ABI:

https://docs.kernel.org/bpf/bpf_design_QA.html

Kernel probe documentation shows the breadth of dynamic kernel instrumentation:

https://docs.kernel.org/trace/kprobes.html

Falco is evidence that syscall/event streams can support rich runtime process/network rules, but its abstraction is runtime rule evaluation rather than baseline/candidate PR drift:

https://falco.org/docs/

## Backend comparison

| Backend | Strength | Main M1 risk | Decision |
|---|---|---|---|
| strace ingestion | mature syscall decoder; follows descendants; easy-to-record fixtures; rootless-friendly for launched child | textual/version-dependent parsing; ptrace timing perturbation | **SELECT** |
| native ptrace | direct control and typed internal protocol | clone/signal/wait state-machine complexity before product semantics are proven | DEFER |
| procfs | useful process/fd metadata | snapshots are racy and not a complete event stream | AUGMENT ONLY |
| eBPF | broad tracing choices and potential lower perturbation | privileges/capabilities, kernel variance, attachment semantics and CI portability | DEFER |

## M1 raw capture contract

The adapter must record:
- backend name/version,
- trace configuration,
- selected event classes,
- raw record reference,
- parse status,
- target process-tree context,
- loss/truncation state,
- known unsupported semantics.

Unparseable selected-class records cannot be silently discarded.

## Known limitations

- tracing can perturb timing/races;
- trace-aware adversarial software can change behavior;
- observer scope does not equal all possible behavior;
- strace syntax/output varies across versions/architectures;
- hostname intent is not proven by `connect(2)`;
- completeness is conditional on trace configuration and backend health.

These are reportable limitations, not footnotes.

## Killed/deferred

**KILLED:** procfs-only observer.

**KILLED FOR M1 ONLY:** eBPF-first architecture.

This does not reject eBPF long-term. It means eBPF must demonstrate material value in completeness, performance or operability before becoming a supported backend.

## Revisit criteria

Reconsider native ptrace/eBPF after M1/M2 measurements for:
- completeness,
- event loss,
- timing overhead,
- parser stability,
- CI compatibility,
- noise,
- portability.
