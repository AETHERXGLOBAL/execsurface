# Post-M9 — Candidate B Tracee-Memory Read Batching Protocol

Date: 2026-09-27
Status: **PREREGISTERED — CANDIDATE B NOT YET AUTHORIZED FOR MERGE**
Parent: GitHub issue #84
Reference source HEAD: `c674d4fd8c55902f3569b49e1168a3dee109381c`
Prior candidate: A2 — `A2_REJECTED_DO_NOT_MERGE`

## Objective

Evaluate whether ExecSurface can reduce native ptrace observer cost by reducing `PTRACE_PEEKDATA` pressure and batching only semantically fixed-size tracee-memory reads, while preserving every existing syscall stop/resume boundary and every current evidence semantic.

Candidate B targets a different measured cost class from A2.

Phase A recorded approximately 4.4k `PTRACE_PEEKDATA` requests on each accepted pinned external diagnostic run:

- ripgrep: `4,438`;
- fzf: `4,396`.

Current `linux_ptrace.rs` reads tracee C strings and fixed-size metadata blocks word-by-word through `PTRACE_PEEKDATA`. Candidate B evaluates a bounded Linux `process_vm_readv(2)` fast path with conservative fallback to the existing word reader.

This protocol does **not** claim that those PEEKDATA calls dominate total observer cost or that the candidate will meet the engineering-value threshold. Those are experimental questions.

## Why Candidate B follows A2

Candidate A2 reduced exit-side `PTRACE_GET_SYSCALL_INFO` activation in its controlled gate, but the frozen external value experiment rejected A2:

- ripgrep established no end-to-end speed improvement;
- fzf exposed a fail-closed unsafe phase state before timing acceptance.

The result shows that reducing one classification request class is not sufficient evidence of practical value.

Candidate B instead targets an independently measured observer request class without changing tracee syscall phase, stop count, resume behavior, seccomp state, privilege state, backend authority, or observed event families.

## Candidate B — bounded mechanism

### Fixed-size metadata reads

For tracee-memory regions whose exact semantic length is already known by current code (`read_memory`):

1. attempt one bounded `process_vm_readv` read for exactly the same requested length;
2. inspect the returned byte count explicitly;
3. accept the fast-path bytes only if the full requested length is returned;
4. on partial read, permission failure, unsupported behavior, ESRCH, or any ambiguity, discard the candidate result and execute the current `PTRACE_PEEKDATA` reader from the original address;
5. never expand the requested address range beyond the existing semantic request.

This may batch several current word-by-word PEEKDATA requests into one read only where the requested metadata length is already fixed by current semantics.

### NUL-terminated C strings — privacy-constrained direct read

A design review identified a privacy boundary that is frozen **before implementation**:

> Candidate B must not accelerate an unknown-length string by speculatively copying an entire page or large chunk beyond the first possible NUL.

The current reference reader obtains one machine word at a time and therefore may inspect only the remainder of that word beyond a NUL. Candidate B must not increase that speculative memory-access footprint merely for performance.

Therefore `read_c_string` may use `process_vm_readv` only in chunks no larger than the current machine-word read width (`size_of::<libc::c_long>()`) and no farther than the current `MAX_PATH_BYTES` semantic limit. For each word-equivalent chunk:

1. request only the same bytes the reference `peek_word` could inspect at that step;
2. require a complete chunk result before consuming it;
3. scan bytes in the same order and stop at the first NUL;
4. on a short/partial/error result, fall back to the current `PTRACE_PEEKDATA` word read for that same address;
5. continue with the same string-length and lossy-UTF-8 behavior as the reference.

This preserves the current maximum speculative string over-read footprint instead of trading privacy for speed.

### Common invariants

Candidate B must preserve the current protocol errors, warnings, completeness behavior, string limit, lossy UTF-8 conversion, sockaddr parsing, open_how/clone metadata interpretation, and privacy boundary.

No target memory is written.

## Linux semantic constraint

`process_vm_readv` may return partial reads when a remote region becomes inaccessible, and remote memory is validated as the read progresses. Every candidate read must therefore inspect the returned byte count; a short read is never treated as complete evidence.

Permission is subject to the Linux ptrace access-mode check. A failure of the candidate read is never evidence that the reference PEEKDATA path would fail; Candidate B therefore falls back rather than weakening observation.

## Explicit non-goals / prohibited shortcuts

Candidate B does not authorize:

- removing or filtering any syscall stop;
- changing `PTRACE_SYSCALL` behavior;
- changing syscall-entry/exit semantics;
- seccomp or `no_new_privs` changes;
- eBPF substitution;
- event sampling or frequency-based syscall suppression;
- changing baseline schema, canonicalization, diff, policy, verdict or report semantics;
- changing backend ID or public backend selection;
- increasing metadata capture or speculative C-string memory range beyond the current word reader;
- argv, env, file-content, stdin or network-payload capture;
- threshold weakening after results are known.

## Implementation isolation

Before merge eligibility, Candidate B must remain experiment-scoped.

Preferred experiment path:

`experiments/post_m9_memory_read_batching/`

