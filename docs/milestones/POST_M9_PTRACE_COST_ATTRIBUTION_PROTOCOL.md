# Post-M9 — Ptrace Cost Attribution Protocol

Date: 2026-09-27
Status: **PREREGISTERED — NO OPTIMIZATION AUTHORIZED YET**
Tracking: GitHub issue #84

## Objective

Identify where the current native `ptrace` correctness-reference backend spends observer time on accepted external runtime workloads before proposing any optimization.

This protocol is diagnostic. It does not change runtime semantics, evidence authority, supported scope, verdicts, baselines, policies, privacy, or backend authority.

## Frozen product boundary

- public release remains `v0.1.0-alpha.3`;
- public scope remains Linux x86_64;
- native `ptrace` remains the public correctness-reference backend;
- eBPF remains research-only and is not part of this experiment;
- no existing M9 result may be rewritten or promoted.

## Frozen targets

### PTRACE-COST-RG-001

- repository: `BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- use the same accepted runtime workload shape recorded by M9.1;
- toolchain must satisfy the pinned upstream `rust-version` requirement;
- exact command/wrapper must be copied from the accepted M9.1 evidence before execution and recorded in the resulting evidence file.

### PTRACE-COST-FZF-001

- repository: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- use the corrected executable path grounded in the pinned upstream Makefile;
- exact command/wrapper must be copied from the accepted M9.1 runtime evidence before execution and recorded in the resulting evidence file.

No target substitution after measurements begin.

## Measurement protocol

For each target, on one recorded host/run class:

1. record source HEAD, OS, kernel, architecture and CPU description;
2. reproduce/build the pinned upstream target before accepted measurement;
3. run 3 direct warmups;
4. run 3 ptrace warmups;
5. require all ptrace warmups to report complete observation;
6. collect 15 direct measured samples;
7. collect 15 ptrace measured samples;
8. remove no clean sample post hoc;
9. record median and MAD for direct and ptrace wall time;
10. record median absolute overhead and slowdown ratio;
11. preserve raw samples and all failed/excluded attempts with explicit reasons.

Mode ordering should rotate/interleave where the harness can do so without changing workload semantics, to reduce simple time-order bias. The exact ordering must be frozen in the measurement workflow before accepted samples are collected.

## Attribution counters

Diagnostic instrumentation may count or time internal observer categories only if it does not alter evidence decisions. Candidate counters include:

- total wait/trace-stop observations;
- syscall-entry stops;
- syscall-exit stops;
- ptrace event stops by event class;
- process/thread create/fork/clone/exec/exit lifecycle events;
- counts by currently covered syscall family;
- register-read operations where measurable;
- tracee-memory reads where measurable;
- path-resolution operations;
- fd-state bookkeeping operations;
- userspace canonical raw-event emission count;
- observer loop wall/CPU time separated from target command wall time where technically supportable.

The implementation must not infer counters that cannot be measured reliably. Unknown attribution remains `UNKNOWN` rather than estimated as fact.

## Counter instrumentation boundary

Attribution-only counters must:

- be disabled by default or test/experiment scoped;
- not change which target events are observed;
- not alter ordering/canonicalization semantics;
- not change completeness decisions;
- not change event-budget accounting;
- not persist prohibited payload data;
- not add public product authority;
- not become baseline content;
- not affect policy/verdict logic.

## Attribution decision rule

A cost source may be labeled:

- `MEASURED` only when directly instrumented;
- `SUPPORTED_CORRELATION` when a reproducible association exists but causality is not isolated;
- `OPEN` when attribution is unresolved.

No optimization is authorized solely from `SUPPORTED_CORRELATION`.

## Candidate optimization gate

Before modifying observer behavior, create a separate candidate record containing:

1. measured dominant cost mechanism;
2. exact proposed implementation change;
3. semantic-equivalence argument;
4. adversarial cases that could expose missing evidence;
5. before/after protocol using the same pinned targets and host class;
6. rollback condition.

A candidate is rejected if its speedup depends on dropping authority-relevant observations, weakening completeness, changing baseline semantics, or selective benchmark treatment.

## Required semantic regression gates

Any candidate later proposed must keep existing tests green and add targeted tests for the affected mechanism. At minimum retain:

- descendant exec/process lifecycle;
- syscall entry/exit pairing where required;
- successful-open fd identity;
- fd-attributed read/write effects;
- dup/close/fork/`CLONE_FILES` state;
- openat/openat2 relative/path semantics;
- rename/delete coverage;
- network connect destinations;
- causal executable chain evidence;
- event-budget truncation fail-closed behavior;
- observer fault/incompleteness fail-closed behavior;
- privacy sentinels;
- public Action/distribution behavior.

## Phase A acceptance

Phase A closes only when both frozen external runtime targets have:

- reproducible direct/ptrace raw samples;
- complete ptrace observations;
- machine-readable attribution counters for the counters actually implemented;
- an auditable attribution report distinguishing `MEASURED`, `SUPPORTED_CORRELATION` and `OPEN`;
- no runtime-authority change.

The correct Phase A outcome may be `NO ACTIONABLE DOMINANT COST FOUND`. A speedup is not required to call the experiment scientifically complete.
