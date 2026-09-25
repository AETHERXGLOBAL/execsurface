# M6.5 — Semantic Fidelity & Completeness Hardening Evidence

Date: 2026-09-26

Gate verdict: **ACCEPTED**

Issue: #15

Branch: `milestone/m6.5-semantic-fidelity`

Final pre-close implementation commit:

`a5d454d39e5ad9ad56c4609c69d5b92846d36cc8`

## Objective

Strengthen the evidence-acquisition and canonical-semantics core before M7 external validation.

M6.5 was not a feature-count milestone. Its purpose was to reduce false confidence caused by pathname-only attempts, unstable runtime identities, silent evidence truncation and insufficient causal context.

## Accepted semantics

### FD lifecycle

The Linux ptrace observer now pairs selected syscall entry/exit state and records successful-open fd identity.

Implemented lifecycle handling includes:

- open/openat/openat2/creat;
- read/pread64/readv;
- write/pwrite64/writev;
- sendfile and copy_file_range read/write attribution;
- close and close_range;
- dup/dup2/dup3;
- fcntl F_DUPFD/F_DUPFD_CLOEXEC and FD_CLOEXEC;
- fork/vfork inheritance;
- clone/clone3 CLONE_FILES sharing when flags are evidenced;
- close-on-exec handling.

Actual fd-attributed positive-byte I/O is distinct from pathname-based syscall attempts.

### Path semantics

M6.5 adds:

- trace-time cwd resolution;
- trace-time dirfd resolution through procfs fd links;
- openat2 flags and resolve metadata;
- explicit `KernelFdResolved` canonical resolution;
- narrow self-proc identity normalization for the proven tracee/TGID only.

The Action regression demonstrated why the last rule matters: actual reads of `/proc/<PID>/maps` changed PID between learn/check. The correction maps only the current tracee/TGID to stable self/thread-self identities. Access to another PID remains distinct.

### Causal executable chain

Canonical file and network effects carry a bounded executable chain.

PIDs/TIDs remain excluded from canonical identity.

The chain has a fail-closed bound rather than unbounded evidence growth.

### Completeness

The observer has a finite event budget.

Budget overflow:

- records `event_limit_exceeded`;
- marks `complete=false`;
- causes canonicalization to reject the observation.

Fault injection with an unreadable pathname pointer also marks the observation incomplete.

Observer failure therefore cannot become PASS.

## Versioning

M6.5 changes semantic meaning and therefore versions affected contracts:

- raw observation schema v2;
- canonical surface schema v2;
- normalization profile v2;
- baseline lock schema v2;
- digest format v2;
- diff report v2;
- policy schema v2;
- verdict report v2.

Existing v1 baseline locks are rejected rather than reinterpreted.

Policy v1 remains accepted only for v1-compatible matchers. `file_read` and `file_write` require policy v2.

See `docs/milestones/M6_5_SCHEMA_MIGRATION.md`.

## Final branch gates

### Rust CI

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36188369954

Result: **PASS**

- cargo fmt — PASS;
- strict Clippy — PASS;
- locked tests — PASS;
- Cargo.lock integrity — PASS;
- **78 tests PASS, 0 failed**.

Coverage includes:

- actual fd read/write attribution;
- duplicate/fork/thread fd semantics;
- openat2 metadata;
- high-volume event stress;
- event-budget truncation;
- observer fault injection;
- real shell -> cat causal read chain;
- execution-chain bound;
- v1 lock rejection;
- policy v1/v2 compatibility boundaries;
- prior learn/check/policy/report privacy regressions.

### GitHub Action regression

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36188370146

Result: **PASS**

All six M6 scenarios pass on the v2 core:

- PASS;
- REVIEW;
- REVIEW with `fail-on-review=true`;
- BLOCK;
- ERROR;
- argv privacy evidence.

### Performance evidence

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36188370027

Result: **PASS**

| Workload | Direct median | Observed median | Absolute overhead | Slowdown | Events |
|---|---:|---:|---:|---:|---:|
| `/bin/true` | 0.375 ms | 1.435 ms | 1.060 ms | 3.827× | 6 |
| spawn fixture | 0.956 ms | 4.301 ms | 3.345 ms | 4.499× | 19 |
| 128-file burst | 18.944 ms | 70.513 ms | 51.569 ms | 3.722× | 524 |

No low-overhead claim is made.

See `docs/milestones/M6_5_BACKEND_DECISION.md`.

## Preserved negative evidence

M6.5 preserved failed iterations rather than rewriting history.

Notable failures included:

- strict lint/format failures while propagating v2 models;
- benchmark harness hardening failures before fail-closed behavior was correct;
- final-contract formatting failure;
- Action PASS regression after fd-level reads exposed volatile `/proc/<PID>/maps` identity.

The Action regression was especially valuable: the new semantics were working, but runtime PID identity made two equivalent Bash executions appear different.

The fix was deliberately narrow. Only procfs paths proven to refer to the current tracee/TGID are normalized to self/thread-self forms; arbitrary `/proc/<other-pid>/...` paths remain security-significant.

## Remaining limitations

M6.5 does not yet attribute:

- mmap/memory-mapped file I/O;
- io_uring data access;
- every Linux syscall capable of filesystem I/O;
- causal source-code file/line provenance.

ptrace may perturb scheduling and trace-aware programs.

Linux x86_64 remains the supported observer platform.

## Red-Team conclusion

The gate is accepted because:

1. file evidence now distinguishes path attempts from actual fd I/O;
2. incompleteness is explicitly fail-closed;
3. causal executable context is stronger;
4. schema meaning is versioned rather than silently changed;
5. Action behavior remains intact after the v2 migration;
6. performance cost is measured instead of assumed;
7. eBPF optimization remains evidence-triggered.

## Gate decision

**M6.5 ACCEPTED.**

M7 — External Real-World Proof is next and has not started.