The candidate may be applied deterministically to the frozen reference source inside an experiment workflow, matching the governance pattern used for A2. Public runtime source on `main` must not be changed merely to obtain benchmark numbers.

## Semantic equivalence gate

Candidate B becomes eligible for value testing only if reference and candidate have equivalent behavior across existing repository tests plus dedicated adversarial memory-read fixtures.

### Required existing semantics

At minimum preserve:

- open/openat/openat2/creat path capture and success-dependent fd identity;
- unlink/delete and rename path attempts;
- exec path capture;
- clone/clone3 metadata reads;
- connect sockaddr reads and endpoint decoding;
- fd lifecycle and fd-attributed I/O;
- descendant lineage and exec/TID handling;
- event-budget fail-closed behavior;
- completeness/warnings/errors;
- command outcome;
- privacy boundary.

### Mandatory Candidate B adversarial fixtures

The gate must cover at least:

1. valid short C string wholly inside one mapped page;
2. empty C string;
3. valid C string ending exactly at or immediately before a page boundary;
4. valid C string spanning two readable pages;
5. readable first page followed by an inaccessible/unmapped page before NUL;
6. null address;
7. string that reaches `MAX_PATH_BYTES` without NUL;
8. fixed-size read wholly inside one page;
9. fixed-size read spanning two readable pages;
10. fixed-size read where the later page is inaccessible;
11. short/partial `process_vm_readv` result with successful reference fallback where applicable;
12. tracee exit/ESRCH race during metadata read;
13. open_how metadata parity;
14. clone/clone3 flags parity;
15. IPv4/IPv6/Unix or currently-supported connect target parsing parity;
16. rename with two independent tracee strings;
17. fork/exec/thread interleaving around metadata reads;
18. proof that the candidate C-string fast path never requests a larger per-step remote range than the reference machine-word reader.

No fixture may be weakened merely because the candidate fast path cannot handle it.

## Equivalence comparison

For each adversarial fixture:

- reference/reference stability must be established first where ordering is expected deterministic;
- candidate/reference comparison must use existing production canonical/baseline/diff semantics for any legitimately concurrent ordering case;
- raw events, warnings, completeness, command outcome and backend metadata must remain equivalent within the already-authorized comparison method;
- no new normalizer or policy suppression may be introduced to force parity.

Any unresolved evidence mismatch, privacy-footprint expansion, or fail-open behavior kills Candidate B for this gate.

## Activation / attribution gate

Before external value timing, retain observer-side diagnostics that distinguish at minimum:

- `PTRACE_PEEKDATA` request count;
- `process_vm_readv` request count;
- accepted fast-path fixed reads;
- accepted word-equivalent C-string reads;
- fast-path partial/error fallbacks;
- total bytes requested and accepted by the candidate fast path.

Use diagnostics only outside the primary timing samples.

Candidate activation is established only if a controlled fixture proves that:

- candidate `process_vm_readv` is actually exercised; and
- candidate PEEKDATA requests are lower than the reference for the same semantic workload; and
- all semantic outputs remain equivalent; and
- the C-string remote-range invariant is preserved.

This activation result alone is not a performance claim.

## Frozen external value gate

Only after semantic + activation gates pass, use the same pinned external runtime targets used by Post-M9 Phase A and A2:

### PTR-B-VALUE-RG-001

`BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`

Runtime command remains the accepted pinned ripgrep command from the Post-M9 protocol.

### PTR-B-VALUE-FZF-001

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Runtime command remains the accepted pinned fzf command from the Post-M9 protocol.

Per target and mode:

- same GitHub-hosted Ubuntu 24.04 host class;
- reference and candidate in the same job;
- 3 warmups;
- 15 measured samples;
- alternating reference/candidate order;
- no outlier removal;
- no clean-sample replacement;
- all observations must be complete, warning-free and command-exit equivalent;
- production baseline/diff parity must pass before timing is accepted.

Primary metric:

`execsurface observe` end-to-end ptrace observer wall time.

## Predeclared engineering-value threshold

Candidate B establishes external engineering value only if **both** pinned targets independently satisfy:

1. semantic prerequisite PASS;
2. all accepted reference and candidate observations complete and warning-free;
3. candidate median ptrace observer wall time < reference median;
4. median reduction >= `10%`;
5. no sample is removed or replaced after timing is observed.

If semantic gates pass but either target is below `10%`, classify:

`SEMANTICALLY_VALID / VALUE_NOT_ESTABLISHED`

If any semantic gate fails, timing from that target is not accepted as positive value evidence.

The threshold will not be lowered after results are known.

## Merge boundary

Even a value PASS does not automatically merge Candidate B.

A separate integration gate must verify:

- ordinary repository CI;
- current ptrace lifecycle/ESRCH hardening;
- supported Linux x86_64 behavior;
- no regression when `process_vm_readv` is unavailable or denied;
- no dependency or packaging regression;
- no authority change;
- no release/public claim broader than the measured evidence.

Until that integration gate passes:

- public release remains unchanged;
- native ptrace remains the correctness-reference/public backend;
- Candidate B remains experimental;
- eBPF authority remains unchanged.